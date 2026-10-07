use emulator::{bus::Bus, cpu::Cpu, memory::Memory, ports::Ports};

fn main() {
    let memory = Memory::new();
    let ports = Ports::new();

    let bus = Bus::new(memory, ports);

    let mut cpu = Cpu::new(bus);
    cpu.reset();
}
