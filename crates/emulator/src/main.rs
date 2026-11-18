use emulator_core::bus::Bus;
use emulator_core::controller::interrupt::ProgrammableInterruptController;
use emulator_core::cpu::Cpu;
use emulator_core::machine::Machine;
use emulator_core::memory::Memory;
use emulator_core::video::{AdapterKind, VideoAdapter};

fn main() {
    let memory = Memory::new();
    let pic = ProgrammableInterruptController::new();
    let video_adapter = VideoAdapter::new(AdapterKind::Monochrome);

    let bus = Bus::new(memory, pic, video_adapter);
    let cpu = Cpu::new();

    let mut machine = Machine::new(cpu, bus);
    machine.initialize();

    loop {
        machine.step();
    }
}
