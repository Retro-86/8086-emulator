use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::subtract;
use crate::cpu::{AddressingMode, Cpu};

pub fn register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let operand_value = operand.read(cpu, size);
    let register_value = register.read(cpu, size);

    compare(cpu, operand_value, register_value, size);
}

pub fn operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let operand_value = operand.read(cpu, size);
    let register_value = register.read(cpu, size);

    compare(cpu, register_value, operand_value, size);
}

pub fn immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    compare(cpu, register_value, immediate_value, size);
}

pub fn compare(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    size: OperandSize,
) -> OperandValue {
    let result = destination - source;

    subtract::update_flags(cpu, destination, source, result, size);

    result
}
