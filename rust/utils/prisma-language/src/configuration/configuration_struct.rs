use super::Datasource;
use crate::diagnostics::{Diagnostics, SchemaError, SchemaWarning};

#[derive(Debug)]
pub struct Configuration {
    pub datasources: Vec<Datasource>,
    pub warnings: Vec<SchemaWarning>,
}

impl Configuration {
    pub fn validate_that_one_datasource_is_provided(&self) -> Result<(), Diagnostics> {
        if self.datasources.is_empty() {
            Err(SchemaError::new_validation_error(
                "You defined no datasource. You must define exactly one datasource.",
                prisma_parser::ast::Span::new(0, 0),
            )
            .into())
        } else {
            Ok(())
        }
    }

    /// Resolve datasource url for query engine.
    ///
    /// The main interesting thing here is we want to ignore any error that may arise from resolving
    /// direct_url.
    pub fn resolve_datasource_urls_query_engine<F>(
        &mut self,
        url_overrides: &[(String, String)],
        env: F,
        ignore_env_errors: bool,
    ) -> Result<(), Diagnostics>
    where
        F: Fn(&str) -> Option<String> + Copy,
    {
        for datasource in &mut self.datasources {
            if let Some((_, url)) = url_overrides
                .iter()
                .find(|(name, _url)| name == &datasource.name)
            {
                datasource.url.value = Some(url.clone());
                datasource.url.from_env_var = None;
            }

            if datasource.url.from_env_var.is_some() && datasource.url.value.is_none() {
                datasource.url.value = match datasource.load_url(env) {
                    Ok(url) => Some(url),
                    Err(_) if ignore_env_errors => None,
                    Err(error) => return Err(error),
                };
            }
        }

        Ok(())
    }

    /// Resolve datasource URL's for getConfig.
    /// The main reason this exists is:
    ///   - we want to error if we can't resolve direct_url
    ///   - we want to skip validation for url IF we have a direct_url
    ///
    /// For that last bit, we only do this currently because our validation errors on URL's starting
    /// with 'prisma://'. We would ideally like to do the other validations and ignore in this case.
    pub fn resolve_datasource_urls_prisma_fmt<F>(
        &mut self,
        url_overrides: &[(String, String)],
        env: F,
    ) -> Result<(), Diagnostics>
    where
        F: Fn(&str) -> Option<String> + Copy,
    {
        for datasource in &mut self.datasources {
            if let Some((_, url)) = url_overrides
                .iter()
                .find(|(name, _url)| name == &datasource.name)
            {
                datasource.url.value = Some(url.clone());
                datasource.url.from_env_var = None;
            }

            if datasource.url.from_env_var.is_some() && datasource.url.value.is_none() {
                datasource.url.value = Some(datasource.load_url(env)?);
            }
        }

        Ok(())
    }
}
