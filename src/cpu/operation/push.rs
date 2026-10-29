use crate::bus::Bus;
use crate::cpu::operand::OperandSize;
use crate::cpu::{AddressingMode, Cpu, PointerRegisterKind, SegmentRegisterKind};

pub fn segment_register(cpu: &mut Cpu, bus: &mut Bus, register_kind: SegmentRegisterKind) {
    let register_value = cpu.segment_registers.get(register_kind);
    push(cpu, bus, register_value);
}

pub fn word_register(cpu: &mut Cpu, bus: &mut Bus, base: u8, operation_code: u8) {
    let register_index = operation_code - base;
    let mut register_value = cpu.get_word_register(register_index);

    if operation_code == 0x54 {
        register_value = register_value.wrapping_sub(2);
    }

    push(cpu, bus, register_value);
}

pub fn word_operand(cpu: &mut Cpu, bus: &mut Bus) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let operand = addressing_mode.operand(cpu, bus);

    let operand_value = operand.read(cpu, bus, OperandSize::Word);
    push(cpu, bus, operand_value.as_word());
}

pub fn flags(cpu: &mut Cpu, bus: &mut Bus) {
    let flags = cpu.flags;
    push(cpu, bus, flags);
}

pub fn push(cpu: &mut Cpu, bus: &mut Bus, value: u16) {
    let stack_pointer_value = cpu
        .pointer_registers
        .get(PointerRegisterKind::SP)
        .wrapping_sub(2);

    cpu.pointer_registers
        .set(PointerRegisterKind::SP, stack_pointer_value);

    let segment_register = cpu.segment_registers.get(SegmentRegisterKind::SS);
    let physical_address = cpu.physical_address(segment_register, stack_pointer_value);

    bus.write_word_to_memory(physical_address, value);
}
