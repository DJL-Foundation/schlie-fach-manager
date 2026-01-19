use crate::config;
use color_eyre::eyre::Result;
use rusqlite::Connection;
use std::fs;
use std::path::Path;

const DB_FILENAME: &str = "schliessfach-manager.db";

/// High-level handle around the SQLite connection that powers the locker manager.
pub struct Database {
    conn: Connection,
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
        let mut db = Self { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    /// Opens an ephemeral in-memory database, primarily intended for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let mut db = Self { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    fn apply_migrations(&mut self) -> Result<()> {
        super::migrations::run_migrations(&self.conn)?;
        Ok(())
    }

    /// Access to the underlying rusqlite connection for advanced use-cases.
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Mutable access to the underlying rusqlite connection.
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}
