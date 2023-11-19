mod custom_error;
mod error_object;
mod error_object_macro;
mod event;

use custom_error::CustomError;
use error_object::ErrorObject;
use error_object_macro::ErrorObjectMacro;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

pub use event::Event;

#[wasm_bindgen(js_name = "parseWithStringError")]
pub fn parse_with_string_error(event: &str) -> Result<Event, String> {
    let event: Event = serde_json::from_str(event).map_err(|e| e.to_string())?;
    Ok(event)
}

#[wasm_bindgen(js_name = "parseWithError")]
pub fn parse_with_error(event: &str) -> Result<Event, JsError> {
    let event: Event = serde_json::from_str(event).map_err(|e| JsError::from(e))?;
    Ok(event)
}

#[wasm_bindgen(js_name = "parseWithCustomError")]
pub fn parse_with_custom_error(event: &str) -> Result<Event, JsError> {
    let event: Event = serde_json::from_str(event).map_err(|e| CustomError::new(e))?;
    Ok(event)
}

#[wasm_bindgen(js_name = "parseWithErrorObject")]
pub fn parse_with_error_object(event: &str) -> Result<Event, ErrorObject> {
    let event: Event = serde_json::from_str(event).map_err(|e| ErrorObject {
        code: "ERROR_OBJECT_CODE".to_string(),
        message: e.to_string(),
    })?;
    Ok(event)
}

#[wasm_bindgen(js_name = "parseWithErrorObjectMacro")]
pub fn parse_with_error_object_macro(event: &str) -> Result<Event, ErrorObjectMacro> {
    let event: Event = serde_json::from_str(event).map_err(|e| ErrorObjectMacro {
        code: "ERROR_OBJECT_CODE".to_string(),
        some_message: e.to_string(),
        some_number: 42,
    })?;
    Ok(event)
}

#[wasm_bindgen(js_name = "parseWithPanic")]
pub fn parse_with_panic(event: &str) -> Event {
    let event: Event = serde_json::from_str(event).unwrap();
    event
}
