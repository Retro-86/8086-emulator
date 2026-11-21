use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{
    add, add_carry, and, compare, decrement, increment, or, push, rotate, shift, subtract,
    subtract_borrow, xor,
};
use crate::cpu::register::{ByteSharedRegisterKind, SegmentRegisterKind};
use crate::cpu::{AddressingMode, Cpu};

pub fn one(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let first_operand = addressing_mode.operand(cpu, bus);

    let first_operand_value = first_operand.read(cpu, bus, OperandSize::Word);
    let second_operand_value = match size {
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

pub fn four(cpu: &mut Cpu, bus: &mut Bus) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));
    let operand = addressing_mode.operand(cpu, bus);

    match addressing_mode.register {
        0 => increment::operand(cpu, bus, operand, OperandSize::Byte),
        1 => decrement::operand(cpu, bus, operand, OperandSize::Byte),
        _ => unreachable!(),
    }
}

pub fn five(cpu: &mut Cpu, bus: &mut Bus) {
    let mode = AddressingMode::new(cpu.fetch_byte(bus));
    let operand = mode.operand(cpu, bus);

    match mode.register {
        0 => increment::operand(cpu, bus, operand, OperandSize::Word),
        1 => decrement::operand(cpu, bus, operand, OperandSize::Word),
        2 | 4 => {
            let target_instruction_pointer = operand.read(cpu, bus, OperandSize::Word).as_word();

            if mode.register == 2 {
                push::push(cpu, bus, cpu.instruction_pointer);
            }

            cpu.instruction_pointer = target_instruction_pointer;
        }
        3 | 5 => {
            let Operand::Memory(address) = operand else {
                panic!("FF /{} requires a memory operand", mode.register);
            };

            let target_instruction_pointer = bus.read_word_from_memory(address);
            let target_code_segment = bus.read_word_from_memory(address + 2);

            if mode.register == 3 {
                push::segment_register(cpu, bus, SegmentRegisterKind::CS);
                push::push(cpu, bus, cpu.instruction_pointer);
            }

            cpu.instruction_pointer = target_instruction_pointer;
            cpu.segment_registers
                .set(SegmentRegisterKind::CS, target_code_segment);
        }
        6 => {
            let mut value = operand.read(cpu, bus, OperandSize::Word).as_word();

            if matches!(operand, Operand::Register(4)) {
                value = value.wrapping_sub(2);
            }

            push::push(cpu, bus, value);
        }
        _ => unreachable!(),
    }
}
