use super::SchemaError;
use crate::warning::SchemaWarning;

/// Represents a list of validation or parser errors and warnings.
///
/// This is used to accumulate multiple errors and warnings during validation.
/// It is used to not error out early and instead show multiple errors at once.
#[derive(Debug)]
pub struct Diagnostics {
    errors: Vec<SchemaError>,
    warnings: Vec<SchemaWarning>,
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        Diagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn warnings(&self) -> &[SchemaWarning] {
        &self.warnings
    }

    pub fn into_warnings(self) -> Vec<SchemaWarning> {
        self.warnings
    }

    pub fn errors(&self) -> &[SchemaError] {
        &self.errors
    }

    pub fn push_error(&mut self, err: SchemaError) {
        self.errors.push(err)
    }

    pub fn push_warning(&mut self, warning: SchemaWarning) {
        self.warnings.push(warning)
    }

    /// Returns true, if there is at least one error in this collection.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn to_result(&mut self) -> Result<(), Diagnostics> {
        if self.has_errors() {
            Err(std::mem::take(self))
        } else {
            Ok(())
        }
    }

    pub fn to_pretty_string(&self, file_name: &str, schema_string: &str) -> String {
        let mut message: Vec<u8> = Vec::new();

        for err in self.errors() {
            err.pretty_print(&mut message, file_name, schema_string)
                .expect("printing schema error");
        }

        String::from_utf8_lossy(&message).into_owned()
    }

    pub fn warnings_to_pretty_string(&self, file_name: &str, schema_string: &str) -> String {
        let mut message: Vec<u8> = Vec::new();

        for warn in self.warnings() {
            warn.pretty_print(&mut message, file_name, schema_string)
                .expect("printing schema warning");
        }

        String::from_utf8_lossy(&message).into_owned()
    }
}

impl From<SchemaError> for Diagnostics {
    fn from(error: SchemaError) -> Self {
        let mut col = Diagnostics::new();
        col.push_error(error);
        col
    }
}

impl From<SchemaWarning> for Diagnostics {
    fn from(warning: SchemaWarning) -> Self {
        let mut col = Diagnostics::new();
        col.push_warning(warning);
        col
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}
