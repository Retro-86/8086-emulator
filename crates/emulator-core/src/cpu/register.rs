use std::convert::TryFrom;

#[derive(Debug)]
pub struct SharedRegisters {
    bytes: [u8; 8],
}

impl SharedRegisters {
    pub fn new() -> Self {
        Self { bytes: [0; 8] }
    }

    pub fn set_byte(&mut self, kind: ByteSharedRegisterKind, value: u8) {
        self.bytes[kind as usize] = value;
    }

    pub fn get_byte(&self, kind: ByteSharedRegisterKind) -> u8 {
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
    pub fn new() -> Self {
        Self { registers: [0; 2] }
    }

    pub fn set(&mut self, kind: IndexRegisterKind, value: u16) {
        self.registers[kind as usize] = value;
    }

    pub fn get(&self, kind: IndexRegisterKind) -> u16 {
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
    pub fn new() -> Self {
        Self { registers: [0; 2] }
    }

    pub fn set(&mut self, kind: PointerRegisterKind, value: u16) {
        self.registers[kind as usize] = value;
    }

    pub fn get(&self, kind: PointerRegisterKind) -> u16 {
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
    pub fn new() -> Self {
        Self { registers: [0; 4] }
    }

    pub fn set(&mut self, kind: SegmentRegisterKind, value: u16) {
        self.registers[kind as usize] = value;
    }

    pub fn get(&self, kind: SegmentRegisterKind) -> u16 {
        self.registers[kind as usize]
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
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
