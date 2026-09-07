use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{
    AddressingMode, ByteSharedRegisterKind, Cpu, FlagKind, PARITY_TABLE, WordSharedRegisterKind,
};

pub fn or_register_immediate(cpu: &mut Cpu, size: OperandSize) {
    let (register_value, immediate_value) = match size {
        OperandSize::Byte => {
            let register_value =
                OperandValue::Byte(cpu.shared_registers.get_byte(ByteSharedRegisterKind::AL));

            let immediate_value = OperandValue::Byte(cpu.fetch_byte());
            (register_value, immediate_value)
        }
        OperandSize::Word => {
            let register_value =
                OperandValue::Word(cpu.shared_registers.get_word(WordSharedRegisterKind::AX));

            let immediate_value = OperandValue::Word(cpu.fetch_word());
            (register_value, immediate_value)
        }
    };

    let result = immediate_value | register_value;

    update_or_flags(cpu, result);

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

pub fn or_operand_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let register = Operand::Register(addressing_mode.register);

    let operand_value = operand.read(cpu, size);
    let register_value = register.read(cpu, size);

    let result = operand_value | register_value;

    update_or_flags(cpu, result);

    operand.write(cpu, result)
}

pub fn or_register_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let register = Operand::Register(addressing_mode.register);

    let operand_value = operand.read(cpu, size);
    let register_value = register.read(cpu, size);

    let result = register_value | operand_value;

    update_or_flags(cpu, result);

    register.write(cpu, result);
}

fn update_or_flags(cpu: &mut Cpu, value: OperandValue) {
    cpu.set_flag(
        FlagKind::Z,
        match value {
            OperandValue::Byte(value) => value == 0,
            OperandValue::Word(value) => value == 0,
        },
    );

    cpu.set_flag(
        FlagKind::S,
        match value {
            OperandValue::Byte(value) => (value & 0x80) != 0,
            OperandValue::Word(value) => (value & 0x8000) != 0,
        },
    );

    cpu.set_flag(FlagKind::P, PARITY_TABLE[value.as_word() as usize] != 0);

    cpu.set_flag(FlagKind::C, false);
    cpu.set_flag(FlagKind::O, false);
}
