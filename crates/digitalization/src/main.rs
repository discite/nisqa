use std::io::{self, Read};
use digitalization::to_binary;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let n: u32 = input.trim().parse().expect("Failed to parse input");

    let binary = to_binary(n);

    for digit in binary.iter() {
        print!("{}", digit);
    }
}