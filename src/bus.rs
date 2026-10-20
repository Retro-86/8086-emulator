use crate::controller::interrupt::ProgrammableInterruptController;
use crate::memory::Memory;

pub enum AddressSpace {
    Memory,
    Port,
}

#[derive(Debug)]
pub struct Bus {
    memory: Memory,
    pic: ProgrammableInterruptController,
}

impl Bus {
    pub fn new(memory: Memory, pic: ProgrammableInterruptController) -> Self {
        Self { memory, pic }
    }

    pub fn request_interrupt(&mut self, line: u8) {
        self.pic.request(line);
    }

    pub fn acknowledge_interrupt(&mut self) -> Option<u8> {
        self.pic.acknowledge()
    }

    pub fn read_byte_from_port(&self, port: u16) -> u8 {
        match port {
            0x20 | 0x21 => self.pic.read(port),
            _ => unimplemented!(),
        }
    }

    pub fn read_byte_from_memory(&self, address: u32) -> u8 {
        self.memory.read_byte(address)
    }

    pub fn read_word_from_port(&self, port: u16) -> u16 {
        todo!()
    }

    pub fn read_word_from_memory(&self, address: u32) -> u16 {
        self.memory.read_word(address)
    }

    pub fn write_byte_to_port(&mut self, port: u16, value: u8) {
        match port {
            0x20 | 0x21 => self.pic.write(port, value),
            _ => unimplemented!(),
        }
    }

    pub fn write_byte_to_memory(&mut self, address: u32, value: u8) {
        self.memory.write_byte(address, value);
    }

    pub fn write_word_to_port(&mut self, port: u16, value: u16) {
        todo!()
    }

    pub fn write_word_to_memory(&mut self, address: u32, value: u16) {
        self.memory.write_word(address, value);
    }
}
