use emulator::controller::interrupt::ProgrammableInterruptController;
use emulator::machine::Machine;
use emulator::video::{AdapterKind, VideoAdapter};
use emulator::{bus::Bus, cpu::Cpu, memory::Memory};

fn main() {
    let memory = Memory::new();
    let pic = ProgrammableInterruptController::new();
    let video_adapter = VideoAdapter::new(AdapterKind::Monochrome);

    let bus = Bus::new(memory, pic, video_adapter);

    let mut cpu = Cpu::new();
    cpu.reset();

    let _machine = Machine::new(cpu, bus);
}
