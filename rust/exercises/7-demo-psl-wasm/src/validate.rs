use crate::{error_object::ErrorObject, psl};
use std::fmt::Write as _;
use wasm_bindgen::prelude::wasm_bindgen;

pub(crate) static SCHEMA_PARSER_ERROR_CODE: &str = "P1012";

#[wasm_bindgen(js_name = "validate")]
pub fn validate(input_schema: &str) -> Result<(), ErrorObject> {
    let validate_schema = psl::validate(input_schema.into());
    let diagnostics = &validate_schema.diagnostics;

    if !diagnostics.has_errors() {
        return Ok(());
    }

    let mut formatted_error = diagnostics.to_pretty_string("schema.prisma", input_schema);
    let error_count = diagnostics.errors().len();

    let _ = write!(formatted_error, "\nValidation error count: {}", error_count);

    Err(ErrorObject {
        code: SCHEMA_PARSER_ERROR_CODE.into(),
        message: formatted_error,
    })
}
