use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::operand::{Operand, OperandSize};

pub fn operand(cpu: &mut Cpu, bus: &mut Bus, operand: Operand, operand_size: OperandSize) {
    let operand_value = operand.read(cpu, bus, operand_size);
    operand.write(cpu, bus, !operand_value)
}