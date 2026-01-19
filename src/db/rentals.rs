use color_eyre::eyre::Result;
use rusqlite::{Connection, OptionalExtension, params};

/// Represents a rental in the v2.1 schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rental {
    pub id: i64,
    pub locker_id: i64,
    pub renter_name: String,
    pub renter_email: Option<String>,
    pub renter_phone: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub deposit_paid: bool,
    pub deposit_returned: bool,
    pub notes: Option<String>,
    pub created_at: String,
}

/// Creates a new rental.
pub fn create_rental(
    conn: &Connection,
    locker_id: i64,
    renter_name: &str,
    renter_email: Option<&str>,
    renter_phone: Option<&str>,
    start_date: &str,
    end_date: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO rentals (locker_id, renter_name, renter_email, renter_phone, start_date, end_date) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![locker_id, renter_name, renter_email, renter_phone, start_date, end_date],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Updates an existing rental.
pub fn update_rental(conn: &Connection, rental: &Rental) -> Result<()> {
    conn.execute(
        "UPDATE rentals SET 
            locker_id = ?1, 
            renter_name = ?2, 
            renter_email = ?3, 
            renter_phone = ?4, 
            start_date = ?5, 
            end_date = ?6, 
            deposit_paid = ?7, 
            deposit_returned = ?8, 
            notes = ?9 
         WHERE id = ?10",
        params![
            rental.locker_id,
            &rental.renter_name,
            &rental.renter_email,
            &rental.renter_phone,
            &rental.start_date,
            &rental.end_date,
            rental.deposit_paid as i64,
            rental.deposit_returned as i64,
            &rental.notes,
            rental.id
        ],
    )?;
    Ok(())
}

/// Returns a rental by ID.
pub fn get_rental(conn: &Connection, id: i64) -> Result<Option<Rental>> {
    let rental = conn
        .query_row(
            "SELECT id, locker_id, renter_name, renter_email, renter_phone, 
                    start_date, end_date, deposit_paid, deposit_returned, notes, created_at 
             FROM rentals 
             WHERE id = ?1",
            params![id],
            |row| {
                Ok(Rental {
                    id: row.get(0)?,
                    locker_id: row.get(1)?,
                    renter_name: row.get(2)?,
                    renter_email: row.get(3)?,
                    renter_phone: row.get(4)?,
                    start_date: row.get(5)?,
                    end_date: row.get(6)?,
                    deposit_paid: row.get::<_, i64>(7)? != 0,
                    deposit_returned: row.get::<_, i64>(8)? != 0,
                    notes: row.get(9)?,
                    created_at: row.get(10)?,
                })
            },
        )
        .optional()?;
    
    Ok(rental)
}

/// Returns all rentals.
pub fn list_rentals(conn: &Connection) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        "SELECT id, locker_id, renter_name, renter_email, renter_phone, 
                start_date, end_date, deposit_paid, deposit_returned, notes, created_at 
         FROM rentals 
         ORDER BY created_at DESC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(Rental {
            id: row.get(0)?,
            locker_id: row.get(1)?,
            renter_name: row.get(2)?,
            renter_email: row.get(3)?,
            renter_phone: row.get(4)?,
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            deposit_paid: row.get::<_, i64>(7)? != 0,
            deposit_returned: row.get::<_, i64>(8)? != 0,
            notes: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    
    Ok(rentals)
}

/// Returns active rentals (current date is between start and end date).
pub fn list_active_rentals(conn: &Connection) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        "SELECT id, locker_id, renter_name, renter_email, renter_phone, 
                start_date, end_date, deposit_paid, deposit_returned, notes, created_at 
         FROM rentals 
         WHERE date('now') BETWEEN start_date AND end_date
         ORDER BY end_date ASC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(Rental {
            id: row.get(0)?,
            locker_id: row.get(1)?,
            renter_name: row.get(2)?,
            renter_email: row.get(3)?,
            renter_phone: row.get(4)?,
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            deposit_paid: row.get::<_, i64>(7)? != 0,
            deposit_returned: row.get::<_, i64>(8)? != 0,
            notes: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    
    Ok(rentals)
}

/// Returns overdue rentals (end date is in the past).
pub fn list_overdue_rentals(conn: &Connection) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        "SELECT id, locker_id, renter_name, renter_email, renter_phone, 
                start_date, end_date, deposit_paid, deposit_returned, notes, created_at 
         FROM rentals 
         WHERE date(end_date) < date('now')
         ORDER BY end_date ASC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(Rental {
            id: row.get(0)?,
            locker_id: row.get(1)?,
            renter_name: row.get(2)?,
            renter_email: row.get(3)?,
            renter_phone: row.get(4)?,
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            deposit_paid: row.get::<_, i64>(7)? != 0,
            deposit_returned: row.get::<_, i64>(8)? != 0,
            notes: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    
    Ok(rentals)
}

/// Returns rentals expiring within the specified number of days.
pub fn list_expiring_soon(conn: &Connection, days: i64) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        "SELECT id, locker_id, renter_name, renter_email, renter_phone, 
                start_date, end_date, deposit_paid, deposit_returned, notes, created_at 
         FROM rentals 
         WHERE date(end_date) BETWEEN date('now') AND date('now', '+' || ?1 || ' days')
         ORDER BY end_date ASC"
    )?;
    
    let rows = stmt.query_map(params![days], |row| {
        Ok(Rental {
            id: row.get(0)?,
            locker_id: row.get(1)?,
            renter_name: row.get(2)?,
            renter_email: row.get(3)?,
            renter_phone: row.get(4)?,
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            deposit_paid: row.get::<_, i64>(7)? != 0,
            deposit_returned: row.get::<_, i64>(8)? != 0,
            notes: row.get(9)?,
            created_at: row.get(10)?,
        })
    })?;
    
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    
    Ok(rentals)
}

/// Returns the current rental for a locker, if any.
pub fn get_current_rental_for_locker(conn: &Connection, locker_id: i64) -> Result<Option<Rental>> {
    let rental = conn
        .query_row(
            "SELECT id, locker_id, renter_name, renter_email, renter_phone, 
                    start_date, end_date, deposit_paid, deposit_returned, notes, created_at 
             FROM rentals 
             WHERE locker_id = ?1 AND date('now') BETWEEN start_date AND end_date
             ORDER BY start_date DESC
             LIMIT 1",
            params![locker_id],
            |row| {
                Ok(Rental {
                    id: row.get(0)?,
                    locker_id: row.get(1)?,
                    renter_name: row.get(2)?,
                    renter_email: row.get(3)?,
                    renter_phone: row.get(4)?,
                    start_date: row.get(5)?,
                    end_date: row.get(6)?,
                    deposit_paid: row.get::<_, i64>(7)? != 0,
                    deposit_returned: row.get::<_, i64>(8)? != 0,
                    notes: row.get(9)?,
                    created_at: row.get(10)?,
                })
            },
        )
        .optional()?;
    
    Ok(rental)
}

/// Marks the deposit as paid for a rental.
pub fn mark_deposit_paid(conn: &Connection, rental_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE rentals SET deposit_paid = 1 WHERE id = ?1",
        params![rental_id],
    )?;
    Ok(())
}

/// Marks the deposit as returned for a rental.
pub fn mark_deposit_returned(conn: &Connection, rental_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE rentals SET deposit_returned = 1 WHERE id = ?1",
        params![rental_id],
    )?;
    Ok(())
}

/// Extends a rental by updating its end date.
pub fn extend_rental(conn: &Connection, rental_id: i64, new_end_date: &str) -> Result<()> {
    conn.execute(
        "UPDATE rentals SET end_date = ?1 WHERE id = ?2",
        params![new_end_date, rental_id],
    )?;
    Ok(())
}

/// Deletes a rental by ID.
pub fn delete_rental(conn: &Connection, id: i64) -> Result<bool> {
    let affected = conn.execute("DELETE FROM rentals WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{lockers, migrations::run_migrations};
    use rusqlite::Connection;

    #[test]
    fn test_create_and_get_rental() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "A-001", "Main", "Small")?;
        let rental_id = create_rental(
            &conn,
            locker_id,
            "John Doe",
            Some("john@example.com"),
            Some("+123456789"),
            "2025-01-01",
            "2025-12-31",
        )?;

        let rental = get_rental(&conn, rental_id)?.expect("rental not found");
        assert_eq!(rental.renter_name, "John Doe");
        assert_eq!(rental.renter_email, Some("john@example.com".to_string()));
        assert_eq!(rental.start_date, "2025-01-01");
        assert_eq!(rental.end_date, "2025-12-31");
        assert!(!rental.deposit_paid);
        assert!(!rental.deposit_returned);

        Ok(())
    }

    #[test]
    fn test_mark_deposit_paid_and_returned() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "B-001", "Main", "Small")?;
        let rental_id = create_rental(
            &conn,
            locker_id,
            "Jane Smith",
            None,
            None,
            "2025-01-01",
            "2025-12-31",
        )?;

        mark_deposit_paid(&conn, rental_id)?;
        let rental = get_rental(&conn, rental_id)?.unwrap();
        assert!(rental.deposit_paid);
        assert!(!rental.deposit_returned);

        mark_deposit_returned(&conn, rental_id)?;
        let rental = get_rental(&conn, rental_id)?.unwrap();
        assert!(rental.deposit_paid);
        assert!(rental.deposit_returned);

        Ok(())
    }

    #[test]
    fn test_extend_rental() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "C-001", "Main", "Small")?;
        let rental_id = create_rental(
            &conn,
            locker_id,
            "Bob",
            None,
            None,
            "2025-01-01",
            "2025-06-30",
        )?;

        extend_rental(&conn, rental_id, "2025-12-31")?;
        let rental = get_rental(&conn, rental_id)?.unwrap();
        assert_eq!(rental.end_date, "2025-12-31");

        Ok(())
    }

    #[test]
    fn test_delete_rental() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "D-001", "Main", "Small")?;
        let rental_id = create_rental(&conn, locker_id, "Alice", None, None, "2025-01-01", "2025-12-31")?;

        assert!(delete_rental(&conn, rental_id)?);
        assert!(!delete_rental(&conn, rental_id)?);
        assert!(get_rental(&conn, rental_id)?.is_none());

        Ok(())
    }

    #[test]
    fn test_get_current_rental_for_locker() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        let locker_id = lockers::create_locker(&conn, "E-001", "Main", "Small")?;
        
        // Create a past rental
        create_rental(&conn, locker_id, "Past Renter", None, None, "2024-01-01", "2024-12-31")?;
        
        // Create a current rental (dates that include 2026-01-19)
        create_rental(&conn, locker_id, "Current Renter", None, None, "2026-01-01", "2026-12-31")?;
        
        // Create a future rental
        create_rental(&conn, locker_id, "Future Renter", None, None, "2027-01-01", "2027-12-31")?;

        let current = get_current_rental_for_locker(&conn, locker_id)?;
        assert!(current.is_some());
        assert_eq!(current.unwrap().renter_name, "Current Renter");

        Ok(())
    }
}
