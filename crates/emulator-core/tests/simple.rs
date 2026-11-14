use emulator_core::machine::Machine;
use emulator_core::video::{AdapterKind, VideoAdapter};
use emulator_core::{
    bus::Bus, controller::interrupt::ProgrammableInterruptController, cpu::Cpu, memory::Memory,
};

#[test]
fn simple_test() {
    let mut memory = Memory::new();

    memory.write_byte(0, 0xB8);
    memory.write_word(1, 0x1234);
    memory.write_byte(3, 0x50);
    memory.write_byte(4, 0x5A);
    memory.write_byte(5, 0x89);
    memory.write_byte(6, 0x16);
    memory.write_word(7, 0x5678);
    memory.write_byte(9, 0xF4);

    println!("{:x}", memory.read_word(7));

    let pic = ProgrammableInterruptController::new();
    let video_adapter = VideoAdapter::new(AdapterKind::Monochrome);

    let bus = Bus::new(memory, pic, video_adapter);

    let mut cpu = Cpu::new();
    cpu.reset();

    let mut machine = Machine::new(cpu, bus);

    for _ in 0..=10 {
        machine.step();
        println!("{:#?}", machine.get_cpu().get_registers());
    }
}
