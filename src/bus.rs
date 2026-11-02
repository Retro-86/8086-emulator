use crate::controller::interrupt::ProgrammableInterruptController;
use crate::controller::serial::Serial;
use crate::controller::timer::ProgrammableIntervalTimer;
use crate::memory::Memory;
use crate::port::Ports;
use crate::video::VideoAdapter;

#[derive(Debug)]
pub struct Bus {
    memory: Memory,
    ports: Ports,
    pic: ProgrammableInterruptController,
    video_adapter: VideoAdapter,
    serial: Serial,
    timer: ProgrammableIntervalTimer,
}

impl Bus {
    pub fn new(
        memory: Memory,
        pic: ProgrammableInterruptController,
        video_adapter: VideoAdapter,
    ) -> Self {
        let mut ports = Ports::new();

        ports.register_byte_reader(&[0x20, 0x21], |bus, port_address| {
            bus.pic.read(port_address)
        });

        ports.register_byte_writer(&[0x20, 0x21], |bus, port_address, value| {
            bus.pic.write(port_address, value);
        });

        ports.register_byte_writer(&[0x3B4, 0x3B5, 0x3B8], |bus, port, value| {
            bus.video_adapter.write(port, value);
        });

        ports.register_byte_reader(&[0x3B5, 0x3BA], |bus, port| bus.video_adapter.read(port));

        ports.register_byte_writer(
            &[0x3F8, 0x3F9, 0x3FA, 0x3FB, 0x3FC, 0x3FD, 0x3FE, 0x3FF],
            |bus, _port, value| {
                bus.serial.write(value);
            },
        );

        ports.register_byte_reader(&[0x40, 0x41, 0x42], |bus, port| bus.timer.read(port));

        ports.register_byte_writer(
            &[0x40, 0x41, 0x42, 0x43],
            |bus, port_address, byte_value| {
                bus.timer.write(port_address, byte_value);
                bus.timer_interrupt();
            },
        );

        Self {
            memory,
            ports,
            pic,
            video_adapter,
            serial: Serial::new(),
            timer: ProgrammableIntervalTimer::new(),
        }
    }

    pub fn request_interrupt(&mut self, line: u8) {
        self.pic.request(line);
    }

    pub fn advance_timer_ticks(&mut self, elapsed_ticks: u32) {
        self.timer.advance_ticks(elapsed_ticks);
        self.timer_interrupt();
    }

    pub fn timer(&self) -> &ProgrammableIntervalTimer {
        &self.timer
    }

    pub fn interrupt_controller(&self) -> &ProgrammableInterruptController {
        &self.pic
    }

    fn timer_interrupt(&mut self) {
        if self.timer.take_output_rising_edge(0) {
            self.request_interrupt(0);
        }
    }

    pub fn acknowledge_interrupt(&mut self) -> Option<u8> {
        self.pic.acknowledge()
    }

    pub fn read_byte_from_port(&mut self, port: u16) -> u8 {
        match self.ports.byte_read_handler(port) {
            Some(reader) => reader(self, port),
            _ => panic!("unmapped byte read from port {port:#x}"),
        }
    }

    pub fn read_byte_from_memory(&self, address: u32) -> u8 {
        self.memory.read_byte(address)
    }

    pub fn read_word_from_port(&mut self, port: u16) -> u16 {
        let low = self.read_byte_from_port(port) as u16;
        let high = self.read_byte_from_port(port.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn read_word_from_memory(&self, address: u32) -> u16 {
        self.memory.read_word(address)
    }

    pub fn write_byte_to_port(&mut self, port: u16, value: u8) {
        if let Some(writer) = self.ports.byte_write_handler(port) {
            writer(self, port, value);
        }
    }

    pub fn write_byte_to_memory(&mut self, address: u32, value: u8) {
        self.memory.write_byte(address, value);
    }

    pub fn write_word_to_port(&mut self, port: u16, value: u16) {
        self.write_byte_to_port(port, value as u8);
        self.write_byte_to_port(port.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn write_word_to_memory(&mut self, address: u32, value: u16) {
        self.memory.write_word(address, value);
    }
}
