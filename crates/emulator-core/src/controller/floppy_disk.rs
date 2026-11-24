use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

const DRIVE_NUMBER_MASK: u8 = 0x03;
const DRIVE_AND_HEAD_MASK: u8 = 0x07;
const HEAD_SELECT_BIT: u8 = 0x04;
const RESET_RELEASED: u8 = 0x04;
const DRIVE_A_MOTOR: u8 = 0x10;

const STATUS_COMMAND_BUSY: u8 = 0x10;
const STATUS_NON_DMA: u8 = 0x20;
const STATUS_CONTROLLER_TO_CPU: u8 = 0x40;
const STATUS_READY: u8 = 0x80;

const COMMAND_SPECIFY: u8 = 0x03;
const COMMAND_READ_DATA_MFM: u8 = 0x46;
const SPECIFY_NON_DMA: u8 = 0x01;

const ST0_NOT_READY: u8 = 0x08;
const ST0_ABNORMAL_TERMINATION: u8 = 0x40;
const ST0_INVALID_COMMAND: u8 = 0x80;
const ST1_SECTOR_NOT_FOUND: u8 = 0x04;
const ST1_DATA_ERROR: u8 = 0x20;

const SECTOR_SIZE_CODE: u8 = 2;
const SECTOR_SIZE: usize = 512;
const RESULT_SIZE: usize = 7;
const OUTPUT_BUFFER_SIZE: usize = SECTOR_SIZE + RESULT_SIZE;

#[derive(Debug, Clone, Copy)]
struct Geometry {
    cylinders: usize,
    heads: usize,
    sectors_per_track: usize,
}

impl Geometry {
    const fn new(cylinders: usize, heads: usize, sectors_per_track: usize) -> Self {
        Self {
            cylinders,
            heads,
            sectors_per_track,
        }
    }

    fn from_image_size(size: u64) -> io::Result<Self> {
        let geometry = match size {
            163_840 => Self::new(40, 1, 8),
            184_320 => Self::new(40, 1, 9),
            327_680 => Self::new(40, 2, 8),
            368_640 => Self::new(40, 2, 9),
            737_280 => Self::new(80, 2, 9),
            1_228_800 => Self::new(80, 2, 15),
            1_474_560 => Self::new(80, 2, 18),
            1_720_320 => Self::new(80, 2, 21),
            1_763_328 => Self::new(82, 2, 21),
            2_949_120 => Self::new(80, 2, 36),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown floppy image size {size}bytes"),
                ));
            }
        };

        Ok(geometry)
    }

    fn image_size(self) -> io::Result<u64> {
        if !(1..=256).contains(&self.cylinders)
            || !(1..=2).contains(&self.heads)
            || !(1..=255).contains(&self.sectors_per_track)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid floppy geometry",
            ));
        }

        Ok((self.cylinders * self.heads * self.sectors_per_track * SECTOR_SIZE) as u64)
    }

    fn sector_offset(self, cylinder: u8, head: u8, sector: u8) -> Option<u64> {
        let cylinder = usize::from(cylinder);
        let head = usize::from(head);
        let sector = usize::from(sector);

        if cylinder >= self.cylinders
            || head >= self.heads
            || !(1..=self.sectors_per_track).contains(&sector)
        {
            return None;
        }

        let track = cylinder * self.heads + head;
        let index = track * self.sectors_per_track + sector - 1;

        Some((index * SECTOR_SIZE) as u64)
    }
}

#[derive(Debug)]
struct FloppyDisk {
    file: File,
    geometry: Geometry,
}

impl FloppyDisk {
    fn new(file: File) -> io::Result<Self> {
        let size = file.metadata()?.len();
        let geometry = Geometry::from_image_size(size)?;

        Ok(Self { file, geometry })
    }

    fn read_sector(
        &mut self,
        cylinder: u8,
        head: u8,
        sector_number: u8,
        destination: &mut [u8],
    ) -> io::Result<()> {
        let offset = self
            .geometry
            .sector_offset(cylinder, head, sector_number)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Sector outside floppy geometry",
                )
            })?;

        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(destination)
    }
}

#[derive(Debug)]
pub struct FloppyDiskController {
    disk: Option<FloppyDisk>,
    digital_output_register: u8,
    command_buffer: [u8; 9],
    command_bytes_received: usize,
    output_buffer: [u8; OUTPUT_BUFFER_SIZE],
    output_read_position: usize,
    output_end: usize,
}

impl FloppyDiskController {
    pub fn new() -> Self {
        Self {
            disk: None,
            digital_output_register: 0,
            command_buffer: [0; 9],
            command_bytes_received: 0,
            output_buffer: [0; OUTPUT_BUFFER_SIZE],
            output_read_position: 0,
            output_end: 0,
        }
    }

    pub fn insert(&mut self, path_to_image: &Path) -> io::Result<()> {
        let file = File::open(path_to_image)?;
        self.disk = Some(FloppyDisk::new(file)?);

        Ok(())
    }

    pub fn read(&mut self, port: u16) -> u8 {
        match port {
            0x3F4 => self.read_status(),
            0x3F5 if self.output_read_position < self.output_end => {
                let value = self.output_buffer[self.output_read_position];
                self.output_read_position += 1;
                value
            }
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, port: u16, value: u8) {
        match port {
            0x3F2 => {
                self.digital_output_register = value;

                if value & RESET_RELEASED == 0 {
                    self.command_bytes_received = 0;
                    self.output_read_position = self.output_end;
                }
            }
            0x3F5
                if self.digital_output_register & RESET_RELEASED != 0
                    && self.output_read_position == self.output_end =>
            {
                self.write_command(value);
            }
            _ => {}
        }
    }

    fn read_status(&self) -> u8 {
        if self.digital_output_register & RESET_RELEASED == 0 {
            return 0;
        }

        if self.output_read_position == self.output_end {
            return if self.command_bytes_received == 0 {
                STATUS_READY
            } else {
                STATUS_READY | STATUS_COMMAND_BUSY
            };
        }

        let status = STATUS_READY | STATUS_COMMAND_BUSY | STATUS_CONTROLLER_TO_CPU;

        if self.output_read_position < SECTOR_SIZE {
            status | STATUS_NON_DMA
        } else {
            status
        }
    }

    fn write_command(&mut self, value: u8) {
        self.command_buffer[self.command_bytes_received] = value;
        self.command_bytes_received += 1;

        let command = self.command_buffer[0];
        let length = match command {
            COMMAND_SPECIFY => 3,
            COMMAND_READ_DATA_MFM => 9,
            _ => {
                self.command_bytes_received = 0;
                self.output_buffer[SECTOR_SIZE] = ST0_INVALID_COMMAND;
                self.output_read_position = SECTOR_SIZE;
                self.output_end = SECTOR_SIZE + 1;
                return;
            }
        };

        if self.command_bytes_received < length {
            return;
        }

        self.command_bytes_received = 0;

        if command == COMMAND_READ_DATA_MFM {
            self.execute_read_data();
        }
    }

    fn execute_read_data(&mut self) {
        let [
            _,
            drive_and_head,
            cylinder,
            head,
            sector_number,
            sector_size_code,
            last_sector_number,
            _,
            _,
        ] = self.command_buffer;

        let mut primary_status = drive_and_head & DRIVE_AND_HEAD_MASK;
        let mut error_status = 0;
        self.output_read_position = SECTOR_SIZE;
        self.output_end = OUTPUT_BUFFER_SIZE;

        let requested_drive = drive_and_head & DRIVE_NUMBER_MASK;
        let selected_drive = self.digital_output_register & DRIVE_NUMBER_MASK;
        let motor_running = self.digital_output_register & DRIVE_A_MOTOR != 0;
        let drive_ready = requested_drive == 0 && selected_drive == 0 && motor_running;

        let selected_head = u8::from(drive_and_head & HEAD_SELECT_BIT != 0);
        let supported_request = selected_head == head
            && sector_size_code == SECTOR_SIZE_CODE
            && last_sector_number == sector_number;

        match self.disk.as_mut() {
            Some(disk) if drive_ready && supported_request => {
                let destination = &mut self.output_buffer[..SECTOR_SIZE];

                match disk.read_sector(cylinder, head, sector_number, destination) {
                    Ok(()) => self.output_read_position = 0,
                    Err(error) if error.kind() == io::ErrorKind::InvalidInput => {
                        error_status = ST1_SECTOR_NOT_FOUND;
                    }
                    Err(_) => error_status = ST1_DATA_ERROR,
                }
            }
            Some(_) if drive_ready => error_status = ST1_SECTOR_NOT_FOUND,
            _ => primary_status |= ST0_NOT_READY,
        }

        if self.output_read_position == SECTOR_SIZE {
            primary_status |= ST0_ABNORMAL_TERMINATION;
        }

        self.output_buffer[SECTOR_SIZE..].copy_from_slice(&[
            primary_status,
            error_status,
            0,
            cylinder,
            head,
            sector_number,
            sector_size_code,
        ]);
    }
}
