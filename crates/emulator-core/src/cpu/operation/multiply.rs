use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize, OperandValue};
use crate::cpu::register::WordSharedRegisterKind;
use crate::cpu::{Cpu, FlagKind};

pub fn operand(cpu: &mut Cpu, bus: &mut Bus, operand: Operand, size: OperandSize) {
    let register = Operand::Register(0);

    let register_value = register.read(cpu, bus, size);
    let operand_value = operand.read(cpu, bus, size);
    let result = register_value.as_word() as u32 * operand_value.as_word() as u32;

    register.write(cpu, bus, OperandValue::Word(result as u16));

    let high = match size {
        OperandSize::Byte => (result >> 8) as u16,
        OperandSize::Word => {
            let high = (result >> 16) as u16;
            cpu.shared_registers.set_word(WordSharedRegisterKind::DX, high);

            high
        }
    };

    update_flags(cpu, high);
}

fn update_flags(cpu: &mut Cpu, high: u16) {
    let carry = high != 0;

    cpu.set_flag(FlagKind::C, carry);
    cpu.set_flag(FlagKind::O, carry);
}
