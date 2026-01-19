//! TOML import functionality.
//!
//! This module provides functions to import locker management data from TOML format.
//! TOML is a human-readable configuration format that is excellent for backups
//! that may need manual editing.
//!
//! # Example
//!
//! ```rust,no_run
//! use schliessfach_manager::import::toml::import_full_backup_toml;
//! use schliessfach_manager::db::Database;
//! use std::path::Path;
//!
//! let db = Database::open(Path::new("data.db")).unwrap();
//! let stats = import_full_backup_toml(&db.conn, Path::new("backup.toml")).unwrap();
//! println!("Imported {} lockers, {} rentals", stats.lockers_imported, stats.rentals_imported);
//! ```

use crate::import::json::{import_backup_data, ImportStats};
use crate::models::stats::FullBackup;
use color_eyre::eyre::{Result, WrapErr};
use rusqlite::Connection;
use std::path::Path;

/// Imports a full backup from a TOML file into the database.
///
/// This function reads a TOML backup file and imports all data (lockers, rentals,
/// payments, and locations) into the provided database connection. The import
/// uses a transaction to ensure atomicity.
///
/// # Arguments
///
/// * `conn` - A reference to an open SQLite connection
/// * `path` - Path to the TOML backup file
///
/// # Returns
///
/// Returns `ImportStats` containing counts of imported and skipped records.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The TOML is malformed or doesn't match the expected schema
/// - Database operations fail
///
/// # Example
///
/// ```rust,no_run
/// use schliessfach_manager::import::toml::import_full_backup_toml;
/// use schliessfach_manager::db::Database;
/// use std::path::Path;
///
/// let db = Database::open(Path::new("data.db")).unwrap();
/// let stats = import_full_backup_toml(&db.conn, Path::new("backup.toml")).unwrap();
///
/// if stats.has_skipped() {
///     println!("Warning: {} records skipped", stats.total_skipped());
/// }
/// println!("{}", stats.format_summary());
/// ```
pub fn import_full_backup_toml(conn: &Connection, path: &Path) -> Result<ImportStats> {
    let toml_content = std::fs::read_to_string(path)
        .wrap_err_with(|| format!("Konnte Datei nicht lesen: {}", path.display()))?;

    let backup: FullBackup = toml::from_str(&toml_content)
        .wrap_err("TOML-Datei konnte nicht geparst werden")?;

    import_backup_data(conn, &backup)
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
                    label: "T-001".to_string(),
                    location: "Testgebäude".to_string(),
                    height: 150,
                    is_damaged: false,
                    created_at: now,
                },
                Locker {
                    id: 2,
                    label: "T-002".to_string(),
                    location: "Testgebäude".to_string(),
                    height: 120,
                    is_damaged: true,
                    created_at: now,
                },
            ],
            rentals: vec![Rental {
                id: 1,
                locker_id: 1,
                tenant_username: "test.user".to_string(),
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
                name: "Testgebäude".to_string(),
                description: Some("Test building".to_string()),
                created_at: now,
            }],
        }
    }

    #[test]
    fn test_import_full_backup_toml() -> Result<()> {
        // Create test backup
        let backup = create_test_backup();
        let toml_content = toml::to_string_pretty(&backup)?;

        // Write to temp file
        let file = NamedTempFile::new()?;
        std::fs::write(file.path(), &toml_content)?;

        // Create test database
        let db = Database::open_in_memory()?;

        // Import
        let stats = import_full_backup_toml(&db.conn, file.path())?;

        assert_eq!(stats.lockers_imported, 2);
        assert_eq!(stats.rentals_imported, 1);
        assert_eq!(stats.payments_imported, 1);
        assert_eq!(stats.locations_imported, 1);

        Ok(())
    }

    #[test]
    fn test_import_toml_handles_duplicates() -> Result<()> {
        // Create test backup
        let backup = create_test_backup();
        let toml_content = toml::to_string_pretty(&backup)?;

        // Write to temp file
        let file = NamedTempFile::new()?;
        std::fs::write(file.path(), &toml_content)?;

        // Create test database
        let db = Database::open_in_memory()?;

        // Import twice
        let stats1 = import_full_backup_toml(&db.conn, file.path())?;
        let stats2 = import_full_backup_toml(&db.conn, file.path())?;

        // First import should succeed
        assert_eq!(stats1.lockers_imported, 2);

        // Second import should skip duplicates
        assert_eq!(stats2.lockers_skipped, 2);

        Ok(())
    }

    #[test]
    fn test_toml_roundtrip() -> Result<()> {
        use crate::export::toml::export_full_backup_toml;

        // Create and export
        let original_backup = create_test_backup();
        let export_file = NamedTempFile::new()?;
        export_full_backup_toml(
            &original_backup.lockers,
            &original_backup.rentals,
            &original_backup.payments,
            &original_backup.locations,
            export_file.path(),
        )?;

        // Import into database
        let db = Database::open_in_memory()?;
        let stats = import_full_backup_toml(&db.conn, export_file.path())?;

        // Verify import succeeded
        assert_eq!(stats.lockers_imported, 2);
        assert_eq!(stats.rentals_imported, 1);
        assert_eq!(stats.payments_imported, 1);

        // Verify data in database
        let locker_count: i32 = db.conn.query_row(
            "SELECT COUNT(*) FROM lockers",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(locker_count, 2);

        Ok(())
    }
}
