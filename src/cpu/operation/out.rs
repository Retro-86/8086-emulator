use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::register::WordSharedRegisterKind;

pub fn register_to_immediate(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let port = cpu.fetch_byte(bus) as u16;
    output(cpu, bus, size, port);
}

pub fn register_to_port(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let port = cpu.shared_registers.get_word(WordSharedRegisterKind::DX);
    output(cpu, bus, size, port);
}

fn output(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize, port: u16) {
    let value = Operand::Register(0).read(cpu, bus, size);

    match value {
        OperandValue::Byte(value) => bus.write_byte_to_port(port, value),
        OperandValue::Word(value) => bus.write_word_to_port(port, value),
    }
}
