use crate::cpu::flag::update_result_flags;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{Cpu, FlagKind};

pub fn inc_word_register(cpu: &mut Cpu, base_operation_code: u8, operation_code: u8) {
    let register = Operand::Register(operation_code - base_operation_code);

    let register_value = register.read(cpu, OperandSize::Word);

    let result = inc(cpu, register_value, OperandSize::Word);

    register.write(cpu, result);
}

fn inc(cpu: &mut Cpu, destination: OperandValue, size: OperandSize) -> OperandValue {
    let source = match size {
        OperandSize::Byte => OperandValue::Byte(1),
        OperandSize::Word => OperandValue::Word(1),
    };

    let result = destination + source;

    update_flags(cpu, destination, result);

    result
}

fn update_flags(cpu: &mut Cpu, destination: OperandValue, result: OperandValue) {
    update_result_flags(cpu, result);

    let overflow = !destination.is_negative() && result.is_negative();
    cpu.set_flag(FlagKind::O, overflow);

    let auxiliary_carry = ((destination.as_word() ^ 1 ^ result.as_word()) & 0x10) != 0;
    cpu.set_flag(FlagKind::A, auxiliary_carry);
}