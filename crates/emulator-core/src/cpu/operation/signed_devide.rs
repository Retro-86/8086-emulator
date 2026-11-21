use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::operation::interrupt::interrupt;
use crate::cpu::register::WordSharedRegisterKind;

pub fn operand(cpu: &mut Cpu, bus: &mut Bus, operand: Operand, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, OperandSize::Word).as_word();
    let operand_value = operand.read(cpu, bus, size);

    let (dividend, divisor, max_quotient) = match size {
        OperandSize::Byte => (
            register_value as i16 as i32,
            operand_value.as_byte() as i8 as i32,
            i8::MAX as i32,
        ),
        OperandSize::Word => {
            let dx = cpu.shared_registers.get_word(WordSharedRegisterKind::DX) as i32;
            let dividend = (dx << 16) | register_value as i32;
            (
                dividend,
                operand_value.as_word() as i16 as i32,
                i16::MAX as i32,
            )
        }
    };

    let quotient = match dividend.checked_div(divisor) {
        Some(quotient) if (-max_quotient..=max_quotient).contains(&quotient) => quotient,
        _ => {
            interrupt(cpu, bus, 0);
            return;
        }
    };

    let remainder = dividend % divisor;
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
