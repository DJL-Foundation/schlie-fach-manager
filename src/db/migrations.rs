use anyhow::Result;
use chrono::Utc;
use rusqlite::Connection;

const TARGET_VERSION: i32 = 3;

/// Runs all database migrations required for version 2.1.
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

    set_schema_version(conn, TARGET_VERSION)?;
    Ok(())
}

/// Returns the current schema version stored in SQLite.
fn get_schema_version(conn: &Connection) -> Result<i32> {
    let version: i32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
    Ok(version)
}

/// Persists the schema version back into SQLite.
fn set_schema_version(conn: &Connection, version: i32) -> Result<()> {
    conn.pragma_update(None, "user_version", version)?;
    Ok(())
}

/// Creates the baseline schema used by version 2.0 and later.
fn create_initial_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS lockers (
            id INTEGER PRIMARY KEY,
            number TEXT NOT NULL,
            location TEXT NOT NULL,
            size TEXT NOT NULL,
            is_damaged INTEGER NOT NULL DEFAULT 0,
            notes TEXT,
            created_at TEXT NOT NULL
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
            created_at TEXT NOT NULL,
            FOREIGN KEY (locker_id) REFERENCES lockers(id)
        );

        CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rental_id INTEGER NOT NULL,
            amount_cents INTEGER NOT NULL,
            payment_date TEXT NOT NULL,
            payment_type TEXT NOT NULL,
            notes TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (rental_id) REFERENCES rentals(id)
        );

        CREATE TABLE IF NOT EXISTS locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL
        );
        "#,
    )?;

    migrate_legacy_lockers(conn)?;
    seed_default_locations(conn)?;

    Ok(())
}

/// Migrates the legacy locker table structure into the new schema.
fn migrate_legacy_lockers(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(lockers)")?;
    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<Vec<_>, _>>()?;

    if columns.iter().any(|col| col == "number") {
        return Ok(());
    }

    if columns.iter().any(|col| col == "label") {
        conn.execute_batch(
            r#"
            ALTER TABLE lockers RENAME TO lockers_legacy;
            CREATE TABLE lockers (
                id INTEGER PRIMARY KEY,
                number TEXT NOT NULL,
                location TEXT NOT NULL,
                size TEXT NOT NULL,
                is_damaged INTEGER NOT NULL DEFAULT 0,
                notes TEXT,
                created_at TEXT NOT NULL
            );
            "#,
        )?;

        let now = Utc::now().to_rfc3339();
        conn.execute(
            r#"
            INSERT INTO lockers (id, number, location, size, is_damaged, notes, created_at)
            SELECT id,
                   label,
                   'Hauptgebäude' AS location,
                   'Standard' AS size,
                   CASE WHEN status = 2 THEN 1 ELSE 0 END AS is_damaged,
                   note,
                   ?1
            FROM lockers_legacy;
            "#,
            [now],
        )?;
        conn.execute_batch("DROP TABLE lockers_legacy;")?;
    }

    Ok(())
}

/// Seeds a default location so the UI can render locations immediately.
fn seed_default_locations(conn: &Connection) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR IGNORE INTO locations (name, created_at) VALUES (?1, ?2)",
        ("Hauptgebäude", &now),
    )?;
    Ok(())
}

/// Adds the settings table and default records.
fn add_settings_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            description TEXT,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        "#,
    )?;

    conn.execute_batch(
        r#"
        INSERT OR IGNORE INTO settings (key, value, description) VALUES
            ('deposit_cents', '1000', 'Pfandbetrag in Cents (Standard: 10.00€)'),
            ('yearly_fee_cents', '1000', 'Jahresgebühr in Cents (Standard: 10.00€)'),
            ('billing_period', 'yearly', 'Berechnungszeitraum: monthly oder yearly'),
            ('currency', 'EUR', 'Währung (ISO 4217 Code)'),
            ('screensaver_timeout_seconds', '60', 'Sekunden Inaktivität bis Screensaver (Standard: 60)'),
            ('export_format', 'toml', 'Standard-Exportformat (toml, json, csv)'),
            ('export_directory', '', 'Export-Verzeichnis'),
            ('app_version', '2.1.0', 'Anwendungsversion');
        "#,
    )?;
    Ok(())
}

/// Adds the audit log table and indexes.
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

/// Adds the optional occupancy history table.
fn add_occupancy_history_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS occupancy_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            snapshot_date TEXT NOT NULL,
            total_lockers INTEGER NOT NULL,
            occupied_lockers INTEGER NOT NULL,
            occupancy_percent REAL NOT NULL,
            notes TEXT,
            UNIQUE(snapshot_date)
        );

        CREATE INDEX IF NOT EXISTS idx_occupancy_date ON occupancy_history(snapshot_date);
        "#,
    )?;
    Ok(())
}
