#![no_main]
#[macro_use] extern crate libfuzzer_sys;
use digitalization::{to_binary};

fuzz_target!(|data: u32| {
    let _ = to_binary(data);
});
