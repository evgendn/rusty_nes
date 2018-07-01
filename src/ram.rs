use std::cell::RefCell;

#[derive(Debug)]
pub struct RAM {
    pub data: RefCell<Vec<u8>>,
}   

impl RAM {
    pub fn new(buf: Vec<u8>) -> RAM {
        RAM {
            data: RefCell::new(buf)
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        self.data.borrow()[address as usize] as u8
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let upper_byte = self.data.borrow()[address as usize + 1] as u16;
        let lower_byte = self.data.borrow()[address as usize] as u16;
        (upper_byte << 8 | lower_byte) as u16
    }
    
    pub fn write(&self, address: u16, byte: u8) {
        self.data.borrow_mut()[address as usize] = byte;
    }
}
