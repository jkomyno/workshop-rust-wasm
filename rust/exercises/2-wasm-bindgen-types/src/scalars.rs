// use wasm_bindgen::prelude::wasm_bindgen;

/// Models a struct having fields of several different types, all scalar.
pub struct Scalars {
    pub n: u32,
    pub id: u64,
    pub letter: char,
    pub toggle: bool,
}

pub fn new_scalars(n: u32, id: u64, letter: char, toggle: bool) -> Scalars {
    Scalars {
        n,
        id,
        letter,
        toggle,
    }
}

/// Given a `Scalars` instance, return its `letter` field.
pub fn get_letter(params: Scalars) -> char {
    params.letter
}
