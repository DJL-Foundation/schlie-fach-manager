use crate::db::rentals;
use anyhow::Result;
use chrono::{Datelike, Duration, Months, NaiveDate, Utc};
use rusqlite::{Connection, Row};

/// Returns occupancy history for the last 12 months.
pub fn get_occupancy_history(
    conn: &Connection,
    total_lockers: usize,
) -> Result<Vec<(String, f64)>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT snapshot_date, occupancy_percent
        FROM occupancy_history
        ORDER BY snapshot_date DESC
        LIMIT 12
        "#,
    )?;
    let history_rows = stmt.query_map([], row_to_history)?;
    let mut history = Vec::new();
    for row in history_rows {
        history.push(row?);
    }
    if !history.is_empty() {
        history.reverse();
        return Ok(history);
    }

    let rentals = rentals::list_rentals(conn)?;
    let today = Utc::now().date_naive();
    let this_month_start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap_or(today);
    let mut results = Vec::with_capacity(12);
    let total = total_lockers.max(1) as f64;

    for offset in (0..12).rev() {
        let month_start = this_month_start - Months::new(offset as u32);
        let month_end = month_start
            .checked_add_months(Months::new(1))
            .unwrap_or(month_start)
            - Duration::days(1);
        let occupied = rentals
            .iter()
            .filter(|rental| rental.start_date <= month_end && rental.end_date >= month_start)
            .count() as f64;
        let percent = (occupied / total) * 100.0;
        results.push((month_start.format("%Y-%m").to_string(), percent));
    }

    Ok(results)
}

/// Maps a history row to a (date, percent) tuple.
fn row_to_history(row: &Row<'_>) -> rusqlite::Result<(String, f64)> {
    let date: String = row.get("snapshot_date")?;
    let percent: f64 = row.get("occupancy_percent")?;
    Ok((date, percent))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::Database;

    #[test]
    fn history_fallback_returns_12_months() -> Result<()> {
        let db = Database::open_in_memory()?;
        let history = get_occupancy_history(db.connection(), 5)?;
        assert_eq!(history.len(), 12);
        Ok(())
    }
}
