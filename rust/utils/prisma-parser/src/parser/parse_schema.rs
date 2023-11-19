use super::{parse_source_and_generator::parse_config_block, PrismaSchemaParser, Rule};
use crate::ast::*;
use crate::diagnostics::{Diagnostics, SchemaError};
use pest::Parser;

/// Parse a PSL string and return its AST.
pub fn parse_schema(schema_string: &str, diagnostics: &mut Diagnostics) -> SchemaAst {
    let schema_result = PrismaSchemaParser::parse(Rule::schema, schema_string);

    match schema_result {
        Ok(mut schema_wrapped) => {
            let schema = schema_wrapped.next().unwrap();
            let mut top_level_definitions: Vec<Top> = vec![];
            let mut pairs = schema.into_inner().peekable();

            while let Some(current) = pairs.next() {
                match current.as_rule() {
                    Rule::config_block => {
                        top_level_definitions.push(parse_config_block(current, diagnostics));
                    },
                    Rule::comment_block => {
                        // free floating
                    },
                    Rule::EOI => {}
                    Rule::CATCH_ALL => diagnostics.push_error(SchemaError::new_validation_error(
                        "This line is invalid. It does not start with any known Prisma schema keyword.",
                        current.as_span().into(),
                    )),
                    // TODO: Add view when we want it to be more visible as a feature.
                    Rule::arbitrary_block => diagnostics.push_error(SchemaError::new_validation_error(
                        "This block is invalid. It does not start with any known Prisma schema keyword. Valid keywords include \'model\', \'enum\', \'type\', \'datasource\' and \'generator\'.",
                        current.as_span().into(),
                    )),
                    Rule::empty_lines => (),
                    _ => unreachable!(),
                }
            }

            SchemaAst {
                tops: top_level_definitions,
            }
        }
        Err(err) => {
            let location: pest::Span<'_> = match err.location {
                pest::error::InputLocation::Pos(pos) => {
                    pest::Span::new(schema_string, pos, pos).unwrap()
                }
                pest::error::InputLocation::Span((from, to)) => {
                    pest::Span::new(schema_string, from, to).unwrap()
                }
            };

            let expected = match err.variant {
                pest::error::ErrorVariant::ParsingError { positives, .. } => {
                    get_expected_from_error(&positives)
                }
                _ => panic!("Could not construct parsing error. This should never happend."),
            };

            diagnostics.push_error(SchemaError::new_parser_error(expected, location.into()));

            SchemaAst { tops: Vec::new() }
        }
    }
}

fn get_expected_from_error(positives: &[Rule]) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(positives.len() * 6);

    for positive in positives {
        write!(out, "{positive:?}").unwrap();
    }

    out
}
