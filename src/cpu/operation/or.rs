use crate::cpu::Cpu;
use crate::cpu::flag::update_logic_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{OperationDirection, register_operand_operation};

pub fn or_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::OperandToRegister, or)
}

pub fn or_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::RegisterToOperand, or)
}

pub fn or(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    _size: OperandSize,
) -> OperandValue {
    let result = destination | source;

    update_logic_flags(cpu, result);

    result
}

pub fn or_immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    let result = register_value | immediate_value;

    update_logic_flags(cpu, result);

    register.write(cpu, result);
}
