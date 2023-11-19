use std::str::FromStr;

// use wasm_bindgen::prelude::wasm_bindgen;

/// Models a closed set of database providers as a C-style enum.
#[derive(PartialEq, Debug)]
pub enum DbProvider {
    Postgres,
    MySQL,
    SQLite,
}

/// Given a `DbProvider` C-style enum instance, return its human-friendly label.
pub fn enum_to_string(provider: DbProvider) -> String {
    match provider {
        DbProvider::Postgres => "postgres",
        DbProvider::MySQL => "mysql",
        DbProvider::SQLite => "sqlite",
    }
    .to_string()
}

/// Given a `DbProvider` enum label, construct its relative `Provider` C-style enum instance.
pub fn enum_from_string(label: String) -> Result<DbProvider, String> {
    DbProvider::from_str(&label)
}

impl FromStr for DbProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "postgres" => Ok(DbProvider::Postgres),
            "mysql" => Ok(DbProvider::MySQL),
            "sqlite" => Ok(DbProvider::SQLite),
            _ => Err(format!("Unknown database provider: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_to_string_example() {
        let result = enum_to_string(DbProvider::Postgres);
        assert_eq!(result, "postgres");

        let result = enum_to_string(DbProvider::MySQL);
        assert_eq!(result, "mysql");

        let result = enum_to_string(DbProvider::SQLite);
        assert_eq!(result, "sqlite");
    }

    #[test]
    fn enum_from_string_example() {
        let result = enum_from_string("postgres".to_owned());
        assert_eq!(result.unwrap(), DbProvider::Postgres);

        let result = enum_from_string("mysql".to_owned());
        assert_eq!(result.unwrap(), DbProvider::MySQL);

        let result = enum_from_string("sqlite".to_owned());
        assert_eq!(result.unwrap(), DbProvider::SQLite);

        let result = enum_from_string("lmao".to_owned());
        assert_eq!(
            result.unwrap_err(),
            "Unknown database provider: lmao".to_owned()
        );
    }
}
