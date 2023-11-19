pub use js_error_macros::*;

use serde::Serialize;

#[derive(Debug, Serialize, IntoJsError)]
#[js_error(message_field = "some_message")]
#[serde(rename_all = "camelCase")]
pub struct ErrorObjectMacro {
    pub code: String,
    pub some_message: String,
    pub some_number: i32,
}
