use crate::cpu::flag::update_result_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{OperationDirection, register_operand_operation};
use crate::cpu::{AddressingMode, Cpu, FlagKind};

pub fn add_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::RegisterToOperand, add)
}

pub fn add_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::OperandToRegister, add)
}

pub fn add_immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    let result = add(cpu, register_value, immediate_value, size);

    register.write(cpu, result);
}

pub fn add_immediate_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);

    let operand_value = operand.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    let result = add(cpu, operand_value, immediate_value, size);

    operand.write(cpu, result);
}

pub fn add(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    size: OperandSize,
) -> OperandValue {
    let result = destination + source;

    update_add_flags(cpu, destination, source, result, size);

    result
}

fn update_add_flags(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    result: OperandValue,
    size: OperandSize,
) {
    update_result_flags(cpu, result);

    let carry = match (first, second, size) {
        (OperandValue::Byte(first), OperandValue::Byte(second), OperandSize::Byte) => {
            (first as u16 + second as u16) > u8::MAX as u16
        }
        (OperandValue::Word(first), OperandValue::Word(second), OperandSize::Word) => {
            (first as u32 + second as u32) > u16::MAX as u32
        }
        _ => false,
    };
    cpu.set_flag(FlagKind::C, carry);

    let overflow =
        first.is_negative() == second.is_negative() && first.is_negative() != result.is_negative();

    cpu.set_flag(FlagKind::O, overflow);

    let auxiliary_carry =
        ((first.as_word() ^ second.as_word() ^ result.as_word()) & 0x10) != 0;

    cpu.set_flag(FlagKind::A, auxiliary_carry);
}
