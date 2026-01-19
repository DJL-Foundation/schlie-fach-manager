/// Export functionality for data export to various formats
/// Per v2.1 spec section 10

pub mod toml;
pub mod json;
pub mod csv;
pub mod markdown;

use color_eyre::eyre::Result;
use crate::db::Database;

/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Toml,
    Json,
    Csv,
    Markdown,
}

impl ExportFormat {
    pub fn name(&self) -> &str {
        match self {
            ExportFormat::Toml => "TOML",
            ExportFormat::Json => "JSON",
            ExportFormat::Csv => "CSV",
            ExportFormat::Markdown => "Markdown",
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            ExportFormat::Toml => "toml",
            ExportFormat::Json => "json",
            ExportFormat::Csv => "csv",
            ExportFormat::Markdown => "md",
        }
    }
}

/// Export data to a file in the specified format
pub fn export_data(
    db: &Database,
    format: ExportFormat,
    path: &str,
) -> Result<()> {
    match format {
        ExportFormat::Toml => toml::export(db, path),
        ExportFormat::Json => json::export(db, path),
        ExportFormat::Csv => csv::export(db, path),
        ExportFormat::Markdown => markdown::export(db, path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_names() {
        assert_eq!(ExportFormat::Toml.name(), "TOML");
        assert_eq!(ExportFormat::Json.name(), "JSON");
        assert_eq!(ExportFormat::Csv.name(), "CSV");
        assert_eq!(ExportFormat::Markdown.name(), "Markdown");
    }

    #[test]
    fn test_format_extensions() {
        assert_eq!(ExportFormat::Toml.extension(), "toml");
        assert_eq!(ExportFormat::Json.extension(), "json");
        assert_eq!(ExportFormat::Csv.extension(), "csv");
        assert_eq!(ExportFormat::Markdown.extension(), "md");
    }
}
