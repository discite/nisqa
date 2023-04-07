#![no_main]
#[macro_use] extern crate libfuzzer_sys;
use digitalization::{to_binary, to_decimal};

fuzz_target!(|data: u32| {
    let binary = to_binary(data);
    if let Some(decimal) = to_decimal(binary) {
        assert_eq!(data, decimal);
    } 
});
