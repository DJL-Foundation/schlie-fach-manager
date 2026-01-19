//! Export/Import service for Schließfach-Manager

use std::path::Path;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use crate::db::{Locker, Rental, Payment, Settings};
use crate::error::Result;

/// Export data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub exported_at: String,
    pub lockers: Vec<Locker>,
    pub rentals: Vec<Rental>,
    pub payments: Vec<Payment>,
    pub settings: Settings,
}

/// Export all data to TOML format
pub fn export_to_toml(conn: &Connection, path: &Path) -> Result<()> {
    let data = collect_export_data(conn)?;
    let toml_str = toml::to_string_pretty(&data)
        .map_err(|e| crate::error::AppError::Operation(e.to_string()))?;
    std::fs::write(path, toml_str)?;
    Ok(())
}

/// Export all data to JSON format
pub fn export_to_json(conn: &Connection, path: &Path) -> Result<()> {
    let data = collect_export_data(conn)?;
    let json_str = serde_json::to_string_pretty(&data)?;
    std::fs::write(path, json_str)?;
    Ok(())
}

/// Export lockers to CSV format
pub fn export_to_csv(conn: &Connection, path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| crate::error::AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    
    // Get all lockers
    let mut stmt = conn.prepare(
        "SELECT id, number, location, size, is_damaged, notes, created_at FROM lockers"
    )?;
    
    let lockers = stmt.query_map([], |row| {
        Ok(Locker {
            id: row.get(0)?,
            number: row.get(1)?,
            location: row.get(2)?,
            size: row.get(3)?,
            is_damaged: row.get::<_, i64>(4)? != 0,
            notes: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    
    // Write header
    wtr.write_record(&["id", "number", "location", "size", "is_damaged", "notes", "created_at"])
        .map_err(|e| crate::error::AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    
    // Write records
    for locker in lockers {
        let locker = locker?;
        wtr.write_record(&[
            locker.id.to_string(),
            locker.number,
            locker.location,
            locker.size,
            locker.is_damaged.to_string(),
            locker.notes.unwrap_or_default(),
            locker.created_at,
        ])
        .map_err(|e| crate::error::AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    }
    
    wtr.flush()
        .map_err(|e| crate::error::AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    
    Ok(())
}

fn collect_export_data(conn: &Connection) -> Result<ExportData> {
    // Get lockers
    let mut stmt = conn.prepare(
        "SELECT id, number, location, size, is_damaged, notes, created_at FROM lockers"
    )?;
    let lockers: Vec<Locker> = stmt
        .query_map([], |row| {
            Ok(Locker {
                id: row.get(0)?,
                number: row.get(1)?,
                location: row.get(2)?,
                size: row.get(3)?,
                is_damaged: row.get::<_, i64>(4)? != 0,
                notes: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    
    // Get rentals
    let mut stmt = conn.prepare(
        "SELECT r.id, r.locker_id, l.number, r.renter_name, r.renter_email, r.renter_phone,
                r.start_date, r.end_date, r.deposit_paid, r.deposit_returned, r.notes, r.created_at
         FROM rentals r JOIN lockers l ON r.locker_id = l.id"
    )?;
    let rentals: Vec<Rental> = stmt
        .query_map([], |row| {
            Ok(Rental {
                id: row.get(0)?,
                locker_id: row.get(1)?,
                locker_number: row.get(2)?,
                renter_name: row.get(3)?,
                renter_email: row.get(4)?,
                renter_phone: row.get(5)?,
                start_date: row.get(6)?,
                end_date: row.get(7)?,
                deposit_paid: row.get::<_, i64>(8)? != 0,
                deposit_returned: row.get::<_, i64>(9)? != 0,
                notes: row.get(10)?,
                created_at: row.get(11)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    
    // Get payments
    let mut stmt = conn.prepare(
        "SELECT id, rental_id, amount_cents, payment_date, payment_type, notes, created_at 
         FROM payments"
    )?;
    let payments: Vec<Payment> = stmt
        .query_map([], |row| {
            Ok(Payment {
                id: row.get(0)?,
                rental_id: row.get(1)?,
                amount_cents: row.get(2)?,
                payment_date: row.get(3)?,
                payment_type: row.get(4)?,
                notes: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    
    // Get settings (simplified)
    let settings = Settings {
        deposit_cents: 1000,
        yearly_fee_cents: 1000,
        billing_period: "yearly".to_string(),
        currency: "EUR".to_string(),
        screensaver_timeout_seconds: 300,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    };
    
    Ok(ExportData {
        version: env!("CARGO_PKG_VERSION").to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        lockers,
        rentals,
        payments,
        settings,
    })
}
