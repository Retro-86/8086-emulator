use std::io::Read;
use std::sync::mpsc;
use std::{io, thread};
use std::path::Path;
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

    let mut bus = Bus::new(memory, pic, video_adapter);
    bus.insert_floppy_image(Path::new("resources/boot.img")).unwrap();

    let cpu = Cpu::new();

    let mut machine = Machine::new(cpu, bus);

    let bios_image_path = Path::new("resources/bios.bin");
    machine.initialize(bios_image_path);

    let (sender, receiver) = mpsc::channel::<u8>();

    thread::spawn(move || {
        for byte in io::stdin().lock().bytes().flatten() {
            if sender.send(byte).is_err() {
                break;
            }
        }
    });

    loop {
        while let Ok(byte) = receiver.try_recv() {
            if let Some(scan_code) = ascii_to_scan_code(byte) {
                machine.push_scan_code(scan_code);
                machine.push_scan_code(scan_code | 0x80);
            }
        }

        machine.step();
    }
}

fn ascii_to_scan_code(byte: u8) -> Option<u8> {
    Some(match byte.to_ascii_lowercase() {
        b'`' | b'~' => 0x29,
        b'1' | b'!' => 0x02,
        b'2' | b'@' => 0x03,
        b'3' | b'#' => 0x04,
        b'4' | b'$' => 0x05,
        b'5' | b'%' => 0x06,
        b'6' | b'^' => 0x07,
        b'7' | b'&' => 0x08,
        b'8' | b'*' => 0x09,
        b'9' | b'(' => 0x0A,
        b'0' | b')' => 0x0B,
        b'-' | b'_' => 0x0C,
        b'=' | b'+' => 0x0D,

        b'q' => 0x10,
        b'w' => 0x11,
        b'e' => 0x12,
        b'r' => 0x13,
        b't' => 0x14,
        b'y' => 0x15,
        b'u' => 0x16,
        b'i' => 0x17,
        b'o' => 0x18,
        b'p' => 0x19,
        b'[' | b'{' => 0x1A,
        b']' | b'}' => 0x1B,

        b'a' => 0x1E,
        b's' => 0x1F,
        b'd' => 0x20,
        b'f' => 0x21,
        b'g' => 0x22,
        b'h' => 0x23,
        b'j' => 0x24,
        b'k' => 0x25,
        b'l' => 0x26,
        b';' | b':' => 0x27,
        b'\'' | b'"' => 0x28,
        b'\\' | b'|' => 0x2B,

        b'z' => 0x2C,
        b'x' => 0x2D,
        b'c' => 0x2E,
        b'v' => 0x2F,
        b'b' => 0x30,
        b'n' => 0x31,
        b'm' => 0x32,
        b',' | b'<' => 0x33,
        b'.' | b'>' => 0x34,
        b'/' | b'?' => 0x35,

        0x1B => 0x01,
        b'\t' => 0x0F,
        b'\r' | b'\n' => 0x1C,
        0x7F | 0x08 => 0x0E,
        b' ' => 0x39,

        _ => return None,
    })
}
