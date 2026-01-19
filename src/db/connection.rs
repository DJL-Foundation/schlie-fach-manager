use crate::config;
use color_eyre::eyre::Result;
use rusqlite::Connection;
use std::fs;
use std::path::Path;

const DB_FILENAME: &str = "schliessfach-manager.db";

/// High-level handle around the SQLite connection.
pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Opens (and creates if necessary) a database inside the platform specific data directory.
    pub fn open_default() -> Result<Self> {
        let path = config::database_path(DB_FILENAME)?;
        Self::open(path)
    }

    /// Opens a database located at `path`, creating parent directories and running migrations.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let db = Self { conn };
        super::migrations::apply_migrations(&db.conn)?;
        Ok(db)
    }

    /// Opens an ephemeral in-memory database, primarily intended for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        super::migrations::apply_migrations(&db.conn)?;
        Ok(db)
    }

    /// Access to the underlying rusqlite connection.
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() -> Result<()> {
        let db = Database::open_in_memory()?;
        // Verify that migrations were applied
        db.conn.execute(
            "INSERT INTO lockers (label, location, height, is_damaged, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params!["A-01", "Test", 100, false, "2024-01-01T00:00:00Z"],
        )?;
        Ok(())
    }
}
