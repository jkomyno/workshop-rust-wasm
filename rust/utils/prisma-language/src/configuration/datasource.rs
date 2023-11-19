use crate::{
    configuration::StringFromEnvVar,
    connector::Connector,
    diagnostics::{Diagnostics, SchemaError, Span},
};
use std::{any::Any, borrow::Cow, path::Path};

/// a `datasource` from the prisma schema.
pub struct Datasource {
    pub name: String,
    /// The provider string
    pub provider: String,
    pub url: StringFromEnvVar,
    pub url_span: Span,
    pub documentation: Option<String>,
    /// the connector of the active provider
    pub(crate) active_connector: Connector,
}

pub enum UrlValidationError {
    EmptyUrlValue,
    EmptyEnvValue(String),
    NoEnvValue(String),
    NoUrlOrEnv,
}

#[derive(Default)]
pub struct DatasourceConnectorData {
    data: Option<Box<dyn Any + Send + Sync + 'static>>,
}

impl DatasourceConnectorData {
    pub fn new(data: Box<dyn Any + Send + Sync + 'static>) -> Self {
        Self { data: Some(data) }
    }

    #[track_caller]
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.data.as_ref().map(|data| data.downcast_ref().unwrap())
    }
}

impl std::fmt::Debug for Datasource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Datasource")
            .field("name", &self.name)
            .field("provider", &self.provider)
            .field("url", &"<url>")
            .field("documentation", &self.documentation)
            .field("active_connector", &&"...")
            .finish()
    }
}

impl Datasource {
    /// Load the database URL, validating it and resolving env vars in the
    /// process. Also see `load_url_with_config_dir()`.
    pub fn load_url<F>(&self, env: F) -> Result<String, Diagnostics>
    where
        F: Fn(&str) -> Option<String>,
    {
        let url = self.load_url_no_validation(env)?;

        self.active_connector
            .validate_url(&url)
            .map_err(|err_str| {
                let err_str = Cow::from(err_str);
                SchemaError::new_source_validation_error(
                    &format!("the URL {}", &err_str),
                    &self.name,
                    self.url_span,
                )
            })?;

        Ok(url)
    }

    /// Load the database URL, without validating it and resolve env vars in the
    /// process.
    pub fn load_url_no_validation<F>(&self, env: F) -> Result<String, Diagnostics>
    where
        F: Fn(&str) -> Option<String>,
    {
        from_url(&self.url, env).map_err(|err| match err {
                UrlValidationError::EmptyUrlValue => {
                    let msg = "You must provide a nonempty URL";
                    SchemaError::new_source_validation_error(msg, &self.name, self.url_span).into()
                }
                UrlValidationError::EmptyEnvValue(env_var) => {
                    SchemaError::new_source_validation_error(
                        &format!("You must provide a nonempty URL. The environment variable `{env_var}` resolved to an empty string."),
                        &self.name,
                        self.url_span,
                    )
                    .into()
                }
                UrlValidationError::NoEnvValue(env_var) => {
                    SchemaError::new_environment_functional_evaluation_error(env_var, self.url_span).into()
                }
                UrlValidationError::NoUrlOrEnv => unreachable!("Missing url in datasource"),
        })
    }

    /// Same as `load_url()`, with the following difference.
    ///
    /// By default we treat relative paths (in the connection string and
    /// datasource url value) as relative to the CWD. This does not work in all
    /// cases, so we need a way to prefix these relative paths with a
    /// config_dir.
    ///
    /// This is, at the time of this writing (2021-05-05), only used in the
    /// context of Node-API integration.
    ///
    /// P.S. Don't forget to add new parameters here if needed!
    pub fn load_url_with_config_dir<F>(
        &self,
        _config_dir: &Path,
        env: F,
    ) -> Result<String, Diagnostics>
    where
        F: Fn(&str) -> Option<String>,
    {
        let url = self.load_url(env)?;

        Ok(url)
    }

    // Validation for property existence
    pub fn provider_defined(&self) -> bool {
        !self.provider.is_empty()
    }

    pub fn url_defined(&self) -> bool {
        self.url_span.end > self.url_span.start
    }
}

pub(crate) fn from_url<F>(url: &StringFromEnvVar, env: F) -> Result<String, UrlValidationError>
where
    F: Fn(&str) -> Option<String>,
{
    let url = match (&url.value, &url.from_env_var) {
        (Some(lit), _) if lit.trim().is_empty() => {
            return Err(UrlValidationError::EmptyUrlValue);
        }
        (Some(lit), _) => lit.clone(),
        (None, Some(env_var)) => match env(env_var) {
            Some(var) if var.trim().is_empty() => {
                return Err(UrlValidationError::EmptyEnvValue(env_var.clone()));
            }
            Some(var) => var,
            None => return Err(UrlValidationError::NoEnvValue(env_var.clone())),
        },
        (None, None) => return Err(UrlValidationError::NoUrlOrEnv),
    };

    Ok(url)
}
