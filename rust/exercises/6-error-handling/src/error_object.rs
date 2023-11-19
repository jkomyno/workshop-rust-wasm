use js_sys::JsString;
use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorObject {
    pub code: String,
    pub message: String,
}

/// Custom bindings to avoid using fallible `Reflect` for plain objects.
/// We do so because `TryInto<wasm_bindgen::JsValue>` isn't ABI-compatible with `wasm_bindgen`,
/// and we don't want `.unwrap()`.
#[wasm_bindgen]
extern "C" {
    type ObjectExt;

    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set(this: &ObjectExt, key: JsString, value: JsValue);
}

impl Into<wasm_bindgen::JsValue> for ErrorObject {
    fn into(self) -> wasm_bindgen::JsValue {
        let error_object = wasm_bindgen::JsError::new(&self.message);
        let error_object_as_value = JsValue::from(error_object);
        let error_object = error_object_as_value.unchecked_into::<ObjectExt>();

        // set all properties of `self` on `error_object`, one by one, except `message`
        error_object.set("code".into(), self.code.into());

        error_object.into()
    }
}
