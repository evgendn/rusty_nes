pub struct Memory {
    pub ram: RAM,
    pub ppu: PPU,
}   

impl Memory {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram.read(address)
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let upper_byte = self.ram.read(address + 1) as u16;
        let lower_byte = self.ram(address) as u16;
        (upper_byte << 8 | lower_byte) as u 16
    }
    
    pub fn write(&self, address: u16, byte: u8) {
        self.ram.write(address, byte)
    }
}

