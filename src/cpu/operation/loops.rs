use crate::cpu::{Cpu, register::WordSharedRegisterKind};

pub fn loop_while_not_zero(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    let counter_value = cpu
        .shared_registers
        .get_word(WordSharedRegisterKind::CX)
        .wrapping_sub(1);
    cpu.shared_registers
        .set_word(WordSharedRegisterKind::CX, counter_value);

    if counter_value != 0 && !cpu.get_flag(crate::cpu::FlagKind::Z) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn loop_while_equal(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    let counter_value = cpu
        .shared_registers
        .get_word(WordSharedRegisterKind::CX)
        .wrapping_sub(1);
    cpu.shared_registers
        .set_word(WordSharedRegisterKind::CX, counter_value);

    if counter_value != 0 && cpu.get_flag(crate::cpu::FlagKind::Z) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn loops(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    let counter_value = cpu
        .shared_registers
        .get_word(WordSharedRegisterKind::CX)
        .wrapping_sub(1);
    cpu.shared_registers
        .set_word(WordSharedRegisterKind::CX, counter_value);

    if counter_value != 0 {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}
