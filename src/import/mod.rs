/// Import functionality for data import from various formats
/// Per v2.1 spec section 11

pub mod toml;
pub mod json;
pub mod csv;

use color_eyre::eyre::Result;
use crate::db::Database;

/// Supported import formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    Toml,
    Json,
    Csv,
}

impl ImportFormat {
    pub fn name(&self) -> &str {
        match self {
            ImportFormat::Toml => "TOML",
            ImportFormat::Json => "JSON",
            ImportFormat::Csv => "CSV",
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            ImportFormat::Toml => "toml",
            ImportFormat::Json => "json",
            ImportFormat::Csv => "csv",
        }
    }
}

/// Import data from a file in the specified format
pub fn import_data(
    db: &Database,
    format: ImportFormat,
    path: &str,
) -> Result<()> {
    match format {
        ImportFormat::Toml => toml::import(db, path),
        ImportFormat::Json => json::import(db, path),
        ImportFormat::Csv => csv::import(db, path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_names() {
        assert_eq!(ImportFormat::Toml.name(), "TOML");
        assert_eq!(ImportFormat::Json.name(), "JSON");
        assert_eq!(ImportFormat::Csv.name(), "CSV");
    }

    #[test]
    fn test_format_extensions() {
        assert_eq!(ImportFormat::Toml.extension(), "toml");
        assert_eq!(ImportFormat::Json.extension(), "json");
        assert_eq!(ImportFormat::Csv.extension(), "csv");
    }
}
