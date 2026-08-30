use std::fmt::Debug;

#[derive(Default)]
pub struct Ports {
    values: Vec<u8>,
}

impl Debug for Ports {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("count = {}", &self.values.len()))
    }
}

impl Ports {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        self.values[port as usize] = value;
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        self.values[port as usize]
    }
}
