pub mod flag;
pub mod operand;
pub mod operation;
pub mod register;

use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::group::ShiftSource;
use crate::cpu::operation::interrupt::{interrupt, interrupt_immediate};
use crate::cpu::operation::{
    add, add_carry, and, call, clear, compare, decrement, group, increment, input, jump, load,
    loops, mov, or, out, pop, push, ret, set, subtract, subtract_borrow, test, xor,
};
use crate::cpu::register::{
    ByteSharedRegisterKind, IndexRegisterKind, IndexRegisters, PointerRegisterKind,
    PointerRegisters, SegmentRegisterKind, SegmentRegisters, SharedRegisters,
    WordSharedRegisterKind,
};
use std::cmp::PartialEq;
use std::fmt::Debug;

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

    fn operand(&self, cpu: &mut Cpu, bus: &mut Bus) -> Operand {
        match self.kind {
            AddressingModeKind::Register => Operand::Register(self.operand),
            _ => Operand::Memory(cpu.physical_address_from_addressing_mode(bus, self)),
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
    halted: bool,
    segment_register_override: Option<SegmentRegisterKind>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            shared_registers: SharedRegisters::new(),
            index_registers: IndexRegisters::new(),
            pointer_registers: PointerRegisters::new(),
            flags: 0b0000000000000000,
            segment_registers: SegmentRegisters::new(),
            instruction_pointer: 0,
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

    fn physical_address_from_addressing_mode(
        &mut self,
        bus: &mut Bus,
        addressing_mode: &AddressingMode,
    ) -> u32 {
        let address = self.effective_address(bus, addressing_mode);

        let segment_register_kind = self
            .segment_register_override
            .unwrap_or_else(|| addressing_mode.get_segment_register_kind());

        let register_value = self.segment_registers.get(segment_register_kind);

        self.physical_address(register_value, address)
    }

    fn effective_address(&mut self, bus: &mut Bus, addressing_mode: &AddressingMode) -> u16 {
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

        let displacement = self.fetch_displacement(bus, addressing_mode);

        base.wrapping_add(displacement)
    }

    fn fetch_displacement(&mut self, bus: &mut Bus, addressing_mode: &AddressingMode) -> u16 {
        match addressing_mode.kind {
            AddressingModeKind::NoDisplacement => {
                if addressing_mode.operand == 0b110 {
                    self.fetch_word(bus)
                } else {
                    0
                }
            }
            AddressingModeKind::ByteDisplacement => (self.fetch_byte(bus) as i8) as u16,
            AddressingModeKind::WordDisplacement => self.fetch_word(bus),
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

    fn fetch_byte(&mut self, bus: &mut Bus) -> u8 {
        let code_segment = self.segment_registers.get(SegmentRegisterKind::CS);

        let address = self.physical_address(code_segment, self.instruction_pointer);
        let value = bus.read_byte_from_memory(address);

        self.instruction_pointer = self.instruction_pointer.wrapping_add(1);

        value
    }

    fn fetch_sign_extended_byte(&mut self, bus: &mut Bus) -> u16 {
        self.fetch_byte(bus) as i8 as u16
    }

    fn fetch_word(&mut self, bus: &mut Bus) -> u16 {
        let low_byte = self.fetch_byte(bus);
        let high_byte = self.fetch_byte(bus);

        u16::from_le_bytes([low_byte, high_byte])
    }

    fn fetch_value(&mut self, bus: &mut Bus, size: OperandSize) -> OperandValue {
        match size {
            OperandSize::Byte => OperandValue::Byte(self.fetch_byte(bus)),
            OperandSize::Word => OperandValue::Word(self.fetch_word(bus)),
        }
    }

    pub fn can_accept_interrupt(&self) -> bool {
        self.get_flag(FlagKind::I)
    }

    pub fn handle_interrupt(&mut self, bus: &mut Bus, vector: u8) {
        self.halted = false;
        interrupt(self, bus, vector);
    }

    pub fn step(&mut self, bus: &mut Bus) {
        if self.can_accept_interrupt() {
            if let Some(vector) = bus.acknowledge_interrupt() {
                self.handle_interrupt(bus, vector);
            }
        }

        if self.halted {
            return;
        }

        loop {
            let operation_code = self.fetch_byte(bus);

            match operation_code {
                0x26 => self.segment_register_override = Some(SegmentRegisterKind::ES),
                0x2E => self.segment_register_override = Some(SegmentRegisterKind::CS),
                0x36 => self.segment_register_override = Some(SegmentRegisterKind::SS),
                0x3E => self.segment_register_override = Some(SegmentRegisterKind::DS),
                _ => {
                    self.execute(bus, operation_code);
                    break;
                }
            }
        }

        self.segment_register_override = None;
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    fn execute(&mut self, bus: &mut Bus, operation_code: u8) {
        match operation_code {
            0x00 => add::register_to_operand(self, bus, OperandSize::Byte),
            0x01 => add::register_to_operand(self, bus, OperandSize::Word),
            0x02 => add::operand_to_register(self, bus, OperandSize::Byte),
            0x03 => add::operand_to_register(self, bus, OperandSize::Word),
            0x04 => add::immediate_to_register(self, bus, OperandSize::Byte),
            0x05 => add::immediate_to_register(self, bus, OperandSize::Word),

            0x06 => push::segment_register(self, bus, SegmentRegisterKind::ES),
            0x07 => pop::segment_register(self, bus, SegmentRegisterKind::ES),

            0x08 => or::register_to_operand(self, bus, OperandSize::Byte),
            0x09 => or::register_to_operand(self, bus, OperandSize::Word),
            0x0A => or::operand_to_register(self, bus, OperandSize::Byte),
            0x0B => or::operand_to_register(self, bus, OperandSize::Word),
            0x0C => or::immediate_to_register(self, bus, OperandSize::Byte),
            0x0D => or::immediate_to_register(self, bus, OperandSize::Word),

            0x0E => push::segment_register(self, bus, SegmentRegisterKind::CS),
            0x0F => pop::segment_register(self, bus, SegmentRegisterKind::CS),

            0x10 => add_carry::register_to_operand(self, bus, OperandSize::Byte),
            0x11 => add_carry::register_to_operand(self, bus, OperandSize::Word),
            0x12 => add_carry::operand_to_register(self, bus, OperandSize::Byte),
            0x13 => add_carry::operand_to_register(self, bus, OperandSize::Word),
            0x14 => add_carry::immediate_to_register(self, bus, OperandSize::Byte),
            0x15 => add_carry::immediate_to_register(self, bus, OperandSize::Word),

            0x16 => push::segment_register(self, bus, SegmentRegisterKind::SS),
            0x17 => pop::segment_register(self, bus, SegmentRegisterKind::SS),

            0x18 => subtract_borrow::register_to_operand(self, bus, OperandSize::Byte),
            0x19 => subtract_borrow::register_to_operand(self, bus, OperandSize::Word),
            0x1A => subtract_borrow::operand_to_register(self, bus, OperandSize::Byte),
            0x1B => subtract_borrow::operand_to_register(self, bus, OperandSize::Word),
            0x1C => subtract_borrow::immediate_to_register(self, bus, OperandSize::Byte),
            0x1D => subtract_borrow::immediate_to_register(self, bus, OperandSize::Word),

            0x1E => push::segment_register(self, bus, SegmentRegisterKind::DS),
            0x1F => pop::segment_register(self, bus, SegmentRegisterKind::DS),

            0x20 => and::register_to_operand(self, bus, OperandSize::Byte),
            0x21 => and::register_to_operand(self, bus, OperandSize::Word),
            0x22 => and::operand_to_register(self, bus, OperandSize::Byte),
            0x23 => and::operand_to_register(self, bus, OperandSize::Word),
            0x24 => and::immediate_to_register(self, bus, OperandSize::Byte),
            0x25 => and::immediate_to_register(self, bus, OperandSize::Word),

            0x28 => subtract::register_to_operand(self, bus, OperandSize::Byte),
            0x29 => subtract::register_to_operand(self, bus, OperandSize::Word),
            0x2A => subtract::operand_to_register(self, bus, OperandSize::Byte),
            0x2B => subtract::operand_to_register(self, bus, OperandSize::Word),
            0x2C => subtract::immediate_to_register(self, bus, OperandSize::Byte),
            0x2D => subtract::immediate_to_register(self, bus, OperandSize::Word),

            0x30 => xor::register_to_operand(self, bus, OperandSize::Byte),
            0x31 => xor::register_to_operand(self, bus, OperandSize::Word),
            0x32 => xor::operand_to_register(self, bus, OperandSize::Byte),
            0x33 => xor::operand_to_register(self, bus, OperandSize::Word),
            0x34 => xor::immediate_to_register(self, bus, OperandSize::Byte),
            0x35 => xor::immediate_to_register(self, bus, OperandSize::Word),

            0x38 => compare::register_to_operand(self, bus, OperandSize::Byte),
            0x39 => compare::register_to_operand(self, bus, OperandSize::Word),
            0x3A => compare::operand_to_register(self, bus, OperandSize::Byte),
            0x3B => compare::operand_to_register(self, bus, OperandSize::Word),
            0x3C => compare::immediate_to_register(self, bus, OperandSize::Byte),
            0x3D => compare::immediate_to_register(self, bus, OperandSize::Word),

            0x40..=0x47 => increment::word_register(self, bus, 0x40, operation_code),
            0x48..=0x4F => decrement::word_register(self, bus, 0x48, operation_code),

            0x50..=0x57 => push::word_register(self, bus, 0x50, operation_code),
            0x58..=0x5F => pop::word_register(self, bus, 0x58, operation_code),

            0x70 => jump::if_overflow(self, bus),
            0x71 => jump::if_not_overflow(self, bus),
            0x72 => jump::if_below(self, bus),
            0x73 => jump::if_not_below(self, bus),
            0x74 => jump::if_zero(self, bus),
            0x75 => jump::if_not_zero(self, bus),
            0x76 => jump::if_below_or_equal(self, bus),
            0x77 => jump::if_above(self, bus),
            0x78 => jump::if_sign(self, bus),
            0x79 => jump::if_not_sign(self, bus),
            0x7A => jump::if_parity_even(self, bus),
            0x7B => jump::if_parity_odd(self, bus),
            0x7C => jump::if_less(self, bus),
            0x7D => jump::if_greater_or_equal(self, bus),
            0x7E => jump::if_less_or_equal(self, bus),
            0x7F => jump::if_greater(self, bus),

            0x81 => group::one(self, bus, OperandSize::Word),
            0x83 => group::one(self, bus, OperandSize::Byte),

            0x84 => test::register_and_operand(self, bus, OperandSize::Byte),
            0x85 => test::register_and_operand(self, bus, OperandSize::Word),

            0x88 => mov::register_to_operand(self, bus, OperandSize::Byte),
            0x89 => mov::register_to_operand(self, bus, OperandSize::Word),
            0x8A => mov::operand_to_register(self, bus, OperandSize::Byte),
            0x8B => mov::operand_to_register(self, bus, OperandSize::Word),
            0x8C => mov::segment_register_to_operand(self, bus),
            0x8E => mov::operand_to_segment_register(self, bus),

            0x8F => pop::word_operand(self, bus),

            0x90 => {} //NOP

            0x9A => call::absolute(self, bus),
            0x9B => {} //WAIT

            0x9C => push::flags(self, bus),
            0x9D => pop::flags(self, bus),

            0xA0 => mov::memory_to_register(self, bus, OperandSize::Byte),
            0xA1 => mov::memory_to_register(self, bus, OperandSize::Word),
            0xA2 => mov::register_to_memory(self, bus, OperandSize::Byte),
            0xA3 => mov::register_to_memory(self, bus, OperandSize::Word),

            0xA8 => test::register_and_immediate(self, bus, OperandSize::Byte),
            0xA9 => test::register_and_immediate(self, bus, OperandSize::Word),

            0xAC => load::string(self, bus, OperandSize::Byte),
            0xAD => load::string(self, bus, OperandSize::Word),

            0xB0..=0xB7 => {
                mov::immediate_to_register(self, bus, OperandSize::Byte, operation_code, 0xB0)
            }
            0xB8..=0xBF => {
                mov::immediate_to_register(self, bus, OperandSize::Word, operation_code, 0xB8)
            }

            0xC2 => ret::near_with_stack_adjustment(self, bus),
            0xC3 => ret::near(self, bus),

            0xC6 => mov::immediate_to_operand(self, bus, OperandSize::Byte),
            0xC7 => mov::immediate_to_operand(self, bus, OperandSize::Word),

            0xCA => ret::far_with_stack_adjustment(self, bus),
            0xCB => ret::far(self, bus),

            0xCC => interrupt(self, bus, 3),
            0xCD => interrupt_immediate(self, bus),

            0xCF => ret::from_interrupt(self, bus),

            0xD0 => group::two(self, bus, OperandSize::Byte, ShiftSource::One),
            0xD1 => group::two(self, bus, OperandSize::Word, ShiftSource::One),
            0xD2 => group::two(self, bus, OperandSize::Byte, ShiftSource::Register),
            0xD3 => group::two(self, bus, OperandSize::Word, ShiftSource::Register),

            0xE0 => loops::while_not_zero(self, bus),
            0xE1 => loops::while_equal(self, bus),
            0xE2 => loops::counted(self, bus),

            0xE4 => input::from_immediate_port(self, bus, OperandSize::Byte),
            0xE5 => input::from_immediate_port(self, bus, OperandSize::Word),
            0xE6 => out::to_immediate_port(self, bus, OperandSize::Byte),
            0xE7 => out::to_immediate_port(self, bus, OperandSize::Word),

            0xE8 => call::relative(self, bus),

            0xE9 => jump::relative(self, bus, OperandSize::Word),
            0xEA => jump::absolute(self, bus),
            0xEB => jump::relative(self, bus, OperandSize::Byte),

            0xEC => input::from_port(self, bus, OperandSize::Byte),
            0xED => input::from_port(self, bus, OperandSize::Word),
            0xEE => out::to_port(self, bus, OperandSize::Byte),
            0xEF => out::to_port(self, bus, OperandSize::Word),

            0xF4 => halt(self),

            0xF6 => group::three(self, bus, OperandSize::Byte),
            0xF7 => group::three(self, bus, OperandSize::Word),

            0xF8 => clear::carry_flag(self),
            0xF9 => set::carry_flag(self),
            0xFA => clear::interrupt_flag(self),
            0xFB => set::interrupt_flag(self),
            0xFC => clear::direction_flag(self),
            0xFD => set::direction_flag(self),

            0xFE => group::four(self, bus),
            0xFF => group::five(self, bus),

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
