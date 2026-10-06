use crate::cpu::operand::OperandSize;
use crate::cpu::register::SegmentRegisterKind;
use crate::cpu::{Cpu, FlagKind};

pub fn jmp_relative(cpu: &mut Cpu, size: OperandSize) {
    let displacement = match size {
        OperandSize::Byte => cpu.fetch_byte() as i8 as u16,
        OperandSize::Word => cpu.fetch_word(),
    };

    cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
}

pub fn jmp_absolute(cpu: &mut Cpu) {
    let offset = cpu.fetch_word();
    let segment = cpu.fetch_word();

    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
    cpu.instruction_pointer = offset;
}

pub fn jo(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::O));
}

pub fn jno(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::O));
}

pub fn jb(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::C));
}

pub fn jnb(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::C));
}

pub fn jz(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::Z));
}

pub fn jnz(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::Z));
}

pub fn jg(cpu: &mut Cpu) {
    let condition =
        !cpu.get_flag(FlagKind::Z) && cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn jle(cpu: &mut Cpu) {
    let condition =
        cpu.get_flag(FlagKind::Z) || cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn jge(cpu: &mut Cpu) {
    let condition = cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn jl(cpu: &mut Cpu) {
    let condition = cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O);

    jump_with_condition(cpu, condition);
}

pub fn jpo(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::P));
}

pub fn jpe(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::P));
}

pub fn jns(cpu: &mut Cpu) {
    jump_with_condition(cpu, !cpu.get_flag(FlagKind::S));
}

pub fn js(cpu: &mut Cpu) {
    jump_with_condition(cpu, cpu.get_flag(FlagKind::S));
}

pub fn ja(cpu: &mut Cpu) {
    let condition = !cpu.get_flag(FlagKind::C) && !cpu.get_flag(FlagKind::Z);

    jump_with_condition(cpu, condition);
}

pub fn jbe(cpu: &mut Cpu) {
    let condition = cpu.get_flag(FlagKind::C) || cpu.get_flag(FlagKind::Z);

    jump_with_condition(cpu, condition);
}

fn jump_with_condition(cpu: &mut Cpu, condition: bool) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if condition {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}
