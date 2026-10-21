use crate::bus::Bus;

pub type PortByteReadHandler = fn(&mut Bus, u16) -> u8;
pub type PortByteWriteHandler = fn(&mut Bus, u16, u8);
pub type PortWordReadHandler = fn(&mut Bus, u16) -> u16;
pub type PortWordWriteHandler = fn(&mut Bus, u16, u16);

const PORT_ADDRESS_COUNT: usize = 65_536;

#[derive(Debug)]
pub struct Ports {
    byte_read_handlers: Box<[Option<PortByteReadHandler>]>,
    byte_write_handlers: Box<[Option<PortByteWriteHandler>]>,
    word_read_handlers: Box<[Option<PortWordReadHandler>]>,
    word_write_handlers: Box<[Option<PortWordWriteHandler>]>,
}

impl Ports {
    pub fn new() -> Self {
        Self {
            byte_read_handlers: vec![None; PORT_ADDRESS_COUNT].into_boxed_slice(),
            byte_write_handlers: vec![None; PORT_ADDRESS_COUNT].into_boxed_slice(),
            word_read_handlers: vec![None; PORT_ADDRESS_COUNT].into_boxed_slice(),
            word_write_handlers: vec![None; PORT_ADDRESS_COUNT].into_boxed_slice(),
        }
    }

    pub fn register_byte_reader(&mut self, port_addresses: &[u16], handler: PortByteReadHandler) {
        for &port_address in port_addresses {
            self.byte_read_handlers[usize::from(port_address)] = Some(handler);
        }
    }

    pub fn register_byte_writer(&mut self, port_addresses: &[u16], handler: PortByteWriteHandler) {
        for &port_address in port_addresses {
            self.byte_write_handlers[usize::from(port_address)] = Some(handler);
        }
    }

    pub fn register_word_reader(&mut self, port_addresses: &[u16], handler: PortWordReadHandler) {
        for &port_address in port_addresses {
            self.word_read_handlers[usize::from(port_address)] = Some(handler);
        }
    }

    pub fn register_word_writer(&mut self, port_addresses: &[u16], handler: PortWordWriteHandler) {
        for &port_address in port_addresses {
            self.word_write_handlers[usize::from(port_address)] = Some(handler);
        }
    }

    pub fn byte_read_handler(&self, port_address: u16) -> Option<PortByteReadHandler> {
        self.byte_read_handlers[usize::from(port_address)]
    }

    pub fn byte_write_handler(&self, port_address: u16) -> Option<PortByteWriteHandler> {
        self.byte_write_handlers[usize::from(port_address)]
    }

    pub fn word_read_handler(&self, port_address: u16) -> Option<PortWordReadHandler> {
        self.word_read_handlers[usize::from(port_address)]
    }

    pub fn word_write_handler(&self, port_address: u16) -> Option<PortWordWriteHandler> {
        self.word_write_handlers[usize::from(port_address)]
    }
}

impl Default for Ports {
    fn default() -> Self {
        Self::new()
    }
}
