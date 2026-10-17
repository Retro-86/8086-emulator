use crate::cpu::operand::{OperandSize, OperandValue};
use crate::cpu::{Cpu, FlagKind};

enum Direction {
    Left,
    Right,
}

enum RotateMode {
    Cycle,
    ThroughCarry,
}

pub fn left(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
) -> OperandValue {
    rotate(cpu, first, second, size, Direction::Left, RotateMode::Cycle)
}

pub fn right(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
) -> OperandValue {
    rotate(
        cpu,
        first,
        second,
        size,
        Direction::Right,
        RotateMode::Cycle,
    )
}

pub fn through_carry_left(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
) -> OperandValue {
    rotate(
        cpu,
        first,
        second,
        size,
        Direction::Left,
        RotateMode::ThroughCarry,
    )
}

pub fn through_carry_right(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
) -> OperandValue {
    rotate(
        cpu,
        first,
        second,
        size,
        Direction::Right,
        RotateMode::ThroughCarry,
    )
}

fn rotate(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
    direction: Direction,
    mode: RotateMode,
) -> OperandValue {
    let count = second.as_byte();
    if count == 0 {
        return first;
    }

    let bits = match size {
        OperandSize::Byte => 8,
        OperandSize::Word => 16,
    };

    let mask = (1u32 << bits) - 1;
    let sign = 1u32 << (bits - 1);

    let mut value = u32::from(first.as_word()) & mask;
    let mut carry = cpu.get_flag(FlagKind::C);

    for _ in 0..count {
        let outgoing_bit = match direction {
            Direction::Left => value & sign != 0,
            Direction::Right => value & 1 != 0,
        };

        let incoming_bit = u32::from(match mode {
            RotateMode::Cycle => outgoing_bit,
            RotateMode::ThroughCarry => carry,
        });

        value = match direction {
            Direction::Left => ((value << 1) | incoming_bit) & mask,
            Direction::Right => (value >> 1) | (incoming_bit << (bits - 1)),
        };

        carry = outgoing_bit;
    }

    cpu.set_flag(FlagKind::C, carry);

    if count == 1 {
        let highest_bit = value & sign != 0;
        let overflow = match direction {
            Direction::Left => highest_bit ^ carry,
            Direction::Right => highest_bit ^ (value & (sign >> 1) != 0),
        };

        cpu.set_flag(FlagKind::O, overflow);
    }

    match size {
        OperandSize::Byte => OperandValue::Byte(value as u8),
        OperandSize::Word => OperandValue::Word(value as u16),
    }
}
