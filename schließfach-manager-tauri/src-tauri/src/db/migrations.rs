//! Database migrations for Schließfach-Manager.

use rusqlite::Connection;
use crate::error::Result;

/// Run all database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Create migrations table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;
    
    // Run each migration
    run_migration(conn, "001_initial_schema", create_initial_schema)?;
    run_migration(conn, "002_settings_table", create_settings_table)?;
    run_migration(conn, "003_audit_log", create_audit_log_table)?;
    
    Ok(())
}

fn run_migration<F>(conn: &Connection, name: &str, migration_fn: F) -> Result<()>
where
    F: FnOnce(&Connection) -> Result<()>,
{
    // Check if migration already applied
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM migrations WHERE name = ?",
        [name],
        |row| row.get(0),
    )?;
    
    if count == 0 {
        migration_fn(conn)?;
        conn.execute(
            "INSERT INTO migrations (name) VALUES (?)",
            [name],
        )?;
        println!("Applied migration: {}", name);
    }
    
    Ok(())
}

fn create_initial_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Locations table
        CREATE TABLE IF NOT EXISTS locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Lockers table
        CREATE TABLE IF NOT EXISTS lockers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            number TEXT NOT NULL UNIQUE,
            location TEXT NOT NULL,
            size TEXT NOT NULL CHECK (size IN ('S', 'M', 'L', 'XL')),
            is_damaged INTEGER NOT NULL DEFAULT 0,
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Rentals table
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
            FOREIGN KEY (locker_id) REFERENCES lockers(id)
        );

        -- Payments table
        CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rental_id INTEGER NOT NULL,
            amount_cents INTEGER NOT NULL,
            payment_date TEXT,
            payment_type TEXT NOT NULL CHECK (payment_type IN ('deposit', 'yearly_fee', 'extension', 'refund', 'other')),
            notes TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (rental_id) REFERENCES rentals(id)
        );

        -- Create indexes
        CREATE INDEX IF NOT EXISTS idx_lockers_location ON lockers(location);
        CREATE INDEX IF NOT EXISTS idx_lockers_size ON lockers(size);
        CREATE INDEX IF NOT EXISTS idx_rentals_locker_id ON rentals(locker_id);
        CREATE INDEX IF NOT EXISTS idx_rentals_end_date ON rentals(end_date);
        CREATE INDEX IF NOT EXISTS idx_payments_rental_id ON payments(rental_id);
        "#,
    )?;
    
    Ok(())
}

fn create_settings_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Settings table
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Insert default settings
        INSERT OR IGNORE INTO settings (key, value) VALUES 
            ('deposit_cents', '1000'),
            ('yearly_fee_cents', '1000'),
            ('billing_period', 'yearly'),
            ('currency', 'EUR'),
            ('screensaver_timeout_seconds', '300');
        "#,
    )?;
    
    Ok(())
}

fn create_audit_log_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Audit log table
        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL DEFAULT (datetime('now')),
            action TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            entity_id INTEGER,
            details TEXT,
            username TEXT NOT NULL DEFAULT 'system'
        );

        -- Create index
        CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp);
        CREATE INDEX IF NOT EXISTS idx_audit_log_entity ON audit_log(entity_type, entity_id);
        "#,
    )?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;

    #[test]
    fn test_migrations() {
        let conn = establish_test_connection().unwrap();
        run_migrations(&conn).unwrap();
        
        // Verify tables exist
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='lockers'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 1);
    }
}
