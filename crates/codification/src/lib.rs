#![no_std]

pub fn char_to_u32(character: &char) -> u32 {
    //Each char is 8 bits in utf8 as maximum
    *character as u32
}

pub fn u32_to_char(integer: &u32) -> char {
    //Each char is 8 bits in utf8 as maximum
    char::from_u32(*integer).unwrap_or_default()
}