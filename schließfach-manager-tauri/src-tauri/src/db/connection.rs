//! Database connection handling for Schließfach-Manager.

use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;
use directories::ProjectDirs;

use crate::error::Result;

/// Wrapper around a thread-safe database connection
pub struct DbConnection(pub Arc<Mutex<Connection>>);

impl DbConnection {
    /// Create a new database connection wrapper
    pub fn new(conn: Connection) -> Self {
        Self(Arc::new(Mutex::new(conn)))
    }
}

/// Establish a connection to the SQLite database
pub fn establish_connection() -> Result<Connection> {
    let data_dir = ProjectDirs::from("de", "djl", "schließfach-manager")
        .expect("Could not determine data directory");
    
    let data_path = data_dir.data_dir();
    std::fs::create_dir_all(data_path)?;
    
    let db_path = data_path.join("lockers.db");
    let conn = Connection::open(&db_path)?;
    
    // Enable foreign keys
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    
    Ok(conn)
}

/// Get the database path
pub fn get_db_path() -> Option<std::path::PathBuf> {
    ProjectDirs::from("de", "djl", "schließfach-manager")
        .map(|dirs| dirs.data_dir().join("lockers.db"))
}

#[cfg(test)]
pub fn establish_test_connection() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}
