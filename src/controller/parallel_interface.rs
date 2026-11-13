const RESET_MODE: u8 = 0x9B;

#[derive(Copy, Clone, Debug)]
struct Channel {
    output: u8,
    input: u8,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            output: 0,
            input: 0xFF,
        }
    }
}

impl Channel {
    pub fn reset(&mut self) {
        self.output = 0;
    }
}

#[derive(Debug)]
pub struct ParallelInterfaceController {
    channels: [Channel; 3],
    mode: u8,
}

impl ParallelInterfaceController {
    pub fn new() -> Self {
        Self {
            channels: [Channel::default(); 3],
            mode: RESET_MODE,
        }
    }

    pub fn reset(&mut self) {
        self.reset_channels();
        self.mode = RESET_MODE;
    }

    fn reset_channels(&mut self) {
        self.channels.iter_mut().for_each(|channel| channel.reset());
    }

    fn lines(&self, channel_index: u8, mask: u8) -> u8 {
        let channel = &self.channels[channel_index as usize];
        (channel.output & !mask) | (channel.input & mask)
    }

    pub fn channel_mask(&self, channel_index: u8) -> u8 {
        match channel_index {
            0 => {
                if self.mode & 0x10 != 0 {
                    0xFF
                } else {
                    0x00
                }
            }
            1 => {
                if self.mode & 0x02 != 0 {
                    0xFF
                } else {
                    0x00
                }
            }
            2 => {
                let upper = if self.mode & 0x08 != 0 { 0xF0 } else { 0x00 };
                let lower = if self.mode & 0x01 != 0 { 0x0F } else { 0x00 };

                upper | lower
            }
            _ => 0xFF,
        }
    }

    pub fn read(&mut self, register: u8) -> u8 {
        let channel_index = register & 0x03;
        let channel_mask = self.channel_mask(channel_index);

        self.lines(channel_index, channel_mask)
    }

    pub fn write(&mut self, register: u8, value: u8) {
        let channel_index = register & 0x03;

        match channel_index {
            0..=2 => self.channels[channel_index as usize].output = value,
            _ => {
                if value & 0x80 != 0 {
                    self.mode = value;
                    self.reset_channels();
                } else {
                    let bit = 1 << ((value >> 1) & 0x07);
                    if value & 0x01 != 0 {
                        self.channels[2].output |= bit;
                    } else {
                        self.channels[2].output &= !bit;
                    }
                }
            }
        }
    }
}
