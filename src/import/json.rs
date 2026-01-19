//! JSON import functionality.
//!
//! This module provides functions to import locker management data from JSON format.
//! JSON is the primary format for full backup restoration, supporting all data types.
//!
//! # Example
//!
//! ```rust,no_run
//! use schliessfach_manager::import::json::import_full_backup_json;
//! use schliessfach_manager::db::Database;
//! use std::path::Path;
//!
//! let db = Database::open(Path::new("data.db")).unwrap();
//! let stats = import_full_backup_json(&db.conn, Path::new("backup.json")).unwrap();
//! println!("Imported {} lockers, {} rentals", stats.lockers_imported, stats.rentals_imported);
//! ```

use crate::models::stats::FullBackup;
use color_eyre::eyre::{Result, WrapErr};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Statistics about an import operation.
///
/// This struct provides counts of successfully imported records for each entity type,
/// as well as any records that were skipped due to conflicts or errors.
///
/// # Example
///
/// ```rust
/// use schliessfach_manager::import::ImportStats;
///
/// let stats = ImportStats::default();
/// assert_eq!(stats.lockers_imported, 0);
/// assert_eq!(stats.total_imported(), 0);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportStats {
    /// Number of lockers successfully imported.
    pub lockers_imported: usize,
    /// Number of lockers skipped (e.g., due to duplicate labels).
    pub lockers_skipped: usize,
    /// Number of rentals successfully imported.
    pub rentals_imported: usize,
    /// Number of rentals skipped (e.g., due to missing locker references).
    pub rentals_skipped: usize,
    /// Number of payments successfully imported.
    pub payments_imported: usize,
    /// Number of payments skipped.
    pub payments_skipped: usize,
    /// Number of locations successfully imported.
    pub locations_imported: usize,
    /// Number of locations skipped (e.g., due to duplicate names).
    pub locations_skipped: usize,
}

impl ImportStats {
    /// Returns the total number of records successfully imported.
    pub fn total_imported(&self) -> usize {
        self.lockers_imported
            + self.rentals_imported
            + self.payments_imported
            + self.locations_imported
    }

    /// Returns the total number of records skipped.
    pub fn total_skipped(&self) -> usize {
        self.lockers_skipped
            + self.rentals_skipped
            + self.payments_skipped
            + self.locations_skipped
    }

    /// Returns true if any records were skipped during import.
    pub fn has_skipped(&self) -> bool {
        self.total_skipped() > 0
    }

    /// Formats the import statistics as a human-readable summary in German.
    pub fn format_summary(&self) -> String {
        let mut parts = Vec::new();

        if self.lockers_imported > 0 {
            parts.push(format!("{} Schließfächer", self.lockers_imported));
        }
        if self.rentals_imported > 0 {
            parts.push(format!("{} Vermietungen", self.rentals_imported));
        }
        if self.payments_imported > 0 {
            parts.push(format!("{} Zahlungen", self.payments_imported));
        }
        if self.locations_imported > 0 {
            parts.push(format!("{} Standorte", self.locations_imported));
        }

        if parts.is_empty() {
            "Keine Daten importiert".to_string()
        } else {
            format!("Importiert: {}", parts.join(", "))
        }
    }
}

/// Imports a full backup from a JSON file into the database.
///
/// This function reads a JSON backup file and imports all data (lockers, rentals,
/// payments, and locations) into the provided database connection. The import
/// uses a transaction to ensure atomicity.
///
/// # Arguments
///
/// * `conn` - A reference to an open SQLite connection
/// * `path` - Path to the JSON backup file
///
/// # Returns
///
/// Returns `ImportStats` containing counts of imported and skipped records.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The JSON is malformed or doesn't match the expected schema
/// - Database operations fail
///
/// # Example
///
/// ```rust,no_run
/// use schliessfach_manager::import::json::import_full_backup_json;
/// use schliessfach_manager::db::Database;
/// use std::path::Path;
///
/// let db = Database::open(Path::new("data.db")).unwrap();
/// let stats = import_full_backup_json(&db.conn, Path::new("backup.json")).unwrap();
///
/// if stats.has_skipped() {
///     println!("Warning: {} records skipped", stats.total_skipped());
/// }
/// println!("{}", stats.format_summary());
/// ```
pub fn import_full_backup_json(conn: &Connection, path: &Path) -> Result<ImportStats> {
    let json_content = std::fs::read_to_string(path)
        .wrap_err_with(|| format!("Konnte Datei nicht lesen: {}", path.display()))?;

    let backup: FullBackup = serde_json::from_str(&json_content)
        .wrap_err("JSON-Datei konnte nicht geparst werden")?;

    import_backup_data(conn, &backup)
}

/// Imports backup data into the database.
///
/// Internal function that handles the actual import logic for any backup source.
pub(crate) fn import_backup_data(conn: &Connection, backup: &FullBackup) -> Result<ImportStats> {
    let mut stats = ImportStats::default();

    // Use a transaction for atomicity
    let tx = conn.unchecked_transaction()?;

    // Import locations first (they may be referenced by lockers)
    for location in &backup.locations {
        match tx.execute(
            "INSERT OR IGNORE INTO locations (name, description, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                location.name,
                location.description,
                location.created_at.to_rfc3339()
            ],
        ) {
            Ok(rows) if rows > 0 => stats.locations_imported += 1,
            Ok(_) => stats.locations_skipped += 1,
            Err(_) => stats.locations_skipped += 1,
        }
    }

    // Build a mapping of old locker IDs to new IDs
    let mut locker_id_map: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    // Import lockers
    for locker in &backup.lockers {
        match tx.execute(
            "INSERT OR IGNORE INTO lockers (label, location, height, is_damaged, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                locker.label,
                locker.location,
                locker.height,
                locker.is_damaged,
                locker.created_at.to_rfc3339()
            ],
        ) {
            Ok(rows) if rows > 0 => {
                let new_id = tx.last_insert_rowid();
                locker_id_map.insert(locker.id, new_id);
                stats.lockers_imported += 1;
            }
            Ok(_) => {
                // Locker already exists, try to get its ID by label
                if let Ok(existing_id) = tx.query_row(
                    "SELECT id FROM lockers WHERE label = ?1",
                    [&locker.label],
                    |row| row.get::<_, i64>(0),
                ) {
                    locker_id_map.insert(locker.id, existing_id);
                }
                stats.lockers_skipped += 1;
            }
            Err(_) => stats.lockers_skipped += 1,
        }
    }

    // Build a mapping of old rental IDs to new IDs
    let mut rental_id_map: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    // Import rentals (using the locker ID mapping)
    for rental in &backup.rentals {
        let new_locker_id = locker_id_map.get(&rental.locker_id);

        if let Some(&locker_id) = new_locker_id {
            match tx.execute(
                "INSERT OR IGNORE INTO rentals 
                 (locker_id, tenant_username, tenant_type, rental_start_date, rental_end_date, 
                  deposit_paid, deposit_returned, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![
                    locker_id,
                    rental.tenant_username,
                    rental.tenant_type.as_str(),
                    rental.rental_start_date.to_string(),
                    rental.rental_end_date.to_string(),
                    rental.deposit_paid,
                    rental.deposit_returned,
                    rental.created_at.to_rfc3339()
                ],
            ) {
                Ok(rows) if rows > 0 => {
                    let new_id = tx.last_insert_rowid();
                    rental_id_map.insert(rental.id, new_id);
                    stats.rentals_imported += 1;
                }
                Ok(_) => stats.rentals_skipped += 1,
                Err(_) => stats.rentals_skipped += 1,
            }
        } else {
            stats.rentals_skipped += 1;
        }
    }

    // Import payments (using the rental ID mapping)
    for payment in &backup.payments {
        let new_rental_id = rental_id_map.get(&payment.rental_id);

        if let Some(&rental_id) = new_rental_id {
            match tx.execute(
                "INSERT OR IGNORE INTO payments 
                 (rental_id, amount_cents, payment_type, payment_date, notes)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    rental_id,
                    payment.amount_cents,
                    payment.payment_type.as_str(),
                    payment.payment_date.to_string(),
                    payment.notes,
                ],
            ) {
                Ok(rows) if rows > 0 => stats.payments_imported += 1,
                Ok(_) => stats.payments_skipped += 1,
                Err(_) => stats.payments_skipped += 1,
            }
        } else {
            stats.payments_skipped += 1;
        }
    }

    tx.commit()?;

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::models::{Locker, Payment, PaymentType, Rental, TenantType};
    use crate::models::stats::Location;
    use chrono::{Duration, Utc};
    use tempfile::NamedTempFile;

    fn create_test_backup() -> FullBackup {
        let now = Utc::now();
        let today = now.date_naive();

        FullBackup {
            version: "2.1".to_string(),
            exported_at: now,
            lockers: vec![
                Locker {
                    id: 1,
                    label: "A-001".to_string(),
                    location: "Hauptgebäude".to_string(),
                    height: 150,
                    is_damaged: false,
                    created_at: now,
                },
                Locker {
                    id: 2,
                    label: "A-002".to_string(),
                    location: "Hauptgebäude".to_string(),
                    height: 120,
                    is_damaged: true,
                    created_at: now,
                },
            ],
            rentals: vec![Rental {
                id: 1,
                locker_id: 1,
                tenant_username: "max.mustermann".to_string(),
                tenant_type: TenantType::Schüler,
                rental_start_date: today,
                rental_end_date: today + Duration::days(365),
                deposit_paid: true,
                deposit_returned: false,
                created_at: now,
                returned_at: None,
            }],
            payments: vec![Payment {
                id: 1,
                rental_id: 1,
                amount_cents: 1000,
                payment_type: PaymentType::Deposit,
                payment_date: today,
                notes: None,
            }],
            locations: vec![Location {
                id: 1,
                name: "Hauptgebäude".to_string(),
                description: Some("Main building".to_string()),
                created_at: now,
            }],
        }
    }

    #[test]
    fn test_import_stats_default() {
        let stats = ImportStats::default();
        assert_eq!(stats.total_imported(), 0);
        assert_eq!(stats.total_skipped(), 0);
        assert!(!stats.has_skipped());
    }

    #[test]
    fn test_import_stats_format_summary() {
        let stats = ImportStats {
            lockers_imported: 10,
            rentals_imported: 5,
            payments_imported: 3,
            locations_imported: 2,
            ..Default::default()
        };

        let summary = stats.format_summary();
        assert!(summary.contains("10 Schließfächer"));
        assert!(summary.contains("5 Vermietungen"));
        assert!(summary.contains("3 Zahlungen"));
        assert!(summary.contains("2 Standorte"));
    }

    #[test]
    fn test_import_full_backup_json() -> Result<()> {
        // Create test backup
        let backup = create_test_backup();
        let json = serde_json::to_string_pretty(&backup)?;

        // Write to temp file
        let file = NamedTempFile::new()?;
        std::fs::write(file.path(), &json)?;

        // Create test database
        let db = Database::open_in_memory()?;

        // Import
        let stats = import_full_backup_json(&db.conn, file.path())?;

        assert_eq!(stats.lockers_imported, 2);
        assert_eq!(stats.rentals_imported, 1);
        assert_eq!(stats.payments_imported, 1);
        assert_eq!(stats.locations_imported, 1);

        Ok(())
    }

    #[test]
    fn test_import_handles_duplicates() -> Result<()> {
        // Create test backup
        let backup = create_test_backup();
        let json = serde_json::to_string_pretty(&backup)?;

        // Write to temp file
        let file = NamedTempFile::new()?;
        std::fs::write(file.path(), &json)?;

        // Create test database
        let db = Database::open_in_memory()?;

        // Import twice
        let stats1 = import_full_backup_json(&db.conn, file.path())?;
        let stats2 = import_full_backup_json(&db.conn, file.path())?;

        // First import should succeed
        assert_eq!(stats1.lockers_imported, 2);

        // Second import should skip duplicates
        assert_eq!(stats2.lockers_skipped, 2);

        Ok(())
    }
}
