use color_eyre::eyre::Result;
use rusqlite::{Connection, OptionalExtension, params};

/// Represents a locker in the v2.1 schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locker {
    pub id: i64,
    pub number: String,
    pub location: String,
    pub size: String,
    pub is_damaged: bool,
    pub notes: Option<String>,
    pub created_at: String,
}

impl Locker {
    pub fn new(number: String, location: String, size: String) -> Self {
        Self {
            id: 0,
            number,
            location,
            size,
            is_damaged: false,
            notes: None,
            created_at: String::new(),
        }
    }
}

/// Inserts a new locker into the database.
pub fn create_locker(
    conn: &Connection,
    number: &str,
    location: &str,
    size: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO lockers (number, location, size) VALUES (?1, ?2, ?3)",
        params![number, location, size],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Updates an existing locker.
pub fn update_locker(conn: &Connection, locker: &Locker) -> Result<()> {
    conn.execute(
        "UPDATE lockers SET number = ?1, location = ?2, size = ?3, is_damaged = ?4, notes = ?5 WHERE id = ?6",
        params![
            &locker.number,
            &locker.location,
            &locker.size,
            locker.is_damaged as i64,
            &locker.notes,
            locker.id
        ],
    )?;
    Ok(())
}

/// Returns all lockers.
pub fn list_lockers(conn: &Connection) -> Result<Vec<Locker>> {
    let mut stmt = conn.prepare(
        "SELECT id, number, location, size, is_damaged, notes, created_at 
         FROM lockers 
         ORDER BY number ASC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(Locker {
            id: row.get(0)?,
            number: row.get(1)?,
            location: row.get(2)?,
            size: row.get(3)?,
            is_damaged: row.get::<_, i64>(4)? != 0,
            notes: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    
    let mut lockers = Vec::new();
    for locker in rows {
        lockers.push(locker?);
    }
    
    Ok(lockers)
}

/// Returns a single locker by ID.
pub fn get_locker(conn: &Connection, id: i64) -> Result<Option<Locker>> {
    let locker = conn
        .query_row(
            "SELECT id, number, location, size, is_damaged, notes, created_at 
             FROM lockers 
             WHERE id = ?1",
            params![id],
            |row| {
                Ok(Locker {
                    id: row.get(0)?,
                    number: row.get(1)?,
                    location: row.get(2)?,
                    size: row.get(3)?,
                    is_damaged: row.get::<_, i64>(4)? != 0,
                    notes: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .optional()?;
    
    Ok(locker)
}

/// Returns a locker by number.
pub fn get_locker_by_number(conn: &Connection, number: &str) -> Result<Option<Locker>> {
    let locker = conn
        .query_row(
            "SELECT id, number, location, size, is_damaged, notes, created_at 
             FROM lockers 
             WHERE number = ?1",
            params![number],
            |row| {
                Ok(Locker {
                    id: row.get(0)?,
                    number: row.get(1)?,
                    location: row.get(2)?,
                    size: row.get(3)?,
                    is_damaged: row.get::<_, i64>(4)? != 0,
                    notes: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .optional()?;
    
    Ok(locker)
}

/// Deletes a locker by ID.
pub fn delete_locker(conn: &Connection, id: i64) -> Result<bool> {
    let affected = conn.execute("DELETE FROM lockers WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

/// Marks a locker as damaged.
pub fn mark_locker_damaged(conn: &Connection, id: i64, notes: Option<&str>) -> Result<()> {
    conn.execute(
        "UPDATE lockers SET is_damaged = 1, notes = ?1 WHERE id = ?2",
        params![notes, id],
    )?;
    Ok(())
}

/// Marks a locker as repaired.
pub fn mark_locker_repaired(conn: &Connection, id: i64) -> Result<()> {
    conn.execute(
        "UPDATE lockers SET is_damaged = 0, notes = NULL WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

/// Returns the count of lockers.
pub fn count_lockers(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;
    Ok(count)
}

/// Returns the count of available lockers.
pub fn count_available_lockers(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lockers 
         WHERE is_damaged = 0 
         AND id NOT IN (
             SELECT locker_id FROM rentals 
             WHERE date('now') BETWEEN start_date AND end_date
         )",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

/// Returns the count of damaged lockers.
pub fn count_damaged_lockers(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lockers WHERE is_damaged = 1",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use rusqlite::Connection;

    #[test]
    fn test_create_and_get_locker() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_locker(&conn, "A-001", "Main Building", "Small")?;
        assert!(id > 0);

        let locker = get_locker(&conn, id)?.expect("locker not found");
        assert_eq!(locker.number, "A-001");
        assert_eq!(locker.location, "Main Building");
        assert_eq!(locker.size, "Small");
        assert!(!locker.is_damaged);

        Ok(())
    }

    #[test]
    fn test_get_locker_by_number() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_locker(&conn, "B-042", "Side Building", "Large")?;

        let locker = get_locker_by_number(&conn, "B-042")?.expect("locker not found");
        assert_eq!(locker.number, "B-042");
        assert_eq!(locker.location, "Side Building");

        Ok(())
    }

    #[test]
    fn test_update_locker() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_locker(&conn, "C-003", "Basement", "Medium")?;
        let mut locker = get_locker(&conn, id)?.unwrap();
        
        locker.location = "Attic".to_string();
        locker.notes = Some("Moved".to_string());
        update_locker(&conn, &locker)?;

        let updated = get_locker(&conn, id)?.unwrap();
        assert_eq!(updated.location, "Attic");
        assert_eq!(updated.notes, Some("Moved".to_string()));

        Ok(())
    }

    #[test]
    fn test_mark_damaged_and_repaired() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_locker(&conn, "D-010", "Main", "Small")?;
        
        mark_locker_damaged(&conn, id, Some("Door broken"))?;
        let locker = get_locker(&conn, id)?.unwrap();
        assert!(locker.is_damaged);
        assert_eq!(locker.notes, Some("Door broken".to_string()));

        mark_locker_repaired(&conn, id)?;
        let locker = get_locker(&conn, id)?.unwrap();
        assert!(!locker.is_damaged);
        assert_eq!(locker.notes, None);

        Ok(())
    }

    #[test]
    fn test_delete_locker() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_locker(&conn, "E-001", "Main", "Small")?;
        assert!(delete_locker(&conn, id)?);
        assert!(!delete_locker(&conn, id)?);
        assert!(get_locker(&conn, id)?.is_none());

        Ok(())
    }

    #[test]
    fn test_list_lockers() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_locker(&conn, "Z-001", "Main", "Small")?;
        create_locker(&conn, "A-001", "Main", "Large")?;
        create_locker(&conn, "M-001", "Side", "Medium")?;

        let lockers = list_lockers(&conn)?;
        assert_eq!(lockers.len(), 3);
        
        // Should be sorted by number
        assert_eq!(lockers[0].number, "A-001");
        assert_eq!(lockers[1].number, "M-001");
        assert_eq!(lockers[2].number, "Z-001");

        Ok(())
    }

    #[test]
    fn test_count_functions() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_locker(&conn, "A-001", "Main", "Small")?;
        let id2 = create_locker(&conn, "A-002", "Main", "Small")?;
        create_locker(&conn, "A-003", "Main", "Small")?;

        assert_eq!(count_lockers(&conn)?, 3);
        
        mark_locker_damaged(&conn, id2, Some("Broken"))?;
        assert_eq!(count_damaged_lockers(&conn)?, 1);

        Ok(())
    }
}
