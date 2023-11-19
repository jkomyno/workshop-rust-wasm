use colored::{ColoredString, Colorize};

use crate::{
    pretty_print::{pretty_print, DiagnosticColorer},
    Span,
};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct SchemaError {
    span: Span,
    message: Cow<'static, str>,
}

impl SchemaError {
    pub(crate) fn new(message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        let message = message.into();
        SchemaError { message, span }
    }

    pub fn new_static(message: &'static str, span: Span) -> Self {
        Self::new(message, span)
    }

    pub fn new_literal_parser_error(
        literal_type: &str,
        raw_value: &str,
        span: Span,
    ) -> SchemaError {
        Self::new(
            format!("\"{raw_value}\" is not a valid value for {literal_type}."),
            span,
        )
    }

    pub fn new_argument_not_found_error(argument_name: &str, span: Span) -> SchemaError {
        Self::new(format!("Argument \"{argument_name}\" is missing."), span)
    }

    pub fn new_argument_count_mismatch_error(
        function_name: &str,
        required_count: usize,
        given_count: usize,
        span: Span,
    ) -> SchemaError {
        let msg = format!("Function \"{function_name}\" takes {required_count} arguments, but received {given_count}.");
        Self::new(msg, span)
    }

    pub fn new_attribute_argument_not_found_error(
        argument_name: &str,
        attribute_name: &str,
        span: Span,
    ) -> SchemaError {
        Self::new(
            format!("Argument \"{argument_name}\" is missing in attribute \"@{attribute_name}\"."),
            span,
        )
    }

    pub fn new_source_argument_not_found_error(
        argument_name: &str,
        source_name: &str,
        span: Span,
    ) -> SchemaError {
        Self::new(
            format!(
                "Argument \"{argument_name}\" is missing in data source block \"{source_name}\"."
            ),
            span,
        )
    }

    pub fn new_generator_argument_not_found_error(
        argument_name: &str,
        generator_name: &str,
        span: Span,
    ) -> SchemaError {
        Self::new(
            format!(
                "Argument \"{argument_name}\" is missing in generator block \"{generator_name}\"."
            ),
            span,
        )
    }

    pub fn new_attribute_validation_error(
        message: &str,
        attribute_name: &str,
        span: Span,
    ) -> SchemaError {
        Self::new(
            format!("Error parsing attribute \"{attribute_name}\": {message}"),
            span,
        )
    }

    pub fn new_duplicate_attribute_error(attribute_name: &str, span: Span) -> SchemaError {
        let msg = format!("Attribute \"@{attribute_name}\" can only be defined once.");
        Self::new(msg, span)
    }

    pub fn new_duplicate_top_error(
        name: &str,
        top_type: &str,
        existing_top_type: &str,
        span: Span,
    ) -> SchemaError {
        let msg = format!(
            "The {top_type} \"{name}\" cannot be defined because a {existing_top_type} with that name already exists.",
        );
        Self::new(msg, span)
    }

    pub fn new_duplicate_config_key_error(
        conf_block_name: &str,
        key_name: &str,
        span: Span,
    ) -> SchemaError {
        let msg = format!("Key \"{key_name}\" is already defined in {conf_block_name}.");
        Self::new(msg, span)
    }

    pub fn new_duplicate_argument_error(arg_name: &str, span: Span) -> SchemaError {
        Self::new(
            format!("Argument \"{arg_name}\" is already specified."),
            span,
        )
    }

    pub fn new_unused_argument_error(span: Span) -> SchemaError {
        Self::new("No such argument.", span)
    }

    pub fn new_source_validation_error(message: &str, source: &str, span: Span) -> SchemaError {
        Self::new(
            format!("Error validating datasource `{source}`: {message}"),
            span,
        )
    }

    pub fn new_validation_error(message: &str, span: Span) -> SchemaError {
        Self::new(format!("Error validating: {message}"), span)
    }

    pub fn new_parser_error(expected_str: String, span: Span) -> SchemaError {
        Self::new(
            format!("Unexpected token. Expected one of: {expected_str}"),
            span,
        )
    }

    pub fn new_functional_evaluation_error(
        message: impl Into<Cow<'static, str>>,
        span: Span,
    ) -> SchemaError {
        Self::new(message.into(), span)
    }

    pub fn new_environment_functional_evaluation_error(
        var_name: String,
        span: Span,
    ) -> SchemaError {
        Self::new(format!("Environment variable not found: {var_name}."), span)
    }

    pub fn new_type_not_found_error(type_name: &str, span: Span) -> SchemaError {
        let msg = format!(
            "Type \"{type_name}\" is neither a built-in type, nor refers to another model, custom type, or enum."
        );
        Self::new(msg, span)
    }

    pub fn new_scalar_type_not_found_error(type_name: &str, span: Span) -> SchemaError {
        Self::new(
            format!("Type \"{type_name}\" is not a built-in type."),
            span,
        )
    }

    pub fn new_attribute_not_known_error(attribute_name: &str, span: Span) -> SchemaError {
        Self::new(format!("Attribute not known: \"@{attribute_name}\"."), span)
    }

    pub fn new_property_not_known_error(property_name: &str, span: Span) -> SchemaError {
        Self::new(format!("Property not known: \"{property_name}\"."), span)
    }

    pub fn new_argument_not_known_error(property_name: &str, span: Span) -> SchemaError {
        Self::new(format!("Argument not known: \"{property_name}\"."), span)
    }

    pub fn new_datasource_provider_not_known_error(provider: &str, span: Span) -> SchemaError {
        Self::new(
            format!("Datasource provider not known: \"{provider}\"."),
            span,
        )
    }

    pub fn new_value_parser_error(expected_type: &str, raw: &str, span: Span) -> SchemaError {
        let msg = format!("Expected {expected_type}, but found {raw}.");
        Self::new(msg, span)
    }

    pub fn new_type_mismatch_error(
        expected_type: &str,
        received_type: &str,
        raw: &str,
        span: Span,
    ) -> SchemaError {
        let msg = format!(
            "Expected a {expected_type} value, but received {received_type} value `{raw}`."
        );
        Self::new(msg, span)
    }

    pub fn new_config_property_missing_value_error(
        property_name: &str,
        config_name: &str,
        config_kind: &str,
        span: Span,
    ) -> SchemaError {
        let msg = format!(
            "Property {property_name} in {config_kind} {config_name} needs to be assigned a value"
        );
        Self::new(msg, span)
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn pretty_print(
        &self,
        f: &mut dyn std::io::Write,
        file_name: &str,
        text: &str,
    ) -> std::io::Result<()> {
        pretty_print(
            f,
            file_name,
            text,
            self.span(),
            self.message.as_ref(),
            &SchemaErrorColorer {},
        )
    }
}

struct SchemaErrorColorer {}

impl DiagnosticColorer for SchemaErrorColorer {
    fn title(&self) -> &'static str {
        "error"
    }

    fn primary_color(&self, token: &'_ str) -> ColoredString {
        token.bright_red()
    }
}
