use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::register::WordSharedRegisterKind;

pub fn immediate_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let port = cpu.fetch_byte(bus) as u16;
    input(cpu, bus, size, port);
}

pub fn port_to_register(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let port = cpu.shared_registers.get_word(WordSharedRegisterKind::DX);
    input(cpu, bus, size, port);
}

fn input(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize, port: u16) {
    let value = match size {
        OperandSize::Byte => OperandValue::Byte(bus.read_byte_from_port(port)),
        OperandSize::Word => OperandValue::Word(bus.read_word_from_port(port)),
    };

    Operand::Register(0).write(cpu, bus, value);
}