use js_error_macros::IntoJsError;
use serde::Serialize;

#[derive(Debug, Serialize, IntoJsError)]
#[js_error()]
#[serde(rename_all = "camelCase")]
pub struct WithDefaultMessageField {
    pub code: String,
    pub message: String,
    pub some_number: i32,
}
