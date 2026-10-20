use emulator::controller::interrupt::ProgrammableInterruptController;
use emulator::{bus::Bus, cpu::Cpu, memory::Memory};
use emulator::machine::Machine;

fn main() {
    let memory = Memory::new();
    let pic = ProgrammableInterruptController::new();

    let bus = Bus::new(memory, pic);
    let mut cpu = Cpu::new();

    cpu.reset();

    let _machine = Machine::new(cpu, bus);
}
