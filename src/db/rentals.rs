use crate::model::Rental;
use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

/// Inserts a new rental record and returns its id.
pub fn create_rental(conn: &Connection, rental: &Rental) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO rentals (
            locker_id, renter_name, renter_email, renter_phone,
            start_date, end_date, deposit_paid, deposit_returned, notes, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#,
        params![
            rental.locker_id,
            rental.renter_name,
            rental.renter_email,
            rental.renter_phone,
            rental.start_date.to_string(),
            rental.end_date.to_string(),
            rental.deposit_paid as i64,
            rental.deposit_returned as i64,
            rental.notes,
            rental.created_at.to_rfc3339(),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Inserts or updates a rental record using its id.
pub fn upsert_rental(conn: &Connection, rental: &Rental) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO rentals (
            id, locker_id, renter_name, renter_email, renter_phone,
            start_date, end_date, deposit_paid, deposit_returned, notes, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        ON CONFLICT(id) DO UPDATE SET
            locker_id = excluded.locker_id,
            renter_name = excluded.renter_name,
            renter_email = excluded.renter_email,
            renter_phone = excluded.renter_phone,
            start_date = excluded.start_date,
            end_date = excluded.end_date,
            deposit_paid = excluded.deposit_paid,
            deposit_returned = excluded.deposit_returned,
            notes = excluded.notes
        "#,
        params![
            rental.id,
            rental.locker_id,
            rental.renter_name,
            rental.renter_email,
            rental.renter_phone,
            rental.start_date.to_string(),
            rental.end_date.to_string(),
            rental.deposit_paid as i64,
            rental.deposit_returned as i64,
            rental.notes,
            rental.created_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// Updates an existing rental record.
pub fn update_rental(conn: &Connection, rental: &Rental) -> Result<()> {
    conn.execute(
        r#"
        UPDATE rentals SET
            locker_id = ?1,
            renter_name = ?2,
            renter_email = ?3,
            renter_phone = ?4,
            start_date = ?5,
            end_date = ?6,
            deposit_paid = ?7,
            deposit_returned = ?8,
            notes = ?9
        WHERE id = ?10
        "#,
        params![
            rental.locker_id,
            rental.renter_name,
            rental.renter_email,
            rental.renter_phone,
            rental.start_date.to_string(),
            rental.end_date.to_string(),
            rental.deposit_paid as i64,
            rental.deposit_returned as i64,
            rental.notes,
            rental.id,
        ],
    )?;
    Ok(())
}

/// Returns all rentals sorted by end date.
pub fn list_rentals(conn: &Connection) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, locker_id, renter_name, renter_email, renter_phone,
               start_date, end_date, deposit_paid, deposit_returned, notes, created_at
        FROM rentals
        ORDER BY end_date ASC
        "#,
    )?;
    let rows = stmt.query_map([], row_to_rental)?;
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    Ok(rentals)
}

/// Returns a single rental by its id.
pub fn get_rental(conn: &Connection, id: i64) -> Result<Option<Rental>> {
    let rental = conn
        .query_row(
            r#"
            SELECT id, locker_id, renter_name, renter_email, renter_phone,
                   start_date, end_date, deposit_paid, deposit_returned, notes, created_at
            FROM rentals WHERE id = ?1
            "#,
            params![id],
            row_to_rental,
        )
        .optional()?;
    Ok(rental)
}

/// Maps a database row to a rental domain model.
fn row_to_rental(row: &Row<'_>) -> rusqlite::Result<Rental> {
    let start_date: String = row.get("start_date")?;
    let end_date: String = row.get("end_date")?;
    let created_at: String = row.get("created_at")?;
    Ok(Rental {
        id: row.get("id")?,
        locker_id: row.get("locker_id")?,
        renter_name: row.get("renter_name")?,
        renter_email: row.get("renter_email")?,
        renter_phone: row.get("renter_phone")?,
        start_date: parse_date(&start_date),
        end_date: parse_date(&end_date),
        deposit_paid: row.get::<_, i64>("deposit_paid")? != 0,
        deposit_returned: row.get::<_, i64>("deposit_returned")? != 0,
        notes: row.get("notes")?,
        created_at: parse_datetime(&created_at),
    })
}

/// Parses a YYYY-MM-DD date or returns today's date.
fn parse_date(value: &str) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive())
}

/// Parses a RFC3339 timestamp or returns the current time.
fn parse_datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
