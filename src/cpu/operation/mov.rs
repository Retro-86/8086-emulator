use crate::cpu::{
    AddressingMode, ByteSharedRegisterKind, Cpu, Operand, OperandSize, OperandValue,
    SegmentRegisterKind, WordSharedRegisterKind,
};

pub fn mov_immediate_to_register<V, K>(
    cpu: &mut Cpu,
    operation_code: u8,
    base: u8,
    fetch_function: fn(&mut Cpu) -> V,
    set_register_function: fn(&mut Cpu, K, V),
) where
    K: TryFrom<u8>,
    K::Error: std::fmt::Debug,
{
    let register_index = operation_code - base;
    let register_kind = K::try_from(register_index).unwrap();

    let value = fetch_function(cpu);
    set_register_function(cpu, register_kind, value);
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

    if segment_register_kind != SegmentRegisterKind::CS
        && let OperandValue::Word(value) = value
    {
        cpu.segment_registers.set(segment_register_kind, value);
    }
}

pub fn mov_memory_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register_kind = cpu
        .segment_register_override
        .get(|| SegmentRegisterKind::DS);
    let register_value = cpu.segment_registers.get(register_kind);
    let offset = cpu.fetch_word();
    let address = cpu.physical_address(register_value, offset);

    match size {
        OperandSize::Byte => {
            let value = cpu.bus.read_byte_from_memory(address);
            cpu.shared_registers
                .set_byte(ByteSharedRegisterKind::AL, value);
        }
        OperandSize::Word => {
            let value = cpu.bus.read_word_from_memory(address);
            cpu.shared_registers
                .set_word(WordSharedRegisterKind::AX, value);
        }
    };
}

pub fn mov_register_to_memory(cpu: &mut Cpu, size: OperandSize) {
    let register_kind = cpu
        .segment_register_override
        .get(|| SegmentRegisterKind::DS);
    let register_value = cpu.segment_registers.get(register_kind);
    let offset = cpu.fetch_word();
    let address = cpu.physical_address(register_value, offset);

    match size {
        OperandSize::Byte => {
            let value = cpu.shared_registers.get_byte(ByteSharedRegisterKind::AL);
            cpu.bus.write_byte_to_memory(address, value);
        }
        OperandSize::Word => {
            let value = cpu.shared_registers.get_word(WordSharedRegisterKind::AX);
            cpu.bus.write_word_to_memory(address, value);
        }
    }
}

pub fn mov_immediate_to_operand(cpu: &mut Cpu, operand_size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());
    let operand = addressing_mode.operand(cpu);

    let value = match operand_size {
        OperandSize::Byte => OperandValue::Byte(cpu.fetch_byte()),
        OperandSize::Word => OperandValue::Word(cpu.fetch_word()),
    };

    operand.write(cpu, value);
}
