use crate::models::{Rental, TenantType};
use chrono::{DateTime, NaiveDate, Utc};
use color_eyre::eyre::{Result, eyre};
use rusqlite::{params, Connection, OptionalExtension, Row};

/// Creates a new rental in the database.
pub fn create_rental(conn: &Connection, rental: &Rental) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO rentals (
            locker_id, tenant_username, tenant_type, rental_start_date,
            rental_end_date, deposit_paid, deposit_returned, created_at, returned_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        params![
            rental.locker_id,
            &rental.tenant_username,
            rental.tenant_type.as_str(),
            rental.rental_start_date.to_string(),
            rental.rental_end_date.to_string(),
            rental.deposit_paid,
            rental.deposit_returned,
            rental.created_at.to_rfc3339(),
            rental.returned_at.map(|dt| dt.to_rfc3339()),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Updates an existing rental.
pub fn update_rental(conn: &Connection, rental: &Rental) -> Result<()> {
    let affected = conn.execute(
        r#"
        UPDATE rentals SET
            locker_id = ?1,
            tenant_username = ?2,
            tenant_type = ?3,
            rental_start_date = ?4,
            rental_end_date = ?5,
            deposit_paid = ?6,
            deposit_returned = ?7,
            returned_at = ?8
        WHERE id = ?9
        "#,
        params![
            rental.locker_id,
            &rental.tenant_username,
            rental.tenant_type.as_str(),
            rental.rental_start_date.to_string(),
            rental.rental_end_date.to_string(),
            rental.deposit_paid,
            rental.deposit_returned,
            rental.returned_at.map(|dt| dt.to_rfc3339()),
            rental.id,
        ],
    )?;
    if affected == 0 {
        return Err(eyre!("Rental with id {} not found", rental.id));
    }
    Ok(())
}

/// Deletes a rental by id.
pub fn delete_rental(conn: &Connection, id: i64) -> Result<bool> {
    let affected = conn.execute("DELETE FROM rentals WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

/// Returns a single rental by its id.
pub fn get_rental(conn: &Connection, id: i64) -> Result<Option<Rental>> {
    let rental = conn
        .query_row(
            r#"
            SELECT id, locker_id, tenant_username, tenant_type, rental_start_date,
                   rental_end_date, deposit_paid, deposit_returned, created_at, returned_at
            FROM rentals WHERE id = ?1
            "#,
            params![id],
            row_to_rental,
        )
        .optional()?;
    Ok(rental)
}

/// Returns all rentals sorted by start date.
pub fn list_rentals(conn: &Connection) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, locker_id, tenant_username, tenant_type, rental_start_date,
               rental_end_date, deposit_paid, deposit_returned, created_at, returned_at
        FROM rentals ORDER BY rental_start_date DESC
        "#,
    )?;
    let rows = stmt.query_map([], row_to_rental)?;
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    Ok(rentals)
}

/// Returns all active (non-returned) rentals.
pub fn list_active_rentals(conn: &Connection) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, locker_id, tenant_username, tenant_type, rental_start_date,
               rental_end_date, deposit_paid, deposit_returned, created_at, returned_at
        FROM rentals WHERE returned_at IS NULL ORDER BY rental_start_date DESC
        "#,
    )?;
    let rows = stmt.query_map([], row_to_rental)?;
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    Ok(rentals)
}

/// Returns the active rental for a specific locker.
pub fn get_active_rental_for_locker(conn: &Connection, locker_id: i64) -> Result<Option<Rental>> {
    let rental = conn
        .query_row(
            r#"
            SELECT id, locker_id, tenant_username, tenant_type, rental_start_date,
                   rental_end_date, deposit_paid, deposit_returned, created_at, returned_at
            FROM rentals WHERE locker_id = ?1 AND returned_at IS NULL
            "#,
            params![locker_id],
            row_to_rental,
        )
        .optional()?;
    Ok(rental)
}

/// Returns all rentals for a specific tenant.
pub fn get_rentals_by_tenant(conn: &Connection, username: &str) -> Result<Vec<Rental>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, locker_id, tenant_username, tenant_type, rental_start_date,
               rental_end_date, deposit_paid, deposit_returned, created_at, returned_at
        FROM rentals WHERE tenant_username = ?1 ORDER BY rental_start_date DESC
        "#,
    )?;
    let rows = stmt.query_map(params![username], row_to_rental)?;
    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    Ok(rentals)
}

/// Returns the active rental for a specific tenant.
pub fn get_active_rental_by_tenant(conn: &Connection, username: &str) -> Result<Option<Rental>> {
    let rental = conn
        .query_row(
            r#"
            SELECT id, locker_id, tenant_username, tenant_type, rental_start_date,
                   rental_end_date, deposit_paid, deposit_returned, created_at, returned_at
            FROM rentals WHERE tenant_username = ?1 AND returned_at IS NULL
            "#,
            params![username],
            row_to_rental,
        )
        .optional()?;
    Ok(rental)
}

/// Extends a rental by updating the end date.
pub fn extend_rental(conn: &Connection, rental_id: i64, new_end_date: NaiveDate) -> Result<()> {
    let affected = conn.execute(
        "UPDATE rentals SET rental_end_date = ?1 WHERE id = ?2",
        params![new_end_date.to_string(), rental_id],
    )?;
    if affected == 0 {
        return Err(eyre!("Rental with id {} not found", rental_id));
    }
    Ok(())
}

/// Returns a locker by marking it as returned.
pub fn return_rental(conn: &Connection, rental_id: i64, returned_at: DateTime<Utc>) -> Result<()> {
    let affected = conn.execute(
        "UPDATE rentals SET returned_at = ?1 WHERE id = ?2",
        params![returned_at.to_rfc3339(), rental_id],
    )?;
    if affected == 0 {
        return Err(eyre!("Rental with id {} not found", rental_id));
    }
    Ok(())
}

/// Marks deposit as paid for a rental.
pub fn mark_deposit_paid(conn: &Connection, rental_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE rentals SET deposit_paid = 1 WHERE id = ?1",
        params![rental_id],
    )?;
    Ok(())
}

/// Marks deposit as returned for a rental.
pub fn mark_deposit_returned(conn: &Connection, rental_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE rentals SET deposit_returned = 1 WHERE id = ?1",
        params![rental_id],
    )?;
    Ok(())
}

fn row_to_rental(row: &Row) -> rusqlite::Result<Rental> {
    let tenant_type_str: String = row.get("tenant_type")?;
    let tenant_type = TenantType::from_str(&tenant_type_str).unwrap_or(TenantType::Schüler);

    let start_date_str: String = row.get("rental_start_date")?;
    let rental_start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")
        .unwrap_or_else(|_| Utc::now().date_naive());

    let end_date_str: String = row.get("rental_end_date")?;
    let rental_end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
        .unwrap_or_else(|_| Utc::now().date_naive());

    let created_at_str: String = row.get("created_at")?;
    let created_at = DateTime::parse_from_rfc3339(&created_at_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let returned_at_str: Option<String> = row.get("returned_at")?;
    let returned_at = returned_at_str
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    Ok(Rental {
        id: row.get("id")?,
        locker_id: row.get("locker_id")?,
        tenant_username: row.get("tenant_username")?,
        tenant_type,
        rental_start_date,
        rental_end_date,
        deposit_paid: row.get("deposit_paid")?,
        deposit_returned: row.get("deposit_returned")?,
        created_at,
        returned_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{lockers, migrations::apply_migrations};
    use crate::models::Locker;
    use chrono::Duration;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_and_get_rental() -> Result<()> {
        let conn = setup_db();
        let locker_id = lockers::create_locker(&conn, &Locker::new("A-01", "Test", 100))?;

        let today = Utc::now().date_naive();
        let rental = Rental::new(
            locker_id,
            "max.mustermann",
            TenantType::Schüler,
            today,
            today + Duration::days(365),
        );
        let id = create_rental(&conn, &rental)?;

        let fetched = get_rental(&conn, id)?.unwrap();
        assert_eq!(fetched.locker_id, locker_id);
        assert_eq!(fetched.tenant_username, "max.mustermann");
        assert_eq!(fetched.tenant_type, TenantType::Schüler);

        Ok(())
    }

    #[test]
    fn test_active_rentals() -> Result<()> {
        let conn = setup_db();
        let locker_id = lockers::create_locker(&conn, &Locker::new("A-01", "Test", 100))?;

        let today = Utc::now().date_naive();
        let rental = Rental::new(
            locker_id,
            "test.user",
            TenantType::Schüler,
            today,
            today + Duration::days(365),
        );
        let rental_id = create_rental(&conn, &rental)?;

        // Should be active
        let active = list_active_rentals(&conn)?;
        assert_eq!(active.len(), 1);

        // Return it
        return_rental(&conn, rental_id, Utc::now())?;

        // Should no longer be active
        let active = list_active_rentals(&conn)?;
        assert_eq!(active.len(), 0);

        Ok(())
    }

    #[test]
    fn test_extend_rental() -> Result<()> {
        let conn = setup_db();
        let locker_id = lockers::create_locker(&conn, &Locker::new("A-01", "Test", 100))?;

        let today = Utc::now().date_naive();
        let rental = Rental::new(
            locker_id,
            "test.user",
            TenantType::Schüler,
            today,
            today + Duration::days(365),
        );
        let rental_id = create_rental(&conn, &rental)?;

        let new_end = today + Duration::days(730);
        extend_rental(&conn, rental_id, new_end)?;

        let fetched = get_rental(&conn, rental_id)?.unwrap();
        assert_eq!(fetched.rental_end_date, new_end);

        Ok(())
    }
}
