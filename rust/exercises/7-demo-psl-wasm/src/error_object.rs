pub use js_error_macros::*;

use serde::Serialize;

#[derive(Debug, Serialize, IntoJsError)]
#[serde(rename_all = "camelCase")]
#[js_error(message_field = "message")]
pub struct ErrorObject {
    pub code: String,
    pub message: String,
}
