use crate::bus::Bus;
use crate::cpu::Cpu;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Sub};

#[derive(Copy, Clone)]
pub enum OperandValue {
    Byte(u8),
    Word(u16),
}

impl OperandValue {
    pub fn as_word(self) -> u16 {
        match self {
            OperandValue::Byte(value) => value as u16,
            OperandValue::Word(value) => value,
        }
    }

    pub fn as_byte(self) -> u8 {
        match self {
            OperandValue::Byte(value) => value,
            OperandValue::Word(value) => value as u8,
        }
    }

    pub fn is_zero(self) -> bool {
        self.as_word() == 0
    }

    pub fn is_negative(self) -> bool {
        match self {
            Self::Byte(value) => value & 0x80 != 0,
            Self::Word(value) => value & 0x8000 != 0,
        }
    }

    pub fn zero(size: OperandSize) -> Self {
        match size {
            OperandSize::Byte => OperandValue::Byte(0),
            OperandSize::Word => OperandValue::Word(0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OperandSize {
    Byte,
    Word,
}

#[derive(Debug, Copy, Clone)]

pub enum Operand {
    Memory(u32),
    Register(u8),
}

impl Operand {
    pub fn read(self, cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) -> OperandValue {
        match (self, size) {
            (Operand::Register(register), OperandSize::Byte) => {
                OperandValue::Byte(cpu.get_byte_register(register))
            }
            (Operand::Register(register), OperandSize::Word) => {
                OperandValue::Word(cpu.get_word_register(register))
            }
            (Operand::Memory(address), OperandSize::Byte) => {
                OperandValue::Byte(bus.read_byte_from_memory(address))
            }
            (Operand::Memory(address), OperandSize::Word) => {
                OperandValue::Word(bus.read_word_from_memory(address))
            }
        }
    }

    pub fn write(self, cpu: &mut Cpu, bus: &mut Bus, value: OperandValue) {
        match (self, value) {
            (Operand::Register(register), OperandValue::Byte(value)) => {
                cpu.set_byte_register(register, value)
            }
            (Operand::Register(register), OperandValue::Word(value)) => {
                cpu.set_word_register(register, value)
            }

            (Operand::Memory(address), OperandValue::Byte(value)) => {
                bus.write_byte_to_memory(address, value)
            }
            (Operand::Memory(address), OperandValue::Word(value)) => {
                bus.write_word_to_memory(address, value)
            }
        }
    }
}

impl Not for OperandValue {
    type Output = OperandValue;

    fn not(self) -> Self::Output {
        match self {
            Self::Byte(value) => Self::Byte(!value),
            Self::Word(value) => Self::Word(!value),
        }
    }
}

macro_rules! impl_operand_op {
    ($trait:ident, $method:ident, $op:ident) => {
        impl $trait for OperandValue {
            type Output = OperandValue;
            fn $method(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (OperandValue::Byte(a), OperandValue::Byte(b)) => OperandValue::Byte(a.$op(b)),
                    (OperandValue::Word(a), OperandValue::Word(b)) => OperandValue::Word(a.$op(b)),
                    _ => panic!("mismatched operand sizes"),
                }
            }
        }
    };
}

impl_operand_op!(Add, add, wrapping_add);
impl_operand_op!(Sub, sub, wrapping_sub);
impl_operand_op!(BitOr, bitor, bitor);
impl_operand_op!(BitAnd, bitand, bitand);
impl_operand_op!(BitXor, bitxor, bitxor);
impl_operand_op!(Mul, mul, wrapping_mul);
impl_operand_op!(Div, div, wrapping_div);
