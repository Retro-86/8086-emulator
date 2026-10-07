use crate::cpu::{Cpu, operation::pop::pop, register::{PointerRegisterKind, SegmentRegisterKind}};

pub fn ret_word(cpu: &mut Cpu) {
    let displacement = cpu.fetch_word();
    ret(cpu);

    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP) + displacement;
    cpu.pointer_registers.set(PointerRegisterKind::SP, stack_pointer);
}

pub fn ret(cpu: &mut Cpu) {
    cpu.instruction_pointer = pop(cpu);
}

pub fn retf(cpu: &mut Cpu) {
    ret(cpu);

    let value = pop(cpu);
    cpu.segment_registers.set(SegmentRegisterKind::CS, value);
}

pub fn retf_word(cpu: &mut Cpu) {
    let displacement = cpu.fetch_word();
    retf(cpu);

    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP) + displacement;
    cpu.pointer_registers.set(PointerRegisterKind::SP, stack_pointer);
}