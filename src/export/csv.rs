use crate::models::{DebtorInfo, Locker, RentalWithLocker, TenantType};
use chrono::Utc;
use color_eyre::eyre::Result;
use std::path::Path;

/// Exports debtors to CSV.
pub fn export_debtors_csv(debtors: &[DebtorInfo], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record(["Email", "Typ", "Schulden_EUR", "Ueberfaellig_Tage"])?;

    for debtor in debtors {
        let amount_eur = format!("{:.2}", debtor.total_debt_cents as f64 / 100.0);
        let tenant_type = match debtor.tenant_type {
            TenantType::Schüler => "Schüler",
            TenantType::Lehrer => "Lehrer",
        };

        wtr.write_record([
            &debtor.email,
            tenant_type,
            &amount_eur,
            &debtor.days_overdue.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

/// Exports lockers to CSV.
pub fn export_lockers_csv(lockers: &[Locker], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record(["ID", "Label", "Standort", "Hoehe_cm", "Defekt", "Erstellt"])?;

    for locker in lockers {
        let damaged = if locker.is_damaged { "Ja" } else { "Nein" };
        wtr.write_record([
            &locker.id.to_string(),
            &locker.label,
            &locker.location,
            &locker.height.to_string(),
            damaged,
            &locker.created_at.to_rfc3339(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

/// Exports active rentals to CSV.
pub fn export_active_rentals_csv(rentals: &[RentalWithLocker], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record([
        "Schliessfach",
        "Standort",
        "Mieter",
        "Typ",
        "Beginn",
        "Ende",
        "Status",
    ])?;

    let today = Utc::now().date_naive();

    for item in rentals {
        let status = if item.rental.rental_end_date < today {
            "Überfällig"
        } else {
            "Aktiv"
        };

        let tenant_type = match item.rental.tenant_type {
            TenantType::Schüler => "Schüler",
            TenantType::Lehrer => "Lehrer",
        };

        wtr.write_record([
            &item.locker.label,
            &item.locker.location,
            &format!("{}@athenetz.de", item.rental.tenant_username),
            tenant_type,
            &item.rental.rental_start_date.to_string(),
            &item.rental.rental_end_date.to_string(),
            status,
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_lockers_csv() -> Result<()> {
        let lockers = vec![
            Locker::new("A-001", "Test", 100),
            Locker::new("A-002", "Test", 200),
        ];

        let file = NamedTempFile::new()?;
        export_lockers_csv(&lockers, file.path())?;

        let content = std::fs::read_to_string(file.path())?;
        assert!(content.contains("A-001"));
        assert!(content.contains("A-002"));

        Ok(())
    }
}
