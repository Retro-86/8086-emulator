use emulator_core::video::{AdapterKind, VideoAdapter};
use emulator_core::{
    bus::Bus,
    controller::interrupt::ProgrammableInterruptController as Pic,
    cpu::{
        register::{PointerRegisterKind, WordSharedRegisterKind},
        Cpu,
    },
    machine::Machine,
    memory::Memory,
};

fn write_bytes(memory: &mut Memory, address: u32, bytes: &[u8]) {
    for (offset, &byte) in bytes.iter().enumerate() {
        memory.write_byte(address + offset as u32, byte);
    }
}

fn write_reset_jump(memory: &mut Memory) {
    write_bytes(memory, 0xFFFF0, &[0xEA, 0x00, 0x01, 0x00, 0x00]);
}

fn ax(machine: &Machine) -> u16 {
    machine
        .cpu()
        .get_registers()
        .0
        .get_word(WordSharedRegisterKind::AX)
}

#[test]
fn software_interrupt_and_restore_state() {
    let mut memory = Memory::new();
    write_reset_jump(&mut memory);

    memory.write_word(0x30 * 4, 0x0200);
    memory.write_word(0x30 * 4 + 2, 0);

    write_bytes(&mut memory, 0x0100, &[0xFB, 0xCD, 0x30, 0xF4]);
    write_bytes(&mut memory, 0x0200, &[0xB8, 0x34, 0x12, 0xCF]);

    let mut cpu = Cpu::new();
    cpu.reset();

    let bus = Bus::new(
        memory,
        Pic::new(),
        VideoAdapter::new(AdapterKind::Monochrome),
    );

    let mut machine = Machine::new(cpu, bus);

    machine.step();
    machine.step();
    machine.step();
    assert!(!machine.cpu().can_accept_interrupt());

    machine.step();
    machine.step();

    assert_eq!(ax(&machine), 0x1234);
    assert!(machine.cpu().can_accept_interrupt());
    assert_eq!(
        machine
            .cpu()
            .get_registers()
            .2
            .get(PointerRegisterKind::SP),
        0
    );

    machine.step();
    assert!(machine.cpu().halted());
}

#[test]
fn priority_and_end_of_interrupt() {
    let mut pic = Pic::new();

    pic.write(0x20, 0x13);
    pic.write(0x21, 0x20);
    pic.write(0x21, 0x01);
    pic.write(0x21, 0xFC);

    pic.request(1);
    pic.request(0);

    assert_eq!(pic.acknowledge(), Some(0x20));
    assert_eq!(pic.acknowledge(), None);

    pic.write(0x20, 0x20);
    assert_eq!(pic.acknowledge(), Some(0x21));
}

#[test]
fn hardware_interrupt_reaches_guest_handler() {
    let mut memory = Memory::new();
    write_reset_jump(&mut memory);

    memory.write_word(0x21 * 4, 0x0200);
    memory.write_word(0x21 * 4 + 2, 0);

    write_bytes(&mut memory, 0x0100, &[0xFB, 0x90, 0xF4]);
    write_bytes(&mut memory, 0x0200, &[0xB8, 0xFE, 0xCA, 0xCF]);

    let mut bus = Bus::new(
        memory,
        Pic::new(),
        VideoAdapter::new(AdapterKind::Monochrome),
    );

    bus.write_byte_to_port(0x20, 0x13);
    bus.write_byte_to_port(0x21, 0x20);
    bus.write_byte_to_port(0x21, 0x01);
    bus.write_byte_to_port(0x21, 0xFD);

    bus.request_interrupt(1);

    let mut cpu = Cpu::new();
    cpu.reset();

    let mut machine = Machine::new(cpu, bus);

    for _ in 0..4 {
        machine.step();
    }

    assert_eq!(ax(&machine), 0xCAFE);
}
