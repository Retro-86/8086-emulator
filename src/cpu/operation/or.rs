use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::flag::update_logic_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{OperationDirection, register_operand_operation};

pub fn operand_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    register_operand_operation(cpu, bus, size, OperationDirection::OperandToRegister, or)
}

pub fn register_to_operand(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    register_operand_operation(cpu, bus, size, OperationDirection::RegisterToOperand, or)
}

pub fn immediate_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, size);
    let immediate_value = cpu.fetch_value(bus, size);

    let result = or(cpu, register_value, immediate_value, size);

    register.write(cpu, bus, result);
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
