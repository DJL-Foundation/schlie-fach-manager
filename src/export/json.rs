use crate::models::stats::{FullBackup, Location};
use crate::models::{Locker, Payment, Rental};
use chrono::Utc;
use color_eyre::eyre::Result;
use std::path::Path;

/// Exports a full backup to JSON.
pub fn export_full_backup(
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

    let json = serde_json::to_string_pretty(&backup)?;
    std::fs::write(path, json)?;

    Ok(())
}

/// Imports a full backup from JSON.
pub fn import_full_backup(path: &Path) -> Result<FullBackup> {
    let json = std::fs::read_to_string(path)?;
    let backup: FullBackup = serde_json::from_str(&json)?;
    Ok(backup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_import_backup() -> Result<()> {
        let lockers = vec![Locker::new("A-001", "Test", 100)];
        let rentals = vec![];
        let payments = vec![];
        let locations = vec![];

        let file = NamedTempFile::new()?;
        export_full_backup(&lockers, &rentals, &payments, &locations, file.path())?;

        let backup = import_full_backup(file.path())?;
        assert_eq!(backup.lockers.len(), 1);
        assert_eq!(backup.lockers[0].label, "A-001");

        Ok(())
    }
}
