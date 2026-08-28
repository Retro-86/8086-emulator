use crate::memory::Memory;

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
        let byte_index = word_index * 2;

        self.bytes[byte_index] = value as u8;
        self.bytes[byte_index + 1] = (value >> 8) as u8;
    }

    pub fn get_word(&self, kind: WordSharedRegisterKind) -> u16 {
        let word_index = kind as usize;
        let byte_index = word_index * 2;

        let low_byte = self.bytes[byte_index] as u16;
        let high_byte = self.bytes[byte_index + 1] as u16;

        low_byte | (high_byte << 8)
    }
}

pub enum WordSharedRegisterKind {
    AX = 0,
    BX = 1,
    CX = 2,
    DX = 3,
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
    AH = 1,
    BL = 2,
    BH = 3,
    CL = 4,
    CH = 5,
    DL = 6,
    DH = 7,
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

pub enum DataUnitType {
    BYTE,
    WORD,
}

#[derive(Debug)]
pub struct Cpu {
    shared_registers: SharedRegisters,
    index_registers: IndexRegisters,
    pointer_registers: PointerRegisters,
    flags: u16,
    segment_registers: SegmentRegisters,
    instruction_pointer: u16,
    memory: Memory,
    halted: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            shared_registers: SharedRegisters::new(),
            index_registers: IndexRegisters::new(),
            pointer_registers: PointerRegisters::new(),
            flags: 0b0000000000000000,
            segment_registers: SegmentRegisters::new(),
            instruction_pointer: 0,
            memory,
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

    fn fetch_byte(&mut self) -> u8 {
        let code_segment = self.segment_registers.get(SegmentRegisterKind::CS);

        let address = self.physical_address(code_segment, self.instruction_pointer);
        let value = self.memory.read_byte(address);

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

    pub fn execute(&mut self, operation_code: u8) {
        match operation_code {
            0xB0..=0xB7 => mov(
                self,
                operation_code,
                0xB0,
                Self::fetch_byte,
                |cpu, kind, value| cpu.shared_registers.set_byte(kind, value),
            ),
            0xB8..=0xBB => mov(
                self,
                operation_code,
                0xB8,
                Self::fetch_word,
                |cpu, kind, value| cpu.shared_registers.set_word(kind, value),
            ),
            0xBC..=0xBD => mov(
                self,
                operation_code,
                0xBC,
                Self::fetch_word,
                |cpu, kind, value| cpu.pointer_registers.set(kind, value),
            ),
            0xBE..=0xBF => mov(
                self,
                operation_code,
                0xBE,
                Self::fetch_word,
                |cpu, kind, value| cpu.index_registers.set(kind, value),
            ),
            _ => {}
        }
    }
}

fn mov<V, K>(
    cpu: &mut Cpu,
    operation_code: u8,
    base: u8,
    fetch_function: fn(&mut Cpu) -> V,
    set_function: fn(&mut Cpu, K, V),
) where
    K: TryFrom<u8>,
    K::Error: std::fmt::Debug,
{
    let register_index = operation_code - base;
    let register_kind = K::try_from(register_index).unwrap();

    let value = fetch_function(cpu);
    set_function(cpu, register_kind, value);
}
