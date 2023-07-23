use codification::{char_to_u32, u32_to_char};
use digitalization::{u32_to_binary, binary_to_u32};

fn main() {
    // Test the library codification and digitalization
    let base_string = "Hello World!";
    println!("String: {}", base_string);
    let base_chars: Vec<char> = base_string.chars().collect();
    println!("VecChars: {:?}", base_chars);
    let base_u32: Vec<u32> = base_chars.iter().map(|c| char_to_u32(c)).collect();
    println!("VecUnicode: {:?} with lenght {}", base_u32, base_u32.len());
    let base_binary: Vec<[u8; 32]> = base_u32.iter().map(|u| u32_to_binary(*u)).collect();
    println!("VecBinaries: {:?}", base_binary);
    let received_binary = base_binary.clone();
    let received_u32: Vec<u32> = received_binary.iter().map(|b| binary_to_u32(*b).unwrap_or_default()).collect();
    let received_chars: Vec<char> = received_u32.iter().map(|u| u32_to_char(u)).collect();
    println!("String: {:?}", received_chars);
}
