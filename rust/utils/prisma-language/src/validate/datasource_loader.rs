use crate::{
    coerce_expression::coerce_opt,
    configuration::StringFromEnvVar,
    connector::Connector,
    diagnostics::{Diagnostics, SchemaError},
    schema_ast::ast::{self, Expression},
    Datasource,
};
use std::{borrow::Cow, collections::HashMap};

const URL_KEY: &str = "url";
const PROVIDER_KEY: &str = "provider";

/// Loads all datasources from the provided schema AST.
/// - `ignore_datasource_urls`: datasource URLs are not parsed. They are replaced with dummy values.
/// - `datasource_url_overrides`: datasource URLs are not parsed and overridden with the provided ones.
pub(crate) fn load_datasources_from_ast(
    ast_schema: &ast::SchemaAst,
    diagnostics: &mut Diagnostics,
) -> Vec<Datasource> {
    let mut sources = Vec::new();

    for src in ast_schema.sources() {
        if let Some(source) = lift_datasource(src, diagnostics) {
            sources.push(source)
        }
    }

    if sources.len() > 1 {
        for src in ast_schema.sources() {
            diagnostics.push_error(SchemaError::new_source_validation_error(
                "You defined more than one datasource. This is not allowed yet because support for multiple databases has not been implemented yet.",
                &src.name.name,
                src.span,
            ));
        }
    }

    sources
}

fn lift_datasource(
    ast_source: &ast::SourceConfig,
    diagnostics: &mut Diagnostics,
) -> Option<Datasource> {
    let source_name = ast_source.name.name.as_str();
    let mut args: HashMap<_, (_, &Expression)> = ast_source
        .properties
        .iter()
        .map(|arg| match &arg.value {
            Some(expr) => Some((arg.name.name.as_str(), (arg.span, expr))),
            None => {
                diagnostics.push_error(SchemaError::new_config_property_missing_value_error(
                    &arg.name.name,
                    source_name,
                    "datasource",
                    ast_source.span,
                ));
                None
            }
        })
        .collect::<Option<HashMap<_, (_, _)>>>()?;

    let (provider, provider_arg) = match args.remove(PROVIDER_KEY) {
        Some((_span, provider_arg)) => {
            if provider_arg.is_env_expression() {
                let msg = Cow::Borrowed(
                    "A datasource must not use the env() function in the provider argument.",
                );
                diagnostics.push_error(SchemaError::new_functional_evaluation_error(
                    msg,
                    ast_source.span,
                ));
                return None;
            }

            let provider = match coerce_opt::string(provider_arg) {
                Some("") => {
                    diagnostics.push_error(SchemaError::new_source_validation_error(
                        "The provider argument in a datasource must not be empty",
                        source_name,
                        provider_arg.span(),
                    ));
                    return None;
                }
                None => {
                    diagnostics.push_error(SchemaError::new_source_validation_error(
                        "The provider argument in a datasource must be a string literal",
                        source_name,
                        provider_arg.span(),
                    ));
                    return None;
                }
                Some(provider) => provider,
            };

            (provider, provider_arg)
        }

        None => {
            diagnostics.push_error(SchemaError::new_source_argument_not_found_error(
                "provider",
                source_name,
                ast_source.span,
            ));
            return None;
        }
    };

    let active_connector = match Connector::new(provider) {
        Some(c) => c,
        None => {
            diagnostics.push_error(SchemaError::new_datasource_provider_not_known_error(
                provider,
                provider_arg.span(),
            ));

            return None;
        }
    };

    let (url, url_span) = match args.remove(URL_KEY) {
        Some((_span, url_arg)) => (
            StringFromEnvVar::coerce(url_arg, diagnostics)?,
            url_arg.span(),
        ),

        None => {
            diagnostics.push_error(SchemaError::new_source_argument_not_found_error(
                URL_KEY,
                source_name,
                ast_source.span,
            ));

            return None;
        }
    };

    use prisma_parser::ast::WithDocumentation;
    let documentation = ast_source.documentation().map(String::from);

    for (name, (span, _)) in args.into_iter() {
        diagnostics.push_error(SchemaError::new_property_not_known_error(name, span));
    }

    Some(Datasource {
        name: source_name.to_owned(),
        provider: provider.to_owned(),
        url,
        url_span,
        documentation,
        active_connector,
    })
}
