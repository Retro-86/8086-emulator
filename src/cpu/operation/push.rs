use crate::cpu::{Cpu, PointerRegisterKind, SegmentRegisterKind};

pub fn push(cpu: &mut Cpu, value: u16) {
    let stack_pointer_value = cpu
        .pointer_registers
        .get(PointerRegisterKind::SP)
        .wrapping_sub(2);

    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer_value);

    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::SS);
    let physical_address = cpu.physical_address(segment_register, stack_pointer_value);

    cpu.bus.write_word_to_memory(physical_address, value);
}

pub fn push_segment_register(cpu: &mut Cpu, register_kind: SegmentRegisterKind) {
    let register_value = cpu.segment_registers.get(register_kind);
    push(cpu, register_value);
}

pub fn push_word_register(cpu: &mut Cpu, base: u8, operation_code: u8) {
    let register_index = operation_code - base;
    let mut register_value = cpu.get_word_register(register_index);

    if operation_code == 0x54 {
        register_value = register_value.wrapping_sub(2);
    }

    push(cpu, register_value);
}