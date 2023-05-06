#![no_main]
#[macro_use] extern crate libfuzzer_sys;
use digitalization::{to_decimal};

fuzz_target!(|binary: [u8; 32]| {
    if let Some(decimal) = to_decimal(binary) {
        let _ = decimal;
    } 
});
