pub fn get_bit(byte: &u8, i: u8) -> u8 {
    (byte >> i) & 1u8
}

pub fn set_bit(byte: &mut u8, i: u8, value: bool) {
    match value {
        true => *byte |= 1u8 << i, 
        false => *byte &= !(1u8 << i), 
    } 
}
