use crate::bus::Bus;
use crate::cpu::operand::{Operand, OperandSize};
use crate::cpu::register::{IndexRegisterKind, SegmentRegisterKind};
use crate::cpu::{Cpu, FlagKind};

pub fn string(cpu: &mut Cpu, bus: &mut Bus, size: OperandSize) {
    let segment_register_kind = cpu
        .segment_register_override
        .unwrap_or(SegmentRegisterKind::DS);

    let segment_register_value = cpu.segment_registers.get(segment_register_kind);

    let source_index = cpu.index_registers.get(IndexRegisterKind::SI);
    let address = cpu.physical_address(segment_register_value, source_index);

    let operand = Operand::Memory(address);
    let register = Operand::Register(0);

    let value = operand.read(cpu, bus, size);
    register.write(cpu, bus, value);

    let step = match size {
        OperandSize::Byte => 1,
        OperandSize::Word => 2,
    };

    let next_source_index = if cpu.get_flag(FlagKind::D) {
        source_index.wrapping_sub(step)
    } else {
        source_index.wrapping_add(step)
    };

    cpu.index_registers.set(IndexRegisterKind::SI, next_source_index);
}
