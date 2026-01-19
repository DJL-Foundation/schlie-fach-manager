use crate::export::ExportData;
use anyhow::Result;
use csv::Writer;
use std::fs;
use std::path::Path;

/// Writes export data into multiple CSV files within the provided directory.
pub fn export(dir: &Path, data: &ExportData) -> Result<()> {
    fs::create_dir_all(dir)?;
    write_lockers(&dir.join("lockers.csv"), data)?;
    write_rentals(&dir.join("rentals.csv"), data)?;
    write_payments(&dir.join("payments.csv"), data)?;
    write_locations(&dir.join("locations.csv"), data)?;
    Ok(())
}

/// Writes lockers data to CSV.
fn write_lockers(path: &Path, data: &ExportData) -> Result<()> {
    let mut writer = Writer::from_path(path)?;
    writer.write_record([
        "id",
        "number",
        "location",
        "size",
        "is_damaged",
        "notes",
        "created_at",
    ])?;
    for locker in &data.lockers {
        writer.write_record([
            locker.id.to_string(),
            locker.number.clone(),
            locker.location.clone(),
            locker.size.clone(),
            locker.is_damaged.to_string(),
            locker.notes.clone().unwrap_or_default(),
            locker.created_at.to_rfc3339(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

/// Writes rentals data to CSV.
fn write_rentals(path: &Path, data: &ExportData) -> Result<()> {
    let mut writer = Writer::from_path(path)?;
    writer.write_record([
        "id",
        "locker_id",
        "renter_name",
        "renter_email",
        "renter_phone",
        "start_date",
        "end_date",
        "deposit_paid",
        "deposit_returned",
        "notes",
        "created_at",
    ])?;
    for rental in &data.rentals {
        writer.write_record([
            rental.id.to_string(),
            rental.locker_id.to_string(),
            rental.renter_name.clone(),
            rental.renter_email.clone().unwrap_or_default(),
            rental.renter_phone.clone().unwrap_or_default(),
            rental.start_date.to_string(),
            rental.end_date.to_string(),
            rental.deposit_paid.to_string(),
            rental.deposit_returned.to_string(),
            rental.notes.clone().unwrap_or_default(),
            rental.created_at.to_rfc3339(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

/// Writes payments data to CSV.
fn write_payments(path: &Path, data: &ExportData) -> Result<()> {
    let mut writer = Writer::from_path(path)?;
    writer.write_record([
        "id",
        "rental_id",
        "amount_cents",
        "payment_date",
        "payment_type",
        "notes",
        "created_at",
    ])?;
    for payment in &data.payments {
        writer.write_record([
            payment.id.to_string(),
            payment.rental_id.to_string(),
            payment.amount_cents.to_string(),
            payment.payment_date.to_string(),
            payment.payment_type.as_str().to_string(),
            payment.notes.clone().unwrap_or_default(),
            payment.created_at.to_rfc3339(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

/// Writes locations data to CSV.
fn write_locations(path: &Path, data: &ExportData) -> Result<()> {
    let mut writer = Writer::from_path(path)?;
    writer.write_record(["id", "name", "created_at"])?;
    for location in &data.locations {
        writer.write_record([
            location.id.to_string(),
            location.name.clone(),
            location.created_at.to_rfc3339(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}
