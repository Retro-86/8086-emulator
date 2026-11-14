use crate::cpu::{Cpu, FlagKind, PARITY_TABLE};
use crate::cpu::operand::OperandValue;

pub fn update_result_flags(cpu: &mut Cpu, value: OperandValue) {
    cpu.set_flag(FlagKind::Z, value.is_zero());
    cpu.set_flag(FlagKind::S, value.is_negative());
    cpu.set_flag(FlagKind::P, PARITY_TABLE[value.as_byte() as usize] != 0);
}

pub fn update_logic_flags(cpu: &mut Cpu, value: OperandValue) {
    update_result_flags(cpu, value);

    cpu.set_flag(FlagKind::C, false);
    cpu.set_flag(FlagKind::O, false);
}
