use std::fmt::{Debug, Formatter};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TimerMode {
    InterruptOnTerminalCount = 0,
    HardwareRetriggerOneShot = 1,
    RateGenerator = 2,
    SquareWaveGenerator = 3,
    SoftwareTriggeredStrobe = 4,
    HardwareTriggeredStrobe = 5,
}

impl From<u8> for TimerMode {
    #[inline]
    fn from(mut value: u8) -> TimerMode {
        value &= 0b111;
        if (value & 0b010) != 0 {
            value &= 0b011;
        }

        match value {
            0 => Self::InterruptOnTerminalCount,
            1 => Self::HardwareRetriggerOneShot,
            2 => Self::RateGenerator,
            3 => Self::SquareWaveGenerator,
            4 => Self::SoftwareTriggeredStrobe,
            _ => Self::HardwareTriggeredStrobe,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AccessMode {
    Latch = 0b00,
    LowOnly = 0b01,
    HighOnly = 0b10,
    Word = 0b11,
}

#[repr(usize)]
pub enum ChannelRegisterKind {
    Reload = 0,
    CurrentCount = 1,
    Latch = 2,
}

#[derive(Clone, Copy, Default)]
pub struct State {
    pub active: bool,
    pub latched: bool,
    pub read_high_next: bool,
    pub write_high_next: bool,
    pub out: bool,
    output_rising_edge_pending: bool,
    gate: bool,
}

#[derive(Copy, Clone)]
pub struct Channel {
    mode: TimerMode,
    access_mode: AccessMode,
    binary_code_decimal_mode: bool,

    state: State,
    registers: [u16; 3],

    counting_period: u32,
    remaining_ticks: u32,
    updated_at: u32,
}

impl Channel {
    #[inline]
    pub fn get_register(&self, register_kind: ChannelRegisterKind) -> u16 {
        self.registers[register_kind as usize]
    }

    #[inline]
    fn set_register(&mut self, register_kind: ChannelRegisterKind, register_value: u16) {
        self.registers[register_kind as usize] = register_value;
    }

    #[inline(always)]
    pub fn effective_reload(&self) -> u32 {
        let reload_value = u32::from(self.get_register(ChannelRegisterKind::Reload));
        if reload_value == 0 {
            65536
        } else {
            reload_value
        }
    }

    fn configure(&mut self, control_word: u8) {
        let access_bits = (control_word >> 4) & 3;
        if access_bits == 0 {
            self.latch_current_count();
            return;
        }

        self.mode = TimerMode::from(control_word >> 1);
        self.access_mode = match access_bits {
            1 => AccessMode::LowOnly,
            2 => AccessMode::HighOnly,
            _ => AccessMode::Word,
        };

        self.binary_code_decimal_mode = control_word & 1 != 0;

        self.set_register(ChannelRegisterKind::Reload, 0);

        self.counting_period = 0;

        self.state = State {
            out: self.state.out,
            output_rising_edge_pending: self.state.output_rising_edge_pending,
            gate: self.state.gate,
            ..State::default()
        };

        let output_level = !matches!(self.mode, TimerMode::InterruptOnTerminalCount);
        self.set_output_level(output_level);
    }

    fn latch_current_count(&mut self) {
        if !self.state.latched {
            let current_count = self.get_register(ChannelRegisterKind::CurrentCount);
            self.set_register(ChannelRegisterKind::Latch, current_count);

            self.state.latched = true;
        }
    }

    pub fn read(&mut self) -> u8 {
        let register_kind = if self.state.latched {
            ChannelRegisterKind::Latch
        } else {
            ChannelRegisterKind::CurrentCount
        };
        let register_value = self.get_register(register_kind);

        match self.access_mode {
            AccessMode::Word => self.read_next_word_byte(register_value),
            AccessMode::LowOnly => {
                self.state.latched = false;
                register_value as u8
            }
            AccessMode::HighOnly => {
                self.state.latched = false;
                (register_value >> 8) as u8
            }
            AccessMode::Latch => unreachable!("latching does not change the access mode"),
        }
    }

    fn read_next_word_byte(&mut self, register_value: u16) -> u8 {
        let reading_high_byte = self.state.read_high_next;
        self.state.read_high_next = !reading_high_byte;

        if reading_high_byte {
            self.state.latched = false;
            (register_value >> 8) as u8
        } else {
            register_value as u8
        }
    }

    pub fn write(&mut self, byte_value: u8) {
        match self.access_mode {
            AccessMode::LowOnly => self.load_counter(u16::from(byte_value)),
            AccessMode::HighOnly => self.load_counter(u16::from(byte_value) << 8),
            AccessMode::Word => self.write_next_word_byte(byte_value),
            AccessMode::Latch => unreachable!("latching does not change the access mode"),
        }
    }

    fn write_next_word_byte(&mut self, byte_value: u8) {
        let reload_value = self.get_register(ChannelRegisterKind::Reload);
        let writing_high_byte = self.state.write_high_next;
        self.state.write_high_next = !writing_high_byte;

        if writing_high_byte {
            self.load_counter((reload_value & 0x00ff) | (u16::from(byte_value) << 8));
        } else {
            self.set_register(
                ChannelRegisterKind::Reload,
                (reload_value & 0xff00) | u16::from(byte_value),
            );
            if matches!(self.mode, TimerMode::InterruptOnTerminalCount) {
                self.state.active = false;
                self.set_output_level(false);
            }
        }
    }

    fn load_counter(&mut self, reload_value: u16) {
        self.set_register(ChannelRegisterKind::Reload, reload_value);

        self.counting_period = self.effective_reload();
        self.reload_current_count();

        let output_level = !matches!(self.mode, TimerMode::InterruptOnTerminalCount);
        self.set_output_level(output_level);

        self.state.active = self.supports_counting();
    }

    fn reload_current_count(&mut self) {
        self.remaining_ticks = self.counting_period;
        self.set_register(
            ChannelRegisterKind::CurrentCount,
            self.remaining_ticks as u16,
        );
    }

    fn supports_counting(&self) -> bool {
        if self.binary_code_decimal_mode {
            return false;
        }

        match self.mode {
            TimerMode::InterruptOnTerminalCount => true,
            TimerMode::RateGenerator | TimerMode::SquareWaveGenerator => self.counting_period >= 2,
            _ => false,
        }
    }

    fn set_gate(&mut self, enable: bool) {
        let rising_edge = enable && !self.state.gate;
        self.state.gate = enable;

        if matches!(
            self.mode,
            TimerMode::RateGenerator | TimerMode::SquareWaveGenerator
        ) {
            if !enable {
                self.set_output_level(true);
            }

            if rising_edge && self.counting_period != 0 {
                self.reload_current_count();
                self.set_output_level(true);
            }
        }
    }

    fn set_output_level(&mut self, output_level: bool) {
        self.state.output_rising_edge_pending |= !self.state.out && output_level;
        self.state.out = output_level;
    }

    fn take_output_rising_edge(&mut self) -> bool {
        std::mem::take(&mut self.state.output_rising_edge_pending)
    }

    fn ticks_until_output_rising_edge(&self) -> Option<u32> {
        if !self.state.active || !self.state.gate {
            return None;
        }

        match self.mode {
            TimerMode::InterruptOnTerminalCount if !self.state.out => Some(self.remaining_ticks),
            TimerMode::RateGenerator | TimerMode::SquareWaveGenerator => Some(self.remaining_ticks),
            _ => None,
        }
    }

    fn update_to(&mut self, ticks: u32) {
        let elapsed_ticks = ticks - self.updated_at;
        self.updated_at = ticks;
        self.advance_ticks(elapsed_ticks);
    }

    fn output_level_after(&self, elapsed_ticks: u32) -> bool {
        if !self.state.active || !self.state.gate || elapsed_ticks == 0 {
            return self.state.out;
        }

        match self.mode {
            TimerMode::InterruptOnTerminalCount => {
                self.state.out || elapsed_ticks >= self.remaining_ticks
            }
            TimerMode::RateGenerator => self.periodic_remaining_after(elapsed_ticks) != 1,
            TimerMode::SquareWaveGenerator => {
                self.periodic_remaining_after(elapsed_ticks) > self.counting_period / 2
            }
            _ => self.state.out,
        }
    }

    fn ticks_until_output_rising_edge_after(&self, elapsed_ticks: u32) -> Option<u32> {
        let remaining_ticks = self.ticks_until_output_rising_edge()?;
        if elapsed_ticks < remaining_ticks {
            return Some(remaining_ticks - elapsed_ticks);
        }

        match self.mode {
            TimerMode::RateGenerator | TimerMode::SquareWaveGenerator => {
                Some(self.periodic_remaining_after(elapsed_ticks))
            }
            _ => None,
        }
    }

    fn advance_ticks(&mut self, elapsed_ticks: u32) {
        if elapsed_ticks == 0 || !self.state.active || !self.state.gate {
            return;
        }

        match self.mode {
            TimerMode::InterruptOnTerminalCount => self.advance_terminal_count(elapsed_ticks),
            TimerMode::RateGenerator | TimerMode::SquareWaveGenerator => {
                self.advance_periodic_count(elapsed_ticks)
            }
            _ => {}
        }

        self.update_current_count();
    }

    fn advance_terminal_count(&mut self, elapsed_ticks: u32) {
        if self.state.out {
            self.remaining_ticks =
                (self.remaining_ticks as u16).wrapping_sub(elapsed_ticks as u16) as u32;
            return;
        }

        if elapsed_ticks < self.remaining_ticks {
            self.remaining_ticks -= elapsed_ticks;
            return;
        }

        let ticks_after_terminal_count = elapsed_ticks - self.remaining_ticks;
        self.remaining_ticks = 0u16.wrapping_sub(ticks_after_terminal_count as u16) as u32;

        self.set_output_level(true);
    }

    fn advance_periodic_count(&mut self, elapsed_ticks: u32) {
        self.state.output_rising_edge_pending |= elapsed_ticks >= self.remaining_ticks;
        self.remaining_ticks = self.periodic_remaining_after(elapsed_ticks);

        let output_level = match self.mode {
            TimerMode::RateGenerator => self.remaining_ticks != 1,
            _ => self.remaining_ticks > self.counting_period / 2,
        };

        self.set_output_level(output_level);
    }

    fn periodic_remaining_after(&self, elapsed_ticks: u32) -> u32 {
        if elapsed_ticks < self.remaining_ticks {
            self.remaining_ticks - elapsed_ticks
        } else {
            self.counting_period - (elapsed_ticks - self.remaining_ticks) % self.counting_period
        }
    }

    fn update_current_count(&mut self) {
        let current_count = if matches!(self.mode, TimerMode::SquareWaveGenerator) {
            self.square_wave_count()
        } else {
            self.remaining_ticks as u16
        };

        self.set_register(ChannelRegisterKind::CurrentCount, current_count);
    }

    fn square_wave_count(&self) -> u16 {
        let elapsed_ticks = self.counting_period - self.remaining_ticks;
        let high_phase_ticks = self.counting_period.div_ceil(2);
        let odd_period_adjustment = self.counting_period & 1;

        let current_count = if elapsed_ticks < high_phase_ticks {
            if elapsed_ticks == 0 {
                self.counting_period
            } else {
                self.counting_period - 2 * elapsed_ticks + odd_period_adjustment
            }
        } else {
            let low_phase_elapsed_ticks = elapsed_ticks - high_phase_ticks;
            if low_phase_elapsed_ticks == 0 {
                self.counting_period
            } else {
                self.counting_period - 2 * low_phase_elapsed_ticks - odd_period_adjustment
            }
        };

        current_count as u16
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            mode: TimerMode::RateGenerator,
            access_mode: AccessMode::Word,
            binary_code_decimal_mode: false,
            state: State::default(),
            registers: [0; 3],
            counting_period: 0,
            remaining_ticks: 0,
            updated_at: 0,
        }
    }
}

pub struct ProgrammableIntervalTimer {
    channels: [Channel; 3],
    ticks: u32,
}

impl Debug for ProgrammableIntervalTimer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "ProgrammableIntervalTimer")
    }
}

impl ProgrammableIntervalTimer {
    pub fn new() -> Self {
        let mut channels = [Channel::default(); 3];
        channels[0].state.gate = true;
        channels[1].state.gate = true;
        channels[2].state.gate = false;

        Self { channels, ticks: 0 }
    }

    pub fn read(&mut self, port_address: u16) -> u8 {
        match port_address {
            0x40..=0x42 => {
                let channel = &mut self.channels[(port_address - 0x40) as usize];
                if !channel.state.latched {
                    channel.update_to(self.ticks);
                }
                channel.read()
            }
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, port_address: u16, value: u8) {
        match port_address {
            0x40..=0x42 => {
                let channel = &mut self.channels[(port_address - 0x40) as usize];
                channel.update_to(self.ticks);
                channel.write(value);
            }
            0x43 => self.write_control(value),
            _ => unreachable!(),
        }
    }

    fn write_control(&mut self, value: u8) {
        let channel_index = usize::from(value >> 6);
        if let Some(channel) = self.channels.get_mut(channel_index) {
            if value & 0x30 == 0 && channel.state.latched {
                return;
            }
            channel.update_to(self.ticks);
            channel.configure(value);
        }
    }

    pub fn output_level(&self, channel_index: usize) -> bool {
        let channel = &self.channels[channel_index];
        channel.output_level_after(self.ticks - channel.updated_at)
    }

    pub fn set_gate(&mut self, channel_index: usize, enabled: bool) {
        let channel = &mut self.channels[channel_index];
        if channel.state.gate != enabled {
            channel.update_to(self.ticks);
            channel.set_gate(enabled);
        }
    }

    pub fn take_output_rising_edge(&mut self, channel_index: usize) -> bool {
        let channel = &mut self.channels[channel_index];
        if channel
            .ticks_until_output_rising_edge()
            .is_some_and(|remaining| self.ticks - channel.updated_at >= remaining)
        {
            channel.update_to(self.ticks);
        }
        channel.take_output_rising_edge()
    }

    pub fn ticks_until_output_rising_edge(&self, channel_index: usize) -> Option<u32> {
        let channel = &self.channels[channel_index];
        channel.ticks_until_output_rising_edge_after(self.ticks - channel.updated_at)
    }

    pub fn advance_ticks(&mut self, elapsed_ticks: u32) {
        if let Some(total) = self.ticks.checked_add(elapsed_ticks) {
            self.ticks = total;
        } else {
            self.rebase_ticks();
            self.ticks = elapsed_ticks;
        }
    }

    fn rebase_ticks(&mut self) {
        for channel in &mut self.channels {
            channel.update_to(self.ticks);
            channel.updated_at = 0;
        }
    }
}
