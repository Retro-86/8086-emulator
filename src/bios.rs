use crate::bus::Bus;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Format(String),
    Read(String),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Format(error) => write!(formatter, "Bios format error {error}"),
            Error::Read(error) => write!(formatter, "Error while reading bios image {error}"),
        }
    }
}

impl std::error::Error for Error {}

pub fn load(path: &Path, bus: &mut Bus) -> Result<(), Error> {
    let data = read(path)?;

    let start_address = 0x100000 - data.len() as u32;

    for (offset, byte) in data.iter().enumerate() {
        bus.write_byte_to_memory(start_address + offset as u32, *byte);
    }

    Ok(())
}

fn read(path: &Path) -> Result<Vec<u8>, Error> {
    let bytes = fs::read(path).map_err(|error| Error::Read(error.to_string()))?;
    validate(&bytes)?;

    Ok(bytes)
}

fn validate(bios_data: &Vec<u8>) -> Result<(), Error> {
    if !(16..=0x10000).contains(&bios_data.len()) {
        return Err(Error::Format("Image to big".to_string()));
    }

    Ok(())
}
