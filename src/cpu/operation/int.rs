use crate::cpu::{Cpu, FlagKind};
use crate::cpu::operation::push::push;
use crate::cpu::register::SegmentRegisterKind;

pub fn interrupt(cpu: &mut Cpu, vector: u8) {
    if let Some(handler) = cpu.interrupts.get(vector as usize) {
        handler(cpu, vector);
        return;
    }

    push(cpu, cpu.flags);

    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::CS);
    push(cpu, segment_register);

    push(cpu, cpu.instruction_pointer);

    cpu.set_flag(FlagKind::I, false);
    cpu.set_flag(FlagKind::T, false);

    let address = (vector as usize) * 4;
    let instruction_pointer = cpu.bus.read_word_from_memory(address as u32);
    let code_segment = cpu.bus.read_word_from_memory((address + 2) as u32);

    cpu.instruction_pointer = instruction_pointer;
    cpu.segment_registers.set(SegmentRegisterKind::CS, code_segment);
}

pub fn interrupt_immediate(cpu: &mut Cpu) {
    let vector = cpu.fetch_byte();
    interrupt(cpu, vector);
}