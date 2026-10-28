use crate::bios;
use crate::bus::Bus;
use crate::cpu::Cpu;
use std::path::Path;

pub struct Machine {
    cpu: Cpu,
    bus: Bus,
}

impl Machine {
    pub fn new(cpu: Cpu, bus: Bus) -> Self {
        Self { cpu, bus }
    }

    pub fn initialize(&mut self) {
        self.cpu.reset();

        let bios_image_path = Path::new("./resources/bios.bin");
        bios::load(bios_image_path, &mut self.bus).unwrap();
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }

    pub fn get_cpu(&self) -> &Cpu {
        &self.cpu
    }
}
