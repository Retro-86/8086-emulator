use crate::cpu::{Cpu, FlagKind};

pub fn interrupt_flag(cpu: &mut Cpu) {
    cpu.set_flag(FlagKind::I, true);
}