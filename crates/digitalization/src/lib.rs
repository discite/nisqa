#![no_std]

pub fn u32_to_binary(mut n: u32) -> [u8; 32] {
    let mut binary = [0; 32];
    let mut i = 31;
    while n > 0 && i > 0 {
        binary[i] = (n % 2) as u8;
        n /= 2;
        i -= 1;
    }
    if n == 0 {
        binary[i] = 0;
    } else {
        binary[i] = 1;
    }
    binary
}



pub fn binary_to_u32(binary: [u8; 32]) -> Option<u32> {
    let mut decimal: u32 = 0;
    for digit in binary.iter() {
        if *digit > 1 {
            return None; // Invalid binary digit
        }
        decimal = decimal.checked_mul(2)?.checked_add(u32::from(*digit))?;
    }
    Some(decimal)
}
