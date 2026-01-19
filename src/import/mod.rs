pub mod csv;
pub mod json;
pub mod toml;

use crate::{db, export::ExportData, export::ExportFormat};
use anyhow::Result;
use std::path::Path;

/// Imports data from the specified format.
pub fn import_all(db: &db::connection::Database, format: ExportFormat, path: &Path) -> Result<()> {
    let data = match format {
        ExportFormat::Toml => toml::import(path)?,
        ExportFormat::Json => json::import(path)?,
        ExportFormat::Csv => csv::import(path)?,
        ExportFormat::Markdown => {
            return Ok(());
        }
    };
    apply_data(db, &data)
}

/// Applies imported data to the database.
fn apply_data(db: &db::connection::Database, data: &ExportData) -> Result<()> {
    for locker in &data.lockers {
        db::lockers::upsert_locker(db.connection(), locker)?;
    }
    for location in &data.locations {
        db::locations::upsert_location(db.connection(), location)?;
    }
    for rental in &data.rentals {
        db::rentals::upsert_rental(db.connection(), rental)?;
    }
    for payment in &data.payments {
        db::payments::upsert_payment(db.connection(), payment)?;
    }
    Ok(())
}
