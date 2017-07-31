pub fn get_bit(byte: &u8, i: u8) -> u8 {
    return (byte & 2u8.pow(i as u32)) >> i;
}
