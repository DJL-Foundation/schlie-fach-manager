//! Settings management for the Schließfach-Manager system.
//!
//! This module provides functions to read and write application settings
//! stored in the settings table.

use color_eyre::eyre::{Result, eyre};
use rusqlite::{Connection, params};
use std::collections::HashMap;

/// Application settings with typed fields.
#[derive(Debug, Clone, PartialEq)]
pub struct AppSettings {
    /// Deposit amount in cents (default: 1000 = 10.00€).
    pub deposit_cents: i32,
    /// Yearly fee in cents (default: 1000 = 10.00€).
    pub yearly_fee_cents: i32,
    /// Billing period: "monthly" or "yearly".
    pub billing_period: String,
    /// Currency code (ISO 4217).
    pub currency: String,
    /// Screensaver timeout in seconds.
    pub screensaver_timeout_seconds: i32,
    /// Application version.
    pub app_version: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            deposit_cents: 1000,
            yearly_fee_cents: 1000,
            billing_period: "yearly".to_string(),
            currency: "EUR".to_string(),
            screensaver_timeout_seconds: 60,
            app_version: "2.1.0".to_string(),
        }
    }
}

/// Retrieves a single setting value by key.
pub fn get_setting(conn: &Connection, key: &str) -> Result<String> {
    let value: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .map_err(|_| eyre!("Setting '{}' not found", key))?;
    Ok(value)
}

/// Sets a setting value, updating the timestamp.
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    let affected = conn.execute(
        "UPDATE settings SET value = ?1, updated_at = datetime('now') WHERE key = ?2",
        params![value, key],
    )?;

    if affected == 0 {
        // Key doesn't exist, insert it
        conn.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))",
            params![key, value],
        )?;
    }

    Ok(())
}

/// Retrieves all settings as a key-value map.
pub fn get_all_settings(conn: &Connection) -> Result<HashMap<String, String>> {
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut settings = HashMap::new();
    for row in rows {
        let (key, value) = row?;
        settings.insert(key, value);
    }

    Ok(settings)
}

/// Loads all settings into the typed AppSettings struct.
pub fn load_settings(conn: &Connection) -> Result<AppSettings> {
    let all = get_all_settings(conn)?;

    let deposit_cents = all
        .get("deposit_cents")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000);

    let yearly_fee_cents = all
        .get("yearly_fee_cents")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000);

    let billing_period = all
        .get("billing_period")
        .cloned()
        .unwrap_or_else(|| "yearly".to_string());

    let currency = all
        .get("currency")
        .cloned()
        .unwrap_or_else(|| "EUR".to_string());

    let screensaver_timeout_seconds = all
        .get("screensaver_timeout_seconds")
        .and_then(|v| v.parse().ok())
        .unwrap_or(60);

    let app_version = all
        .get("app_version")
        .cloned()
        .unwrap_or_else(|| "2.1.0".to_string());

    Ok(AppSettings {
        deposit_cents,
        yearly_fee_cents,
        billing_period,
        currency,
        screensaver_timeout_seconds,
        app_version,
    })
}

/// Saves all settings from the AppSettings struct.
pub fn save_settings(conn: &Connection, settings: &AppSettings) -> Result<()> {
    set_setting(conn, "deposit_cents", &settings.deposit_cents.to_string())?;
    set_setting(conn, "yearly_fee_cents", &settings.yearly_fee_cents.to_string())?;
    set_setting(conn, "billing_period", &settings.billing_period)?;
    set_setting(conn, "currency", &settings.currency)?;
    set_setting(
        conn,
        "screensaver_timeout_seconds",
        &settings.screensaver_timeout_seconds.to_string(),
    )?;
    set_setting(conn, "app_version", &settings.app_version)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::apply_migrations;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_get_setting() -> Result<()> {
        let conn = setup_db();
        let value = get_setting(&conn, "deposit_cents")?;
        assert_eq!(value, "1000");
        Ok(())
    }

    #[test]
    fn test_set_setting() -> Result<()> {
        let conn = setup_db();
        set_setting(&conn, "deposit_cents", "2000")?;
        let value = get_setting(&conn, "deposit_cents")?;
        assert_eq!(value, "2000");
        Ok(())
    }

    #[test]
    fn test_set_new_setting() -> Result<()> {
        let conn = setup_db();
        set_setting(&conn, "custom_key", "custom_value")?;
        let value = get_setting(&conn, "custom_key")?;
        assert_eq!(value, "custom_value");
        Ok(())
    }

    #[test]
    fn test_get_all_settings() -> Result<()> {
        let conn = setup_db();
        let settings = get_all_settings(&conn)?;
        assert!(settings.contains_key("deposit_cents"));
        assert!(settings.contains_key("currency"));
        Ok(())
    }

    #[test]
    fn test_load_settings() -> Result<()> {
        let conn = setup_db();
        let settings = load_settings(&conn)?;
        assert_eq!(settings.deposit_cents, 1000);
        assert_eq!(settings.currency, "EUR");
        assert_eq!(settings.billing_period, "yearly");
        Ok(())
    }

    #[test]
    fn test_save_settings() -> Result<()> {
        let conn = setup_db();
        let mut settings = load_settings(&conn)?;
        settings.deposit_cents = 2500;
        settings.screensaver_timeout_seconds = 120;

        save_settings(&conn, &settings)?;

        let loaded = load_settings(&conn)?;
        assert_eq!(loaded.deposit_cents, 2500);
        assert_eq!(loaded.screensaver_timeout_seconds, 120);
        Ok(())
    }
}
