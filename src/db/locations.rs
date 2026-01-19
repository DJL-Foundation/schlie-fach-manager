use crate::model::Location;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

/// Inserts a new location entry and returns its id.
pub fn create_location(conn: &Connection, location: &Location) -> Result<i64> {
    conn.execute(
        "INSERT INTO locations (name, created_at) VALUES (?1, ?2)",
        params![location.name, location.created_at.to_rfc3339()],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Inserts or updates a location entry using its id.
pub fn upsert_location(conn: &Connection, location: &Location) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO locations (id, name, created_at)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name
        "#,
        params![location.id, location.name, location.created_at.to_rfc3339()],
    )?;
    Ok(())
}

/// Returns all locations ordered by name.
pub fn list_locations(conn: &Connection) -> Result<Vec<Location>> {
    let mut stmt = conn.prepare("SELECT id, name, created_at FROM locations ORDER BY name ASC")?;
    let rows = stmt.query_map([], row_to_location)?;
    let mut locations = Vec::new();
    for location in rows {
        locations.push(location?);
    }
    Ok(locations)
}

/// Returns a location by its name.
pub fn get_location_by_name(conn: &Connection, name: &str) -> Result<Option<Location>> {
    let location = conn
        .query_row(
            "SELECT id, name, created_at FROM locations WHERE name = ?1",
            params![name],
            row_to_location,
        )
        .optional()?;
    Ok(location)
}

/// Maps a database row to a location domain model.
fn row_to_location(row: &Row<'_>) -> rusqlite::Result<Location> {
    let created_at: String = row.get("created_at")?;
    Ok(Location {
        id: row.get("id")?,
        name: row.get("name")?,
        created_at: parse_datetime(&created_at),
    })
}

/// Parses a RFC3339 timestamp or returns the current time.
fn parse_datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
