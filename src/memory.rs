static MEMORY_SIZE: usize = 1048576;

pub struct Memory {
    values: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            values: [0; MEMORY_SIZE],
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
        let high_byte_adress = (address + 1);

        let bytes = value.to_le_bytes();

        self.write_byte(low_byte_address, bytes[0]);
        self.write_byte(high_byte_adress, bytes[1]);
    }
}
