use crate::db::{lockers, rentals, payments};
use crate::models::{Locker, Rental, TenantType, Payment, PaymentType};
use chrono::{Duration, Utc};
use color_eyre::eyre::Result;
use rusqlite::Connection;

/// Seeds the database with test data if it's empty.
pub fn seed_test_data(conn: &Connection) -> Result<()> {
    // Check if data already exists
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    // Create locations
    let locations = ["Hauptgebäude", "Turnhalle", "Neubau"];

    // Create lockers
    for (loc_idx, location) in locations.iter().enumerate() {
        let prefix = match loc_idx {
            0 => "A",
            1 => "B",
            _ => "C",
        };
        let locker_count = match loc_idx {
            0 => 60,
            1 => 50,
            _ => 40,
        };

        for i in 1..=locker_count {
            let label = format!("{}-{:03}", prefix, i);
            let height = ((i - 1) as f64 / locker_count as f64 * 300.0) as i32;
            let locker = Locker::new(&label, *location, height);
            lockers::create_locker(conn, &locker)?;
        }
    }

    // Mark some lockers as damaged
    conn.execute("UPDATE lockers SET is_damaged = 1 WHERE label IN ('A-015', 'B-025', 'C-010')", [])?;

    // Create some sample rentals
    let today = Utc::now().date_naive();

    // Active rentals
    let sample_rentals = [
        ("A-001", "max.mustermann", TenantType::Schüler, -100, 265, true),
        ("A-010", "anna.schmidt", TenantType::Schüler, -200, 165, true),
        ("A-020", "peter.mueller", TenantType::Lehrer, -50, 315, true),
        ("B-005", "lisa.weber", TenantType::Schüler, -300, 65, true),
        // Overdue rentals
        ("A-042", "overdue.student", TenantType::Schüler, -400, -35, true),
        ("B-015", "another.overdue", TenantType::Schüler, -500, -85, true),
        // Expiring soon
        ("C-005", "expiring.soon", TenantType::Schüler, -350, 15, true),
        ("C-010", "also.expiring", TenantType::Lehrer, -340, 25, true),
    ];

    for (label, username, tenant_type, start_offset, end_offset, deposit_paid) in sample_rentals {
        let locker = lockers::get_locker_by_label(conn, label)?.unwrap();
        let mut rental = Rental::new(
            locker.id,
            username,
            tenant_type,
            today + Duration::days(start_offset),
            today + Duration::days(end_offset),
        );
        rental.deposit_paid = deposit_paid;

        let rental_id = rentals::create_rental(conn, &rental)?;

        // Add payments
        payments::create_payment(
            conn,
            &Payment::new(rental_id, 1000, PaymentType::Deposit, today + Duration::days(start_offset)),
        )?;
        payments::create_payment(
            conn,
            &Payment::new(rental_id, 1000, PaymentType::Extension, today + Duration::days(start_offset)),
        )?;
    }

    // Add some returned rentals for historical data
    let returned_rentals = [
        ("A-003", "returned.user1", TenantType::Schüler, -500, -100, true, true),
        ("A-004", "returned.user2", TenantType::Lehrer, -600, -200, true, true),
    ];

    for (label, username, tenant_type, start_offset, end_offset, deposit_paid, deposit_returned) in returned_rentals {
        let locker = lockers::get_locker_by_label(conn, label)?.unwrap();
        let mut rental = Rental::new(
            locker.id,
            username,
            tenant_type,
            today + Duration::days(start_offset),
            today + Duration::days(end_offset),
        );
        rental.deposit_paid = deposit_paid;
        rental.deposit_returned = deposit_returned;
        rental.returned_at = Some(Utc::now() - chrono::Duration::days(end_offset.abs()));

        let rental_id = rentals::create_rental(conn, &rental)?;

        // Add payments
        payments::create_payment(
            conn,
            &Payment::new(rental_id, 1000, PaymentType::Deposit, today + Duration::days(start_offset)),
        )?;
        payments::create_payment(
            conn,
            &Payment::new(rental_id, 1000, PaymentType::Extension, today + Duration::days(start_offset)),
        )?;
        if deposit_returned {
            payments::create_payment(
                conn,
                &Payment::new(rental_id, -1000, PaymentType::DepositReturn, today + Duration::days(end_offset)),
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::apply_migrations;

    #[test]
    fn test_seed_data() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        apply_migrations(&conn)?;

        seed_test_data(&conn)?;

        // Verify data was created
        let locker_count: i64 = conn.query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;
        assert_eq!(locker_count, 150);

        let rental_count: i64 = conn.query_row("SELECT COUNT(*) FROM rentals", [], |row| row.get(0))?;
        assert!(rental_count > 0);

        // Verify seed doesn't run twice
        seed_test_data(&conn)?;
        let locker_count2: i64 = conn.query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;
        assert_eq!(locker_count2, 150);

        Ok(())
    }
}
