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

        let bios_image_path = Path::new("../resources/bios.bin");
        bios::load(bios_image_path, &mut self.bus).unwrap();
    }

    pub fn step(&mut self) {
        let elapsed_ticks = self.timer_ticks_for_step();
        self.bus.advance_timer_ticks(elapsed_ticks);

        self.cpu.step(&mut self.bus);
    }

    fn timer_ticks_for_step(&self) -> u32 {
        if self.cpu.halted() && self.cpu.can_accept_interrupt() {
            let interrupt_controller = self.bus.interrupt_controller();
            if interrupt_controller.get_pending().is_some() {
                return 0;
            }

            if interrupt_controller.read(0x21) & 1 == 0 {
                return self
                    .bus
                    .timer()
                    .ticks_until_output_rising_edge(0)
                    .unwrap_or(1);
            }
        }

        1
    }

    pub fn get_cpu(&self) -> &Cpu {
        &self.cpu
    }
}
