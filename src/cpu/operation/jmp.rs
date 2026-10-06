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
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::O) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jno(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if !cpu.get_flag(FlagKind::O) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jb(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::C) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jnb(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if !cpu.get_flag(FlagKind::C) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jz(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::Z) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jnz(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if !cpu.get_flag(FlagKind::Z) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jg(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if !cpu.get_flag(FlagKind::Z) || (cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O)) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jle(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O) || cpu.get_flag(FlagKind::Z) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jge(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::S) == cpu.get_flag(FlagKind::O) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jl(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::S) != cpu.get_flag(FlagKind::O) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jpo(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if !cpu.get_flag(FlagKind::P) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jpe(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::P) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jns(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if !cpu.get_flag(FlagKind::S) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn js(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::S) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn ja(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if !(cpu.get_flag(FlagKind::C) || cpu.get_flag(FlagKind::Z)) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}

pub fn jbe(cpu: &mut Cpu) {
    let displacement = cpu.fetch_byte() as i8 as u16;

    if cpu.get_flag(FlagKind::C) || cpu.get_flag(FlagKind::Z) {
        cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
    }
}