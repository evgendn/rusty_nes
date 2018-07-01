use cpu::cpu::CPU;
use rom::ROM;
use rom::ROMReadError;
use ram::RAM;

#[derive(Debug)]
pub struct NES {
    pub cpu: CPU,
    pub rom: Result<ROM, ROMReadError>,
    pub wram: RAM,
}

impl NES {
    pub fn new(path: &str) -> NES {
        NES {
            cpu: CPU::new(),
            rom: ROM::load(path),
            wram: RAM::new(vec![256]),
        }
    }
}
