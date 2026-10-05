use crate::cpu::Cpu;
use crate::cpu::operand::OperandSize;
use crate::cpu::register::SegmentRegisterKind;

pub fn jmp_relative(cpu: &mut Cpu, size: OperandSize) {
    let displacement = match size {
        OperandSize::Byte => cpu.fetch_byte() as i8 as u16,
        OperandSize::Word => cpu.fetch_word(),
    };

    cpu.instruction_pointer =
        cpu.instruction_pointer.wrapping_add(displacement);
}

pub fn jmp_absolute(cpu: &mut Cpu) {
    let offset = cpu.fetch_word();
    let segment = cpu.fetch_word();

    cpu.segment_registers.set(SegmentRegisterKind::CS, segment);
    cpu.instruction_pointer = offset;
}
