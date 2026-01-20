//! Locker-related Tauri commands

use tauri::State;
use crate::db::{DbConnection, Locker, CreateLockerInput, UpdateLockerInput};

/// Get all lockers
#[tauri::command]
pub async fn get_all_lockers(db: State<'_, DbConnection>) -> Result<Vec<Locker>, String> {
    let conn = db.0.lock().await;
    
    let mut stmt = conn
        .prepare(
            "SELECT id, number, location, size, is_damaged, notes, created_at 
             FROM lockers ORDER BY number",
        )
        .map_err(|e| e.to_string())?;
    
    let rows = stmt
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
        })
        .map_err(|e| e.to_string())?;
    
    let mut lockers = Vec::new();
    for row in rows {
        lockers.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(lockers)
}

/// Get available lockers (not currently rented and not damaged)
#[tauri::command]
pub async fn get_available_lockers(db: State<'_, DbConnection>) -> Result<Vec<Locker>, String> {
    let conn = db.0.lock().await;
    
    let mut stmt = conn
        .prepare(
            "SELECT l.id, l.number, l.location, l.size, l.is_damaged, l.notes, l.created_at 
             FROM lockers l
             WHERE l.is_damaged = 0
               AND l.id NOT IN (
                 SELECT locker_id FROM rentals 
                 WHERE date(end_date) >= date('now') AND deposit_returned = 0
               )
             ORDER BY l.number",
        )
        .map_err(|e| e.to_string())?;
    
    let rows = stmt
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
        })
        .map_err(|e| e.to_string())?;
    
    let mut lockers = Vec::new();
    for row in rows {
        lockers.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(lockers)
}

/// Get a single locker by ID
#[tauri::command]
pub async fn get_locker(db: State<'_, DbConnection>, id: i64) -> Result<Locker, String> {
    let conn = db.0.lock().await;
    
    conn.query_row(
        "SELECT id, number, location, size, is_damaged, notes, created_at 
         FROM lockers WHERE id = ?",
        [id],
        |row| {
            Ok(Locker {
                id: row.get(0)?,
                number: row.get(1)?,
                location: row.get(2)?,
                size: row.get(3)?,
                is_damaged: row.get::<_, i64>(4)? != 0,
                notes: row.get(5)?,
                created_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// Create a new locker
#[tauri::command]
pub async fn create_locker(
    db: State<'_, DbConnection>,
    locker: CreateLockerInput,
) -> Result<i64, String> {
    let conn = db.0.lock().await;
    
    conn.execute(
        "INSERT INTO lockers (number, location, size, notes) VALUES (?, ?, ?, ?)",
        (
            &locker.number,
            &locker.location,
            &locker.size,
            &locker.notes,
        ),
    )
    .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    // Log to audit
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details) VALUES (?, ?, ?, ?)",
        (
            "create",
            "locker",
            id,
            format!("Created locker {}", locker.number),
        ),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(id)
}

/// Update an existing locker
#[tauri::command]
pub async fn update_locker(
    db: State<'_, DbConnection>,
    locker: UpdateLockerInput,
) -> Result<(), String> {
    let conn = db.0.lock().await;
    
    conn.execute(
        "UPDATE lockers SET number = ?, location = ?, size = ?, is_damaged = ?, notes = ? 
         WHERE id = ?",
        (
            &locker.number,
            &locker.location,
            &locker.size,
            if locker.is_damaged { 1 } else { 0 },
            &locker.notes,
            locker.id,
        ),
    )
    .map_err(|e| e.to_string())?;
    
    // Log to audit
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details) VALUES (?, ?, ?, ?)",
        (
            "update",
            "locker",
            locker.id,
            format!("Updated locker {}", locker.number),
        ),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Delete a locker
#[tauri::command]
pub async fn delete_locker(db: State<'_, DbConnection>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().await;
    
    // Check if locker has active rentals
    let active_rentals: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM rentals WHERE locker_id = ? AND deposit_returned = 0",
            [id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    if active_rentals > 0 {
        return Err("Schließfach hat aktive Verleihe und kann nicht gelöscht werden.".to_string());
    }
    
    conn.execute("DELETE FROM lockers WHERE id = ?", [id])
        .map_err(|e| e.to_string())?;
    
    // Log to audit
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details) VALUES (?, ?, ?, ?)",
        ("delete", "locker", id, format!("Deleted locker id={}", id)),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
