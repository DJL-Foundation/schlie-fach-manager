//! Occupancy history tracking for the Schließfach-Manager system.
//!
//! This module provides functions to record and retrieve occupancy snapshots
//! for analyzing locker usage over time.

use chrono::{NaiveDate, Utc};
use color_eyre::eyre::Result;
use rusqlite::{Connection, params, Row};

/// Represents an occupancy snapshot at a point in time.
#[derive(Debug, Clone)]
pub struct OccupancySnapshot {
    /// Unique identifier for the snapshot.
    pub id: i64,
    /// Date of the snapshot.
    pub snapshot_date: NaiveDate,
    /// Total number of lockers at snapshot time.
    pub total_lockers: i32,
    /// Number of occupied lockers.
    pub occupied_lockers: i32,
    /// Occupancy percentage (0.0 - 100.0).
    pub occupancy_percent: f64,
    /// Optional notes about the snapshot.
    pub notes: Option<String>,
}

/// Records a snapshot of current occupancy.
pub fn record_occupancy_snapshot(conn: &Connection) -> Result<()> {
    let today = Utc::now().format("%Y-%m-%d").to_string();

    // Get total lockers
    let total_lockers: i32 =
        conn.query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;

    // Get occupied lockers (those with active rentals)
    let occupied_lockers: i32 = conn.query_row(
        "SELECT COUNT(DISTINCT locker_id) FROM rentals WHERE returned_at IS NULL",
        [],
        |row| row.get(0),
    )?;

    let occupancy_percent = if total_lockers > 0 {
        (occupied_lockers as f64 / total_lockers as f64) * 100.0
    } else {
        0.0
    };

    // Insert or replace (UNIQUE constraint on snapshot_date)
    conn.execute(
        r#"
        INSERT OR REPLACE INTO occupancy_history 
            (snapshot_date, total_lockers, occupied_lockers, occupancy_percent)
        VALUES (?1, ?2, ?3, ?4)
        "#,
        params![today, total_lockers, occupied_lockers, occupancy_percent],
    )?;

    Ok(())
}

/// Records a snapshot with custom values and optional notes.
pub fn record_occupancy_snapshot_with_notes(
    conn: &Connection,
    notes: Option<&str>,
) -> Result<()> {
    let today = Utc::now().format("%Y-%m-%d").to_string();

    let total_lockers: i32 =
        conn.query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;

    let occupied_lockers: i32 = conn.query_row(
        "SELECT COUNT(DISTINCT locker_id) FROM rentals WHERE returned_at IS NULL",
        [],
        |row| row.get(0),
    )?;

    let occupancy_percent = if total_lockers > 0 {
        (occupied_lockers as f64 / total_lockers as f64) * 100.0
    } else {
        0.0
    };

    conn.execute(
        r#"
        INSERT OR REPLACE INTO occupancy_history 
            (snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![today, total_lockers, occupied_lockers, occupancy_percent, notes],
    )?;

    Ok(())
}

/// Retrieves occupancy history for the specified number of months.
pub fn get_occupancy_history(conn: &Connection, months: u32) -> Result<Vec<OccupancySnapshot>> {
    let cutoff_date = Utc::now()
        .checked_sub_months(chrono::Months::new(months))
        .unwrap_or(Utc::now())
        .format("%Y-%m-%d")
        .to_string();

    let mut stmt = conn.prepare(
        r#"
        SELECT id, snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes
        FROM occupancy_history
        WHERE snapshot_date >= ?1
        ORDER BY snapshot_date ASC
        "#,
    )?;

    let rows = stmt.query_map(params![cutoff_date], row_to_snapshot)?;

    let mut snapshots = Vec::new();
    for snapshot in rows {
        snapshots.push(snapshot?);
    }

    Ok(snapshots)
}

/// Retrieves all occupancy history records.
pub fn get_all_occupancy_history(conn: &Connection) -> Result<Vec<OccupancySnapshot>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes
        FROM occupancy_history
        ORDER BY snapshot_date ASC
        "#,
    )?;

    let rows = stmt.query_map([], row_to_snapshot)?;

    let mut snapshots = Vec::new();
    for snapshot in rows {
        snapshots.push(snapshot?);
    }

    Ok(snapshots)
}

/// Gets the most recent occupancy snapshot.
pub fn get_latest_occupancy(conn: &Connection) -> Result<Option<OccupancySnapshot>> {
    let snapshot = conn
        .query_row(
            r#"
            SELECT id, snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes
            FROM occupancy_history
            ORDER BY snapshot_date DESC
            LIMIT 1
            "#,
            [],
            row_to_snapshot,
        )
        .ok();

    Ok(snapshot)
}

/// Deletes occupancy records older than the specified number of months.
pub fn cleanup_old_history(conn: &Connection, months_to_keep: u32) -> Result<u64> {
    let cutoff_date = Utc::now()
        .checked_sub_months(chrono::Months::new(months_to_keep))
        .unwrap_or(Utc::now())
        .format("%Y-%m-%d")
        .to_string();

    let deleted = conn.execute(
        "DELETE FROM occupancy_history WHERE snapshot_date < ?1",
        params![cutoff_date],
    )?;

    Ok(deleted as u64)
}

fn row_to_snapshot(row: &Row) -> rusqlite::Result<OccupancySnapshot> {
    let date_str: String = row.get("snapshot_date")?;
    let snapshot_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .unwrap_or_else(|_| Utc::now().date_naive());

    Ok(OccupancySnapshot {
        id: row.get("id")?,
        snapshot_date,
        total_lockers: row.get("total_lockers")?,
        occupied_lockers: row.get("occupied_lockers")?,
        occupancy_percent: row.get("occupancy_percent")?,
        notes: row.get("notes")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::apply_migrations;
    use crate::db::lockers;
    use crate::models::Locker;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_record_occupancy_snapshot_empty() -> Result<()> {
        let conn = setup_db();
        record_occupancy_snapshot(&conn)?;

        let history = get_all_occupancy_history(&conn)?;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].total_lockers, 0);
        assert_eq!(history[0].occupied_lockers, 0);
        assert_eq!(history[0].occupancy_percent, 0.0);
        Ok(())
    }

    #[test]
    fn test_record_occupancy_with_lockers() -> Result<()> {
        let conn = setup_db();

        // Add some lockers
        lockers::create_locker(&conn, &Locker::new("A-01", "Test", 100))?;
        lockers::create_locker(&conn, &Locker::new("A-02", "Test", 100))?;
        lockers::create_locker(&conn, &Locker::new("A-03", "Test", 100))?;

        record_occupancy_snapshot(&conn)?;

        let history = get_all_occupancy_history(&conn)?;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].total_lockers, 3);
        assert_eq!(history[0].occupied_lockers, 0);
        assert_eq!(history[0].occupancy_percent, 0.0);
        Ok(())
    }

    #[test]
    fn test_record_with_notes() -> Result<()> {
        let conn = setup_db();
        record_occupancy_snapshot_with_notes(&conn, Some("Manual snapshot"))?;

        let latest = get_latest_occupancy(&conn)?;
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().notes, Some("Manual snapshot".to_string()));
        Ok(())
    }

    #[test]
    fn test_get_occupancy_history() -> Result<()> {
        let conn = setup_db();
        record_occupancy_snapshot(&conn)?;

        let history = get_occupancy_history(&conn, 12)?;
        assert_eq!(history.len(), 1);
        Ok(())
    }

    #[test]
    fn test_snapshot_replaces_same_day() -> Result<()> {
        let conn = setup_db();

        record_occupancy_snapshot(&conn)?;
        record_occupancy_snapshot(&conn)?;

        let history = get_all_occupancy_history(&conn)?;
        // Should only have one entry for today
        assert_eq!(history.len(), 1);
        Ok(())
    }

    #[test]
    fn test_get_latest_occupancy_empty() -> Result<()> {
        let conn = setup_db();
        let latest = get_latest_occupancy(&conn)?;
        assert!(latest.is_none());
        Ok(())
    }
}
