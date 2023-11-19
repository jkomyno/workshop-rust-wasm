mod helpers;
mod parse_arguments;
mod parse_comments;
mod parse_expression;
mod parse_schema;
mod parse_source_and_generator;

pub use parse_schema::parse_schema;

// The derive is placed here because it generates the `Rule` enum which is used in all parsing functions.
// It is more convenient if this enum is directly available here.
#[derive(pest_derive::Parser)]
#[grammar = "parser/schema.pest"]
pub(crate) struct PrismaSchemaParser;