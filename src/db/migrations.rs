use color_eyre::eyre::Result;
use rusqlite::{Connection, params};

/// Runs all database migrations in order.
pub fn run_migrations(conn: &Connection) -> Result<()> {
    let current_version = get_schema_version(conn)?;
    
    if current_version < 1 {
        create_initial_schema(conn)?;
    }
    
    if current_version < 2 {
        add_settings_table(conn)?;
        add_audit_log_table(conn)?;
    }
    
    if current_version < 3 {
        add_occupancy_history_table(conn)?;
    }
    
    set_schema_version(conn, 3)?;
    Ok(())
}

/// Gets the current schema version from the database.
fn get_schema_version(conn: &Connection) -> Result<i64> {
    // First check if schema_version table exists
    let table_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_version')",
            [],
            |row| row.get(0),
        )?;
    
    if !table_exists {
        return Ok(0);
    }
    
    let version: i64 = conn
        .query_row("SELECT version FROM schema_version ORDER BY id DESC LIMIT 1", [], |row| row.get(0))
        .unwrap_or(0);
    
    Ok(version)
}

/// Sets the schema version in the database.
fn set_schema_version(conn: &Connection, version: i64) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            version INTEGER NOT NULL,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;
    
    conn.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        params![version],
    )?;
    
    Ok(())
}

/// Creates the initial schema (v1): lockers, rentals, payments, locations tables.
fn create_initial_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS lockers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            number TEXT NOT NULL UNIQUE,
            location TEXT NOT NULL,
            size TEXT NOT NULL,
            is_damaged INTEGER NOT NULL DEFAULT 0,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS rentals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            locker_id INTEGER NOT NULL,
            renter_name TEXT NOT NULL,
            renter_email TEXT,
            renter_phone TEXT,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL,
            deposit_paid INTEGER NOT NULL DEFAULT 0,
            deposit_returned INTEGER NOT NULL DEFAULT 0,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (locker_id) REFERENCES lockers(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_rentals_locker_id ON rentals(locker_id);
        CREATE INDEX IF NOT EXISTS idx_rentals_dates ON rentals(start_date, end_date);

        CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rental_id INTEGER NOT NULL,
            amount_cents INTEGER NOT NULL,
            payment_date TEXT NOT NULL,
            payment_type TEXT NOT NULL,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (rental_id) REFERENCES rentals(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_payments_rental_id ON payments(rental_id);
        CREATE INDEX IF NOT EXISTS idx_payments_date ON payments(payment_date);

        CREATE TABLE IF NOT EXISTS locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        "#,
    )?;
    
    Ok(())
}

/// Adds the settings and audit_log tables (v2).
fn add_settings_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            description TEXT,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        INSERT OR IGNORE INTO settings (key, value, description) VALUES
            ('deposit_cents', '1000', 'Pfandbetrag in Cents (Standard: 10.00€)'),
            ('yearly_fee_cents', '1000', 'Jahresgebühr in Cents (Standard: 10.00€)'),
            ('billing_period', 'yearly', 'Berechnungszeitraum: monthly oder yearly'),
            ('currency', 'EUR', 'Währung (ISO 4217 Code)'),
            ('screensaver_timeout_seconds', '60', 'Sekunden Inaktivität bis Screensaver (Standard: 60)'),
            ('app_version', '2.1.0', 'Anwendungsversion');
        "#,
    )?;
    
    Ok(())
}

/// Adds the audit_log table (v2).
fn add_audit_log_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL DEFAULT (datetime('now')),
            action TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            entity_id INTEGER,
            details TEXT,
            username TEXT DEFAULT 'system'
        );

        CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp);
        CREATE INDEX IF NOT EXISTS idx_audit_entity ON audit_log(entity_type, entity_id);
        "#,
    )?;
    
    Ok(())
}

/// Adds the occupancy_history table (v3).
fn add_occupancy_history_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS occupancy_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            snapshot_date TEXT NOT NULL UNIQUE,
            total_lockers INTEGER NOT NULL,
            occupied_lockers INTEGER NOT NULL,
            occupancy_percent REAL NOT NULL,
            notes TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_occupancy_date ON occupancy_history(snapshot_date);
        "#,
    )?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migration_creates_all_tables() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        // Check that all tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        assert!(tables.contains(&"lockers".to_string()));
        assert!(tables.contains(&"rentals".to_string()));
        assert!(tables.contains(&"payments".to_string()));
        assert!(tables.contains(&"locations".to_string()));
        assert!(tables.contains(&"settings".to_string()));
        assert!(tables.contains(&"audit_log".to_string()));
        assert!(tables.contains(&"occupancy_history".to_string()));
        assert!(tables.contains(&"schema_version".to_string()));

        Ok(())
    }

    #[test]
    fn test_schema_version_tracking() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let version = get_schema_version(&conn)?;
        assert_eq!(version, 3);

        Ok(())
    }

    #[test]
    fn test_settings_defaults() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let deposit: String = conn.query_row(
            "SELECT value FROM settings WHERE key = 'deposit_cents'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(deposit, "1000");

        let currency: String = conn.query_row(
            "SELECT value FROM settings WHERE key = 'currency'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(currency, "EUR");

        Ok(())
    }

    #[test]
    fn test_indices_created() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let indices: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        assert!(indices.contains(&"idx_rentals_locker_id".to_string()));
        assert!(indices.contains(&"idx_rentals_dates".to_string()));
        assert!(indices.contains(&"idx_payments_rental_id".to_string()));
        assert!(indices.contains(&"idx_payments_date".to_string()));
        assert!(indices.contains(&"idx_audit_timestamp".to_string()));
        assert!(indices.contains(&"idx_audit_entity".to_string()));
        assert!(indices.contains(&"idx_occupancy_date".to_string()));

        Ok(())
    }

    #[test]
    fn test_multiple_migrations_are_idempotent() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        
        // Run migrations multiple times
        run_migrations(&conn)?;
        run_migrations(&conn)?;
        run_migrations(&conn)?;

        // Should still be at version 3
        let version = get_schema_version(&conn)?;
        assert_eq!(version, 3);

        // Settings should not be duplicated
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM settings WHERE key = 'deposit_cents'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 1);

        Ok(())
    }
}
