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
    memory.write_byte(3, 0xB9);
    memory.write_word(4, 0x44ff);
    
    let ports = Ports::new();
    
    let bus = Bus::new(memory, ports);

    let mut cpu = Cpu::new(bus);

    for _ in 0..=10 {
        cpu.step();
    }
    println!("{:#?}", cpu);
}
