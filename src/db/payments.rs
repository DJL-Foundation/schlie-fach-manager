use crate::model::{Payment, PaymentType};
use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{Connection, Row, params};

/// Inserts a new payment record and returns its id.
pub fn create_payment(conn: &Connection, payment: &Payment) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO payments (
            rental_id, amount_cents, payment_date, payment_type, notes, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![
            payment.rental_id,
            payment.amount_cents,
            payment.payment_date.to_string(),
            payment.payment_type.as_str(),
            payment.notes,
            payment.created_at.to_rfc3339(),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Inserts or updates a payment record using its id.
pub fn upsert_payment(conn: &Connection, payment: &Payment) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO payments (
            id, rental_id, amount_cents, payment_date, payment_type, notes, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ON CONFLICT(id) DO UPDATE SET
            rental_id = excluded.rental_id,
            amount_cents = excluded.amount_cents,
            payment_date = excluded.payment_date,
            payment_type = excluded.payment_type,
            notes = excluded.notes
        "#,
        params![
            payment.id,
            payment.rental_id,
            payment.amount_cents,
            payment.payment_date.to_string(),
            payment.payment_type.as_str(),
            payment.notes,
            payment.created_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// Returns all payments ordered by date.
pub fn list_payments(conn: &Connection) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, rental_id, amount_cents, payment_date, payment_type, notes, created_at
        FROM payments
        ORDER BY payment_date DESC
        "#,
    )?;
    let rows = stmt.query_map([], row_to_payment)?;
    let mut payments = Vec::new();
    for payment in rows {
        payments.push(payment?);
    }
    Ok(payments)
}

/// Maps a database row to a payment domain model.
fn row_to_payment(row: &Row<'_>) -> rusqlite::Result<Payment> {
    let payment_date: String = row.get("payment_date")?;
    let created_at: String = row.get("created_at")?;
    Ok(Payment {
        id: row.get("id")?,
        rental_id: row.get("rental_id")?,
        amount_cents: row.get("amount_cents")?,
        payment_date: parse_date(&payment_date),
        payment_type: PaymentType::from_str(&row.get::<_, String>("payment_type")?),
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
