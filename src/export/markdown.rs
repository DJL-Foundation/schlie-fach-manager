use crate::export::ExportData;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Writes export data to a Markdown report.
pub fn export(path: &Path, data: &ExportData) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut content = String::new();
    content.push_str("# Schließfach-Manager Export\n\n");
    content.push_str(&format!(
        "- Version: {}\n- Export Datum: {}\n- Datenbank-Version: {}\n\n",
        data.metadata.version, data.metadata.export_date, data.metadata.database_version
    ));

    content.push_str("## Schließfächer\n\n");
    content.push_str("| ID | Nummer | Standort | Größe | Defekt |\n");
    content.push_str("|----|--------|----------|-------|--------|\n");
    for locker in &data.lockers {
        content.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            locker.id,
            locker.number,
            locker.location,
            locker.size,
            if locker.is_damaged { "Ja" } else { "Nein" }
        ));
    }

    content.push_str("\n## Verleihvorgänge\n\n");
    content.push_str("| ID | Schließfach | Verleiher | Zeitraum |\n");
    content.push_str("|----|-------------|-----------|----------|\n");
    for rental in &data.rentals {
        content.push_str(&format!(
            "| {} | {} | {} | {} → {} |\n",
            rental.id, rental.locker_id, rental.renter_name, rental.start_date, rental.end_date
        ));
    }

    fs::write(path, content)?;
    Ok(())
}
