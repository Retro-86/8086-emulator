use crate::cpu::operand::OperandValue;
use crate::cpu::{AddressingMode, Cpu, PointerRegisterKind, SegmentRegisterKind};

pub fn pop_segment_register(cpu: &mut Cpu, register_kind: SegmentRegisterKind) {
    let value = pop(cpu);
    cpu.segment_registers.set(register_kind, value);
}

pub fn pop_word_register(cpu: &mut Cpu, base: u8, operation_code: u8) {
    let value = pop(cpu);

    let register_index = operation_code - base;
    cpu.set_word_register(register_index, value);
}

pub fn pop_word_operand(cpu: &mut Cpu) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);

    let value = pop(cpu);
    
    operand.write(cpu, OperandValue::Word(value));
}

fn pop(cpu: &mut Cpu) -> u16 {
    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP);
    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::SS);

    let address = cpu.physical_address(segment_register, stack_pointer);
    let temporary_value = cpu.bus.read_word_from_memory(address);

    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer.wrapping_add(2));

    temporary_value
}
