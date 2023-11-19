#![allow(clippy::derive_partial_eq_without_eq)]

/// Fallible coercions of PSL expressions to more specific types.
mod coerce_expression;
mod configuration;
mod connector;
mod validate;

pub use crate::configuration::{
    Configuration, Datasource, DatasourceConnectorData, StringFromEnvVar,
};
pub(crate) use prisma_diagnostics as diagnostics;
pub(crate) use prisma_parser as schema_ast;

use self::validate::datasource_loader;
use diagnostics::Diagnostics;
use schema_ast::ast;

pub struct ValidatedSchema {
    pub configuration: Configuration,
    pub diagnostics: Diagnostics,
}

impl std::fmt::Debug for ValidatedSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<Prisma schema>")
    }
}

/// The most general API for dealing with Prisma schemas. It accumulates what analysis and
/// validation information it can, and returns it along with any error and warning diagnostics.
pub fn validate(file: &str) -> ValidatedSchema {
    let mut diagnostics = Diagnostics::new();
    let ast = schema_ast::parse_schema(file, &mut diagnostics);
    let configuration = validate_configuration(&ast, &mut diagnostics);

    ValidatedSchema {
        diagnostics,
        configuration,
    }
}

/// Parse and analyze a Prisma schema.
pub fn parse_schema(file: &str) -> Result<ValidatedSchema, String> {
    let mut schema = validate(file.into());
    schema
        .diagnostics
        .to_result()
        .map_err(|err| err.to_pretty_string("schema.prisma", file))?;
    Ok(schema)
}

/// Loads all configuration blocks from a schema using the built-in source definitions.
pub fn parse_configuration(schema: &str) -> Result<Configuration, diagnostics::Diagnostics> {
    let mut diagnostics = Diagnostics::default();
    let ast = schema_ast::parse_schema(schema, &mut diagnostics);
    let out = validate_configuration(&ast, &mut diagnostics);
    diagnostics.to_result().map(|_| out)
}

fn validate_configuration(
    schema_ast: &ast::SchemaAst,
    diagnostics: &mut Diagnostics,
) -> Configuration {
    let datasources = datasource_loader::load_datasources_from_ast(schema_ast, diagnostics);

    Configuration {
        datasources,
        warnings: diagnostics.warnings().to_owned(),
    }
}
