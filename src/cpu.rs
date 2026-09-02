use crate::bus::Bus;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct SharedRegisters {
    bytes: [u8; 8],
}

impl SharedRegisters {
    fn new() -> Self {
        Self { bytes: [0; 8] }
    }

    fn set_byte(&mut self, kind: ByteSharedRegisterKind, value: u8) {
        self.bytes[kind as usize] = value;
    }

    fn get_byte(&self, kind: ByteSharedRegisterKind) -> u8 {
        self.bytes[kind as usize]
    }

    pub fn set_word(&mut self, kind: WordSharedRegisterKind, value: u16) {
        let word_index = kind as usize;
        let byte_index = word_index + 4;

        self.bytes[word_index] = value as u8;
        self.bytes[byte_index] = (value >> 8) as u8;
    }

    pub fn get_word(&self, kind: WordSharedRegisterKind) -> u16 {
        let word_index = kind as usize;
        let byte_index = word_index + 4;

        let low_byte = self.bytes[word_index] as u16;
        let high_byte = self.bytes[byte_index] as u16;

        low_byte | (high_byte << 8)
    }
}

pub enum WordSharedRegisterKind {
    AX = 0,
    CX = 1,
    DX = 2,
    BX = 3,
}

impl TryFrom<u8> for WordSharedRegisterKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AX),
            1 => Ok(Self::CX),
            2 => Ok(Self::DX),
            3 => Ok(Self::BX),
            _ => Err(()),
        }
    }
}

pub enum ByteSharedRegisterKind {
    AL = 0,
    CL = 1,
    DL = 2,
    BL = 3,
    AH = 4,
    CH = 5,
    DH = 6,
    BH = 7,
}

impl TryFrom<u8> for ByteSharedRegisterKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AL),
            1 => Ok(Self::CL),
            2 => Ok(Self::DL),
            3 => Ok(Self::BL),
            4 => Ok(Self::AH),
            5 => Ok(Self::CH),
            6 => Ok(Self::DH),
            7 => Ok(Self::BH),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct IndexRegisters {
    registers: [u16; 2],
}

impl IndexRegisters {
    fn new() -> Self {
        Self { registers: [0; 2] }
    }

    fn set(&mut self, kind: IndexRegisterKind, value: u16) {
        self.registers[kind as usize] = value;
    }

    fn get(&self, kind: IndexRegisterKind) -> u16 {
        self.registers[kind as usize]
    }
}

pub enum IndexRegisterKind {
    SI = 0,
    DI = 1,
}

impl TryFrom<u8> for IndexRegisterKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SI),
            1 => Ok(Self::DI),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct PointerRegisters {
    registers: [u16; 2],
}

impl PointerRegisters {
    fn new() -> Self {
        Self { registers: [0; 2] }
    }

    fn set(&mut self, kind: PointerRegisterKind, value: u16) {
        self.registers[kind as usize] = value;
    }

    fn get(&self, kind: PointerRegisterKind) -> u16 {
        self.registers[kind as usize]
    }
}

pub enum PointerRegisterKind {
    SP = 0,
    BP = 1,
}

impl TryFrom<u8> for PointerRegisterKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SP),
            1 => Ok(Self::BP),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct SegmentRegisters {
    registers: [u16; 4],
}

impl SegmentRegisters {
    fn new() -> Self {
        Self { registers: [0; 4] }
    }

    fn set(&mut self, kind: SegmentRegisterKind, value: u16) {
        self.registers[kind as usize] = value;
    }

    fn get(&self, kind: SegmentRegisterKind) -> u16 {
        self.registers[kind as usize]
    }
}

pub enum SegmentRegisterKind {
    CS = 0,
    DS = 1,
    ES = 2,
    SS = 3,
}

impl TryFrom<u8> for SegmentRegisterKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ES),
            1 => Ok(Self::CS),
            2 => Ok(Self::SS),
            3 => Ok(Self::DS),
            _ => Err(()),
        }
    }
}

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

enum OperandValue {
    Byte(u8),
    Word(u16),
}

enum OperandSize {
    Byte,
    Word,
}

enum Operand {
    Memory(u32),
    Register(u8),
}

impl Operand {
    fn read(self, cpu: &mut Cpu, size: OperandSize) -> OperandValue {
        match self {
            Operand::Register(register) => match size {
                OperandSize::Byte => OperandValue::Byte(cpu.get_byte_register(register)),
                OperandSize::Word => OperandValue::Word(cpu.get_word_register(register)),
            },

            Operand::Memory(address) => match size {
                OperandSize::Byte => OperandValue::Byte(cpu.bus.read_byte_from_memory(address)),
                OperandSize::Word => OperandValue::Word(cpu.bus.read_word_from_memory(address)),
            },
        }
    }

    fn write(self, cpu: &mut Cpu, value: OperandValue) {
        match self {
            Operand::Register(register) => match value {
                OperandValue::Byte(value) => cpu.set_byte_register(register, value),
                OperandValue::Word(value) => cpu.set_word_register(register, value),
            },

            Operand::Memory(address) => match value {
                OperandValue::Byte(value) => cpu.bus.write_byte_to_memory(address, value),
                OperandValue::Word(value) => cpu.bus.write_word_to_memory(address, value),
            },
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
        }
    }
}

impl Cpu {
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

    fn physical_address(&self, segment: u16, offset: u16) -> u32 {
        ((segment as u32) << 4) + offset as u32
    }

    fn physical_address_from_addressing_mode(&mut self, addressing_mode: &AddressingMode) -> u32 {
        let address = self.effective_address(addressing_mode);

        let segment_register_kind = addressing_mode.get_segment_register_kind();
        let offset = self.segment_registers.get(segment_register_kind);

        self.physical_address(offset, address)
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

    pub fn step(&mut self) {
        let operation_code = self.fetch_byte();

        self.execute(operation_code);
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    fn execute(&mut self, operation_code: u8) {
        if self.halted {
            return;
        }

        match operation_code {
            0x06 => push_from_segment_register(self, SegmentRegisterKind::ES),
            0x07 => pop_segment_register(self, SegmentRegisterKind::ES),
            0x16 => push_from_segment_register(self, SegmentRegisterKind::SS),
            0x17 => pop_segment_register(self, SegmentRegisterKind::SS),
            0x0E => push_from_segment_register(self, SegmentRegisterKind::CS),
            0x1E => push_from_segment_register(self, SegmentRegisterKind::DS),
            0x1F => pop_segment_register(self, SegmentRegisterKind::DS),
            0x50..=0x57 => push_from_word_register(self, 0x50, operation_code),
            0x58..=0x5F => pop_from_word_register(self, 0x58, operation_code),
            0x88 => mov_register_to_operand(self, OperandSize::Byte),
            0x89 => mov_register_to_operand(self, OperandSize::Word),
            0x8A => mov_operand_to_register(self, OperandSize::Byte),
            0x8B => mov_operand_to_register(self, OperandSize::Word),
            0x8C => mov_segment_register_to_operand(self),
            0x8E => mov_operand_to_segment_register(self),
            0xB0..=0xB7 => mov_from_immediate_to_register(
                self,
                operation_code,
                0xB0,
                Self::fetch_byte,
                |cpu, kind, value| cpu.shared_registers.set_byte(kind, value),
            ),
            0xB8..=0xBB => mov_from_immediate_to_register(
                self,
                operation_code,
                0xB8,
                Self::fetch_word,
                |cpu, kind, value| cpu.shared_registers.set_word(kind, value),
            ),
            0xBC..=0xBD => mov_from_immediate_to_register(
                self,
                operation_code,
                0xBC,
                Self::fetch_word,
                |cpu, kind, value| cpu.pointer_registers.set(kind, value),
            ),
            0xBE..=0xBF => mov_from_immediate_to_register(
                self,
                operation_code,
                0xBE,
                Self::fetch_word,
                |cpu, kind, value| cpu.index_registers.set(kind, value),
            ),
            0xF4 => halt(self),
            _ => panic!("Unimplemented instruction 0x{:02X}", operation_code),
        }
    }
}

fn push(cpu: &mut Cpu, value: u16) {
    let stack_pointer_value = cpu
        .pointer_registers
        .get(PointerRegisterKind::SP)
        .wrapping_sub(2);

    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer_value);

    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::SS);
    let physical_address = cpu.physical_address(segment_register, stack_pointer_value);

    cpu.bus.write_word_to_memory(physical_address, value);
}

fn push_from_segment_register(cpu: &mut Cpu, register_kind: SegmentRegisterKind) {
    let register_value = cpu.segment_registers.get(register_kind);
    push(cpu, register_value);
}

fn push_from_word_register(cpu: &mut Cpu, base: u8, operation_code: u8) {
    let register_index = operation_code - base;
    let register_value = cpu.get_word_register(register_index);

    push(cpu, register_value);
}

fn pop_segment_register(cpu: &mut Cpu, register_kind: SegmentRegisterKind) {
    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP);
    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::SS);

    let address = cpu.physical_address(segment_register, stack_pointer);
    let temporary_value = cpu.bus.read_word_from_memory(address);

    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer.wrapping_add(2));

    cpu.segment_registers.set(register_kind, temporary_value);
}

fn pop_from_word_register(cpu: &mut Cpu, base: u8, operation_code: u8) {
    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP);
    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::SS);

    let address = cpu.physical_address(segment_register, stack_pointer);
    let temporary_value = cpu.bus.read_word_from_memory(address);

    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer.wrapping_add(2));

    let register_index = operation_code - base;
    cpu.set_word_register(register_index, temporary_value);
}

fn mov_from_immediate_to_register<V, K>(
    cpu: &mut Cpu,
    operation_code: u8,
    base: u8,
    fetch_function: fn(&mut Cpu) -> V,
    set_register_function: fn(&mut Cpu, K, V),
) where
    K: TryFrom<u8>,
    K::Error: std::fmt::Debug,
{
    let register_index = operation_code - base;
    let register_kind = K::try_from(register_index).unwrap();

    let value = fetch_function(cpu);
    set_register_function(cpu, register_kind, value);
}

fn mov_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let value = register.read(cpu, size);
    operand.write(cpu, value);
}

fn mov_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let register = Operand::Register(addressing_mode.register);

    let value = operand.read(cpu, size);
    register.write(cpu, value);
}

fn mov_segment_register_to_operand(cpu: &mut Cpu) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let segment_register_kind = SegmentRegisterKind::try_from(addressing_mode.register).unwrap();
    let value = OperandValue::Word(cpu.segment_registers.get(segment_register_kind));

    let operand = addressing_mode.operand(cpu);
    operand.write(cpu, value);
}

fn mov_operand_to_segment_register(cpu: &mut Cpu) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let value = operand.read(cpu, OperandSize::Word);

    let segment_register_kind = SegmentRegisterKind::try_from(addressing_mode.register).unwrap();

    if let OperandValue::Word(value) = value {
        cpu.segment_registers.set(segment_register_kind, value);
    }
}

fn halt(cpu: &mut Cpu) {
    cpu.halted = true;
}
