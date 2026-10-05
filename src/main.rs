use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::ports::Ports;

pub mod bus;
pub mod cpu;
pub mod memory;
pub mod ports;

fn main() {
    let memory = Memory::new();
    let ports = Ports::new();

    let bus = Bus::new(memory, ports);

    let mut cpu = Cpu::new(bus);
    cpu.reset();
}
