use serde::{self, Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Deserialize, Serialize, Tsify)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Scalars {
    pub r#i32: i32,
    pub r#i128: i128,
    pub r#bool: bool,
    pub r#str: String,
    pub r#char: char,
}

#[wasm_bindgen]
pub fn get_char_from_scalars(scalars: Scalars) -> char {
    scalars.r#char
}

#[wasm_bindgen]
pub fn get_str_from_scalars(scalars: Scalars) -> String {
    scalars.r#str
}
