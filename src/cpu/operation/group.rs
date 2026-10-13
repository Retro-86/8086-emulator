use crate::cpu::operand::{OperandSize, OperandValue};
use crate::cpu::{AddressingMode, Cpu};
use crate::cpu::operation::{adc, add, and, cmp, or, sbb, sub, xor};

pub fn one(cpu: &mut Cpu, operand_size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let first_operand = addressing_mode.operand(cpu);

    let first_operand_value = first_operand.read(cpu, OperandSize::Word);
    let second_operand_value = match operand_size {
        OperandSize::Byte => OperandValue::Word(cpu.fetch_byte() as i8 as u16),
        OperandSize::Word => OperandValue::Word(cpu.fetch_word()),
    };

    let operation: fn(
        &mut Cpu,
        OperandValue,
        OperandValue,
        OperandSize,
    ) -> OperandValue = match addressing_mode.register {
        0 => add::add,
        1 => or::or,
        2 => adc::adc,
        3 => sbb::sbb,
        4 => and::and,
        5 => sub::sub,
        6 => xor::xor,
        7 => cmp::cmp,
        _ => unreachable!(),
    };

    let result = operation(
        cpu,
        first_operand_value,
        second_operand_value,
        OperandSize::Word,
    );

    if addressing_mode.register != 7 {
        first_operand.write(cpu, result);
    }
}

pub fn two(cpu: &mut Cpu, operand_size: OperandSize) {

}