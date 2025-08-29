#![allow(dead_code)]

mod bus;
mod cpu;
mod emulator;
mod operations;
mod shared;

fn main() {
    let mut emul = emulator::Emulator::new();
    emul.reset_cpu();

    println!("Execution completed.");
}
