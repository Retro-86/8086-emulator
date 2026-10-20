use crate::bus::Bus;
use crate::cpu::operand::OperandValue;
use crate::cpu::{AddressingMode, Cpu, PointerRegisterKind, SegmentRegisterKind};

pub fn segment_register(cpu: &mut Cpu, bus: &mut Bus, register_kind: SegmentRegisterKind) {
    let value = pop(cpu, bus);
    cpu.segment_registers.set(register_kind, value);
}

pub fn word_register(cpu: &mut Cpu, bus: &mut Bus, base: u8, operation_code: u8) {
    let value = pop(cpu, bus);

    let register_index = operation_code - base;
    cpu.set_word_register(register_index, value);
}

pub fn word_operand(cpu: &mut Cpu, bus: &mut Bus) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let operand = addressing_mode.operand(cpu, bus);

    let value = pop(cpu, bus);
    
    operand.write(cpu, bus, OperandValue::Word(value));
}

pub fn pop(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP);
    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::SS);

    let address = cpu.physical_address(segment_register, stack_pointer);
    let temporary_value = bus.read_word_from_memory(address);

    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer.wrapping_add(2));

    temporary_value
}
