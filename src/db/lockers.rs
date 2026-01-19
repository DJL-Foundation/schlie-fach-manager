use crate::models::Locker;
use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};
use rusqlite::{params, Connection, OptionalExtension, Row};

/// Creates a new locker in the database.
pub fn create_locker(conn: &Connection, locker: &Locker) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO lockers (label, location, height, is_damaged, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![
            &locker.label,
            &locker.location,
            locker.height,
            locker.is_damaged,
            locker.created_at.to_rfc3339(),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Updates an existing locker.
pub fn update_locker(conn: &Connection, locker: &Locker) -> Result<()> {
    let affected = conn.execute(
        r#"
        UPDATE lockers SET
            label = ?1,
            location = ?2,
            height = ?3,
            is_damaged = ?4
        WHERE id = ?5
        "#,
        params![
            &locker.label,
            &locker.location,
            locker.height,
            locker.is_damaged,
            locker.id,
        ],
    )?;
    if affected == 0 {
        return Err(eyre!("Locker with id {} not found", locker.id));
    }
    Ok(())
}

/// Deletes a locker by id.
pub fn delete_locker(conn: &Connection, id: i64) -> Result<bool> {
    let affected = conn.execute("DELETE FROM lockers WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

/// Returns a single locker by its id.
pub fn get_locker(conn: &Connection, id: i64) -> Result<Option<Locker>> {
    let locker = conn
        .query_row(
            "SELECT id, label, location, height, is_damaged, created_at FROM lockers WHERE id = ?1",
            params![id],
            row_to_locker,
        )
        .optional()?;
    Ok(locker)
}

/// Returns a single locker by its label.
pub fn get_locker_by_label(conn: &Connection, label: &str) -> Result<Option<Locker>> {
    let locker = conn
        .query_row(
            "SELECT id, label, location, height, is_damaged, created_at FROM lockers WHERE label = ?1",
            params![label],
            row_to_locker,
        )
        .optional()?;
    Ok(locker)
}

/// Returns all lockers sorted by label.
pub fn list_lockers(conn: &Connection) -> Result<Vec<Locker>> {
    let mut stmt = conn.prepare(
        "SELECT id, label, location, height, is_damaged, created_at FROM lockers ORDER BY label ASC",
    )?;
    let rows = stmt.query_map([], row_to_locker)?;
    let mut lockers = Vec::new();
    for locker in rows {
        lockers.push(locker?);
    }
    Ok(lockers)
}

/// Returns lockers filtered by location.
pub fn list_lockers_by_location(conn: &Connection, location: &str) -> Result<Vec<Locker>> {
    let mut stmt = conn.prepare(
        "SELECT id, label, location, height, is_damaged, created_at FROM lockers WHERE location = ?1 ORDER BY label ASC",
    )?;
    let rows = stmt.query_map(params![location], row_to_locker)?;
    let mut lockers = Vec::new();
    for locker in rows {
        lockers.push(locker?);
    }
    Ok(lockers)
}

/// Returns free lockers (not currently rented) optionally filtered by location and height range.
pub fn list_free_lockers(
    conn: &Connection,
    location: Option<&str>,
    min_height: Option<i32>,
    max_height: Option<i32>,
) -> Result<Vec<Locker>> {
    let mut query = String::from(
        r#"
        SELECT l.id, l.label, l.location, l.height, l.is_damaged, l.created_at
        FROM lockers l
        WHERE l.id NOT IN (
            SELECT locker_id FROM rentals WHERE returned_at IS NULL
        )
        "#,
    );

    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(loc) = location {
        query.push_str(" AND l.location = ?");
        params_vec.push(Box::new(loc.to_string()));
    }

    if let Some(min) = min_height {
        query.push_str(" AND l.height >= ?");
        params_vec.push(Box::new(min));
    }

    if let Some(max) = max_height {
        query.push_str(" AND l.height <= ?");
        params_vec.push(Box::new(max));
    }

    query.push_str(" ORDER BY l.label ASC");

    let mut stmt = conn.prepare(&query)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    let rows = stmt.query_map(params_refs.as_slice(), row_to_locker)?;

    let mut lockers = Vec::new();
    for locker in rows {
        lockers.push(locker?);
    }
    Ok(lockers)
}

/// Marks a locker as damaged.
pub fn mark_locker_damaged(conn: &Connection, id: i64) -> Result<()> {
    conn.execute(
        "UPDATE lockers SET is_damaged = 1 WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

/// Marks a locker as repaired.
pub fn mark_locker_repaired(conn: &Connection, id: i64) -> Result<()> {
    conn.execute(
        "UPDATE lockers SET is_damaged = 0 WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

/// Returns the count of lockers.
pub fn count_lockers(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row("SELECT COUNT(1) FROM lockers", [], |row| row.get(0))?;
    Ok(count)
}

/// Returns all distinct locations.
pub fn list_distinct_locations(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT location FROM lockers ORDER BY location ASC")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut locations = Vec::new();
    for location in rows {
        locations.push(location?);
    }
    Ok(locations)
}

fn row_to_locker(row: &Row) -> rusqlite::Result<Locker> {
    let created_at_str: String = row.get("created_at")?;
    let created_at = DateTime::parse_from_rfc3339(&created_at_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    Ok(Locker {
        id: row.get("id")?,
        label: row.get("label")?,
        location: row.get("location")?,
        height: row.get("height")?,
        is_damaged: row.get("is_damaged")?,
        created_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::apply_migrations;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_and_get_locker() -> Result<()> {
        let conn = setup_db();
        let locker = Locker::new("A-042", "Hauptgebäude", 100);
        let id = create_locker(&conn, &locker)?;

        let fetched = get_locker(&conn, id)?.unwrap();
        assert_eq!(fetched.label, "A-042");
        assert_eq!(fetched.location, "Hauptgebäude");
        assert_eq!(fetched.height, 100);
        assert!(!fetched.is_damaged);

        Ok(())
    }

    #[test]
    fn test_update_locker() -> Result<()> {
        let conn = setup_db();
        let mut locker = Locker::new("A-042", "Hauptgebäude", 100);
        let id = create_locker(&conn, &locker)?;
        locker.id = id;

        locker.height = 150;
        locker.is_damaged = true;
        update_locker(&conn, &locker)?;

        let fetched = get_locker(&conn, id)?.unwrap();
        assert_eq!(fetched.height, 150);
        assert!(fetched.is_damaged);

        Ok(())
    }

    #[test]
    fn test_list_lockers() -> Result<()> {
        let conn = setup_db();
        create_locker(&conn, &Locker::new("B-02", "Test", 100))?;
        create_locker(&conn, &Locker::new("A-01", "Test", 50))?;

        let lockers = list_lockers(&conn)?;
        assert_eq!(lockers.len(), 2);
        assert_eq!(lockers[0].label, "A-01");
        assert_eq!(lockers[1].label, "B-02");

        Ok(())
    }

    #[test]
    fn test_delete_locker() -> Result<()> {
        let conn = setup_db();
        let id = create_locker(&conn, &Locker::new("A-01", "Test", 100))?;
        assert!(delete_locker(&conn, id)?);
        assert!(!delete_locker(&conn, id)?);

        Ok(())
    }

    #[test]
    fn test_list_distinct_locations() -> Result<()> {
        let conn = setup_db();
        create_locker(&conn, &Locker::new("A-01", "Location1", 100))?;
        create_locker(&conn, &Locker::new("A-02", "Location1", 100))?;
        create_locker(&conn, &Locker::new("B-01", "Location2", 100))?;

        let locations = list_distinct_locations(&conn)?;
        assert_eq!(locations.len(), 2);
        assert!(locations.contains(&"Location1".to_string()));
        assert!(locations.contains(&"Location2".to_string()));

        Ok(())
    }
}
