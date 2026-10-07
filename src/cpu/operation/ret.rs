use crate::cpu::{Cpu, operation::pop::pop, register::PointerRegisterKind};

pub fn ret_word(cpu: &mut Cpu) {
    let displacement = cpu.fetch_word();
    ret(cpu);

    let stack_pointer = cpu.pointer_registers.get(PointerRegisterKind::SP) + displacement;
    cpu.pointer_registers.set(PointerRegisterKind::SP, stack_pointer);
}

pub fn ret(cpu: &mut Cpu) {
    cpu.instruction_pointer = pop(cpu);
}