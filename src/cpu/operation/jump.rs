use crate::cpu::operand::OperandSize;
use crate::cpu::register::SegmentRegisterKind;
use crate::cpu::{Cpu, FlagKind};

pub fn relative(cpu: &mut Cpu, size: OperandSize) {
    let displacement = match size {
        OperandSize::Byte => cpu.fetch_sign_extended_byte(),
        OperandSize::Word => cpu.fetch_word(),
    };

    cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
}

pub fn absolute(cpu: &mut Cpu) {
    let offset = cpu.fetch_word();
    let segment = cpu.fetch_word();

    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
    cpu.instruction_pointer = offset;
}

pub fn if_overflow(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::O));
}

pub fn if_not_overflow(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::O));
}

pub fn if_below(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::C));
}

pub fn if_not_below(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::C));
}

pub fn if_zero(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::Z));
}

pub fn if_not_zero(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::Z));
}

pub fn if_greater(cpu: &mut Cpu) {
    let condition =
        !cpu.get_flag(FlagKind::Z) && cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn if_less_or_equal(cpu: &mut Cpu) {
    let condition =
        cpu.get_flag(FlagKind::Z) || cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn if_greater_or_equal(cpu: &mut Cpu) {
    let condition = cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn if_less(cpu: &mut Cpu) {
    let condition = cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn if_parity_odd(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::P));
}

pub fn if_parity_even(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::P));
}

pub fn if_not_sign(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::S));
}

pub fn if_sign(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::S));
}

pub fn if_above(cpu: &mut Cpu) {
    let condition = !cpu.get_flag(FlagKind::C) && !cpu.get_flag(FlagKind::Z);

    jump_with_condition(cpu, condition);
}

pub fn if_below_or_equal(cpu: &mut Cpu) {
    let condition = cpu.get_flag(FlagKind::C) || cpu.get_flag(FlagKind::Z);

    jump_with_condition(cpu, condition);
}

fn jump_with_condition(cpu: &mut Cpu, condition: bool) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if condition {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}
