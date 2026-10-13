use crate::cpu::flag::update_result_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{AddressingMode, Cpu, FlagKind};

pub fn cmp_register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let operand_value = operand.read(cpu, size);
    let register_value = register.read(cpu, size);

    cmp(cpu, operand_value, register_value, size);
}

pub fn cmp_operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let operand_value = operand.read(cpu, size);
    let register_value = register.read(cpu, size);

    cmp(cpu, operand_value, register_value, size);
}

pub fn cmp_immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    cmp(cpu, register_value, immediate_value, size);
}

pub fn cmp(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    size: OperandSize,
) -> OperandValue {
    let result = destination - source;

    update_flags(cpu, destination, source, result, size);

    result
}

fn update_flags(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    result: OperandValue,
    size: OperandSize,
) {
    update_result_flags(cpu, result);

    let sign_mask = match size {
        OperandSize::Byte => 0x80,
        OperandSize::Word => 0x8000,
    };

    let destination = destination.as_word();
    let source = source.as_word();
    let result = result.as_word();

    let overflow = ((destination ^ source) & (destination ^ result) & sign_mask) != 0;
    cpu.set_flag(FlagKind::O, overflow);

    let borrow = destination < source;
    cpu.set_flag(FlagKind::C, borrow);

    let auxiliary_carry_flag = ((destination ^ source ^ result) & 0x10) != 0;
    cpu.set_flag(FlagKind::A, auxiliary_carry_flag);
}

