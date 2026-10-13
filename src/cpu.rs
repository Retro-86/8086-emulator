pub mod flag;
pub mod operand;
pub mod operation;
pub mod register;

use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::adc::{
    adc_immediate_to_register, adc_operand_to_register, adc_register_to_operand,
};
use crate::cpu::operation::add::{
    add_immediate_to_register, add_operand_to_register, add_register_to_operand,
};
use crate::cpu::operation::and::{
    and_immediate_to_register, and_operand_to_register, and_register_to_operand,
};
use crate::cpu::operation::cmp::{
    cmp_immediate_to_register, cmp_operand_to_register, cmp_register_to_operand,
};
use crate::cpu::operation::dec::dec_word_register;
use crate::cpu::operation::inc::inc_word_register;
use crate::cpu::operation::jmp::{
    ja, jb, jbe, jg, jge, jl, jle, jmp_absolute, jmp_relative, jnb, jno, jns, jnz, jo, jpe, jpo,
    js, jz,
};
use crate::cpu::operation::loops::{loop_while_equal, loop_while_not_zero, loops};
use crate::cpu::operation::mov::{
    mov_immediate_to_operand, mov_immediate_to_register, mov_memory_to_register,
    mov_operand_to_register, mov_operand_to_segment_register, mov_register_to_memory,
    mov_register_to_operand, mov_segment_register_to_operand,
};
use crate::cpu::operation::or::{
    or_immediate_to_register, or_operand_to_register, or_register_to_operand,
};
use crate::cpu::operation::pop::{pop_segment_register, pop_word_operand, pop_word_register};
use crate::cpu::operation::push::{push_segment_register, push_word_operand, push_word_register};
use crate::cpu::operation::ret::{ret, ret_word, retf, retf_word};
use crate::cpu::operation::sbb::{
    sbb_immediate_to_register, sbb_operand_to_register, sbb_register_to_operand,
};
use crate::cpu::operation::sub::{
    sub_immediate_to_register, sub_operand_to_register, sub_register_to_operand,
};
use crate::cpu::operation::xor::{
    xor_immediate_to_register, xor_operand_to_register, xor_register_to_operand,
};
use crate::cpu::register::{
    ByteSharedRegisterKind, IndexRegisterKind, IndexRegisters, PointerRegisterKind,
    PointerRegisters, SegmentRegisterKind, SegmentRegisters, SharedRegisters,
    WordSharedRegisterKind,
};
use std::cmp::PartialEq;
use std::fmt::Debug;
use crate::cpu::operation::call::call;
use crate::cpu::operation::group;

static PARITY_TABLE: [u8; 256] = [
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
];

pub enum FlagKind {
    O = 11,
    D = 10,
    I = 9,
    T = 8,
    S = 7,
    Z = 6,
    A = 4,
    P = 2,
    C = 0,
}

#[derive(PartialEq)]
enum AddressingModeKind {
    NoDisplacement = 0b00,
    ByteDisplacement = 0b01,
    WordDisplacement = 0b10,
    Register = 0b11,
}

impl TryFrom<u8> for AddressingModeKind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoDisplacement),
            1 => Ok(Self::ByteDisplacement),
            2 => Ok(Self::WordDisplacement),
            3 => Ok(Self::Register),
            _ => Err(()),
        }
    }
}

pub struct AddressingMode {
    kind: AddressingModeKind,
    register: u8,
    operand: u8,
}

impl AddressingMode {
    fn new(byte: u8) -> Self {
        let kind = AddressingModeKind::try_from((byte >> 6) & 0b11).unwrap();

        Self {
            kind,
            register: (byte >> 3) & 0b111,
            operand: byte & 0b111,
        }
    }

    fn get_segment_register_kind(&self) -> SegmentRegisterKind {
        match self.operand {
            2 | 3 => SegmentRegisterKind::SS,
            6 if self.kind != AddressingModeKind::NoDisplacement => SegmentRegisterKind::SS,
            _ => SegmentRegisterKind::DS,
        }
    }

    fn operand(&self, cpu: &mut Cpu) -> Operand {
        match self.kind {
            AddressingModeKind::Register => Operand::Register(self.operand),
            _ => Operand::Memory(cpu.physical_address_from_addressing_mode(self)),
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    shared_registers: SharedRegisters,
    index_registers: IndexRegisters,
    pointer_registers: PointerRegisters,
    flags: u16,
    segment_registers: SegmentRegisters,
    instruction_pointer: u16,
    bus: Bus,
    halted: bool,
    segment_register_override: Option<SegmentRegisterKind>,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            shared_registers: SharedRegisters::new(),
            index_registers: IndexRegisters::new(),
            pointer_registers: PointerRegisters::new(),
            flags: 0b0000000000000000,
            segment_registers: SegmentRegisters::new(),
            instruction_pointer: 0,
            bus,
            halted: false,
            segment_register_override: None,
        }
    }
}

impl Cpu {
    pub fn reset(&mut self) {
        self.reset_flags();

        self.halted = false;

        self.instruction_pointer = 0;

        self.segment_register_override = None;

        self.segment_registers.set(SegmentRegisterKind::CS, 0xFFFF);
        self.segment_registers.set(SegmentRegisterKind::DS, 0);
        self.segment_registers.set(SegmentRegisterKind::SS, 0);
        self.segment_registers.set(SegmentRegisterKind::ES, 0);
    }

    fn set_flag(&mut self, flag: FlagKind, value: bool) {
        let flag = flag as u16;
        self.flags = self.flags & !(1 << flag) | (u16::from(value) << flag);
    }

    fn toggle_flag(&mut self, flag: FlagKind) {
        let flag = flag as u16;
        self.flags = self.flags ^ (1 << flag);
    }

    fn get_flag(&self, flag: FlagKind) -> bool {
        self.flags & (1 << flag as u16) != 0
    }

    fn reset_flags(&mut self) {
        self.flags = 0;
    }

    fn physical_address(&self, segment: u16, offset: u16) -> u32 {
        ((segment as u32) << 4) + offset as u32
    }

    fn physical_address_from_addressing_mode(&mut self, addressing_mode: &AddressingMode) -> u32 {
        let address = self.effective_address(addressing_mode);

        let segment_register_kind = self
            .segment_register_override
            .unwrap_or_else(|| addressing_mode.get_segment_register_kind());

        let register_value = self.segment_registers.get(segment_register_kind);

        self.physical_address(register_value, address)
    }

    fn effective_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
        let base = match addressing_mode.kind {
            AddressingModeKind::NoDisplacement if addressing_mode.operand == 0b110 => 0,
            _ => match addressing_mode.operand {
                0 => self
                    .shared_registers
                    .get_word(WordSharedRegisterKind::BX)
                    .wrapping_add(self.index_registers.get(IndexRegisterKind::SI)),
                1 => self
                    .shared_registers
                    .get_word(WordSharedRegisterKind::BX)
                    .wrapping_add(self.index_registers.get(IndexRegisterKind::DI)),
                2 => self
                    .pointer_registers
                    .get(PointerRegisterKind::BP)
                    .wrapping_add(self.index_registers.get(IndexRegisterKind::SI)),
                3 => self
                    .pointer_registers
                    .get(PointerRegisterKind::BP)
                    .wrapping_add(self.index_registers.get(IndexRegisterKind::DI)),
                4 => self.index_registers.get(IndexRegisterKind::SI),
                5 => self.index_registers.get(IndexRegisterKind::DI),
                6 => self.pointer_registers.get(PointerRegisterKind::BP),
                7 => self.shared_registers.get_word(WordSharedRegisterKind::BX),
                _ => unreachable!(),
            },
        };

        let displacement = self.fetch_displacement(addressing_mode);

        base.wrapping_add(displacement)
    }

    fn fetch_displacement(&mut self, addressing_mode: &AddressingMode) -> u16 {
        match addressing_mode.kind {
            AddressingModeKind::NoDisplacement => {
                if addressing_mode.operand == 0b110 {
                    self.fetch_word()
                } else {
                    0
                }
            }
            AddressingModeKind::ByteDisplacement => (self.fetch_byte() as i8) as u16,
            AddressingModeKind::WordDisplacement => self.fetch_word(),
            AddressingModeKind::Register => 0,
        }
    }

    fn get_word_register(&self, index: u8) -> u16 {
        match index {
            0..=3 => self
                .shared_registers
                .get_word(WordSharedRegisterKind::try_from(index).unwrap()),

            4..=5 => self
                .pointer_registers
                .get(PointerRegisterKind::try_from(index - 4).unwrap()),

            6..=7 => self
                .index_registers
                .get(IndexRegisterKind::try_from(index - 6).unwrap()),

            _ => unreachable!(),
        }
    }

    fn get_byte_register(&self, index: u8) -> u8 {
        let kind = ByteSharedRegisterKind::try_from(index).unwrap();
        self.shared_registers.get_byte(kind)
    }

    fn set_word_register(&mut self, index: u8, value: u16) {
        match index {
            0..=3 => self
                .shared_registers
                .set_word(WordSharedRegisterKind::try_from(index).unwrap(), value),

            4..=5 => self
                .pointer_registers
                .set(PointerRegisterKind::try_from(index - 4).unwrap(), value),

            6..=7 => self
                .index_registers
                .set(IndexRegisterKind::try_from(index - 6).unwrap(), value),

            _ => unreachable!(),
        };
    }

    fn set_byte_register(&mut self, index: u8, value: u8) {
        let kind = ByteSharedRegisterKind::try_from(index).unwrap();
        self.shared_registers.set_byte(kind, value);
    }

    fn fetch_byte(&mut self) -> u8 {
        let code_segment = self.segment_registers.get(SegmentRegisterKind::CS);

        let address = self.physical_address(code_segment, self.instruction_pointer);
        let value = self.bus.read_byte_from_memory(address);

        self.instruction_pointer = self.instruction_pointer.wrapping_add(1);

        value
    }

    fn fetch_word(&mut self) -> u16 {
        let low_byte = self.fetch_byte();
        let high_byte = self.fetch_byte();

        u16::from_le_bytes([low_byte, high_byte])
    }

    fn fetch_value(&mut self, size: OperandSize) -> OperandValue {
        match size {
            OperandSize::Byte => OperandValue::Byte(self.fetch_byte()),
            OperandSize::Word => OperandValue::Word(self.fetch_word()),
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        loop {
            let operation_code = self.fetch_byte();

            match operation_code {
                0x26 => self.segment_register_override = Some(SegmentRegisterKind::ES),
                0x2E => self.segment_register_override = Some(SegmentRegisterKind::CS),
                0x36 => self.segment_register_override = Some(SegmentRegisterKind::SS),
                0x3E => self.segment_register_override = Some(SegmentRegisterKind::DS),
                _ => {
                    self.execute(operation_code);
                    break;
                }
            }
        }

        self.segment_register_override = None;
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    fn execute(&mut self, operation_code: u8) {
        match operation_code {
            0x00 => add_register_to_operand(self, OperandSize::Byte),
            0x01 => add_register_to_operand(self, OperandSize::Word),
            0x02 => add_operand_to_register(self, OperandSize::Byte),
            0x03 => add_operand_to_register(self, OperandSize::Word),
            0x04 => add_immediate_to_register(self, OperandSize::Byte),
            0x05 => add_immediate_to_register(self, OperandSize::Word),

            0x06 => push_segment_register(self, SegmentRegisterKind::ES),
            0x07 => pop_segment_register(self, SegmentRegisterKind::ES),

            0x08 => or_register_to_operand(self, OperandSize::Byte),
            0x09 => or_register_to_operand(self, OperandSize::Word),
            0x0A => or_operand_to_register(self, OperandSize::Byte),
            0x0B => or_operand_to_register(self, OperandSize::Word),
            0x0C => or_immediate_to_register(self, OperandSize::Byte),
            0x0D => or_immediate_to_register(self, OperandSize::Word),

            0x0E => push_segment_register(self, SegmentRegisterKind::CS),
            0x0F => pop_segment_register(self, SegmentRegisterKind::CS),

            0x10 => adc_register_to_operand(self, OperandSize::Byte),
            0x11 => adc_register_to_operand(self, OperandSize::Word),
            0x12 => adc_operand_to_register(self, OperandSize::Byte),
            0x13 => adc_operand_to_register(self, OperandSize::Word),
            0x14 => adc_immediate_to_register(self, OperandSize::Byte),
            0x15 => adc_immediate_to_register(self, OperandSize::Word),

            0x16 => push_segment_register(self, SegmentRegisterKind::SS),
            0x17 => pop_segment_register(self, SegmentRegisterKind::SS),

            0x18 => sbb_register_to_operand(self, OperandSize::Byte),
            0x19 => sbb_register_to_operand(self, OperandSize::Word),
            0x1A => sbb_operand_to_register(self, OperandSize::Byte),
            0x1B => sbb_operand_to_register(self, OperandSize::Word),
            0x1C => sbb_immediate_to_register(self, OperandSize::Byte),
            0x1D => sbb_immediate_to_register(self, OperandSize::Word),

            0x1E => push_segment_register(self, SegmentRegisterKind::DS),
            0x1F => pop_segment_register(self, SegmentRegisterKind::DS),

            0x20 => and_register_to_operand(self, OperandSize::Byte),
            0x21 => and_register_to_operand(self, OperandSize::Word),
            0x22 => and_operand_to_register(self, OperandSize::Byte),
            0x23 => and_operand_to_register(self, OperandSize::Word),
            0x24 => and_immediate_to_register(self, OperandSize::Byte),
            0x25 => and_immediate_to_register(self, OperandSize::Word),

            0x28 => sub_register_to_operand(self, OperandSize::Byte),
            0x29 => sub_register_to_operand(self, OperandSize::Word),
            0x2A => sub_operand_to_register(self, OperandSize::Byte),
            0x2B => sub_operand_to_register(self, OperandSize::Word),
            0x2C => sub_immediate_to_register(self, OperandSize::Byte),
            0x2D => sub_immediate_to_register(self, OperandSize::Word),

            0x30 => xor_register_to_operand(self, OperandSize::Byte),
            0x31 => xor_register_to_operand(self, OperandSize::Word),
            0x32 => xor_operand_to_register(self, OperandSize::Byte),
            0x33 => xor_operand_to_register(self, OperandSize::Word),
            0x34 => xor_immediate_to_register(self, OperandSize::Byte),
            0x35 => xor_immediate_to_register(self, OperandSize::Word),

            0x38 => cmp_register_to_operand(self, OperandSize::Byte),
            0x39 => cmp_register_to_operand(self, OperandSize::Word),
            0x3A => cmp_operand_to_register(self, OperandSize::Byte),
            0x3B => cmp_operand_to_register(self, OperandSize::Word),
            0x3C => cmp_immediate_to_register(self, OperandSize::Byte),
            0x3D => cmp_immediate_to_register(self, OperandSize::Word),

            0x40..=0x47 => inc_word_register(self, 0x40, operation_code),
            0x48..=0x4F => dec_word_register(self, 0x48, operation_code),

            0x50..=0x57 => push_word_register(self, 0x50, operation_code),
            0x58..=0x5F => pop_word_register(self, 0x58, operation_code),

            0x70 => jo(self),
            0x71 => jno(self),
            0x72 => jb(self),
            0x73 => jnb(self),
            0x74 => jz(self),
            0x75 => jnz(self),
            0x76 => jbe(self),
            0x77 => ja(self),
            0x78 => js(self),
            0x79 => jns(self),
            0x7A => jpe(self),
            0x7B => jpo(self),
            0x7C => jl(self),
            0x7D => jge(self),
            0x7E => jle(self),
            0x7F => jg(self),

            0x81 => group::one(self, OperandSize::Word),
            0x83 => group::one(self, OperandSize::Byte),

            0x88 => mov_register_to_operand(self, OperandSize::Byte),
            0x89 => mov_register_to_operand(self, OperandSize::Word),
            0x8A => mov_operand_to_register(self, OperandSize::Byte),
            0x8B => mov_operand_to_register(self, OperandSize::Word),
            0x8C => mov_segment_register_to_operand(self),
            0x8E => mov_operand_to_segment_register(self),

            0x8F => pop_word_operand(self),

            0x90 => {} //NOP
            
            0x9A => call(self),
            0x9B => {} //WAIT

            0xA0 => mov_memory_to_register(self, OperandSize::Byte),
            0xA1 => mov_memory_to_register(self, OperandSize::Word),
            0xA2 => mov_register_to_memory(self, OperandSize::Byte),
            0xA3 => mov_register_to_memory(self, OperandSize::Word),

            0xB0..=0xB7 => mov_immediate_to_register(self, OperandSize::Byte, operation_code, 0xB0),
            0xB8..=0xBF => mov_immediate_to_register(self, OperandSize::Word, operation_code, 0xB8),

            0xC2 => ret_word(self),
            0xC3 => ret(self),
                
            0xC6 => mov_immediate_to_operand(self, OperandSize::Byte),
            0xC7 => mov_immediate_to_operand(self, OperandSize::Word),

            0xCA => retf_word(self),
            0xCB => retf(self),

            0xE0 => loop_while_not_zero(self),
            0xE1 => loop_while_equal(self),
            0xE2 => loops(self),

            0xE9 => jmp_relative(self, OperandSize::Word),
            0xEA => jmp_absolute(self),
            0xEB => jmp_relative(self, OperandSize::Byte),

            0xF4 => halt(self),

            0xFF => push_word_operand(self),
            _ => panic!("Unimplemented instruction 0x{:02X}", operation_code),
        }
    }

    pub fn get_registers(
        &self,
    ) -> (
        &SharedRegisters,
        &IndexRegisters,
        &PointerRegisters,
        &SegmentRegisters,
    ) {
        (
            &self.shared_registers,
            &self.index_registers,
            &self.pointer_registers,
            &self.segment_registers,
        )
    }
}

fn halt(cpu: &mut Cpu) {
    cpu.halted = true;
}
