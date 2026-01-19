//! CSV import functionality.
//!
//! This module provides functions to import locker management data from CSV format.
//! CSV is useful for importing data from spreadsheets or other tabular data sources.
//!
//! # CSV Format Requirements
//!
//! ## Lockers CSV
//!
//! Expected columns:
//! - `Label` or `Nummer` - Locker identifier (required)
//! - `Standort` or `Location` - Location name (required)
//! - `Hoehe_cm` or `Height` - Height in centimeters (optional, defaults to 100)
//! - `Defekt` or `Damaged` - Damage status ("Ja"/"Nein" or "true"/"false", optional)
//!
//! ## Rentals CSV
//!
//! Expected columns:
//! - `Schliessfach` or `Locker` - Locker label (required)
//! - `Mieter` or `Tenant` - Tenant username (required)
//! - `Typ` or `Type` - Tenant type ("Schüler"/"Lehrer", optional)
//! - `Beginn` or `Start` - Start date YYYY-MM-DD (required)
//! - `Ende` or `End` - End date YYYY-MM-DD (required)
//!
//! # Example
//!
//! ```rust,no_run
//! use schliessfach_manager::import::csv::{import_lockers_csv, import_rentals_csv};
//! use schliessfach_manager::db::Database;
//! use std::path::Path;
//!
//! let db = Database::open(Path::new("data.db")).unwrap();
//!
//! let lockers_imported = import_lockers_csv(&db.conn, Path::new("lockers.csv")).unwrap();
//! println!("Imported {} lockers", lockers_imported);
//!
//! let rentals_imported = import_rentals_csv(&db.conn, Path::new("rentals.csv")).unwrap();
//! println!("Imported {} rentals", rentals_imported);
//! ```

use chrono::NaiveDate;
use color_eyre::eyre::{Result, WrapErr};
use rusqlite::Connection;
use std::path::Path;

/// Imports lockers from a CSV file into the database.
///
/// The CSV file should have a header row with column names. Supported column names
/// are flexible to accommodate both German and English naming conventions.
///
/// # Arguments
///
/// * `conn` - A reference to an open SQLite connection
/// * `path` - Path to the CSV file
///
/// # Returns
///
/// Returns the number of lockers successfully imported.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The CSV is malformed
/// - Required columns are missing
/// - Database operations fail
///
/// # Example
///
/// ```rust,no_run
/// use schliessfach_manager::import::csv::import_lockers_csv;
/// use schliessfach_manager::db::Database;
/// use std::path::Path;
///
/// let db = Database::open(Path::new("data.db")).unwrap();
/// let count = import_lockers_csv(&db.conn, Path::new("lockers.csv")).unwrap();
/// println!("Imported {} lockers", count);
/// ```
pub fn import_lockers_csv(conn: &Connection, path: &Path) -> Result<usize> {
    let mut reader = csv::Reader::from_path(path)
        .wrap_err_with(|| format!("Konnte CSV-Datei nicht öffnen: {}", path.display()))?;

    let headers = reader.headers()?.clone();

    // Find column indices with flexible naming
    let label_idx = find_column_index(&headers, &["Label", "Nummer", "Number", "ID"]);
    let location_idx = find_column_index(&headers, &["Standort", "Location", "Ort"]);
    let height_idx = find_column_index(&headers, &["Hoehe_cm", "Höhe_cm", "Height", "Höhe"]);
    let damaged_idx = find_column_index(&headers, &["Defekt", "Damaged", "Beschädigt"]);

    let label_idx = label_idx
        .ok_or_else(|| color_eyre::eyre::eyre!("Spalte 'Label' oder 'Nummer' nicht gefunden"))?;
    let location_idx = location_idx.ok_or_else(|| {
        color_eyre::eyre::eyre!("Spalte 'Standort' oder 'Location' nicht gefunden")
    })?;

    let tx = conn.unchecked_transaction()?;
    let mut count = 0;

    for result in reader.records() {
        let record = result?;

        let label = record.get(label_idx).unwrap_or("").trim();
        let location = record.get(location_idx).unwrap_or("").trim();

        if label.is_empty() || location.is_empty() {
            continue;
        }

        let height: i32 = height_idx
            .and_then(|idx| record.get(idx))
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(100);

        let is_damaged = damaged_idx
            .and_then(|idx| record.get(idx))
            .map(|s| {
                let s = s.trim().to_lowercase();
                s == "ja" || s == "yes" || s == "true" || s == "1"
            })
            .unwrap_or(false);

        let result = tx.execute(
            "INSERT OR IGNORE INTO lockers (label, location, height, is_damaged, created_at) 
             VALUES (?1, ?2, ?3, ?4, datetime('now'))",
            rusqlite::params![label, location, height, is_damaged],
        );

        if let Ok(rows) = result {
            if rows > 0 {
                count += 1;
            }
        }
    }

    tx.commit()?;
    Ok(count)
}

/// Imports rentals from a CSV file into the database.
///
/// The CSV file should have a header row with column names. The locker must already
/// exist in the database (referenced by label).
///
/// # Arguments
///
/// * `conn` - A reference to an open SQLite connection
/// * `path` - Path to the CSV file
///
/// # Returns
///
/// Returns the number of rentals successfully imported.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The CSV is malformed
/// - Required columns are missing
/// - Database operations fail
///
/// # Example
///
/// ```rust,no_run
/// use schliessfach_manager::import::csv::import_rentals_csv;
/// use schliessfach_manager::db::Database;
/// use std::path::Path;
///
/// let db = Database::open(Path::new("data.db")).unwrap();
/// let count = import_rentals_csv(&db.conn, Path::new("rentals.csv")).unwrap();
/// println!("Imported {} rentals", count);
/// ```
pub fn import_rentals_csv(conn: &Connection, path: &Path) -> Result<usize> {
    let mut reader = csv::Reader::from_path(path)
        .wrap_err_with(|| format!("Konnte CSV-Datei nicht öffnen: {}", path.display()))?;

    let headers = reader.headers()?.clone();

    // Find column indices with flexible naming
    let locker_idx = find_column_index(
        &headers,
        &["Schliessfach", "Schließfach", "Locker", "Label"],
    );
    let tenant_idx = find_column_index(&headers, &["Mieter", "Tenant", "Username", "Benutzer"]);
    let type_idx = find_column_index(&headers, &["Typ", "Type", "Mietertyp"]);
    let start_idx = find_column_index(&headers, &["Beginn", "Start", "Start_Date", "Startdatum"]);
    let end_idx = find_column_index(&headers, &["Ende", "End", "End_Date", "Enddatum"]);

    let locker_idx = locker_idx.ok_or_else(|| {
        color_eyre::eyre::eyre!("Spalte 'Schliessfach' oder 'Locker' nicht gefunden")
    })?;
    let tenant_idx = tenant_idx
        .ok_or_else(|| color_eyre::eyre::eyre!("Spalte 'Mieter' oder 'Tenant' nicht gefunden"))?;
    let start_idx = start_idx
        .ok_or_else(|| color_eyre::eyre::eyre!("Spalte 'Beginn' oder 'Start' nicht gefunden"))?;
    let end_idx = end_idx
        .ok_or_else(|| color_eyre::eyre::eyre!("Spalte 'Ende' oder 'End' nicht gefunden"))?;

    let tx = conn.unchecked_transaction()?;
    let mut count = 0;

    for result in reader.records() {
        let record = result?;

        let locker_label = record.get(locker_idx).unwrap_or("").trim();
        let tenant = record.get(tenant_idx).unwrap_or("").trim();
        let start_date_str = record.get(start_idx).unwrap_or("").trim();
        let end_date_str = record.get(end_idx).unwrap_or("").trim();

        if locker_label.is_empty() || tenant.is_empty() {
            continue;
        }

        // Parse dates
        let start_date = match NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };

        let end_date = match NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };

        // Get locker ID
        let locker_id: Option<i64> = tx
            .query_row(
                "SELECT id FROM lockers WHERE label = ?1",
                [locker_label],
                |row| row.get(0),
            )
            .ok();

        let locker_id = match locker_id {
            Some(id) => id,
            None => continue, // Skip if locker doesn't exist
        };

        // Parse tenant type
        let tenant_type = type_idx
            .and_then(|idx| record.get(idx))
            .map(|s| {
                let s = s.trim().to_lowercase();
                if s.contains("lehrer") || s.contains("teacher") {
                    "Lehrer"
                } else {
                    "Schüler"
                }
            })
            .unwrap_or("Schüler");

        let result = tx.execute(
            "INSERT OR IGNORE INTO rentals 
             (locker_id, tenant_username, tenant_type, rental_start_date, rental_end_date, 
              deposit_paid, deposit_returned, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 0, 0, datetime('now'))",
            rusqlite::params![
                locker_id,
                tenant,
                tenant_type,
                start_date.to_string(),
                end_date.to_string()
            ],
        );

        if let Ok(rows) = result {
            if rows > 0 {
                count += 1;
            }
        }
    }

    tx.commit()?;
    Ok(count)
}

/// Finds a column index by trying multiple possible column names.
fn find_column_index(headers: &csv::StringRecord, names: &[&str]) -> Option<usize> {
    for name in names {
        if let Some(idx) = headers
            .iter()
            .position(|h| h.trim().eq_ignore_ascii_case(name))
        {
            return Some(idx);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_import_lockers_csv() -> Result<()> {
        let db = Database::open_in_memory()?;

        // Create CSV content
        let csv_content = r#"Label,Standort,Hoehe_cm,Defekt
A-001,Hauptgebäude,150,Nein
A-002,Hauptgebäude,120,Ja
B-001,Turnhalle,100,Nein
"#;

        let mut file = NamedTempFile::new()?;
        file.write_all(csv_content.as_bytes())?;
        file.flush()?;

        let count = import_lockers_csv(&db.conn, file.path())?;
        assert_eq!(count, 3);

        // Verify data
        let locker_count: i32 = db
            .conn
            .query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;
        assert_eq!(locker_count, 3);

        let damaged_count: i32 = db.conn.query_row(
            "SELECT COUNT(*) FROM lockers WHERE is_damaged = 1",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(damaged_count, 1);

        Ok(())
    }

    #[test]
    fn test_import_lockers_csv_english_columns() -> Result<()> {
        let db = Database::open_in_memory()?;

        let csv_content = r#"Number,Location,Height,Damaged
C-001,Building A,180,true
C-002,Building B,160,false
"#;

        let mut file = NamedTempFile::new()?;
        file.write_all(csv_content.as_bytes())?;
        file.flush()?;

        let count = import_lockers_csv(&db.conn, file.path())?;
        assert_eq!(count, 2);

        Ok(())
    }

    #[test]
    fn test_import_rentals_csv() -> Result<()> {
        let db = Database::open_in_memory()?;

        // First, create lockers
        db.conn.execute(
            "INSERT INTO lockers (label, location, height, is_damaged, created_at) 
             VALUES ('A-001', 'Test', 100, 0, datetime('now'))",
            [],
        )?;
        db.conn.execute(
            "INSERT INTO lockers (label, location, height, is_damaged, created_at) 
             VALUES ('A-002', 'Test', 100, 0, datetime('now'))",
            [],
        )?;

        // Create CSV content
        let csv_content = r#"Schliessfach,Mieter,Typ,Beginn,Ende
A-001,max.mustermann,Schüler,2024-01-01,2024-12-31
A-002,anna.schmidt,Lehrer,2024-02-01,2025-01-31
"#;

        let mut file = NamedTempFile::new()?;
        file.write_all(csv_content.as_bytes())?;
        file.flush()?;

        let count = import_rentals_csv(&db.conn, file.path())?;
        assert_eq!(count, 2);

        // Verify data
        let rental_count: i32 = db
            .conn
            .query_row("SELECT COUNT(*) FROM rentals", [], |row| row.get(0))?;
        assert_eq!(rental_count, 2);

        Ok(())
    }

    #[test]
    fn test_import_rentals_csv_skips_missing_locker() -> Result<()> {
        let db = Database::open_in_memory()?;

        // Create only one locker
        db.conn.execute(
            "INSERT INTO lockers (label, location, height, is_damaged, created_at) 
             VALUES ('A-001', 'Test', 100, 0, datetime('now'))",
            [],
        )?;

        // CSV references non-existent locker
        let csv_content = r#"Locker,Tenant,Type,Start,End
A-001,user1,Student,2024-01-01,2024-12-31
X-999,user2,Student,2024-01-01,2024-12-31
"#;

        let mut file = NamedTempFile::new()?;
        file.write_all(csv_content.as_bytes())?;
        file.flush()?;

        let count = import_rentals_csv(&db.conn, file.path())?;
        assert_eq!(count, 1); // Only one should be imported

        Ok(())
    }

    #[test]
    fn test_find_column_index() {
        let headers = csv::StringRecord::from(vec!["Label", "Location", "Height"]);

        assert_eq!(find_column_index(&headers, &["Label", "Nummer"]), Some(0));
        assert_eq!(
            find_column_index(&headers, &["Standort", "Location"]),
            Some(1)
        );
        assert_eq!(find_column_index(&headers, &["Unknown"]), None);
    }
}
