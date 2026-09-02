use crate::{cpu::Cpu, memory::Memory};
use crate::bus::Bus;
use crate::ports::Ports;

pub mod cpu;
pub mod memory;
pub mod bus;
pub mod ports;

fn main() {
    let mut memory = Memory::new();

    memory.write_byte(0, 0xB8);
    memory.write_word(1, 0x1234);
    memory.write_byte(3, 0x50);
    memory.write_byte(4, 0x5A);
    memory.write_byte(5, 0x89);
    memory.write_byte(6, 0x16);
    memory.write_word(7, 0x5678);
    memory.write_byte(9, 0xF4);

    println!("{:x}", memory.read_word(7));

    let ports = Ports::new();
    
    let bus = Bus::new(memory, ports);

    let mut cpu = Cpu::new(bus);

    for _ in 0..=100 {
        cpu.step();
    }
    println!("{:#?}", cpu);
}
