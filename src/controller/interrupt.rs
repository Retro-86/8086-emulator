#[derive(Debug, Clone, Copy)]
enum InitializationStage {
    Ready,
    WaitingCommandWordTwo { needs_three: bool, needs_four: bool },
    WaitingCommandWordThree { needs_four: bool },
    WaitingCommandWordFour,
}

#[derive(Debug)]
enum ReadMode {
    RequestRegister,
    InServiceRegister,
}

#[derive(Debug)]
pub struct ProgrammableInterruptController {
    request_register: u8,
    in_service_register: u8,
    mask_register: u8,

    vector_offset: u8,
    auto_end: bool,

    initialization_stage: InitializationStage,
    read_mode: ReadMode,
}

impl ProgrammableInterruptController {
    pub fn new() -> Self {
        Self {
            request_register: 0,
            in_service_register: 0,
            mask_register: 1,
            vector_offset: 0,
            auto_end: false,
            initialization_stage: InitializationStage::Ready,
            read_mode: ReadMode::RequestRegister,
        }
    }

    pub fn initialize(&mut self, configuration: u8) {
        self.request_register = 0;
        self.in_service_register = 0;
        self.mask_register = 0;
        self.auto_end = false;
        self.read_mode = ReadMode::RequestRegister;

        self.initialization_stage = InitializationStage::WaitingCommandWordTwo {
            needs_three: configuration & 0x02 == 0,
            needs_four: configuration & 0x01 != 0,
        };
    }

    pub fn write(&mut self, port: u16, value: u8) {
        match port {
            0x20 => self.write_command(value),
            0x21 => self.write_data(value),
            _ => panic!("Unknown port {port:#x}"),
        }
    }

    fn write_command(&mut self, value: u8) {
        if value & 0x10 != 0 {
            self.initialize(value);
        } else if value & 0x08 != 0 {
            self.write_operation_command_three(value);
        } else {
            self.write_operation_command_two(value);
        }
    }

    fn write_operation_command_three(&mut self, value: u8) {
        if value & 0x02 != 0 {
            self.read_mode = if value & 0x01 != 0 {
                ReadMode::InServiceRegister
            } else {
                ReadMode::RequestRegister
            };
        }
    }

    fn write_operation_command_two(&mut self, value: u8) {
        match value & 0xE0 {
            0x20 if self.in_service_register != 0 => {
                self.end(self.in_service_register.trailing_zeros() as u8);
            }
            0x60 => self.end(value & 7),
            _ => {}
        }
    }

    fn write_data(&mut self, value: u8) {
        match self.initialization_stage {
            InitializationStage::Ready => self.mask_register = value,
            InitializationStage::WaitingCommandWordTwo {
                needs_three,
                needs_four,
            } => self.write_command_word_two(value, needs_three, needs_four),
            InitializationStage::WaitingCommandWordThree { needs_four } => {
                self.write_command_word_three(needs_four);
            }
            InitializationStage::WaitingCommandWordFour => self.write_command_word_four(value),
        }
    }

    fn write_command_word_two(&mut self, value: u8, needs_three: bool, needs_four: bool) {
        self.vector_offset = value & 0xF8;
        self.initialization_stage = if needs_three {
            InitializationStage::WaitingCommandWordThree { needs_four }
        } else if needs_four {
            InitializationStage::WaitingCommandWordFour
        } else {
            InitializationStage::Ready
        };
    }

    fn write_command_word_three(&mut self, needs_four: bool) {
        self.initialization_stage = if needs_four {
            InitializationStage::WaitingCommandWordFour
        } else {
            InitializationStage::Ready
        };
    }

    fn write_command_word_four(&mut self, value: u8) {
        self.auto_end = value & 0x02 != 0;
        self.initialization_stage = InitializationStage::Ready;
    }

    pub fn read(&self, port: u16) -> u8 {
        match port {
            0x20 => match &self.read_mode {
                ReadMode::RequestRegister => self.request_register,
                ReadMode::InServiceRegister => self.in_service_register,
            },
            0x21 => self.mask_register,
            _ => panic!("Unknown port {port:#x}"),
        }
    }

    pub fn request(&mut self, line: u8) {
        assert!(line < 8);
        self.request_register |= 1 << line;
    }

    pub fn get_pending(&self) -> Option<u8> {
        let pending = self.request_register & !self.mask_register;

        (0..8).find(|&line| {
            let bit = 1 << line;
            pending & bit != 0
                && (self.in_service_register == 0
                    || line < self.in_service_register.trailing_zeros() as u8)
        })
    }

    pub fn acknowledge(&mut self) -> Option<u8> {
        let line = self.get_pending()?;
        let bit = 1 << line;

        self.request_register &= !bit;
        if !self.auto_end {
            self.in_service_register |= bit;
        }

        Some(self.vector_offset.wrapping_add(line))
    }

    pub fn end(&mut self, line: u8) {
        assert!(line < 8);
        self.in_service_register &= !(1 << line);
    }
}
