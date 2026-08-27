use crate::{cpu::Cpu, memory::Memory};

pub mod cpu;
pub mod memory;

fn main() {
    let memory = Memory::new();
    let mut cpu = Cpu::new(memory);

    cpu.execute(0xFF);
}
