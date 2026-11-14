use crate::bus::Bus;
use crate::cpu::{Cpu, FlagKind};
use crate::cpu::operation::push::push;
use crate::cpu::register::SegmentRegisterKind;

pub fn interrupt(cpu: &mut Cpu, bus: &mut Bus, vector: u8) {
    push(cpu, bus, cpu.flags);

    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::CS);
    push(cpu, bus, segment_register);

    push(cpu, bus, cpu.instruction_pointer);

    cpu.set_flag(FlagKind::I, false);
    cpu.set_flag(FlagKind::T, false);

    let address = (vector as usize) * 4;
    let instruction_pointer = bus.read_word_from_memory(address as u32);
    let code_segment = bus.read_word_from_memory((address + 2) as u32);

    cpu.instruction_pointer = instruction_pointer;
    cpu.segment_registers.set(SegmentRegisterKind::CS, code_segment);
}

pub fn interrupt_immediate(cpu: &mut Cpu, bus: &mut Bus) {
    let vector = cpu.fetch_byte(bus);
    interrupt(cpu, bus, vector);
}