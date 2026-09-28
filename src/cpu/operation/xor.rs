use crate::cpu::Cpu;
use crate::cpu::flag::update_logic_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{register_operand_operation, OperationDirection};

pub fn xor_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::RegisterToOperand, xor)
}

pub fn xor_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::RegisterToOperand, xor)
}

pub fn xor_immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    let result = xor(cpu, register_value, immediate_value, size);

    register.write(cpu, result);
}

fn xor(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    _size: OperandSize,
) -> OperandValue {
    let result = source | destination;

    update_logic_flags(cpu, result);
    
    result
}