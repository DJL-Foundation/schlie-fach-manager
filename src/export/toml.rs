//! TOML export and import functionality.
//!
//! This module provides functions to export and import locker management data
//! in TOML format. TOML is a human-readable configuration file format that is
//! excellent for backups that may need manual editing.
//!
//! # Example
//!
//! ```rust,no_run
//! use schliessfach_manager::export::toml::{export_full_backup_toml, import_full_backup_toml};
//! use std::path::Path;
//!
//! // Export data to TOML
//! let lockers = vec![];
//! let rentals = vec![];
//! let payments = vec![];
//! let locations = vec![];
//!
//! export_full_backup_toml(&lockers, &rentals, &payments, &locations, Path::new("backup.toml"))
//!     .expect("Failed to export backup");
//!
//! // Import data from TOML
//! let backup = import_full_backup_toml(Path::new("backup.toml"))
//!     .expect("Failed to import backup");
//! ```

use crate::models::stats::{FullBackup, Location};
use crate::models::{Locker, Payment, Rental};
use chrono::Utc;
use color_eyre::eyre::Result;
use std::path::Path;

/// Exports a full backup to TOML format.
///
/// Creates a complete backup of all locker management data including lockers,
/// rentals, payments, and locations. The TOML format is human-readable and
/// can be manually edited if necessary.
///
/// # Arguments
///
/// * `lockers` - Slice of all lockers to export
/// * `rentals` - Slice of all rentals to export
/// * `payments` - Slice of all payments to export
/// * `locations` - Slice of all locations to export
/// * `path` - Path where the TOML file will be written
///
/// # Errors
///
/// Returns an error if:
/// - The data cannot be serialized to TOML
/// - The file cannot be written to the specified path
///
/// # Example
///
/// ```rust,no_run
/// use schliessfach_manager::export::toml::export_full_backup_toml;
/// use schliessfach_manager::models::Locker;
/// use std::path::Path;
///
/// let lockers = vec![Locker::new("A-001", "Hauptgebäude", 150)];
/// export_full_backup_toml(&lockers, &[], &[], &[], Path::new("backup.toml")).unwrap();
/// ```
pub fn export_full_backup_toml(
    lockers: &[Locker],
    rentals: &[Rental],
    payments: &[Payment],
    locations: &[Location],
    path: &Path,
) -> Result<()> {
    let backup = FullBackup {
        version: "2.0".to_string(),
        exported_at: Utc::now(),
        lockers: lockers.to_vec(),
        rentals: rentals.to_vec(),
        payments: payments.to_vec(),
        locations: locations.to_vec(),
    };

    let toml_string = toml::to_string_pretty(&backup)?;
    std::fs::write(path, toml_string)?;

    Ok(())
}

/// Imports a full backup from TOML format.
///
/// Reads and deserializes a TOML backup file created by [`export_full_backup_toml`].
/// The backup contains all locker management data including lockers, rentals,
/// payments, and locations.
///
/// # Arguments
///
/// * `path` - Path to the TOML backup file
///
/// # Returns
///
/// Returns the deserialized [`FullBackup`] containing all data.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The file is not valid TOML
/// - The TOML structure doesn't match the expected backup format
///
/// # Example
///
/// ```rust,no_run
/// use schliessfach_manager::export::toml::import_full_backup_toml;
/// use std::path::Path;
///
/// let backup = import_full_backup_toml(Path::new("backup.toml")).unwrap();
/// println!("Imported {} lockers", backup.lockers.len());
/// ```
pub fn import_full_backup_toml(path: &Path) -> Result<FullBackup> {
    let toml_string = std::fs::read_to_string(path)?;
    let backup: FullBackup = toml::from_str(&toml_string)?;
    Ok(backup)
}

/// Exports only lockers to TOML format.
///
/// Useful for exporting just the locker inventory without rental data.
///
/// # Arguments
///
/// * `lockers` - Slice of lockers to export
/// * `path` - Path where the TOML file will be written
///
/// # Errors
///
/// Returns an error if serialization or file writing fails.
pub fn export_lockers_toml(lockers: &[Locker], path: &Path) -> Result<()> {
    /// Wrapper struct for lockers-only export.
    #[derive(serde::Serialize)]
    struct LockersExport<'a> {
        version: &'static str,
        exported_at: chrono::DateTime<chrono::Utc>,
        lockers: &'a [Locker],
    }

    let export = LockersExport {
        version: "2.0",
        exported_at: Utc::now(),
        lockers,
    };

    let toml_string = toml::to_string_pretty(&export)?;
    std::fs::write(path, toml_string)?;

    Ok(())
}

/// Exports active rentals to TOML format.
///
/// Creates a TOML file containing only active rental information,
/// useful for generating reports or sharing rental status.
///
/// # Arguments
///
/// * `rentals` - Slice of rentals to export
/// * `path` - Path where the TOML file will be written
///
/// # Errors
///
/// Returns an error if serialization or file writing fails.
pub fn export_rentals_toml(rentals: &[Rental], path: &Path) -> Result<()> {
    /// Wrapper struct for rentals-only export.
    #[derive(serde::Serialize)]
    struct RentalsExport<'a> {
        version: &'static str,
        exported_at: chrono::DateTime<chrono::Utc>,
        rentals: &'a [Rental],
    }

    let export = RentalsExport {
        version: "2.0",
        exported_at: Utc::now(),
        rentals,
    };

    let toml_string = toml::to_string_pretty(&export)?;
    std::fs::write(path, toml_string)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_import_backup_toml() -> Result<()> {
        let lockers = vec![Locker::new("A-001", "Test", 100)];
        let rentals = vec![];
        let payments = vec![];
        let locations = vec![];

        let file = NamedTempFile::new()?;
        export_full_backup_toml(&lockers, &rentals, &payments, &locations, file.path())?;

        let backup = import_full_backup_toml(file.path())?;
        assert_eq!(backup.lockers.len(), 1);
        assert_eq!(backup.lockers[0].label, "A-001");
        assert_eq!(backup.version, "2.0");

        Ok(())
    }

    #[test]
    fn test_export_lockers_toml() -> Result<()> {
        let lockers = vec![
            Locker::new("A-001", "Hauptgebäude", 100),
            Locker::new("A-002", "Hauptgebäude", 150),
        ];

        let file = NamedTempFile::new()?;
        export_lockers_toml(&lockers, file.path())?;

        // Verify file is valid TOML
        let content = std::fs::read_to_string(file.path())?;
        assert!(content.contains("[[lockers]]"));
        assert!(content.contains("A-001"));
        assert!(content.contains("A-002"));

        Ok(())
    }

    #[test]
    fn test_toml_is_human_readable() -> Result<()> {
        let lockers = vec![Locker::new("B-005", "Turnhalle", 120)];

        let file = NamedTempFile::new()?;
        export_lockers_toml(&lockers, file.path())?;

        let content = std::fs::read_to_string(file.path())?;

        // TOML should be human-readable
        assert!(content.contains("label = \"B-005\""));
        assert!(content.contains("location = \"Turnhalle\""));
        assert!(content.contains("height = 120"));

        Ok(())
    }
}
