use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct Machine {
    cpu: Cpu,
    bus: Bus,
}

impl Machine {
    pub fn new(cpu: Cpu, bus: Bus) -> Self {
        Self {
            cpu,
            bus
        }
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }

    pub fn get_cpu(&self) -> &Cpu {
        &self.cpu
    }
}
