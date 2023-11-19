use js_error_macros::IntoJsError;
use serde::Serialize;

#[derive(Debug, Serialize, IntoJsError)]
#[js_error(message_field = "some_message")]
#[serde(rename_all = "camelCase")]
pub struct WithCustomMessageField {
    pub code: String,
    pub some_message: String,
    pub some_number: i32,
}
