use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row, params};
use std::collections::HashMap;

/// Settings record stored in the database.
#[derive(Debug, Clone)]
pub struct SettingRecord {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Returns all settings from the database.
pub fn list_settings(conn: &Connection) -> Result<Vec<SettingRecord>> {
    let mut stmt =
        conn.prepare("SELECT key, value, description, updated_at FROM settings ORDER BY key ASC")?;
    let rows = stmt.query_map([], row_to_setting)?;
    let mut settings = Vec::new();
    for setting in rows {
        settings.push(setting?);
    }
    Ok(settings)
}

/// Updates a single setting value.
pub fn upsert_setting(conn: &Connection, key: &str, value: &str, description: &str) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO settings (key, value, description, updated_at)
        VALUES (?1, ?2, ?3, datetime('now'))
        ON CONFLICT(key) DO UPDATE SET
            value = excluded.value,
            description = excluded.description,
            updated_at = datetime('now')
        "#,
        params![key, value, description],
    )?;
    Ok(())
}

/// Loads settings into a key-value map.
pub fn load_settings_map(conn: &Connection) -> Result<HashMap<String, String>> {
    let settings = list_settings(conn)?;
    Ok(settings
        .into_iter()
        .map(|entry| (entry.key, entry.value))
        .collect())
}

/// Maps a database row to a settings record.
fn row_to_setting(row: &Row<'_>) -> rusqlite::Result<SettingRecord> {
    let updated_at: String = row.get("updated_at")?;
    Ok(SettingRecord {
        key: row.get("key")?,
        value: row.get("value")?,
        description: row.get("description")?,
        updated_at: parse_datetime(&updated_at),
    })
}

/// Parses a RFC3339 timestamp or returns the current time.
fn parse_datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::Database;

    #[test]
    fn upsert_and_load_setting() -> Result<()> {
        let db = Database::open_in_memory()?;
        upsert_setting(db.connection(), "currency", "EUR", "Währung")?;
        let map = load_settings_map(db.connection())?;
        assert_eq!(map.get("currency").map(String::as_str), Some("EUR"));
        Ok(())
    }
}
