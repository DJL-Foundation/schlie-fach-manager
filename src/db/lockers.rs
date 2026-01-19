use crate::model::Locker;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

/// Inserts or updates a locker entry.
pub fn upsert_locker(conn: &Connection, locker: &Locker) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO lockers (id, number, location, size, is_damaged, notes, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ON CONFLICT(id) DO UPDATE SET
            number = excluded.number,
            location = excluded.location,
            size = excluded.size,
            is_damaged = excluded.is_damaged,
            notes = excluded.notes;
        "#,
        params![
            locker.id,
            locker.number,
            locker.location,
            locker.size,
            locker.is_damaged as i64,
            locker.notes,
            locker.created_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// Returns all lockers sorted by their number.
pub fn list_lockers(conn: &Connection) -> Result<Vec<Locker>> {
    let mut stmt = conn.prepare(
        "SELECT id, number, location, size, is_damaged, notes, created_at FROM lockers ORDER BY number ASC",
    )?;
    let rows = stmt.query_map([], row_to_locker)?;
    let mut lockers = Vec::new();
    for locker in rows {
        lockers.push(locker?);
    }
    Ok(lockers)
}

/// Returns a single locker by its id.
pub fn get_locker(conn: &Connection, id: i64) -> Result<Option<Locker>> {
    let locker = conn
        .query_row(
            "SELECT id, number, location, size, is_damaged, notes, created_at FROM lockers WHERE id = ?1",
            params![id],
            row_to_locker,
        )
        .optional()?;
    Ok(locker)
}

/// Returns the number of lockers stored in the database.
pub fn count_lockers(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row("SELECT COUNT(1) FROM lockers", [], |row| row.get(0))?;
    Ok(count)
}

/// Maps a database row to a locker domain model.
fn row_to_locker(row: &Row<'_>) -> rusqlite::Result<Locker> {
    let created_at: String = row.get("created_at")?;
    Ok(Locker {
        id: row.get("id")?,
        number: row.get("number")?,
        location: row.get("location")?,
        size: row.get("size")?,
        is_damaged: row.get::<_, i64>("is_damaged")? != 0,
        notes: row.get("notes")?,
        created_at: parse_datetime(&created_at),
    })
}

/// Parses a RFC3339 timestamp or returns the current time.
fn parse_datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
