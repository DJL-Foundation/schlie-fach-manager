use color_eyre::eyre::Result;
use rusqlite::{Connection, OptionalExtension, params};

/// Represents a payment in the v2.1 schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    pub id: i64,
    pub rental_id: i64,
    pub amount_cents: i64,
    pub payment_date: String,
    pub payment_type: String,
    pub notes: Option<String>,
    pub created_at: String,
}

/// Creates a new payment.
pub fn create_payment(
    conn: &Connection,
    rental_id: i64,
    amount_cents: i64,
    payment_date: &str,
    payment_type: &str,
    notes: Option<&str>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO payments (rental_id, amount_cents, payment_date, payment_type, notes) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![rental_id, amount_cents, payment_date, payment_type, notes],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Returns a payment by ID.
pub fn get_payment(conn: &Connection, id: i64) -> Result<Option<Payment>> {
    let payment = conn
        .query_row(
            "SELECT id, rental_id, amount_cents, payment_date, payment_type, notes, created_at 
             FROM payments 
             WHERE id = ?1",
            params![id],
            |row| {
                Ok(Payment {
                    id: row.get(0)?,
                    rental_id: row.get(1)?,
                    amount_cents: row.get(2)?,
                    payment_date: row.get(3)?,
                    payment_type: row.get(4)?,
                    notes: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .optional()?;
    
    Ok(payment)
}

/// Returns all payments.
pub fn list_payments(conn: &Connection) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        "SELECT id, rental_id, amount_cents, payment_date, payment_type, notes, created_at 
         FROM payments 
         ORDER BY payment_date DESC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(Payment {
            id: row.get(0)?,
            rental_id: row.get(1)?,
            amount_cents: row.get(2)?,
            payment_date: row.get(3)?,
            payment_type: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    
    let mut payments = Vec::new();
    for payment in rows {
        payments.push(payment?);
    }
    
    Ok(payments)
}

/// Returns payments for a specific rental.
pub fn list_payments_for_rental(conn: &Connection, rental_id: i64) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        "SELECT id, rental_id, amount_cents, payment_date, payment_type, notes, created_at 
         FROM payments 
         WHERE rental_id = ?1
         ORDER BY payment_date DESC"
    )?;
    
    let rows = stmt.query_map(params![rental_id], |row| {
        Ok(Payment {
            id: row.get(0)?,
            rental_id: row.get(1)?,
            amount_cents: row.get(2)?,
            payment_date: row.get(3)?,
            payment_type: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    
    let mut payments = Vec::new();
    for payment in rows {
        payments.push(payment?);
    }
    
    Ok(payments)
}

/// Returns payments within a date range.
pub fn list_payments_in_range(conn: &Connection, start_date: &str, end_date: &str) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        "SELECT id, rental_id, amount_cents, payment_date, payment_type, notes, created_at 
         FROM payments 
         WHERE payment_date BETWEEN ?1 AND ?2
         ORDER BY payment_date DESC"
    )?;
    
    let rows = stmt.query_map(params![start_date, end_date], |row| {
        Ok(Payment {
            id: row.get(0)?,
            rental_id: row.get(1)?,
            amount_cents: row.get(2)?,
            payment_date: row.get(3)?,
            payment_type: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    
    let mut payments = Vec::new();
    for payment in rows {
        payments.push(payment?);
    }
    
    Ok(payments)
}

/// Returns the total amount of payments within a date range.
pub fn sum_payments_in_range(conn: &Connection, start_date: &str, end_date: &str) -> Result<i64> {
    let sum: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount_cents), 0) FROM payments 
             WHERE payment_date BETWEEN ?1 AND ?2",
            params![start_date, end_date],
            |row| row.get(0),
        )?;
    Ok(sum)
}

/// Returns the total amount of payments for a specific rental.
pub fn sum_payments_for_rental(conn: &Connection, rental_id: i64) -> Result<i64> {
    let sum: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount_cents), 0) FROM payments WHERE rental_id = ?1",
            params![rental_id],
            |row| row.get(0),
        )?;
    Ok(sum)
}

/// Deletes a payment by ID.
pub fn delete_payment(conn: &Connection, id: i64) -> Result<bool> {
    let affected = conn.execute("DELETE FROM payments WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{lockers, rentals, migrations::run_migrations};
    use rusqlite::Connection;

    #[test]
    fn test_create_and_get_payment() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "A-001", "Main", "Small")?;
        let rental_id = rentals::create_rental(
            &conn, locker_id, "John Doe", None, None, "2025-01-01", "2025-12-31"
        )?;

        let payment_id = create_payment(
            &conn,
            rental_id,
            1000,
            "2025-01-15",
            "deposit",
            Some("Initial deposit"),
        )?;

        let payment = get_payment(&conn, payment_id)?.expect("payment not found");
        assert_eq!(payment.rental_id, rental_id);
        assert_eq!(payment.amount_cents, 1000);
        assert_eq!(payment.payment_date, "2025-01-15");
        assert_eq!(payment.payment_type, "deposit");
        assert_eq!(payment.notes, Some("Initial deposit".to_string()));

        Ok(())
    }

    #[test]
    fn test_list_payments_for_rental() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "B-001", "Main", "Small")?;
        let rental_id = rentals::create_rental(
            &conn, locker_id, "Jane Smith", None, None, "2025-01-01", "2025-12-31"
        )?;

        create_payment(&conn, rental_id, 1000, "2025-01-15", "deposit", None)?;
        create_payment(&conn, rental_id, 1000, "2025-06-01", "yearly_fee", None)?;

        let payments = list_payments_for_rental(&conn, rental_id)?;
        assert_eq!(payments.len(), 2);
        
        // Should be sorted by date DESC
        assert_eq!(payments[0].payment_date, "2025-06-01");
        assert_eq!(payments[1].payment_date, "2025-01-15");

        Ok(())
    }

    #[test]
    fn test_sum_payments() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "C-001", "Main", "Small")?;
        let rental_id = rentals::create_rental(
            &conn, locker_id, "Bob", None, None, "2025-01-01", "2025-12-31"
        )?;

        create_payment(&conn, rental_id, 1000, "2025-01-15", "deposit", None)?;
        create_payment(&conn, rental_id, 2400, "2025-06-01", "yearly_fee", None)?;

        let sum = sum_payments_for_rental(&conn, rental_id)?;
        assert_eq!(sum, 3400);

        Ok(())
    }

    #[test]
    fn test_payments_in_range() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "D-001", "Main", "Small")?;
        let rental_id = rentals::create_rental(
            &conn, locker_id, "Alice", None, None, "2025-01-01", "2025-12-31"
        )?;

        create_payment(&conn, rental_id, 1000, "2025-01-15", "deposit", None)?;
        create_payment(&conn, rental_id, 1000, "2025-06-01", "yearly_fee", None)?;
        create_payment(&conn, rental_id, 1000, "2025-12-01", "yearly_fee", None)?;

        let payments = list_payments_in_range(&conn, "2025-05-01", "2025-12-31")?;
        assert_eq!(payments.len(), 2);

        let sum = sum_payments_in_range(&conn, "2025-05-01", "2025-12-31")?;
        assert_eq!(sum, 2000);

        Ok(())
    }

    #[test]
    fn test_delete_payment() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "E-001", "Main", "Small")?;
        let rental_id = rentals::create_rental(
            &conn, locker_id, "Charlie", None, None, "2025-01-01", "2025-12-31"
        )?;

        let payment_id = create_payment(&conn, rental_id, 1000, "2025-01-15", "deposit", None)?;

        assert!(delete_payment(&conn, payment_id)?);
        assert!(!delete_payment(&conn, payment_id)?);
        assert!(get_payment(&conn, payment_id)?.is_none());

        Ok(())
    }
}
