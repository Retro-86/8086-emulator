use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{AddressingMode, Cpu};

pub mod adc;
pub mod add;
pub mod and;
pub mod mov;
pub mod or;
pub mod pop;
pub mod push;
pub mod sbb;
pub mod sub;
pub mod xor;
pub mod cmp;
pub mod inc;
pub mod dec;
pub mod jmp;
pub mod call;
pub mod ret;
pub mod loops;
pub mod group;
pub mod int;

pub enum OperationDirection {
    RegisterToOperand,
    OperandToRegister,
}

pub fn register_operand_operation(
    cpu: &mut Cpu,
    size: OperandSize,
    direction: OperationDirection,
    operation: fn(
        &mut Cpu,
        OperandValue,
        OperandValue,
        OperandSize,
    ) -> OperandValue,
) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte());

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu);

    let (destination, source) = match direction {
        OperationDirection::RegisterToOperand => (operand, register),
        OperationDirection::OperandToRegister => (register, operand),
    };

    let destination_value = destination.read(cpu, size);
    let source_value = source.read(cpu, size);

    let result = operation(cpu, destination_value, source_value, size);

    destination.write(cpu, result);
}