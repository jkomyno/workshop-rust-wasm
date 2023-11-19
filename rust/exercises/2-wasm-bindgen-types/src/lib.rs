//! Uncomment `pub mod unsupported` to see the compiler errors
//! when trying to use unsupported types with `wasm-bindgen`.

pub mod enum_c;
pub mod scalars;
// pub mod unsupported

// use wasm_bindgen::prelude::wasm_bindgen;

pub fn inc_biguint(x: u64) -> u64 {
    x + 1
}

pub fn inc_uint(x: u32) -> u32 {
    x + 1
}

pub fn inc_int(x: i32) -> i32 {
    x + 1
}

pub fn inc_byte(x: u8) -> u8 {
    x + 1
}

pub fn inc_f64(x: f64) -> f64 {
    x + 1.0
}

pub fn inc_int_maybe(x: Option<i32>) -> Option<i32> {
    x.map(|x| x + 1)
}

pub fn inc_int_or_fail(x: Option<i32>) -> Result<i32, String> {
    x.ok_or_else(|| "No value provided".to_string())
        .map(|x| x + 1)
}
