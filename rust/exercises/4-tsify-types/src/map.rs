use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Index {
    /// `serde(flatten)` flattens `frequency_map` into the parent struct.
    /// This is useful when the parent struct has no other fields.
    #[serde(flatten)]
    pub frequency_map: HashMap<String, u32>,
}

/// `serde(transparent)` collapses `VecStringContainer` into its only field `data`
/// when serializing and deserializing, so that JavaScript views it as a string plain array.
#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(transparent)]
pub struct VecStringContainer {
    pub data: Vec<String>,
}

/// Retrieve the `n` most frequent words in the given index.
#[wasm_bindgen]
pub fn top_frequent(index: Index, n: usize) -> VecStringContainer {
    let mut heap = BinaryHeap::new();
    for (word, frequency) in index.frequency_map {
        heap.push((frequency, word));
    }

    let mut result = Vec::new();
    for _ in 0..n {
        if let Some((_, word)) = heap.pop() {
            result.push(word);
        }
    }

    VecStringContainer { data: result }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_top_frequent_test() {
        let mut frequency_map: HashMap<String, u32> = HashMap::new();
        frequency_map.insert("Python".into(), 3);
        frequency_map.insert("Rust".into(), 10);
        frequency_map.insert("TypeScript".into(), 8);
        frequency_map.insert("Golang".into(), 6);
        frequency_map.insert("WebAssembly".into(), 9);

        let index = Index { frequency_map };

        let top_3 = top_frequent(index, 3).data;
        assert_eq!(top_3, vec!["Rust", "WebAssembly", "TypeScript"]);
    }
}
