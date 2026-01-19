use crate::{
    export::{ExportData, ExportMetadata},
    model::{Location, Locker, Payment, PaymentType, Rental},
};
use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use csv::Reader;
use std::path::Path;

/// Imports export data from a directory containing CSV files.
pub fn import(dir: &Path) -> Result<ExportData> {
    let lockers = read_lockers(&dir.join("lockers.csv"))?;
    let rentals = read_rentals(&dir.join("rentals.csv"))?;
    let payments = read_payments(&dir.join("payments.csv"))?;
    let locations = read_locations(&dir.join("locations.csv"))?;

    Ok(ExportData {
        metadata: ExportMetadata {
            version: "2.1.0".to_string(),
            export_date: Utc::now().to_rfc3339(),
            database_version: 3,
        },
        lockers,
        rentals,
        payments,
        locations,
    })
}

/// Reads lockers from a CSV file.
fn read_lockers(path: &Path) -> Result<Vec<Locker>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = Reader::from_path(path)?;
    let mut lockers = Vec::new();
    for record in reader.records() {
        let record = record?;
        let locker = Locker {
            id: record.get(0).unwrap_or("0").parse().unwrap_or(0),
            number: record.get(1).unwrap_or("").to_string(),
            location: record.get(2).unwrap_or("").to_string(),
            size: record.get(3).unwrap_or("").to_string(),
            is_damaged: record.get(4).unwrap_or("false") == "true",
            notes: empty_to_option(record.get(5).unwrap_or("")),
            created_at: parse_datetime(record.get(6).unwrap_or("")),
        };
        lockers.push(locker);
    }
    Ok(lockers)
}

/// Reads rentals from a CSV file.
fn read_rentals(path: &Path) -> Result<Vec<Rental>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = Reader::from_path(path)?;
    let mut rentals = Vec::new();
    for record in reader.records() {
        let record = record?;
        let rental = Rental {
            id: record.get(0).unwrap_or("0").parse().unwrap_or(0),
            locker_id: record.get(1).unwrap_or("0").parse().unwrap_or(0),
            renter_name: record.get(2).unwrap_or("").to_string(),
            renter_email: empty_to_option(record.get(3).unwrap_or("")),
            renter_phone: empty_to_option(record.get(4).unwrap_or("")),
            start_date: parse_date(record.get(5).unwrap_or("")),
            end_date: parse_date(record.get(6).unwrap_or("")),
            deposit_paid: record.get(7).unwrap_or("false") == "true",
            deposit_returned: record.get(8).unwrap_or("false") == "true",
            notes: empty_to_option(record.get(9).unwrap_or("")),
            created_at: parse_datetime(record.get(10).unwrap_or("")),
        };
        rentals.push(rental);
    }
    Ok(rentals)
}

/// Reads payments from a CSV file.
fn read_payments(path: &Path) -> Result<Vec<Payment>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = Reader::from_path(path)?;
    let mut payments = Vec::new();
    for record in reader.records() {
        let record = record?;
        let payment = Payment {
            id: record.get(0).unwrap_or("0").parse().unwrap_or(0),
            rental_id: record.get(1).unwrap_or("0").parse().unwrap_or(0),
            amount_cents: record.get(2).unwrap_or("0").parse().unwrap_or(0),
            payment_date: parse_date(record.get(3).unwrap_or("")),
            payment_type: PaymentType::from_str(record.get(4).unwrap_or("")),
            notes: empty_to_option(record.get(5).unwrap_or("")),
            created_at: parse_datetime(record.get(6).unwrap_or("")),
        };
        payments.push(payment);
    }
    Ok(payments)
}

/// Reads locations from a CSV file.
fn read_locations(path: &Path) -> Result<Vec<Location>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = Reader::from_path(path)?;
    let mut locations = Vec::new();
    for record in reader.records() {
        let record = record?;
        let location = Location {
            id: record.get(0).unwrap_or("0").parse().unwrap_or(0),
            name: record.get(1).unwrap_or("").to_string(),
            created_at: parse_datetime(record.get(2).unwrap_or("")),
        };
        locations.push(location);
    }
    Ok(locations)
}

/// Converts empty strings into None.
fn empty_to_option(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

/// Parses a YYYY-MM-DD date or returns today's date.
fn parse_date(value: &str) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive())
}

/// Parses a RFC3339 timestamp or returns the current time.
fn parse_datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
