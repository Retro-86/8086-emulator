use crate::cpu::{Cpu, FlagKind};
use crate::cpu::flag::update_result_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::{register_operand_operation, OperationDirection};

pub fn operand_to_register(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::OperandToRegister, subtract)
}

pub fn register_to_operand(cpu: &mut Cpu, size: OperandSize) {
    register_operand_operation(cpu, size, OperationDirection::RegisterToOperand, subtract)
}

pub fn immediate_to_register(cpu: &mut Cpu, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, size);
    let immediate_value = cpu.fetch_value(size);

    let result = subtract(cpu, register_value, immediate_value, size);

    register.write(cpu, result);
}

pub fn subtract(
    cpu: &mut Cpu,
    destination: OperandValue,
    source: OperandValue,
    size: OperandSize,
) -> OperandValue {
    let result = destination - source;

    update_flags(cpu, destination, source, result, size);

    result
}

pub fn update_flags(
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
