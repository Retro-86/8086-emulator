use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::operation::push;
use crate::cpu::operation::push::push;
use crate::cpu::register::SegmentRegisterKind;

pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) {
    let offset = cpu.fetch_word(bus);
    let segment = cpu.fetch_word(bus);

    push::segment_register(cpu, bus, SegmentRegisterKind::CS);
    push(cpu, bus, cpu.instruction_pointer);

    cpu.instruction_pointer = offset;
    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
}

pub fn relative(cpu: &mut Cpu, bus: &mut Bus) {
    let displacement = cpu.fetch_word(bus);

    push(cpu, bus, cpu.instruction_pointer);

    cpu.instruction_pointer = cpu.instruction_pointer.wrapping_add(displacement);
}
