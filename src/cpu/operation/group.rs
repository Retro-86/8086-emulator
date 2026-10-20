use crate::bus::Bus;
use crate::cpu::operand::{OperandSize, OperandValue};
use crate::cpu::operation::{
    add, add_carry, and, compare, or, rotate, shift, subtract, subtract_borrow, xor,
};
use crate::cpu::register::ByteSharedRegisterKind;
use crate::cpu::{AddressingMode, Cpu};

pub fn one(cpu: &mut Cpu, bus: &mut Bus, operand_size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let first_operand = addressing_mode.operand(cpu, bus);

    let first_operand_value = first_operand.read(cpu, bus, OperandSize::Word);
    let second_operand_value = match operand_size {
        OperandSize::Byte => OperandValue::Word(cpu.fetch_sign_extended_byte(bus)),
        OperandSize::Word => OperandValue::Word(cpu.fetch_word(bus)),
    };

    let operation: fn(&mut Cpu, OperandValue, OperandValue, OperandSize) -> OperandValue =
        match addressing_mode.register {
            0 => add::add,
            1 => or::or,
            2 => add_carry::add_carry,
            3 => subtract_borrow::subtract_borrow,
            4 => and::and,
            5 => subtract::subtract,
            6 => xor::xor,
            7 => compare::compare,
            _ => unreachable!(),
        };

    let result = operation(
        cpu,
        first_operand_value,
        second_operand_value,
        OperandSize::Word,
    );

    if addressing_mode.register != 7 {
        first_operand.write(cpu, bus, result);
    }
}

pub enum ShiftSource {
    One,
    Register,
}

pub fn two(cpu: &mut Cpu, bus: &mut Bus, operand_size: OperandSize, source: ShiftSource) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let first_operand = addressing_mode.operand(cpu, bus);

    let shift = match source {
        ShiftSource::One => 1,
        ShiftSource::Register => cpu.shared_registers.get_byte(ByteSharedRegisterKind::CL),
    };

    let first_operand_value = first_operand.read(cpu, bus, operand_size);
    let second_operand_value = OperandValue::Byte(shift);

    let operation: fn(&mut Cpu, OperandValue, OperandValue, OperandSize) -> OperandValue =
        match addressing_mode.register {
            0 => rotate::left,
            1 => rotate::right,
            2 => rotate::through_carry_left,
            3 => rotate::through_carry_right,
            4 => shift::left,
            5 => shift::right,
            7 => shift::arithmetic_right,
            _ => unreachable!(),
        };

    let result = operation(cpu, first_operand_value, second_operand_value, operand_size);
    first_operand.write(cpu, bus, result);
}
