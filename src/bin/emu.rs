use arch::emu::cpu::Cpu;
use arch::emu::bus::bus::Bus;
use std::fs;

fn main() {
    // Create a bus instance
    let mut bus = Bus::default();
    let _ = bus.write32(0xFFFFFFF0, 0x8000);

    // Read in the raw binary file
    let program: Vec<u8> = fs::read("a.out").expect("Failed to read a.out!");
    bus.ram.data[0x8000..(0x8000 + program.len())].copy_from_slice(&program);

    // Create a new CPU instance
    let mut cpu = Cpu::default();
    cpu.reset(&mut bus);

    cpu.step(&mut bus);

    println!("{:#?}", cpu);
}
