mod rom;
mod utils;

use rom::ROM;

fn main() {
    let path = "/home/evgeniy/Development/rusty_nes/\
                roms/Super Mario Bros (E).nes";  
    let rom = match ROM::load(path) {
        Ok(r) => r, 
        Err(e) => panic!("Error: {}", e),
    };
    println!("{}", rom);

