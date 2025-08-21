mod cpu;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.reset_cpu();
    println!("Execution completed.");
}
