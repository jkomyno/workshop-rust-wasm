#![allow(missing_docs, clippy::needless_lifetimes)] // lifetimes are used by the macro
use crate::diagnostics::{Diagnostics, SchemaError};
use crate::schema_ast::ast;

macro_rules! impl_coercions {
    ($lt:lifetime; $($name:ident : $expected_type:expr => $out:ty;)*) => {
        /// Coerce expressions to a specific type, emitting a validation error if the coercion
        /// fails. See the `coerce_opt` module if you do not want to emit validation errors.
        pub mod coerce {
            #![allow(missing_docs)]

            use super::*;

            $(
            pub fn $name<$lt>(expr: & $lt ast::Expression, diagnostics: &mut Diagnostics) -> Option<$out> {
                coerce::<$lt>(super::coerce_opt::$name, $expected_type)(expr, diagnostics)
            }
            )*
        }
    }
}

impl_coercions! {
    'a;
    string : "string" => &'a str;
}

/// Fallible coercions of PSL expressions to more specific types.
pub mod coerce_opt {
    #![allow(missing_docs, clippy::needless_lifetimes)] // lifetimes are used by the macro

    use super::*;

    pub fn string<'a>(expr: &'a ast::Expression) -> Option<&'a str> {
        expr.as_string_value().map(|(s, _)| s)
    }
}

const fn coerce<'a, T>(
    coercion: impl Fn(&'a ast::Expression) -> Option<T>,
    expected_type: &'static str,
) -> impl (Fn(&'a ast::Expression, &mut Diagnostics) -> Option<T>) {
    move |expr, diagnostics| match coercion(expr) {
        Some(t) => Some(t),
        None => {
            diagnostics.push_error(SchemaError::new_type_mismatch_error(
                expected_type,
                expr.describe_value_type(),
                &expr.to_string(),
                expr.span(),
            ));
            None
        }
    }
}
