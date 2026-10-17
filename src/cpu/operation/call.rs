use crate::cpu::Cpu;
use crate::cpu::operation::push;
use crate::cpu::operation::push::push;
use crate::cpu::register::SegmentRegisterKind;

pub fn call(cpu: &mut Cpu) {
    let offset = cpu.fetch_word();
    let segment = cpu.fetch_word();

    push::segment_register(cpu, SegmentRegisterKind::CS);
    push(cpu, cpu.instruction_pointer);

    cpu.instruction_pointer = offset;
    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
}
