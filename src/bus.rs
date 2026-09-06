use crate::memory::Memory;
use crate::ports::Ports;

pub enum AddressSpace {
    Memory,
    Port,
}

#[derive(Debug)]
pub struct Bus {
    memory: Memory,
    ports: Ports,
}

impl Bus {
    pub fn new(memory: Memory, ports: Ports) -> Self {
        Self {
            memory,
            ports,
        }
    }

    pub fn read_byte_from_port(&self, port: u16) -> u8 {
        self.ports.read_byte(port)
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
        self.ports.write_byte(port, value);
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
