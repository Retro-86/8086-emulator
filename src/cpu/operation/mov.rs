use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{AddressingMode, Cpu, SegmentRegisterKind};

pub fn mov_immediate_to_register(
    cpu: &mut Cpu,
    operand_size: OperandSize,
    operation_code: u8,
    base: u8,
) {
    let value = cpu.fetch_value(operand_size);
    Operand::Register(operation_code - base).write(cpu, value);
}

pub fn mov_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let value = register.read(cpu, size);
    operand.write(cpu, value);
}

pub fn mov_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let register = Operand::Register(addressing_mode.register);

    let value = operand.read(cpu, size);
    register.write(cpu, value);
}

pub fn mov_segment_register_to_operand(cpu: &mut Cpu) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let segment_register_kind = SegmentRegisterKind::try_from(addressing_mode.register).unwrap();
    let value = OperandValue::Word(cpu.segment_registers.get(segment_register_kind));

    let operand = addressing_mode.operand(cpu);
    operand.write(cpu, value);
}

pub fn mov_operand_to_segment_register(cpu: &mut Cpu) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let operand = addressing_mode.operand(cpu);
    let value = operand.read(cpu, OperandSize::Word);

    let segment_register_kind = SegmentRegisterKind::try_from(addressing_mode.register).unwrap();

    if segment_register_kind != SegmentRegisterKind::CS {
        cpu.segment_registers
            .set(segment_register_kind, value.as_word());
    }
}

pub fn mov_memory_to_register(cpu: &mut Cpu, size: OperandSize) {
    let address = get_physical_address(cpu);

    let operand = Operand::Memory(address);
    let register = Operand::Register(0);

    let operand_value = operand.read(cpu, size);

    register.write(cpu, operand_value);
}

pub fn mov_register_to_memory(cpu: &mut Cpu, size: OperandSize) {
    let address = get_physical_address(cpu);

    let operand = Operand::Memory(address);
    let register = Operand::Register(0);

    let value = register.read(cpu, size);
    operand.write(cpu, value);
}

fn get_physical_address(cpu: &mut Cpu) -> u32 {
    let segment_register_kind = cpu
        .segment_register_override
        .unwrap_or_else(|| SegmentRegisterKind::DS);

    let segment_register_value = cpu.segment_registers.get(segment_register_kind);
    let offset = cpu.fetch_word();

    cpu.physical_address(segment_register_value, offset)
}

pub fn mov_immediate_to_operand(cpu: &mut Cpu, operand_size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());
    let operand = addressing_mode.operand(cpu);

    let value = cpu.fetch_value(operand_size);

    operand.write(cpu, value);
}
