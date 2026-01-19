use color_eyre::eyre::Result;
use rusqlite::{Connection, OptionalExtension, params};

/// Represents a location in the v2.1 schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

/// Creates a new location.
pub fn create_location(conn: &Connection, name: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO locations (name) VALUES (?1)",
        params![name],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Returns a location by ID.
pub fn get_location(conn: &Connection, id: i64) -> Result<Option<Location>> {
    let location = conn
        .query_row(
            "SELECT id, name, created_at FROM locations WHERE id = ?1",
            params![id],
            |row| {
                Ok(Location {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        )
        .optional()?;
    
    Ok(location)
}

/// Returns a location by name.
pub fn get_location_by_name(conn: &Connection, name: &str) -> Result<Option<Location>> {
    let location = conn
        .query_row(
            "SELECT id, name, created_at FROM locations WHERE name = ?1",
            params![name],
            |row| {
                Ok(Location {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        )
        .optional()?;
    
    Ok(location)
}

/// Returns all locations.
pub fn list_locations(conn: &Connection) -> Result<Vec<Location>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, created_at FROM locations ORDER BY name ASC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(Location {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
        })
    })?;
    
    let mut locations = Vec::new();
    for location in rows {
        locations.push(location?);
    }
    
    Ok(locations)
}

/// Updates a location name.
pub fn update_location(conn: &Connection, id: i64, name: &str) -> Result<()> {
    conn.execute(
        "UPDATE locations SET name = ?1 WHERE id = ?2",
        params![name, id],
    )?;
    Ok(())
}

/// Deletes a location by ID.
pub fn delete_location(conn: &Connection, id: i64) -> Result<bool> {
    let affected = conn.execute("DELETE FROM locations WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

/// Returns locker count by location.
pub fn get_location_stats(conn: &Connection) -> Result<Vec<(String, i64, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT 
            l.location,
            COUNT(*) as total,
            COUNT(CASE WHEN l.is_damaged = 0 AND NOT EXISTS (
                SELECT 1 FROM rentals r 
                WHERE r.locker_id = l.id 
                AND date('now') BETWEEN r.start_date AND r.end_date
            ) THEN 1 END) as available
         FROM lockers l
         GROUP BY l.location
         ORDER BY l.location"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;
    
    let mut stats = Vec::new();
    for row in rows {
        stats.push(row?);
    }
    
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use rusqlite::Connection;

    #[test]
    fn test_create_and_get_location() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_location(&conn, "Main Building")?;
        let location = get_location(&conn, id)?.expect("location not found");
        
        assert_eq!(location.name, "Main Building");
        assert!(location.id > 0);

        Ok(())
    }

    #[test]
    fn test_get_location_by_name() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_location(&conn, "Side Building")?;
        let location = get_location_by_name(&conn, "Side Building")?.expect("location not found");
        
        assert_eq!(location.name, "Side Building");

        Ok(())
    }

    #[test]
    fn test_list_locations() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_location(&conn, "Basement")?;
        create_location(&conn, "Attic")?;
        create_location(&conn, "Main Hall")?;

        let locations = list_locations(&conn)?;
        assert_eq!(locations.len(), 3);
        
        // Should be sorted alphabetically
        assert_eq!(locations[0].name, "Attic");
        assert_eq!(locations[1].name, "Basement");
        assert_eq!(locations[2].name, "Main Hall");

        Ok(())
    }

    #[test]
    fn test_update_location() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_location(&conn, "Old Name")?;
        update_location(&conn, id, "New Name")?;
        
        let location = get_location(&conn, id)?.unwrap();
        assert_eq!(location.name, "New Name");

        Ok(())
    }

    #[test]
    fn test_delete_location() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_location(&conn, "Temp")?;
        assert!(delete_location(&conn, id)?);
        assert!(!delete_location(&conn, id)?);
        assert!(get_location(&conn, id)?.is_none());

        Ok(())
    }

    #[test]
    fn test_location_unique_constraint() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_location(&conn, "Duplicate")?;
        let result = create_location(&conn, "Duplicate");
        
        assert!(result.is_err());

        Ok(())
    }
}
