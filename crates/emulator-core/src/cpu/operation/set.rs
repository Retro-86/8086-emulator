use crate::cpu::{Cpu, FlagKind};

pub fn interrupt_flag(cpu: &mut Cpu) {
    cpu.set_flag(FlagKind::I, true);
}

pub fn direction_flag(cpu: &mut Cpu) {
    cpu.set_flag(FlagKind::D, true);
}

pub fn carry_flag(cpu: &mut Cpu) {
    cpu.set_flag(FlagKind::C, true);
}

