use crate::models::{Payment, PaymentType};
use chrono::NaiveDate;
use color_eyre::eyre::Result;
use rusqlite::{params, Connection, OptionalExtension, Row};

/// Creates a new payment record in the database.
pub fn create_payment(conn: &Connection, payment: &Payment) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO payments (rental_id, amount_cents, payment_type, payment_date, notes)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![
            payment.rental_id,
            payment.amount_cents,
            payment.payment_type.as_str(),
            payment.payment_date.to_string(),
            payment.notes.as_ref(),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Returns a single payment by its id.
pub fn get_payment(conn: &Connection, id: i64) -> Result<Option<Payment>> {
    let payment = conn
        .query_row(
            r#"
            SELECT id, rental_id, amount_cents, payment_type, payment_date, notes
            FROM payments WHERE id = ?1
            "#,
            params![id],
            row_to_payment,
        )
        .optional()?;
    Ok(payment)
}

/// Returns all payments for a rental.
pub fn get_payments_for_rental(conn: &Connection, rental_id: i64) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, rental_id, amount_cents, payment_type, payment_date, notes
        FROM payments WHERE rental_id = ?1 ORDER BY payment_date DESC
        "#,
    )?;
    let rows = stmt.query_map(params![rental_id], row_to_payment)?;
    let mut payments = Vec::new();
    for payment in rows {
        payments.push(payment?);
    }
    Ok(payments)
}

/// Returns all payments sorted by date.
pub fn list_payments(conn: &Connection) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, rental_id, amount_cents, payment_type, payment_date, notes
        FROM payments ORDER BY payment_date DESC
        "#,
    )?;
    let rows = stmt.query_map([], row_to_payment)?;
    let mut payments = Vec::new();
    for payment in rows {
        payments.push(payment?);
    }
    Ok(payments)
}

/// Returns all payments within a date range.
pub fn list_payments_in_range(
    conn: &Connection,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, rental_id, amount_cents, payment_type, payment_date, notes
        FROM payments WHERE payment_date >= ?1 AND payment_date <= ?2
        ORDER BY payment_date DESC
        "#,
    )?;
    let rows = stmt.query_map(
        params![start_date.to_string(), end_date.to_string()],
        row_to_payment,
    )?;
    let mut payments = Vec::new();
    for payment in rows {
        payments.push(payment?);
    }
    Ok(payments)
}

/// Deletes a payment by id.
pub fn delete_payment(conn: &Connection, id: i64) -> Result<bool> {
    let affected = conn.execute("DELETE FROM payments WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

fn row_to_payment(row: &Row) -> rusqlite::Result<Payment> {
    let payment_type_str: String = row.get("payment_type")?;
    let payment_type = PaymentType::from_str(&payment_type_str).unwrap_or(PaymentType::Extension);

    let payment_date_str: String = row.get("payment_date")?;
    let payment_date = NaiveDate::parse_from_str(&payment_date_str, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::Utc::now().date_naive());

    Ok(Payment {
        id: row.get("id")?,
        rental_id: row.get("rental_id")?,
        amount_cents: row.get("amount_cents")?,
        payment_type,
        payment_date,
        notes: row.get("notes")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{lockers, migrations::apply_migrations, rentals};
    use crate::models::{Locker, Rental, TenantType};
    use chrono::{Duration, Utc};

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        conn
    }

    fn create_test_rental(conn: &Connection) -> i64 {
        let locker_id = lockers::create_locker(conn, &Locker::new("A-01", "Test", 100)).unwrap();
        let today = Utc::now().date_naive();
        let rental = Rental::new(
            locker_id,
            "test.user",
            TenantType::Schüler,
            today,
            today + Duration::days(365),
        );
        rentals::create_rental(conn, &rental).unwrap()
    }

    #[test]
    fn test_create_and_get_payment() -> Result<()> {
        let conn = setup_db();
        let rental_id = create_test_rental(&conn);

        let payment = Payment::new(
            rental_id,
            1000,
            PaymentType::Deposit,
            Utc::now().date_naive(),
        );
        let id = create_payment(&conn, &payment)?;

        let fetched = get_payment(&conn, id)?.unwrap();
        assert_eq!(fetched.rental_id, rental_id);
        assert_eq!(fetched.amount_cents, 1000);
        assert_eq!(fetched.payment_type, PaymentType::Deposit);

        Ok(())
    }

    #[test]
    fn test_get_payments_for_rental() -> Result<()> {
        let conn = setup_db();
        let rental_id = create_test_rental(&conn);

        let today = Utc::now().date_naive();
        create_payment(
            &conn,
            &Payment::new(rental_id, 1000, PaymentType::Deposit, today),
        )?;
        create_payment(
            &conn,
            &Payment::new(rental_id, 1000, PaymentType::Extension, today),
        )?;

        let payments = get_payments_for_rental(&conn, rental_id)?;
        assert_eq!(payments.len(), 2);

        Ok(())
    }

    #[test]
    fn test_list_payments_in_range() -> Result<()> {
        let conn = setup_db();
        let rental_id = create_test_rental(&conn);

        let today = Utc::now().date_naive();
        create_payment(
            &conn,
            &Payment::new(rental_id, 1000, PaymentType::Deposit, today),
        )?;
        create_payment(
            &conn,
            &Payment::new(
                rental_id,
                1000,
                PaymentType::Extension,
                today - Duration::days(60),
            ),
        )?;

        let payments = list_payments_in_range(&conn, today - Duration::days(30), today)?;
        assert_eq!(payments.len(), 1);

        Ok(())
    }
}
