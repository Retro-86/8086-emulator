use std::fmt::{Debug, Formatter};

static MEMORY_SIZE: usize = 1048576;

pub struct Memory {
    values: Vec<u8>,
}

impl Debug for Memory {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("size = {}", MEMORY_SIZE))
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            values: vec![0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.values[address as usize & 0xFFFFF]
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        self.values[address as usize & 0xFFFFF] = value;
    }

    pub fn read_word(&self, address: u32) -> u16 {
        let low_byte = self.read_byte(address);
        let high_byte = self.read_byte(address + 1);

        u16::from_le_bytes([low_byte, high_byte])
    }

    pub fn write_word(&mut self, address: u32, value: u16) {
        let low_byte_address = address;
        let high_byte_address = address + 1;

        let bytes = value.to_le_bytes();

        self.write_byte(low_byte_address, bytes[0]);
        self.write_byte(high_byte_address, bytes[1]);
    }
}
