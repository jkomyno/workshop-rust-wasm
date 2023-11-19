use serde::{self, Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

/// When it comes to `enum`s, `wasm-bindgen` supports `C`-style enums only.
/// (see: https://github.com/rustwasm/wasm-bindgen/issues/2407).
/// We can however combine `serde` and `tsify` to support enum variants,
/// allowing us to model Algebraic Data Types and compile them to Wasm.
///
/// When applying the `Tsify` derive crate, remember to add:
/// - the `#[tsify(into_wasm_abi)]` attribute when deriving `Serialize`
/// - the `#[tsify(from_wasm_abi)]` attribute when deriving `Deserialize`

/// Models a closed set of database providers as a C-style enum.
#[derive(Deserialize, Serialize, Tsify)]
#[serde(tag = "_tag", content = "value")]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub enum Either {
    Ok(i32),
    Err(String),
}

impl ToString for Either {
    fn to_string(&self) -> String {
        match self {
            Either::Ok(ok) => format!("Ok({})", ok),
            Either::Err(err) => format!("Err({})", err),
        }
    }
}

// While you can define `impl` blocks for `enum`s and apply the `#[wasm_bindgen]`
// macro to them, `wasm_bindgen` will silently exclude them from the generated bindings.
// Use free functions instead.
#[wasm_bindgen]
impl Either {
    #[wasm_bindgen]
    pub fn to_string_method(self) -> String {
        self.to_string()
    }
}

#[wasm_bindgen]
pub fn either_to_string(either: Either) -> String {
    either.to_string()
}

#[wasm_bindgen]
pub fn either_ok(ok: i32) -> Either {
    Either::Ok(ok)
}

#[wasm_bindgen]
pub fn either_err(err: String) -> Either {
    Either::Err(err)
}
