//! Rental-related Tauri commands

use tauri::State;
use crate::db::{DbConnection, Rental, CreateRentalInput};
use chrono::{NaiveDate, Duration};

/// Get all rentals with locker info
#[tauri::command]
pub async fn get_all_rentals(db: State<'_, DbConnection>) -> Result<Vec<Rental>, String> {
    let conn = db.0.lock().await;
    
    let mut stmt = conn
        .prepare(
            "SELECT r.id, r.locker_id, l.number as locker_number, r.renter_name, r.renter_email, 
                    r.renter_phone, r.start_date, r.end_date, r.deposit_paid, r.deposit_returned, 
                    r.notes, r.created_at
             FROM rentals r
             JOIN lockers l ON r.locker_id = l.id
             ORDER BY r.end_date DESC",
        )
        .map_err(|e| e.to_string())?;
    
    let rows = stmt
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
        })
        .map_err(|e| e.to_string())?;
    
    let mut rentals = Vec::new();
    for row in rows {
        rentals.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(rentals)
}

/// Get active rentals (not returned, not expired)
#[tauri::command]
pub async fn get_active_rentals(db: State<'_, DbConnection>) -> Result<Vec<Rental>, String> {
    let conn = db.0.lock().await;
    
    let mut stmt = conn
        .prepare(
            "SELECT r.id, r.locker_id, l.number as locker_number, r.renter_name, r.renter_email, 
                    r.renter_phone, r.start_date, r.end_date, r.deposit_paid, r.deposit_returned, 
                    r.notes, r.created_at
             FROM rentals r
             JOIN lockers l ON r.locker_id = l.id
             WHERE date(r.end_date) >= date('now') AND r.deposit_returned = 0
             ORDER BY r.end_date ASC",
        )
        .map_err(|e| e.to_string())?;
    
    let rows = stmt
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
        })
        .map_err(|e| e.to_string())?;
    
    let mut rentals = Vec::new();
    for row in rows {
        rentals.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(rentals)
}

/// Get overdue rentals
#[tauri::command]
pub async fn get_overdue_rentals(db: State<'_, DbConnection>) -> Result<Vec<Rental>, String> {
    let conn = db.0.lock().await;
    
    let mut stmt = conn
        .prepare(
            "SELECT r.id, r.locker_id, l.number as locker_number, r.renter_name, r.renter_email, 
                    r.renter_phone, r.start_date, r.end_date, r.deposit_paid, r.deposit_returned, 
                    r.notes, r.created_at
             FROM rentals r
             JOIN lockers l ON r.locker_id = l.id
             WHERE date(r.end_date) < date('now') AND r.deposit_returned = 0
             ORDER BY r.end_date ASC",
        )
        .map_err(|e| e.to_string())?;
    
    let rows = stmt
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
        })
        .map_err(|e| e.to_string())?;
    
    let mut rentals = Vec::new();
    for row in rows {
        rentals.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(rentals)
}

/// Get a single rental by ID
#[tauri::command]
pub async fn get_rental(db: State<'_, DbConnection>, id: i64) -> Result<Rental, String> {
    let conn = db.0.lock().await;
    
    conn.query_row(
        "SELECT r.id, r.locker_id, l.number as locker_number, r.renter_name, r.renter_email, 
                r.renter_phone, r.start_date, r.end_date, r.deposit_paid, r.deposit_returned, 
                r.notes, r.created_at
         FROM rentals r
         JOIN lockers l ON r.locker_id = l.id
         WHERE r.id = ?",
        [id],
        |row| {
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
        },
    )
    .map_err(|e| e.to_string())
}

/// Create a new rental
#[tauri::command]
pub async fn create_rental(
    db: State<'_, DbConnection>,
    rental: CreateRentalInput,
) -> Result<i64, String> {
    let conn = db.0.lock().await;
    
    // Calculate end date
    let start_date = NaiveDate::parse_from_str(&rental.start_date, "%Y-%m-%d")
        .map_err(|e| format!("Ungültiges Startdatum: {}", e))?;
    
    let end_date = start_date + Duration::days(rental.duration_months as i64 * 30);
    let end_date_str = end_date.format("%Y-%m-%d").to_string();
    
    conn.execute(
        "INSERT INTO rentals (locker_id, renter_name, renter_email, renter_phone, 
                              start_date, end_date, deposit_paid, notes) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        (
            rental.locker_id,
            &rental.renter_name,
            &rental.renter_email,
            &rental.renter_phone,
            &rental.start_date,
            &end_date_str,
            if rental.deposit_paid { 1 } else { 0 },
            &rental.notes,
        ),
    )
    .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    // Log to audit
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details) VALUES (?, ?, ?, ?)",
        (
            "create",
            "rental",
            id,
            format!("Created rental for locker_id={} to {}", rental.locker_id, rental.renter_name),
        ),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(id)
}

/// Extend a rental by specified number of months
#[tauri::command]
pub async fn extend_rental(
    db: State<'_, DbConnection>,
    id: i64,
    months: i32,
) -> Result<(), String> {
    let conn = db.0.lock().await;
    
    // Get current end date
    let current_end: String = conn
        .query_row("SELECT end_date FROM rentals WHERE id = ?", [id], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    let end_date = NaiveDate::parse_from_str(&current_end, "%Y-%m-%d")
        .map_err(|e| format!("Ungültiges Enddatum: {}", e))?;
    
    let new_end_date = end_date + Duration::days(months as i64 * 30);
    let new_end_date_str = new_end_date.format("%Y-%m-%d").to_string();
    
    conn.execute(
        "UPDATE rentals SET end_date = ? WHERE id = ?",
        (&new_end_date_str, id),
    )
    .map_err(|e| e.to_string())?;
    
    // Log to audit
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details) VALUES (?, ?, ?, ?)",
        (
            "extend",
            "rental",
            id,
            format!("Extended rental by {} months to {}", months, new_end_date_str),
        ),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Return a rental (mark deposit as returned)
#[tauri::command]
pub async fn return_rental(db: State<'_, DbConnection>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().await;
    
    conn.execute(
        "UPDATE rentals SET deposit_returned = 1 WHERE id = ?",
        [id],
    )
    .map_err(|e| e.to_string())?;
    
    // Log to audit
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details) VALUES (?, ?, ?, ?)",
        ("return", "rental", id, "Rental returned, deposit marked as returned"),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
