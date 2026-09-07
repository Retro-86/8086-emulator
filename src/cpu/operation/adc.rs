use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{AddressingMode, Cpu, FlagKind, PARITY_TABLE};

pub fn adc_immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let carry = cpu.get_flag(FlagKind::C);

    let operand_value = operand.read(cpu, size);
    let immediate_value = match size {
        OperandSize::Byte => OperandValue::Byte(cpu.fetch_byte()),
        OperandSize::Word => OperandValue::Word(cpu.fetch_word()),
    };
    let carry_value = match size {
        OperandSize::Byte => OperandValue::Byte(carry as u8),
        OperandSize::Word => OperandValue::Word(carry as u16),
    };

    let result = immediate_value + operand_value + carry_value;

    update_adc_flags(cpu, operand_value, immediate_value, result, size);

    operand.write(cpu, result);
}

pub fn adc_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);
    let carry = cpu.get_flag(FlagKind::C);

    let register_value = register.read(cpu, size);
    let operand_value = operand.read(cpu, size);
    let carry_value = match size {
        OperandSize::Byte => OperandValue::Byte(carry as u8),
        OperandSize::Word => OperandValue::Word(carry as u16),
    };

    let result = operand_value + register_value + carry_value;

    update_adc_flags(cpu, operand_value, register_value, result, size);

    register.write(cpu, result);
}

pub fn adc_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);
    let carry = cpu.get_flag(FlagKind::C);

    let register_value = register.read(cpu, size);
    let operand_value = operand.read(cpu, size);
    let carry_value = match size {
        OperandSize::Byte => OperandValue::Byte(carry as u8),
        OperandSize::Word => OperandValue::Word(carry as u16),
    };

    let result = register_value + operand_value + carry_value;

    update_adc_flags(cpu, register_value, operand_value, result, size);

    operand.write(cpu, result);
}

fn update_adc_flags(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    result: OperandValue,
    size: OperandSize,
) {
    cpu.set_flag(
        FlagKind::Z,
        match result {
            OperandValue::Byte(value) => value == 0,
            OperandValue::Word(value) => value == 0,
        },
    );

    cpu.set_flag(
        FlagKind::S,
        match result {
            OperandValue::Byte(value) => (value & 0x80) != 0,
            OperandValue::Word(value) => (value & 0x8000) != 0,
        },
    );

    cpu.set_flag(FlagKind::P, PARITY_TABLE[result.as_word() as usize] != 0);

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

    let overflow = match (first, second, result, size) {
        (
            OperandValue::Byte(first),
            OperandValue::Byte(second),
            OperandValue::Byte(result),
            OperandSize::Byte,
        ) => {
            let sign_first = (first & 0x80) != 0;
            let sign_second = (second & 0x80) != 0;
            let sign_result = (result & 0x80) != 0;
            (sign_first == sign_second) && (sign_first != sign_result)
        }
        (
            OperandValue::Word(first),
            OperandValue::Word(second),
            OperandValue::Word(result),
            OperandSize::Word,
        ) => {
            let sign_first = (first & 0x8000) != 0;
            let sign_second = (second & 0x8000) != 0;
            let sign_result = (result & 0x8000) != 0;
            (sign_first == sign_second) && (sign_first != sign_result)
        }
        _ => false,
    };
    cpu.set_flag(FlagKind::O, overflow);
}
