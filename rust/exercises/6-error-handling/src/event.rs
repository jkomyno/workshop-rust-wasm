use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Event {
    name: String,
    year: u16,
}
