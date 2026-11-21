use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::subtract;

pub fn operand(cpu: &mut Cpu, bus: &mut Bus, operand: Operand, operand_size: OperandSize) {
    let value = operand.read(cpu, bus, operand_size);
    let result = OperandValue::zero(operand_size) - value;

    operand.write(cpu, bus, result);

    subtract::update_flags(
        cpu,
        OperandValue::zero(operand_size),
        value,
        result,
        operand_size,
    );
}
