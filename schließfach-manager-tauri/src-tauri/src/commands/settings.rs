//! Settings-related Tauri commands

use tauri::State;
use crate::db::{DbConnection, Settings, AuditLogEntry};

/// Get all settings
#[tauri::command]
pub async fn get_settings(db: State<'_, DbConnection>) -> Result<Settings, String> {
    let conn = db.0.lock().await;
    
    let deposit_cents: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'deposit_cents'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "1000".to_string());
    
    let yearly_fee_cents: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'yearly_fee_cents'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "1000".to_string());
    
    let billing_period: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'billing_period'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "yearly".to_string());
    
    let currency: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'currency'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "EUR".to_string());
    
    let screensaver_timeout: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'screensaver_timeout_seconds'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "300".to_string());
    
    Ok(Settings {
        deposit_cents: deposit_cents.parse().unwrap_or(1000),
        yearly_fee_cents: yearly_fee_cents.parse().unwrap_or(1000),
        billing_period,
        currency,
        screensaver_timeout_seconds: screensaver_timeout.parse().unwrap_or(300),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Update a single setting
#[tauri::command]
pub async fn update_setting(
    db: State<'_, DbConnection>,
    key: String,
    value: String,
) -> Result<(), String> {
    let conn = db.0.lock().await;
    
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
        (&key, &value),
    )
    .map_err(|e| e.to_string())?;
    
    // Log to audit
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details) VALUES (?, ?, ?, ?)",
        ("update", "setting", Option::<i64>::None, format!("Updated setting {} to {}", key, value)),
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Get audit log entries
#[tauri::command]
pub async fn get_audit_log(
    db: State<'_, DbConnection>,
    limit: Option<i64>,
) -> Result<Vec<AuditLogEntry>, String> {
    let conn = db.0.lock().await;
    let limit = limit.unwrap_or(100);
    
    let mut stmt = conn
        .prepare(
            "SELECT id, timestamp, action, entity_type, entity_id, details, username 
             FROM audit_log 
             ORDER BY timestamp DESC 
             LIMIT ?",
        )
        .map_err(|e| e.to_string())?;
    
    let rows = stmt
        .query_map([limit], |row| {
            Ok(AuditLogEntry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                action: row.get(2)?,
                entity_type: row.get(3)?,
                entity_id: row.get(4)?,
                details: row.get(5)?,
                username: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    let mut entries = Vec::new();
    for row in rows {
        entries.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(entries)
}

/// Run database migrations
#[tauri::command]
pub async fn run_database_migrations(db: State<'_, DbConnection>) -> Result<(), String> {
    let conn = db.0.lock().await;
    crate::db::migrations::run_migrations(&conn).map_err(|e| e.to_string())
}
