#[macro_use]
extern crate lazy_static;

mod rom;
mod utils;
mod cpu;
mod ram;
mod nes;

use rom::ROM;
use nes::*;

fn main() {
    let path = "/home/evgeniy/Development/rusty_nes/\
                roms/Super Mario Bros (E).nes";  
    let rom = match ROM::load(path) {
        Ok(r) => r, 
        Err(e) => panic!("Error: {}", e),
    };
    println!("{}", rom);

    let nes = &mut NES::new(path);
    println!("{:?}", nes.cpu);
}

