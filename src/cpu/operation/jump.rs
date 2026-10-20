use crate::bus::Bus;
use crate::cpu::operand::OperandSize;
use crate::cpu::register::SegmentRegisterKind;
use crate::cpu::{Cpu, FlagKind};

pub fn relative(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let displacement = match size {
        OperandSize::Byte => cpu.fetch_sign_extended_byte(bus),
        OperandSize::Word => cpu.fetch_word(bus),
    };

    cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
}

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) {
    let offset = cpu.fetch_word(bus);
    let segment = cpu.fetch_word(bus);

    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
    cpu.instruction_pointer = offset;
}

pub fn if_overflow(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, cpu.get_flag(FlagKind::O));
}

pub fn if_not_overflow(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, !cpu.get_flag(FlagKind::O));
}

pub fn if_below(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, cpu.get_flag(FlagKind::C));
}

pub fn if_not_below(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, !cpu.get_flag(FlagKind::C));
}

pub fn if_zero(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, cpu.get_flag(FlagKind::Z));
}

pub fn if_not_zero(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, !cpu.get_flag(FlagKind::Z));
}

pub fn if_greater(cpu: &mut Cpu, bus: &mut Bus) {
    let condition =
        !cpu.get_flag(FlagKind::Z) && cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, bus, condition);
}

pub fn if_less_or_equal(cpu: &mut Cpu, bus: &mut Bus) {
    let condition =
        cpu.get_flag(FlagKind::Z) || cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, bus, condition);
}

pub fn if_greater_or_equal(cpu: &mut Cpu, bus: &mut Bus) {
    let condition = cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, bus, condition);
}

pub fn if_less(cpu: &mut Cpu, bus: &mut Bus) {
    let condition = cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, bus, condition);
}

pub fn if_parity_odd(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, !cpu.get_flag(FlagKind::P));
}

pub fn if_parity_even(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, cpu.get_flag(FlagKind::P));
}

pub fn if_not_sign(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, !cpu.get_flag(FlagKind::S));
}

pub fn if_sign(cpu: &mut Cpu, bus: &mut Bus) {
    jump_with_condition(cpu, bus, cpu.get_flag(FlagKind::S));
}

pub fn if_above(cpu: &mut Cpu, bus: &mut Bus) {
    let condition = !cpu.get_flag(FlagKind::C) && !cpu.get_flag(FlagKind::Z);

    jump_with_condition(cpu, bus, condition);
}

pub fn if_below_or_equal(cpu: &mut Cpu, bus: &mut Bus) {
    let condition = cpu.get_flag(FlagKind::C) || cpu.get_flag(FlagKind::Z);

    jump_with_condition(cpu, bus, condition);
}

fn jump_with_condition(cpu: &mut Cpu, bus: &mut Bus, condition: bool) {
    let displacement = cpu.fetch_byte(bus) as i8 as u16;

    if condition {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}
