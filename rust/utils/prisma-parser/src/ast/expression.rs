use crate::ast::{self, Span};
use std::fmt;

/// Represents arbitrary, even nested, expressions.
#[derive(Debug, Clone)]
pub enum Expression {
    /// Any string value.
    StringValue(String, Span),
    /// A function call like node with a name and arguments.
    Function(String, ast::ArgumentsList, Span),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::StringValue(val, _) => write!(f, "{}", crate::string_literal(val)),
            Expression::Function(fun, args, _) => {
                let args = args
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{fun}({args})")
            }
        }
    }
}

impl Expression {
    pub fn as_string_value(&self) -> Option<(&str, Span)> {
        match self {
            Expression::StringValue(s, span) => Some((s, *span)),
            _ => None,
        }
    }

    pub fn as_function(&self) -> Option<(&str, &ast::ArgumentsList, Span)> {
        match self {
            Expression::Function(name, args, span) => Some((name, args, *span)),
            _ => None,
        }
    }

    pub fn span(&self) -> Span {
        match &self {
            Self::StringValue(_, span) => *span,
            Self::Function(_, _, span) => *span,
        }
    }

    pub fn is_env_expression(&self) -> bool {
        match &self {
            Self::Function(name, _, _) => name == "env",
            _ => false,
        }
    }

    /// Creates a friendly readable representation for a value's type.
    pub fn describe_value_type(&self) -> &'static str {
        match self {
            Expression::StringValue(_, _) => "string",
            Expression::Function(_, _, _) => "functional",
        }
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Expression::Function(_, _, _))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Expression::StringValue(_, _))
    }
}
