// pub mod panic_simple;
pub mod panic_typed;
pub mod panic_untyped;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn trigger_panic(message: &str) {
    panic!("{:?}", message);
}
