use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{AddressingMode, Cpu, SegmentRegisterKind};

pub fn immediate_to_register(
    cpu: &mut Cpu,
    bus: &mut Bus,
    operand_size: OperandSize,
    operation_code: u8,
    base: u8,
) {
    let value = cpu.fetch_value(bus, operand_size);
    Operand::Register(operation_code - base).write(cpu, bus, value);
}

pub fn register_to_operand(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu, bus);

    let value = register.read(cpu, bus, size);
    operand.write(cpu, bus, value);
}

pub fn operand_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let operand = addressing_mode.operand(cpu, bus);
    let register = Operand::Register(addressing_mode.register);

    let value = operand.read(cpu, bus, size);
    register.write(cpu, bus, value);
}

pub fn segment_register_to_operand(cpu: &mut Cpu, bus: &mut Bus) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let segment_register_kind = SegmentRegisterKind::try_from(addressing_mode.register).unwrap();
    let value = OperandValue::Word(cpu.segment_registers.get(segment_register_kind));

    let operand = addressing_mode.operand(cpu, bus);
    operand.write(cpu, bus, value);
}

pub fn operand_to_segment_register(cpu: &mut Cpu, bus: &mut Bus) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let operand = addressing_mode.operand(cpu, bus);
    let value = operand.read(cpu, bus, OperandSize::Word);

    let segment_register_kind = SegmentRegisterKind::try_from(addressing_mode.register).unwrap();

    if segment_register_kind != SegmentRegisterKind::CS {
        cpu.segment_registers
            .set(segment_register_kind, value.as_word());
    }
}

pub fn memory_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let address = get_physical_address(cpu, bus);

    let operand = Operand::Memory(address);
    let register = Operand::Register(0);

    let operand_value = operand.read(cpu, bus, size);

    register.write(cpu, bus, operand_value);
}

pub fn register_to_memory(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let address = get_physical_address(cpu, bus);

    let operand = Operand::Memory(address);
    let register = Operand::Register(0);

    let value = register.read(cpu, bus, size);
    operand.write(cpu, bus, value);
}

fn get_physical_address(cpu: &mut Cpu, bus: &mut Bus) -> u32 {
    let segment_register_kind = cpu
        .segment_register_override
        .unwrap_or_else(|| SegmentRegisterKind::DS);

    let segment_register_value = cpu.segment_registers.get(segment_register_kind);
    let offset = cpu.fetch_word(bus);

    cpu.physical_address(segment_register_value, offset)
}

pub fn immediate_to_operand(cpu: &mut Cpu, bus: &mut Bus, operand_size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));
    let operand = addressing_mode.operand(cpu, bus);

    let value = cpu.fetch_value(bus, operand_size);

    operand.write(cpu, bus, value);
}
