use crate::bus::Bus;
use crate::cpu::flag::update_result_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{OperationDirection, register_operand_operation};
use crate::cpu::{AddressingMode, Cpu, FlagKind};

pub fn register_to_operand(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    register_operand_operation(cpu, bus, size, OperationDirection::RegisterToOperand, add)
}

pub fn operand_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    register_operand_operation(cpu, bus, size, OperationDirection::OperandToRegister, add)
}

pub fn immediate_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, size);
    let immediate_value = cpu.fetch_value(bus, size);

    let result = add(cpu, register_value, immediate_value, size);

    register.write(cpu, bus, result);
}

pub fn immediate_to_operand(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let operand = addressing_mode.operand(cpu, bus);

    let operand_value = operand.read(cpu, bus, size);
    let immediate_value = cpu.fetch_value(bus, size);

    let result = add(cpu, operand_value, immediate_value, size);

    operand.write(cpu, bus, result);
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
