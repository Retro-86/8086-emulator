use crate::cpu::{Cpu, FlagKind};

pub fn interrupt_flag(cpu: &mut Cpu) {
    cpu.set_flag(FlagKind::I, false);
}

pub fn direction_flag(cpu: &mut Cpu) {
    cpu.set_flag(FlagKind::D, false);
}

pub fn carry_flag(cpu: &mut Cpu) {
    cpu.set_flag(FlagKind::C, false);
}

