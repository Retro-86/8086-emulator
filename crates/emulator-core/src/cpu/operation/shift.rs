use crate::cpu::flag::update_result_flags;
use crate::cpu::operand::{OperandSize, OperandValue};
use crate::cpu::{Cpu, FlagKind};

enum ShiftKind {
    Left,
    Right,
    ArithmeticRight,
}

pub fn left(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
) -> OperandValue {
    shift(cpu, first, second, size, ShiftKind::Left)
}

pub fn right(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
) -> OperandValue {
    shift(cpu, first, second, size, ShiftKind::Right)
}

pub fn arithmetic_right(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
) -> OperandValue {
    shift(cpu, first, second, size, ShiftKind::ArithmeticRight)
}

fn shift(
    cpu: &mut Cpu,
    first: OperandValue,
    second: OperandValue,
    size: OperandSize,
    direction: ShiftKind,
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
    let original_sign = value & sign != 0;
    let mut carry = false;

    for _ in 0..count {
        match direction {
            ShiftKind::Left => {
                carry = value & sign != 0;
                value = (value << 1) & mask;
            }
            ShiftKind::Right => {
                carry = value & 1 != 0;
                value >>= 1;
            }
            ShiftKind::ArithmeticRight => {
                carry = value & 1 != 0;
                value = (value >> 1) | (value & sign);
            }
        }
    }

    let result = match size {
        OperandSize::Byte => OperandValue::Byte(value as u8),
        OperandSize::Word => OperandValue::Word(value as u16),
    };

    update_flags(cpu, direction, count, original_sign, carry, result);
    result
}

fn update_flags(
    cpu: &mut Cpu,
    kind: ShiftKind,
    count: u8,
    original_sign: bool,
    carry: bool,
    result: OperandValue,
) {
    cpu.set_flag(FlagKind::C, carry);

    if count == 1 {
        let overflow = match kind {
            ShiftKind::Left => result.is_negative() ^ carry,
            ShiftKind::Right => original_sign,
            ShiftKind::ArithmeticRight => false,
        };

        cpu.set_flag(FlagKind::O, overflow);
    }

    update_result_flags(cpu, result);
}
