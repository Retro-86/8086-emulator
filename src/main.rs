use crate::{cpu::Cpu, memory::Memory};

pub mod cpu;
pub mod memory;

fn main() {
    let mut memory = Memory::new();

    memory.write_byte(0, 0xB8);
    memory.write_word(1, 0x1234);
    memory.write_byte(3, 0xB9);
    memory.write_word(4, 0x44ff);

    let mut cpu = Cpu::new(memory);

    for _ in 0..=10 {
        cpu.step();
    }
    println!("{:#?}", cpu);
}
