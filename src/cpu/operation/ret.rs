use crate::bus::Bus;
use crate::cpu::{
    Cpu,
    operation::pop::pop,
    register::{PointerRegisterKind, SegmentRegisterKind},
};

pub fn near(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.instruction_pointer = pop(cpu, bus);
}

pub fn near_word(cpu: &mut Cpu, bus: &mut Bus) {
    let displacement = cpu.fetch_word(bus);
    near(cpu, bus);

    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP) + displacement;
    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer);
}

pub fn far(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.instruction_pointer = pop(cpu, bus);
    let segment = pop(cpu, bus);
    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
}

pub fn far_word(cpu: &mut Cpu, bus: &mut Bus) {
    let displacement = cpu.fetch_word(bus);
    far(cpu, bus);

    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP) + displacement;
    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer);
}

pub fn from_interrupt(cpu: &mut Cpu, bus: &mut Bus) {
    cpu.instruction_pointer = pop(cpu, bus);

    let displacement = pop(cpu, bus);
    cpu.segment_registers.set(SegmentRegisterKind::CS, displacement);

    let flags = pop(cpu, bus);
    cpu.flags = flags;
}
