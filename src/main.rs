mod entry;
mod cpu;
mod memory;
mod instructions;
mod registers;
mod utils;
mod types;
mod gpu;
mod emu_window;
mod rom;

mod playground;

fn main() -> () {
    println!("Crusty-Boy greets you!");
    entry::setup();
}