use crate::{
    bus::Bus,
    cpu::{
        AddressingMode, Cpu,
        flag::update_logic_flags,
        operand::{Operand, OperandSize, OperandValue},
    },
};

pub fn register_and_operand(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let addressing_mode = AddressingMode::new(cpu.fetch_byte(bus));

    let register = Operand::Register(addressing_mode.register);
    let operand = addressing_mode.operand(cpu, bus);

    let register_value = register.read(cpu, bus, size);
    let operand_value = operand.read(cpu, bus, size);

    test(cpu, register_value, operand_value);
}

pub fn register_and_immediate(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, size);
    let immediate_value = cpu.fetch_value(bus, size);

    test(cpu, register_value, immediate_value);
}

pub fn operand_and_immediate(cpu: &mut Cpu, bus: &mut Bus, operand: Operand, size: OperandSize) {
    let operand_value = operand.read(cpu, bus, size);
    let immediate_value = cpu.fetch_value(bus, size);

    test(cpu, operand_value, immediate_value);
}

fn test(cpu: &mut Cpu, destination: OperandValue, source: OperandValue) {
    let result = destination & source;

    update_logic_flags(cpu, result);
}
