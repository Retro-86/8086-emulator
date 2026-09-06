use crate::cpu::{AddressingMode, ByteSharedRegisterKind, Cpu, FlagKind, WordSharedRegisterKind, PARITY_TABLE};
use crate::cpu::operand::{Operand, OperandSize, OperandValue};

pub fn add_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let register_value = register.read(cpu, size);
    let operand_value = operand.read(cpu, size);

    let result = register_value + operand_value;

    update_add_flags(cpu, register_value, operand_value, result, size);

    operand.write(cpu, result);
}

pub fn add_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let register_value = register.read(cpu, size);
    let operand_value = operand.read(cpu, size);

    let result = register_value + operand_value;

    update_add_flags(cpu, register_value, operand_value, result, size);

    register.write(cpu, result);
}

pub fn add_immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let (register_value, immediate_value) = match size {
        OperandSize::Byte => {
            let register_value = OperandValue::Byte(
                cpu.shared_registers.get_byte(ByteSharedRegisterKind::AL)
            );

            let immediate_value = OperandValue::Byte(cpu.fetch_byte());
            (register_value, immediate_value)
        }
        OperandSize::Word => {
            let register_value = OperandValue::Word(
                cpu.shared_registers.get_word(WordSharedRegisterKind::AX)
            );

            let immediate_value = OperandValue::Word(cpu.fetch_word());
            (register_value, immediate_value)
        }
    };

    let result = immediate_value + register_value;

    update_add_flags(cpu, register_value, immediate_value, result, size);

    match result {
        OperandValue::Byte(value) => {
            cpu.shared_registers
                .set_byte(ByteSharedRegisterKind::AL, value);
        }
        OperandValue::Word(value) => {
            cpu.shared_registers
                .set_word(WordSharedRegisterKind::AX, value);
        }
    }
}

pub fn add_immediate_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let operand_value = operand.read(cpu, size);

    let immediate_value = match size {
        OperandSize::Byte => OperandValue::Byte(cpu.fetch_byte()),
        OperandSize::Word => OperandValue::Word(cpu.fetch_word()),
    };

    let result = immediate_value + operand_value;

    update_add_flags(cpu, operand_value, immediate_value, result, size);

    operand.write(cpu, result);
}

fn update_add_flags(
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
