use crate::memory::Memory;

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
        let high_byte_index = kind as usize;
        let low_byte_index = high_byte_index * 2;

        self.bytes[low_byte_index] = value as u8;
        self.bytes[low_byte_index + 1] = (value >> 8) as u8;
    }

    pub fn get_word(&self, kind: WordSharedRegisterKind) -> u16 {
        let high_byte_index = kind as usize;
        let low_byte_index = high_byte_index * 2;

        let low_byte = self.bytes[low_byte_index] as u16;
        let high_byte = self.bytes[low_byte_index + 1] as u16;

        low_byte | (high_byte << 8)
    }
}

pub enum WordSharedRegisterKind {
    AX = 0,
    BX = 1,
    CX = 2,
    DX = 3,
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
    BP = 0,
    SP = 1,
}

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

pub struct Cpu {
    shared_registers: SharedRegisters,
    index_registers: IndexRegisters,
    pointer_registers: PointerRegisters,
    flags: u16,
    segment_registers: SegmentRegisters,
    instruction_pointer: u16,

    memory: Memory
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            shared_registers: SharedRegisters::new(),
            index_registers: IndexRegisters::new(),
            pointer_registers: PointerRegisters::new(),
            flags: 0b1000000000000000,
            segment_registers: SegmentRegisters::new(),
            instruction_pointer: 0,
            memory
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

    pub fn execute(&mut self, operation_code: u16) {}
}
