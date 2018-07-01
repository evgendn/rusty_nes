pub fn get_bit(byte: &u8, i: u8) -> u8 {
    if i > 8u8 {
        panic!("Value i must be less than 8, got {}.", i);
    }

    (byte >> i) & 1u8
}

pub fn set_bit(byte: &mut u8, i: u8, value: bool) {
    if i > 8u8 {
        panic!("Value i must be less than 8, got {}.", i);
    }

    match value {
        true => *byte |= 1u8 << i, 
        false => *byte &= !(1u8 << i), 
    } 
}

pub fn toggle_bit(byte: &mut u8, i: u8) {
    if i > 8u8 {
        panic!("Value i must be less than 8, got {}.", i);
    }

    *byte ^= 1u8 << i
}

pub fn clear_bit(byte: &mut u8, i: u8) {
    set_bit(byte, i, false)
}

#[cfg(test)]
mod test {
    use utils::*;

    #[test]
    fn getting_bits() {
        let mut byte = &0b_0000_1000;
        let mut bit = get_bit(byte, 3u8);
        assert_eq!(bit, 1u8);

        bit = get_bit(byte, 2u8);
        assert_eq!(bit, 0u8);

        bit = get_bit(byte, 0u8);
        assert_eq!(bit, 0u8);

        byte = &0b_1111_0000;
        bit = get_bit(byte, 7);
        assert_eq!(bit, 1u8);

        bit = get_bit(byte, 6);
        assert_ne!(bit, 0u8);

        bit = get_bit(byte, 1);
        assert_ne!(bit, 1u8);
    }

    #[test]
    fn setting_bits() {
        let byte = &mut 0b_0000_1000;
        set_bit(byte, 2, true);
        assert_eq!(*byte, 0b_0000_1100);
        
        set_bit(byte, 3, false);
        assert_eq!(*byte, 0b_0000_0100);

        set_bit(byte, 0, true);
        assert_eq!(*byte, 0b_0000_0101);

        set_bit(byte, 2, false);
        set_bit(byte, 7, true);
        assert_eq!(*byte, 0b_1000_0001);

        set_bit(byte, 5, true);
        assert_ne!(*byte, 0b_0001_1111);
    }

    #[test]
    fn toggling_bits() {
        let byte = &mut 0b_0000_0000;
        toggle_bit(byte, 7);
        assert_eq!(*byte, 0b_1000_0000);

        toggle_bit(byte, 5);
        assert_eq!(*byte, 0b_1010_0000);

        toggle_bit(byte, 5);
        assert_eq!(*byte, 0b_1000_0000);

        toggle_bit(byte, 0);
        toggle_bit(byte, 7);
        assert_eq!(*byte, 0b_0000_0001);

        toggle_bit(byte, 4);
        assert_ne!(*byte, 0b_0000_0001);
    }

    #[test]
    fn clearing_bits() {
        let byte = &mut 0b_1111_1111;
        clear_bit(byte, 0);
        assert_eq!(*byte, 0b_1111_1110);

        clear_bit(byte, 7);
        assert_eq!(*byte, 0b_0111_1110);

        clear_bit(byte, 3);
        assert_ne!(*byte, 0b_1111_1110);
    }

    #[test]
    #[should_panic(expected = "Value i must be less than 8")]
    fn calling_panics() {
        let byte = &mut 0b_0000_1111;
        get_bit(byte, 10u8);
        get_bit(byte, 8u8);

        set_bit(byte, 19u8, true);
        set_bit(byte, 9u8, false);

        toggle_bit(byte, 21u8);
        toggle_bit(byte, 8u8);

        clear_bit(byte, 17u8);
        clear_bit(byte, 89u8);
    }
}

