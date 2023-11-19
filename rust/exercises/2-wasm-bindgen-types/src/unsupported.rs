use wasm_bindgen::prelude::wasm_bindgen;

/// Models potentially failed results as an Either type.
/// The `Ok` variant models success, and the `Err` variant models failure.
/// Note that this is structurally equivalent to the `Result<T, E>` type in Rust,
/// with the concrete types `T=i32` and `E=String`.
/// This fails with:
///
/// ```console
/// error: only C-Style enums allowed with #[wasm_bindgen]
/// --> wasm-bindgen-playground/src/types/unsupported.rs:18:5
///    |
/// 18 |   Ok(i32),
///    |
/// ```
#[wasm_bindgen]
pub enum Either {
    Ok(i32),
    Err(String),
}

/// Models an object with a String member.
/// This fails with:
///
/// ```console
/// error[E0277]: the trait bound `String: std::marker::Copy` is not satisfied
///   --> wasm-bindgen-playground/src/types/unsupported.rs:40:11
///    |
/// 40 |   pub id: String,
///    |           ^^^^^^ the trait `std::marker::Copy` is not implemented for `String`
///    |
/// note: required by a bound in `__wbg_get_stringwrap_id::assert_copy`
///   --> wasm-bindgen-playground/src/types/unsupported.rs:38:1
///    |
/// 38 | #[wasm_bindgen]
///    | ^^^^^^^^^^^^^^^ required by this bound in `__wbg_get_stringwrap_id::assert_copy`
/// ```
#[wasm_bindgen]
pub struct StringParams2 {
    pub id: String,
}

/// Models an object with integer arrays as members.
/// This fails with:
///
/// ```console
/// error[E0277]: the trait bound `Vec<u32>: std::marker::Copy` is not satisfied
///   --> wasm-bindgen-playground/src/types/unsupported.rs:63:22
///    |
/// 63 |   pub uint32: Vec<u32>,
///    |                      ^ the trait `std::marker::Copy` is not implemented for `Vec<u32>`
///    |
/// note: required by a bound in `unsupported::_::__wbg_get_arrayparams_uint32::assert_copy`
///   --> wasm-bindgen-playground/src/types/unsupported.rs:60:1
///    |
/// 60 | #[wasm_bindgen]
///    | ^^^^^^^^^^^^^^^ required by this bound in `unsupported::_::__wbg_get_arrayparams_uint32::assert_copy`
///    = note: this error originates in the attribute macro `wasm_bindgen`
/// ```
#[wasm_bindgen]
pub struct NumericArrays2 {
    pub int32: Vec<i32>,
    pub uint32: Vec<u32>,
    pub uint64: Vec<u64>,
    pub int64: Vec<i64>,
    pub float: Vec<f32>,
    pub double: Vec<f64>,
}

/// Models an object with a nested array as a member.
/// This fails with:
///
/// ```console
/// error[E0277]: the trait bound `Vec<i32>: JsObject` is not satisfied
///    --> wasm-bindgen-playground/src/types/unsupported.rs:112:1
///     |
/// 112 | #[wasm_bindgen]
///     | ^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `Vec<i32>`
///     |
///     = help: the following other types implement trait `IntoWasmAbi`:
///               Box<[JsValue]>
///               Box<[T]>
///               Box<[f32]>
///               Box<[f64]>
///               Box<[i16]>
///               Box<[i32]>
///               Box<[i64]>
///               Box<[i8]>
///             and 6 others
///     = note: required for `Box<[Vec<i32>]>` to implement `IntoWasmAbi`
///     = note: this error originates in the attribute macro `wasm_bindgen` (in Nightly builds, run with -Z macro-backtrace for more info)
///
/// error[E0277]: the trait bound `Vec<i32>: JsObject` is not satisfied
///    --> wasm-bindgen-playground/src/types/unsupported.rs:112:1
///     |
/// 112 | #[wasm_bindgen]
///     | ^^^^^^^^^^^^^^^ the trait `JsObject` is not implemented for `Vec<i32>`
///     |
///     = help: the following other types implement trait `FromWasmAbi`:
///               Box<[JsValue]>
///               Box<[T]>
///               Box<[f32]>
///               Box<[f64]>
///               Box<[i16]>
///               Box<[i32]>
///               Box<[i64]>
///               Box<[i8]>
///             and 6 others
///     = note: required for `Box<[Vec<i32>]>` to implement `FromWasmAbi`
///     = note: this error originates in the attribute macro `wasm_bindgen`
/// ```
#[wasm_bindgen]
pub struct NestedArrays {
    pub matrix: Vec<Vec<i32>>,
}
