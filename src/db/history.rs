use chrono;
use color_eyre::eyre::Result;
use rusqlite::{Connection, OptionalExtension, params};

/// Represents an occupancy history snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct OccupancySnapshot {
    pub id: i64,
    pub snapshot_date: String,
    pub total_lockers: i64,
    pub occupied_lockers: i64,
    pub occupancy_percent: f64,
    pub notes: Option<String>,
}

/// Creates a new occupancy history snapshot.
pub fn create_snapshot(
    conn: &Connection,
    snapshot_date: &str,
    total_lockers: i64,
    occupied_lockers: i64,
    occupancy_percent: f64,
    notes: Option<&str>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO occupancy_history (snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Creates a snapshot for the current date based on actual data.
pub fn create_current_snapshot(conn: &Connection) -> Result<i64> {
    let total_lockers: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lockers",
        [],
        |row| row.get(0),
    )?;

    let occupied_lockers: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT locker_id) FROM rentals 
         WHERE date('now') BETWEEN start_date AND end_date",
        [],
        |row| row.get(0),
    )?;

    let occupancy_percent = if total_lockers > 0 {
        (occupied_lockers as f64 / total_lockers as f64) * 100.0
    } else {
        0.0
    };

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Use INSERT OR REPLACE to update if snapshot for today already exists
    conn.execute(
        "INSERT OR REPLACE INTO occupancy_history (snapshot_date, total_lockers, occupied_lockers, occupancy_percent) 
         VALUES (?1, ?2, ?3, ?4)",
        params![today, total_lockers, occupied_lockers, occupancy_percent],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Returns a snapshot by date.
pub fn get_snapshot(conn: &Connection, snapshot_date: &str) -> Result<Option<OccupancySnapshot>> {
    let snapshot = conn
        .query_row(
            "SELECT id, snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes 
             FROM occupancy_history 
             WHERE snapshot_date = ?1",
            params![snapshot_date],
            |row| {
                Ok(OccupancySnapshot {
                    id: row.get(0)?,
                    snapshot_date: row.get(1)?,
                    total_lockers: row.get(2)?,
                    occupied_lockers: row.get(3)?,
                    occupancy_percent: row.get(4)?,
                    notes: row.get(5)?,
                })
            },
        )
        .optional()?;
    
    Ok(snapshot)
}

/// Returns all occupancy snapshots.
pub fn list_snapshots(conn: &Connection) -> Result<Vec<OccupancySnapshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes 
         FROM occupancy_history 
         ORDER BY snapshot_date DESC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(OccupancySnapshot {
            id: row.get(0)?,
            snapshot_date: row.get(1)?,
            total_lockers: row.get(2)?,
            occupied_lockers: row.get(3)?,
            occupancy_percent: row.get(4)?,
            notes: row.get(5)?,
        })
    })?;
    
    let mut snapshots = Vec::new();
    for snapshot in rows {
        snapshots.push(snapshot?);
    }
    
    Ok(snapshots)
}

/// Returns snapshots within a date range.
pub fn list_snapshots_in_range(
    conn: &Connection,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<OccupancySnapshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes 
         FROM occupancy_history 
         WHERE snapshot_date BETWEEN ?1 AND ?2
         ORDER BY snapshot_date ASC"
    )?;
    
    let rows = stmt.query_map(params![start_date, end_date], |row| {
        Ok(OccupancySnapshot {
            id: row.get(0)?,
            snapshot_date: row.get(1)?,
            total_lockers: row.get(2)?,
            occupied_lockers: row.get(3)?,
            occupancy_percent: row.get(4)?,
            notes: row.get(5)?,
        })
    })?;
    
    let mut snapshots = Vec::new();
    for snapshot in rows {
        snapshots.push(snapshot?);
    }
    
    Ok(snapshots)
}

/// Returns the last N snapshots.
pub fn list_recent_snapshots(conn: &Connection, count: i64) -> Result<Vec<OccupancySnapshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes 
         FROM occupancy_history 
         ORDER BY snapshot_date DESC
         LIMIT ?1"
    )?;
    
    let rows = stmt.query_map(params![count], |row| {
        Ok(OccupancySnapshot {
            id: row.get(0)?,
            snapshot_date: row.get(1)?,
            total_lockers: row.get(2)?,
            occupied_lockers: row.get(3)?,
            occupancy_percent: row.get(4)?,
            notes: row.get(5)?,
        })
    })?;
    
    let mut snapshots = Vec::new();
    for snapshot in rows {
        snapshots.push(snapshot?);
    }
    
    // Reverse to get chronological order
    snapshots.reverse();
    
    Ok(snapshots)
}

/// Deletes a snapshot by date.
pub fn delete_snapshot(conn: &Connection, snapshot_date: &str) -> Result<bool> {
    let affected = conn.execute(
        "DELETE FROM occupancy_history WHERE snapshot_date = ?1",
        params![snapshot_date],
    )?;
    Ok(affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{lockers, rentals, migrations::run_migrations};
    use rusqlite::Connection;

    #[test]
    fn test_create_and_get_snapshot() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let id = create_snapshot(&conn, "2025-01-15", 100, 75, 75.0, Some("Test snapshot"))?;
        assert!(id > 0);

        let snapshot = get_snapshot(&conn, "2025-01-15")?.expect("snapshot not found");
        assert_eq!(snapshot.total_lockers, 100);
        assert_eq!(snapshot.occupied_lockers, 75);
        assert_eq!(snapshot.occupancy_percent, 75.0);

        Ok(())
    }

    #[test]
    fn test_create_current_snapshot() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        // Create test data
        let l1 = lockers::create_locker(&conn, "A-001", "Main", "Small")?;
        let l2 = lockers::create_locker(&conn, "A-002", "Main", "Small")?;
        lockers::create_locker(&conn, "A-003", "Main", "Small")?;

        // Create rentals for 2 lockers (dates that include 2026-01-19)
        rentals::create_rental(&conn, l1, "John", None, None, "2026-01-01", "2026-12-31")?;
        rentals::create_rental(&conn, l2, "Jane", None, None, "2026-01-01", "2026-12-31")?;

        let id = create_current_snapshot(&conn)?;
        assert!(id > 0);

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let snapshot = get_snapshot(&conn, &today)?.unwrap();
        
        assert_eq!(snapshot.total_lockers, 3);
        assert_eq!(snapshot.occupied_lockers, 2);
        assert!((snapshot.occupancy_percent - 66.666).abs() < 0.1);

        Ok(())
    }

    #[test]
    fn test_list_snapshots_in_range() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_snapshot(&conn, "2025-01-01", 100, 50, 50.0, None)?;
        create_snapshot(&conn, "2025-01-15", 100, 60, 60.0, None)?;
        create_snapshot(&conn, "2025-02-01", 100, 70, 70.0, None)?;
        create_snapshot(&conn, "2025-02-15", 100, 80, 80.0, None)?;

        let snapshots = list_snapshots_in_range(&conn, "2025-01-10", "2025-02-10")?;
        assert_eq!(snapshots.len(), 2);
        assert_eq!(snapshots[0].snapshot_date, "2025-01-15");
        assert_eq!(snapshots[1].snapshot_date, "2025-02-01");

        Ok(())
    }

    #[test]
    fn test_list_recent_snapshots() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_snapshot(&conn, "2025-01-01", 100, 50, 50.0, None)?;
        create_snapshot(&conn, "2025-01-02", 100, 60, 60.0, None)?;
        create_snapshot(&conn, "2025-01-03", 100, 70, 70.0, None)?;
        create_snapshot(&conn, "2025-01-04", 100, 80, 80.0, None)?;

        let snapshots = list_recent_snapshots(&conn, 2)?;
        assert_eq!(snapshots.len(), 2);
        
        // Should be in chronological order (oldest to newest)
        assert_eq!(snapshots[0].snapshot_date, "2025-01-03");
        assert_eq!(snapshots[1].snapshot_date, "2025-01-04");

        Ok(())
    }

    #[test]
    fn test_delete_snapshot() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_snapshot(&conn, "2025-01-01", 100, 50, 50.0, None)?;

        assert!(delete_snapshot(&conn, "2025-01-01")?);
        assert!(!delete_snapshot(&conn, "2025-01-01")?);
        assert!(get_snapshot(&conn, "2025-01-01")?.is_none());

        Ok(())
    }

    #[test]
    fn test_unique_snapshot_date() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        create_snapshot(&conn, "2025-01-01", 100, 50, 50.0, None)?;
        let result = create_snapshot(&conn, "2025-01-01", 100, 60, 60.0, None);

        assert!(result.is_err());

        Ok(())
    }
}
