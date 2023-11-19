#[derive(Debug)]
pub(crate) struct CustomError(serde_json::error::Error);

impl std::error::Error for CustomError {}

impl CustomError {
    pub fn new(e: serde_json::error::Error) -> Self {
        Self(e)
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[CustomError] ")?;
        self.0.fmt(f)
    }
}
