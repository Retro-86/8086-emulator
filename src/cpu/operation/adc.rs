use crate::cpu::flag::update_result_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{OperationDirection, register_operand_operation};
use crate::cpu::{Cpu, FlagKind};

pub fn adc_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::OperandToRegister, adc)
}

pub fn adc_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::RegisterToOperand, adc)
}

pub fn adc_immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    let result = adc(cpu, register_value, immediate_value, size);

    register.write(cpu, result);
}

pub fn adc(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    size: OperandSize,
) -> OperandValue {
    let carry = cpu.get_flag(FlagKind::C);

    let carry_value = match size {
        OperandSize::Byte => OperandValue::Byte(carry as u8),
        OperandSize::Word => OperandValue::Word(carry as u16),
    };

    let result = destination + source + carry_value;

    update_adc_flags(cpu, destination, source, result, size);

    result
}

fn update_adc_flags(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    result: OperandValue,
    size: OperandSize,
) {
    update_result_flags(cpu, result);

    let (max_value, sign_mask): (u32, u32) = match size {
        OperandSize::Byte => (0xFF, 0x80),
        OperandSize::Word => (0xFFFF, 0x8000),
    };

    let first = first.as_word() as u32;
    let second = second.as_word() as u32;
    let result = result.as_word() as u32;

    let sum = first + second + cpu.get_flag(FlagKind::C) as u32;
    cpu.set_flag(FlagKind::C, sum > max_value);

    let overflow = (!(first ^ second) & (first ^ result) & sign_mask) != 0;
    cpu.set_flag(FlagKind::O, overflow);

    let auxiliary_carry = ((first ^ second ^ result) & 0x10) != 0;
    cpu.set_flag(FlagKind::A, auxiliary_carry);
}
