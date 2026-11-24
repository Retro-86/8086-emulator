mod keyboard;

use emulator_core::bus::Bus;
use emulator_core::controller::interrupt::ProgrammableInterruptController;
use emulator_core::cpu::Cpu;
use emulator_core::machine::Machine;
use emulator_core::memory::Memory;
use emulator_core::video::{AdapterKind, VideoAdapter};
use std::path::Path;

fn main() {
    let memory = Memory::new();
    let pic = ProgrammableInterruptController::new();
    let video_adapter = VideoAdapter::new(AdapterKind::Monochrome);

    let mut bus = Bus::new(memory, pic, video_adapter);
    bus.insert_floppy_image(Path::new("resources/boot.img"))
        .unwrap();

    let cpu = Cpu::new();

    let mut machine = Machine::new(cpu, bus);

    let bios_image_path = Path::new("resources/bios.bin");
    machine.initialize(bios_image_path);

    let scan_codes = keyboard::read_scan_codes();
    loop {
        for scan_code in scan_codes.try_iter().flatten() {
            machine.push_scan_code(scan_code);
        }

        machine.step();
    }
}
