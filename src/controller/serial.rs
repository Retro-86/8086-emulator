use std::io::{self, Write};

#[derive(Debug)]
pub struct Serial;

impl Serial {
    pub fn new() -> Self {
        Self
    }

    pub fn write(&mut self, value: u8) {
        let mut stdout = io::stdout().lock();
        stdout.write_all(&[value]).expect("serial write failed");
        stdout.flush().expect("serial flush failed");
    }
}
