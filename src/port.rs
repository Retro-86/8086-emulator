use crate::bus::Bus;

pub type PortByteReadHandler = fn(&mut Bus, u16) -> u8;
pub type PortByteWriteHandler = fn(&mut Bus, u16, u8);

const PORT_ADDRESS_COUNT: usize = 1024;
const PORT_MASK: u16 = 0x03FF;

#[derive(Debug)]
pub struct Ports {
    byte_read_handlers: Box<[Option<PortByteReadHandler>; PORT_ADDRESS_COUNT]>,
    byte_write_handlers: Box<[Option<PortByteWriteHandler>; PORT_ADDRESS_COUNT]>,
}

impl Ports {
    pub fn new() -> Self {
        Self {
            byte_read_handlers: Box::new([None; PORT_ADDRESS_COUNT]),
            byte_write_handlers: Box::new([None; PORT_ADDRESS_COUNT]),
        }
    }

    pub fn register_byte_reader(&mut self, port_addresses: &[u16], handler: PortByteReadHandler) {
        for &port_address in port_addresses {
            self.byte_read_handlers[usize::from(port_address & PORT_MASK)] = Some(handler);
        }
    }

    pub fn register_byte_writer(&mut self, port_addresses: &[u16], handler: PortByteWriteHandler) {
        for &port_address in port_addresses {
            self.byte_write_handlers[usize::from(port_address & PORT_MASK)] = Some(handler);
        }
    }

    #[inline]
    pub fn byte_read_handler(&self, port_address: u16) -> Option<PortByteReadHandler> {
        self.byte_read_handlers[usize::from(port_address & PORT_MASK)]
    }

    #[inline]
    pub fn byte_write_handler(&self, port_address: u16) -> Option<PortByteWriteHandler> {
        self.byte_write_handlers[usize::from(port_address & PORT_MASK)]
    }
}

impl Default for Ports {
    fn default() -> Self {
        Self::new()
    }
}
