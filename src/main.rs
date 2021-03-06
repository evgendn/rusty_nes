#[macro_use]
extern crate lazy_static;

mod rom;
mod utils;
mod cpu;
mod ram;

use rom::ROM;

fn main() {
    let path = "/home/evgeniy/Development/rusty_nes/\
                roms/Super Mario Bros (E).nes";  
    let rom = match ROM::load(path) {
        Ok(r) => r, 
        Err(e) => panic!("Error: {}", e),
    };
    println!("{}", rom);

    let mut ram = ram::RAM::new();
    ram.data[0] = 0x79;
    ram.data[1] = 0xad;
    let mut cpu = cpu::cpu::CPU::new(ram);
    cpu.step();
}
