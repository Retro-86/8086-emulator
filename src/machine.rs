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
        if self.cpu.can_accept_interrupt() {
            if let Some(vector) = self.bus.acknowledge_interrupt() {
                self.cpu.enter_interrupt(&mut self.bus, vector);
            }
        }

        self.cpu.step(&mut self.bus);
    }

    pub fn get_cpu(&self) -> &Cpu {
        &self.cpu
    }
}
