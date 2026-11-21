use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::interrupt::interrupt;
use crate::cpu::register::WordSharedRegisterKind;
use crate::cpu::Cpu;

pub fn operand(cpu: &mut Cpu, bus: &mut Bus, operand: Operand, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, OperandSize::Word).as_word() as u32;
    let operand_value = operand.read(cpu, bus, size).as_word() as u32;

    let (dividend, max_quotient) = match size {
        OperandSize::Byte => (register_value, u8::MAX as u32),
        OperandSize::Word => {
            let dx = cpu.shared_registers.get_word(WordSharedRegisterKind::DX) as u32;
            ((dx << 16) | register_value, u16::MAX as u32)
        }
    };

    let quotient = match dividend.checked_div(operand_value) {
        Some(quotient) if quotient <= max_quotient => quotient,
        _ => {
            interrupt(cpu, bus, 0);
            return;
        }
    };

    let remainder = dividend % operand_value;
    match size {
        OperandSize::Byte => {
            let result = u16::from_le_bytes([quotient as u8, remainder as u8]);
            register.write(cpu, bus, OperandValue::Word(result));
        }
        OperandSize::Word => {
            register.write(cpu, bus, OperandValue::Word(quotient as u16));
            cpu.shared_registers
                .set_word(WordSharedRegisterKind::DX, remainder as u16);
        }
    }
}
