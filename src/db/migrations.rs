use color_eyre::eyre::Result;
use rusqlite::Connection;

/// Current schema version for tracking migrations.
///
/// Version history:
/// - V1: Core tables (lockers, rentals, payments, locations, audit_log)
/// - V2: Settings table and occupancy_history table
const SCHEMA_VERSION: i32 = 2;

/// Applies all database migrations using versioned migrations.
pub fn apply_migrations(conn: &Connection) -> Result<()> {
    // Create schema_version table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY)",
        [],
    )?;

    // Get current version
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // Apply migrations based on version
    if current_version < 1 {
        apply_v1_migrations(conn)?;
        conn.execute("INSERT INTO schema_version (version) VALUES (1)", [])?;
    }

    if current_version < 2 {
        apply_v2_migrations(conn)?;
        conn.execute("INSERT INTO schema_version (version) VALUES (2)", [])?;
    }

    Ok(())
}

/// V1 migrations: Core tables (lockers, rentals, payments, locations, audit_log).
fn apply_v1_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Lockers table
        CREATE TABLE IF NOT EXISTS lockers (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL UNIQUE,
            location TEXT NOT NULL,
            height INTEGER NOT NULL,
            is_damaged BOOLEAN DEFAULT 0,
            created_at TEXT NOT NULL
        );

        -- Rentals table
        CREATE TABLE IF NOT EXISTS rentals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            locker_id INTEGER NOT NULL,
            tenant_username TEXT NOT NULL,
            tenant_type TEXT NOT NULL,
            rental_start_date TEXT NOT NULL,
            rental_end_date TEXT NOT NULL,
            deposit_paid BOOLEAN DEFAULT 0,
            deposit_returned BOOLEAN DEFAULT 0,
            created_at TEXT NOT NULL,
            returned_at TEXT,
            FOREIGN KEY (locker_id) REFERENCES lockers(id) ON DELETE CASCADE
        );

        -- Payments table
        CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rental_id INTEGER NOT NULL,
            amount_cents INTEGER NOT NULL,
            payment_type TEXT NOT NULL,
            payment_date TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY (rental_id) REFERENCES rentals(id) ON DELETE CASCADE
        );

        -- Locations table
        CREATE TABLE IF NOT EXISTS locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at TEXT NOT NULL
        );

        -- Audit log table
        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            action TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            entity_id INTEGER,
            details TEXT,
            username TEXT
        );

        -- Create indexes for better performance
        CREATE INDEX IF NOT EXISTS idx_rentals_locker ON rentals(locker_id);
        CREATE INDEX IF NOT EXISTS idx_rentals_tenant ON rentals(tenant_username);
        CREATE INDEX IF NOT EXISTS idx_rentals_active ON rentals(returned_at);
        CREATE INDEX IF NOT EXISTS idx_payments_rental ON payments(rental_id);
        CREATE INDEX IF NOT EXISTS idx_lockers_location ON lockers(location);
        "#,
    )?;
    Ok(())
}

/// V2 migrations: Settings and occupancy history tables.
fn apply_v2_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Settings table for application configuration
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            description TEXT,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Insert default settings
        INSERT OR IGNORE INTO settings (key, value, description) VALUES
            ('deposit_cents', '1000', 'Pfandbetrag in Cents (Standard: 10.00€)'),
            ('yearly_fee_cents', '1000', 'Jahresgebühr in Cents (Standard: 10.00€)'),
            ('billing_period', 'yearly', 'Berechnungszeitraum: monthly oder yearly'),
            ('currency', 'EUR', 'Währung (ISO 4217 Code)'),
            ('screensaver_timeout_seconds', '60', 'Sekunden Inaktivität bis Screensaver'),
            ('app_version', '2.1.0', 'Anwendungsversion');

        -- Occupancy history table for tracking locker usage over time
        CREATE TABLE IF NOT EXISTS occupancy_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            snapshot_date TEXT NOT NULL,
            total_lockers INTEGER NOT NULL,
            occupied_lockers INTEGER NOT NULL,
            occupancy_percent REAL NOT NULL,
            notes TEXT,
            UNIQUE(snapshot_date)
        );

        -- Index for efficient date-based queries
        CREATE INDEX IF NOT EXISTS idx_occupancy_date ON occupancy_history(snapshot_date);

        -- Index for audit log timestamp queries
        CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp);
        "#,
    )?;
    Ok(())
}

/// Returns the current schema version.
pub fn get_schema_version(conn: &Connection) -> Result<i32> {
    let version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        apply_migrations(&conn)?;

        // Verify all tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        assert!(tables.contains(&"lockers".to_string()));
        assert!(tables.contains(&"rentals".to_string()));
        assert!(tables.contains(&"payments".to_string()));
        assert!(tables.contains(&"locations".to_string()));
        assert!(tables.contains(&"audit_log".to_string()));
        assert!(tables.contains(&"settings".to_string()));
        assert!(tables.contains(&"occupancy_history".to_string()));

        Ok(())
    }

    #[test]
    fn test_schema_version() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        apply_migrations(&conn)?;

        let version = get_schema_version(&conn)?;
        assert_eq!(version, SCHEMA_VERSION);

        Ok(())
    }

    #[test]
    fn test_default_settings() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        apply_migrations(&conn)?;

        let count: i32 = conn.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;
        assert!(count >= 6); // At least 6 default settings

        let deposit: String = conn.query_row(
            "SELECT value FROM settings WHERE key = 'deposit_cents'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(deposit, "1000");

        Ok(())
    }

    #[test]
    fn test_idempotent_migrations() -> Result<()> {
        let conn = Connection::open_in_memory()?;

        // Apply migrations twice - should not fail
        apply_migrations(&conn)?;
        apply_migrations(&conn)?;

        let version = get_schema_version(&conn)?;
        assert_eq!(version, SCHEMA_VERSION);

        Ok(())
    }
}
