use crate::ast::{traits::WithSpan, SourceConfig, Span};

use super::Identifier;

/// Enum for distinguishing between top-level entries
#[derive(Debug, Clone)]
pub enum Top {
    /// A datasource block
    Source(SourceConfig),
}

impl Top {
    /// A string saying what kind of item this is.
    pub fn get_type(&self) -> &str {
        match self {
            Top::Source(_) => "source",
        }
    }

    /// The name of the item.
    pub fn identifier(&self) -> &Identifier {
        match self {
            Top::Source(x) => &x.name,
        }
    }

    /// The name of the item.
    pub fn name(&self) -> &str {
        &self.identifier().name
    }

    /// Try to interpret the item as a datasource block.
    pub fn as_source(&self) -> Option<&SourceConfig> {
        match self {
            Top::Source(source) => Some(source),
            // _ => None,
        }
    }
}

impl WithSpan for Top {
    fn span(&self) -> Span {
        match self {
            Top::Source(source) => source.span(),
        }
    }
}
