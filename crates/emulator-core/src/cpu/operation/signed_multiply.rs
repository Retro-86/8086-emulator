use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::register::WordSharedRegisterKind;
use crate::cpu::{Cpu, FlagKind};

pub fn operand(cpu: &mut Cpu, bus: &mut Bus, operand: Operand, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, size);
    let operand_value = operand.read(cpu, bus, size);

    let result = match size {
        OperandSize::Byte => {
            i32::from(register_value.as_byte() as i8) * i32::from(operand_value.as_byte() as i8)
        }
        OperandSize::Word => {
            i32::from(register_value.as_word() as i16) * i32::from(operand_value.as_word() as i16)
        }
    };

    register.write(cpu, bus, OperandValue::Word(result as u16));

    if let OperandSize::Word = size {
        cpu.shared_registers
            .set_word(WordSharedRegisterKind::DX, (result >> 16) as u16);
    }

    update_flags(cpu, result, size);
}

fn update_flags(cpu: &mut Cpu, result: i32, size: OperandSize) {
    let overflow = match size {
        OperandSize::Byte => i8::try_from(result).is_err(),
        OperandSize::Word => i16::try_from(result).is_err(),
    };

    cpu.set_flag(FlagKind::C, overflow);
    cpu.set_flag(FlagKind::O, overflow);
}
