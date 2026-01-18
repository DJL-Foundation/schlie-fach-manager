use crate::{
    config,
    model::{Locker, LockerStatus},
};
use color_eyre::eyre::Result;
use rusqlite::types::Type;
use rusqlite::{Connection, OptionalExtension, Row, params};
use std::error::Error as StdError;
use std::fmt;
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
        let db = Self { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    /// Opens an ephemeral in-memory database, primarily intended for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    fn apply_migrations(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS lockers (
                id INTEGER PRIMARY KEY,
                label TEXT NOT NULL,
                status INTEGER NOT NULL,
                occupant TEXT,
                note TEXT
            );
            "#,
        )?;
        Ok(())
    }

    /// Inserts or updates a locker entry.
    pub fn upsert_locker(&self, locker: &Locker) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO lockers (id, label, status, occupant, note)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                label=excluded.label,
                status=excluded.status,
                occupant=excluded.occupant,
                note=excluded.note;
            "#,
            params![
                locker.id,
                &locker.label,
                locker.status.to_db_value(),
                locker.occupant.as_ref(),
                locker.note.as_ref()
            ],
        )?;
        Ok(())
    }

    /// Returns all lockers sorted by their label.
    pub fn list_lockers(&self) -> Result<Vec<Locker>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, label, status, occupant, note FROM lockers ORDER BY label ASC")?;
        let rows = stmt.query_map([], row_to_locker)?;
        let mut lockers = Vec::new();
        for locker in rows {
            lockers.push(locker?);
        }
        Ok(lockers)
    }

    /// Returns a single locker by its id.
    pub fn get_locker(&self, id: i64) -> Result<Option<Locker>> {
        let locker = self
            .conn
            .query_row(
                "SELECT id, label, status, occupant, note FROM lockers WHERE id = ?1",
                params![id],
                row_to_locker,
            )
            .optional()?;
        Ok(locker)
    }

    /// Deletes a locker and reports whether anything was removed.
    pub fn delete_locker(&self, id: i64) -> Result<bool> {
        let affected = self
            .conn
            .execute("DELETE FROM lockers WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    /// Inserts the provided lockers only when the table is still empty.
    pub fn seed_if_empty(&self, seed: &[Locker]) -> Result<()> {
        if self.count_lockers()? == 0 {
            for locker in seed {
                self.upsert_locker(locker)?;
            }
        }
        Ok(())
    }

    /// Returns the number of lockers stored in the database.
    pub fn count_lockers(&self) -> Result<i64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(1) FROM lockers", [], |row| row.get(0))?;
        Ok(count)
    }

    /// Access to the underlying rusqlite connection for advanced use-cases.
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

fn row_to_locker(row: &Row) -> rusqlite::Result<Locker> {
    let status_value: i64 = row.get("status")?;
    let status = LockerStatus::from_db_value(status_value).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            Type::Integer,
            Box::new(StatusDecodeError(status_value)),
        )
    })?;

    Ok(Locker {
        id: row.get("id")?,
        label: row.get("label")?,
        status,
        occupant: row.get("occupant")?,
        note: row.get("note")?,
    })
}

#[derive(Debug)]
struct StatusDecodeError(i64);

impl fmt::Display for StatusDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid locker status value {}", self.0)
    }
}

impl StdError for StatusDecodeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre::Result;

    #[test]
    fn open_in_memory_runs_migrations() -> Result<()> {
        let db = Database::open_in_memory()?;
        db.conn.execute(
            "INSERT INTO lockers (id, label, status) VALUES (?1, ?2, ?3)",
            params![1_i64, "A-01", LockerStatus::Available.to_db_value()],
        )?;
        Ok(())
    }

    #[test]
    fn upsert_and_fetch_roundtrip() -> Result<()> {
        let db = Database::open_in_memory()?;
        let mut locker = Locker::new(42, "Z-01");
        locker.assign("Alice");
        locker.note = Some("Test".into());

        db.upsert_locker(&locker)?;
        let fetched = db.get_locker(42)?.expect("locker missing");
        assert_eq!(fetched, locker);

        Ok(())
    }

    #[test]
    fn delete_reports_if_row_existed() -> Result<()> {
        let db = Database::open_in_memory()?;
        let locker = Locker::new(7, "B-12");
        db.upsert_locker(&locker)?;

        assert!(db.delete_locker(7)?);
        assert!(!db.delete_locker(7)?);

        Ok(())
    }

    #[test]
    fn list_lockers_are_sorted_by_label() -> Result<()> {
        let db = Database::open_in_memory()?;
        let unsorted = vec![
            Locker::new(2, "B-02"),
            Locker::new(1, "A-01"),
            Locker::new(3, "C-03"),
        ];
        for locker in &unsorted {
            db.upsert_locker(locker)?;
        }

        let labels: Vec<String> = db
            .list_lockers()?
            .into_iter()
            .map(|locker| locker.label)
            .collect();

        assert_eq!(
            labels,
            vec!["A-01".to_string(), "B-02".to_string(), "C-03".to_string()]
        );

        Ok(())
    }

    #[test]
    fn seed_if_empty_runs_only_on_empty_table() -> Result<()> {
        let db = Database::open_in_memory()?;
        let initial = vec![Locker::new(1, "A-01"), Locker::new(2, "B-02")];
        db.seed_if_empty(initial.as_slice())?;
        assert_eq!(db.count_lockers()?, 2);

        let custom = Locker::new(3, "C-03");
        db.upsert_locker(&custom)?;

        let extra = vec![Locker::new(99, "Z-99")];
        db.seed_if_empty(extra.as_slice())?;

        let lockers = db.list_lockers()?;
        assert_eq!(lockers.len(), 3);
        let has_z99 = lockers.iter().any(|locker| locker.label == "Z-99");
        assert!(!has_z99);

        Ok(())
    }
}
