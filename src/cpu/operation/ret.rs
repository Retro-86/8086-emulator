use crate::cpu::{
    Cpu,
    operation::pop::pop,
    register::{PointerRegisterKind, SegmentRegisterKind},
};

pub fn near(cpu: &mut Cpu) {
    cpu.instruction_pointer = pop(cpu);
}

pub fn near_word(cpu: &mut Cpu) {
    let displacement = cpu.fetch_word();
    near(cpu);

    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP) + displacement;
    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer);
}

pub fn far(cpu: &mut Cpu) {
    cpu.instruction_pointer = pop(cpu);
    let segment = pop(cpu);
    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
}

pub fn far_word(cpu: &mut Cpu) {
    let displacement = cpu.fetch_word();
    far(cpu);

    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP) + displacement;
    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer);
}
