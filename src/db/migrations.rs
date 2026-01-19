use color_eyre::eyre::Result;
use rusqlite::Connection;

/// Applies all database migrations.
pub fn apply_migrations(conn: &Connection) -> Result<()> {
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

        Ok(())
    }
}
