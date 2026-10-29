use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::{AddressingMode, Cpu};

pub mod add;
pub mod add_carry;
pub mod and;
pub mod call;
pub mod clear;
pub mod compare;
pub mod decrement;
pub mod group;
pub mod increment;
pub mod input;
pub mod interrupt;
pub mod jump;
pub mod load;
pub mod loops;
pub mod mov;
pub mod or;
pub mod out;
pub mod pop;
pub mod push;
pub mod ret;
pub mod rotate;
pub mod set;
pub mod shift;
pub mod subtract;
pub mod subtract_borrow;
pub mod test;
pub mod xor;

pub enum OperationDirection {
    RegisterToOperand,
    OperandToRegister,
}

pub fn register_operand_operation(
    cpu: &mut Cpu,
    bus: &mut Bus,
    size: OperandSize,
    direction: OperationDirection,
    operation: fn(&mut Cpu, OperandValue, OperandValue, OperandSize) -> OperandValue,
) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu, bus);

    let (destination, source) = match direction {
        OperationDirection::RegisterToOperand => (operand, register),
        OperationDirection::OperandToRegister => (register, operand),
    };

    let destination_value = destination.read(cpu, bus, size);
    let source_value = source.read(cpu, bus, size);

    let result = operation(cpu, destination_value, source_value, size);

    destination.write(cpu, bus, result);
}
