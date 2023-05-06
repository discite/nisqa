use digitalization::{to_binary, to_decimal};

fn main() {
    // Test the library digitalization
    let number: u32 = 1234567890;
    println!("Decimal: {}", number);
    let binary: [u8; 32] = to_binary(number);
    println!("Binary: {:?}", binary);
    let decimal_recovery = to_decimal(binary);
    println!("Decimal recovery: {}", decimal_recovery.unwrap());
}
