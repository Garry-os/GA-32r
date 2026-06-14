use arch::emu::cpu::Cpu;
use arch::emu::bus::bus::Bus;

fn main() {
    // Create a bus instance
    let mut bus = Bus::default();
    let _ = bus.write32(0xFFFFFFF0, 0x8000);

    // Create a new CPU instance
    let mut cpu = Cpu::default();
    cpu.reset(&mut bus);

    println!("{:#?}", cpu);
}
