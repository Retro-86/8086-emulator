#[derive(Debug)]
pub struct MonochromeDisplayAdapter {}

impl MonochromeDisplayAdapter {
    pub fn write_port(&mut self, port: u16, value: u8) {}
}

#[derive(Debug)]
pub struct ColorGraphicsAdapter {}

impl ColorGraphicsAdapter {
    pub fn write_port(&mut self, port: u16, value: u8) {}
}

#[derive(Debug)]
pub struct VideoGraphicsArray {}

impl VideoGraphicsArray {
    pub fn write_port(&mut self, port: u16, value: u8) {}
}

#[derive(Debug, Clone, Copy)]
pub enum AdapterKind {
    Monochrome,
    Colored,
    Vga,
}

#[derive(Debug)]
pub enum Adapter {
    Monochrome(MonochromeDisplayAdapter),
    Colored(ColorGraphicsAdapter),
    Vga(VideoGraphicsArray),
}

impl Adapter {
    pub fn new(kind: AdapterKind) -> Self {
        match kind {
            AdapterKind::Monochrome => Self::Monochrome(MonochromeDisplayAdapter {}),
            AdapterKind::Colored => Self::Colored(ColorGraphicsAdapter {}),
            AdapterKind::Vga => Self::Vga(VideoGraphicsArray {}),
        }
    }
}

#[derive(Debug)]
pub struct VideoAdapter {
    adapter: Adapter,
}

impl VideoAdapter {
    pub fn new(adapter: AdapterKind) -> Self {
        Self {
            adapter: Adapter::new(adapter),
        }
    }

    pub fn change_adapter(&mut self, adapter: Adapter) {
        self.adapter = adapter;
    }
}

impl VideoAdapter {
    pub fn write_port(&mut self, port: u16, value: u8) {
        match &mut self.adapter {
            Adapter::Monochrome(adapter) => adapter.write_port(port, value),
            Adapter::Colored(adapter) => adapter.write_port(port, value),
            Adapter::Vga(adapter) => adapter.write_port(port, value),
        }
    }
}
