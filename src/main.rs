use crate::{cpu::Cpu, memory::Memory};

pub mod cpu;
pub mod memory;

fn main() {
    let mut memory = Memory::new();

    memory.write_byte(0, 0xB8);
    memory.write_word(1, 0x1234);

    let mut cpu = Cpu::new(memory);

    cpu.step();
    println!("{:#?}", cpu);
}
