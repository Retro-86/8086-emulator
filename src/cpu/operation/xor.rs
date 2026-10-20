use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::flag::update_logic_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{register_operand_operation, OperationDirection};

pub fn register_to_operand(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    register_operand_operation(cpu, bus, size, OperationDirection::RegisterToOperand, xor)
}

pub fn operand_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    register_operand_operation(cpu, bus, size, OperationDirection::RegisterToOperand, xor)
}

pub fn immediate_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, size);
    let immediate_value = cpu.fetch_value(bus, size);

    let result = xor(cpu, register_value, immediate_value, size);

    register.write(cpu, bus, result);
}

pub fn xor(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    _size: OperandSize,
) -> OperandValue {
    let result = source | destination;

    update_logic_flags(cpu, result);
    
    result
}