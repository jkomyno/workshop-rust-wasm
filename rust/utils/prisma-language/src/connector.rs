const MYSQL: &str = "mysql";
const POSTGRES: &str = "postgres";

pub enum Connector {
    PostgresConnector,
    MySQLConnector,
}

impl Connector {
    pub fn new(provider: &str) -> Option<Connector> {
        match provider {
            POSTGRES => Some(Connector::PostgresConnector),
            MYSQL => Some(Connector::MySQLConnector),
            _ => None,
        }
    }

    pub fn validate_url(&self, url: &str) -> Result<(), String> {
        let error_prefix: Option<String> = match self {
            Connector::PostgresConnector => {
                let expected_prefix = "postgres://";
                if !url.starts_with(&expected_prefix) {
                    Some(expected_prefix.to_owned())
                } else {
                    None
                }
            }
            Connector::MySQLConnector => {
                let expected_prefix = "mysql://";
                if !url.starts_with(&expected_prefix) {
                    Some(expected_prefix.to_owned())
                } else {
                    None
                }
            }
        };

        if let Some(error_prefix) = error_prefix {
            Err(format!(
                "The URL must start with the `{}` protocol.",
                error_prefix
            ))
        } else {
            Ok(())
        }
    }
}
