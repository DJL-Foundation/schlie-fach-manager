pub mod csv;
pub mod json;
pub mod markdown;
pub mod toml;

use crate::{
    config::AppSettings,
    db,
    model::{Location, Locker, Payment, Rental},
};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Supported export formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Toml,
    Json,
    Csv,
    Markdown,
}

impl ExportFormat {
    /// Parses a string into an export format, defaulting to TOML.
    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "json" => ExportFormat::Json,
            "csv" => ExportFormat::Csv,
            "markdown" | "md" => ExportFormat::Markdown,
            _ => ExportFormat::Toml,
        }
    }

    /// Returns the file extension associated with the format.
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Toml => "toml",
            ExportFormat::Json => "json",
            ExportFormat::Csv => "csv",
            ExportFormat::Markdown => "md",
        }
    }
}

/// Metadata bundled with export files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub version: String,
    pub export_date: String,
    pub database_version: i32,
}

/// Complete export payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub metadata: ExportMetadata,
    pub lockers: Vec<Locker>,
    pub rentals: Vec<Rental>,
    pub payments: Vec<Payment>,
    pub locations: Vec<Location>,
}

/// Exports all database data into the requested format.
pub fn export_all(
    db: &db::connection::Database,
    settings: &AppSettings,
    format: ExportFormat,
) -> Result<PathBuf> {
    let data = collect_export_data(db)?;
    let dir = settings.export_directory.clone();
    fs::create_dir_all(&dir)?;
    let timestamp = Utc::now().format("%Y%m%d%H%M%S");

    match format {
        ExportFormat::Toml => {
            let path = dir.join(format!("export-{}.toml", timestamp));
            toml::export(&path, &data)?;
            Ok(path)
        }
        ExportFormat::Json => {
            let path = dir.join(format!("export-{}.json", timestamp));
            json::export(&path, &data)?;
            Ok(path)
        }
        ExportFormat::Markdown => {
            let path = dir.join(format!("report-{}.md", timestamp));
            markdown::export(&path, &data)?;
            Ok(path)
        }
        ExportFormat::Csv => {
            let path = dir.join(format!("csv-export-{}", timestamp));
            fs::create_dir_all(&path)?;
            csv::export(&path, &data)?;
            Ok(path)
        }
    }
}

/// Collects data from the database into an export payload.
fn collect_export_data(db: &db::connection::Database) -> Result<ExportData> {
    let lockers = db::lockers::list_lockers(db.connection())?;
    let rentals = db::rentals::list_rentals(db.connection())?;
    let payments = db::payments::list_payments(db.connection())?;
    let locations = db::locations::list_locations(db.connection())?;
    Ok(ExportData {
        metadata: ExportMetadata {
            version: "2.1.0".to_string(),
            export_date: Utc::now().to_rfc3339(),
            database_version: 3,
        },
        lockers,
        rentals,
        payments,
        locations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db, import,
        model::{Location, Locker, Payment, PaymentType, Rental},
    };
    use chrono::NaiveDate;
    use tempfile::TempDir;

    fn seed_database(db: &db::connection::Database) -> Result<()> {
        let locker = Locker::new(1, "A-01", "Hauptgebäude", "Klein");
        db::lockers::upsert_locker(db.connection(), &locker)?;
        let location = Location::new(1, "Hauptgebäude");
        db::locations::upsert_location(db.connection(), &location)?;
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let mut rental = Rental::new(1, locker.id, "Max", start, end);
        rental.deposit_paid = true;
        db::rentals::upsert_rental(db.connection(), &rental)?;
        let payment = Payment::new(1, rental.id, 1000, start, PaymentType::Deposit);
        db::payments::upsert_payment(db.connection(), &payment)?;
        Ok(())
    }

    #[test]
    fn toml_round_trip_exports_and_imports() -> Result<()> {
        let db = db::connection::Database::open_in_memory()?;
        seed_database(&db)?;
        let temp_dir = TempDir::new()?;
        let settings = AppSettings {
            export_directory: temp_dir.path().to_path_buf(),
            ..AppSettings::default()
        };

        let path = export_all(&db, &settings, ExportFormat::Toml)?;
        let new_db = db::connection::Database::open_in_memory()?;
        import::import_all(&new_db, ExportFormat::Toml, &path)?;
        assert_eq!(db::lockers::list_lockers(new_db.connection())?.len(), 1);
        assert_eq!(db::rentals::list_rentals(new_db.connection())?.len(), 1);
        Ok(())
    }

    #[test]
    fn csv_round_trip_exports_and_imports() -> Result<()> {
        let db = db::connection::Database::open_in_memory()?;
        seed_database(&db)?;
        let temp_dir = TempDir::new()?;
        let settings = AppSettings {
            export_directory: temp_dir.path().to_path_buf(),
            ..AppSettings::default()
        };

        let path = export_all(&db, &settings, ExportFormat::Csv)?;
        let new_db = db::connection::Database::open_in_memory()?;
        import::import_all(&new_db, ExportFormat::Csv, &path)?;
        assert_eq!(db::lockers::list_lockers(new_db.connection())?.len(), 1);
        assert_eq!(db::payments::list_payments(new_db.connection())?.len(), 1);
        Ok(())
    }
}
