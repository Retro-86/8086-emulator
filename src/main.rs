use crate::cpu::Cpu;

pub mod cpu;
pub mod memory;

fn main() {
    let mut cpu = Cpu::new();

    cpu.execute(0xFF);
}
