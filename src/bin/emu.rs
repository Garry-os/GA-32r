use arch::emu::cpu::Cpu;

fn main() {
    // Create a new CPU instance
    let mut cpu = Cpu::default();
    cpu.reset();

    println!("{:#?}", cpu);
}
