use arch::emu::cpu::Cpu;
use arch::emu::bus::bus::Bus;

fn main() {
    // Create a bus instance
    let mut bus = Bus::default();   

    // Create a new CPU instance
    let mut cpu = Cpu::default();
    cpu.reset();

    println!("{:#?}", cpu);
}
