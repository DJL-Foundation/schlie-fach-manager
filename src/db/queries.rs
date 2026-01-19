use crate::models::{
    DashboardStats, DebtorInfo, LocationStats, PaymentSummary, PaymentType, RentalWithLocker,
    TenantType,
};
use chrono::{DateTime, NaiveDate, Utc};
use color_eyre::eyre::Result;
use rusqlite::{params, Connection};

/// Returns dashboard statistics.
pub fn get_dashboard_stats(conn: &Connection) -> Result<DashboardStats> {
    let total_lockers: i32 =
        conn.query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))?;

    let occupied_lockers: i32 = conn.query_row(
        "SELECT COUNT(DISTINCT locker_id) FROM rentals WHERE returned_at IS NULL",
        [],
        |row| row.get(0),
    )?;

    let damaged_lockers: i32 = conn.query_row(
        "SELECT COUNT(*) FROM lockers WHERE is_damaged = 1",
        [],
        |row| row.get(0),
    )?;

    let damaged_and_occupied: i32 = conn.query_row(
        r#"
        SELECT COUNT(DISTINCT l.id)
        FROM lockers l
        JOIN rentals r ON l.id = r.locker_id
        WHERE l.is_damaged = 1 AND r.returned_at IS NULL
        "#,
        [],
        |row| row.get(0),
    )?;

    let locations = get_location_stats(conn)?;
    let expiring_soon = get_expiring_rentals(conn, 30)?;
    let overdue_rentals = get_overdue_rentals(conn)?;
    let total_revenue_cents = get_total_revenue(conn)?;
    let outstanding_payments_cents = get_outstanding_payments(conn)?;

    Ok(DashboardStats {
        total_lockers,
        occupied_lockers,
        damaged_lockers,
        damaged_and_occupied,
        locations,
        expiring_soon,
        overdue_rentals,
        total_revenue_cents,
        outstanding_payments_cents,
    })
}

/// Returns statistics grouped by location.
pub fn get_location_stats(conn: &Connection) -> Result<Vec<LocationStats>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT
            l.location,
            COUNT(l.id) as total,
            COUNT(r.id) as occupied,
            SUM(CASE WHEN l.is_damaged = 1 THEN 1 ELSE 0 END) as damaged
        FROM lockers l
        LEFT JOIN rentals r ON l.id = r.locker_id AND r.returned_at IS NULL
        GROUP BY l.location
        ORDER BY l.location
        "#,
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(LocationStats {
            location: row.get(0)?,
            total: row.get(1)?,
            occupied: row.get(2)?,
            damaged: row.get(3)?,
        })
    })?;

    let mut stats = Vec::new();
    for stat in rows {
        stats.push(stat?);
    }
    Ok(stats)
}

/// Returns rentals expiring within the given number of days.
pub fn get_expiring_rentals(conn: &Connection, days: i32) -> Result<Vec<RentalWithLocker>> {
    let today = Utc::now().date_naive();
    let cutoff_date = today + chrono::Duration::days(days as i64);

    let mut stmt = conn.prepare(
        r#"
        SELECT
            r.id, r.locker_id, r.tenant_username, r.tenant_type,
            r.rental_start_date, r.rental_end_date, r.deposit_paid,
            r.deposit_returned, r.created_at, r.returned_at,
            l.id as l_id, l.label, l.location, l.height, l.is_damaged, l.created_at as l_created_at
        FROM rentals r
        JOIN lockers l ON r.locker_id = l.id
        WHERE r.returned_at IS NULL
        AND r.rental_end_date > ?1
        AND r.rental_end_date <= ?2
        ORDER BY r.rental_end_date ASC
        "#,
    )?;

    let rows = stmt.query_map(
        params![today.to_string(), cutoff_date.to_string()],
        row_to_rental_with_locker,
    )?;

    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    Ok(rentals)
}

/// Returns all overdue rentals.
pub fn get_overdue_rentals(conn: &Connection) -> Result<Vec<RentalWithLocker>> {
    let today = Utc::now().date_naive();

    let mut stmt = conn.prepare(
        r#"
        SELECT
            r.id, r.locker_id, r.tenant_username, r.tenant_type,
            r.rental_start_date, r.rental_end_date, r.deposit_paid,
            r.deposit_returned, r.created_at, r.returned_at,
            l.id as l_id, l.label, l.location, l.height, l.is_damaged, l.created_at as l_created_at
        FROM rentals r
        JOIN lockers l ON r.locker_id = l.id
        WHERE r.returned_at IS NULL
        AND r.rental_end_date < ?1
        ORDER BY r.rental_end_date ASC
        "#,
    )?;

    let rows = stmt.query_map(params![today.to_string()], row_to_rental_with_locker)?;

    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    Ok(rentals)
}

/// Returns all active rentals with their lockers.
pub fn get_active_rentals_with_lockers(conn: &Connection) -> Result<Vec<RentalWithLocker>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT
            r.id, r.locker_id, r.tenant_username, r.tenant_type,
            r.rental_start_date, r.rental_end_date, r.deposit_paid,
            r.deposit_returned, r.created_at, r.returned_at,
            l.id as l_id, l.label, l.location, l.height, l.is_damaged, l.created_at as l_created_at
        FROM rentals r
        JOIN lockers l ON r.locker_id = l.id
        WHERE r.returned_at IS NULL
        ORDER BY l.label ASC
        "#,
    )?;

    let rows = stmt.query_map([], row_to_rental_with_locker)?;

    let mut rentals = Vec::new();
    for rental in rows {
        rentals.push(rental?);
    }
    Ok(rentals)
}

/// Returns the total revenue from all payments.
fn get_total_revenue(conn: &Connection) -> Result<i32> {
    let total: i32 = conn
        .query_row("SELECT COALESCE(SUM(amount_cents), 0) FROM payments", [], |row| {
            row.get(0)
        })?;
    Ok(total)
}

/// Returns the total outstanding payments (debt from overdue rentals).
fn get_outstanding_payments(conn: &Connection) -> Result<i32> {
    let today = Utc::now().date_naive();

    let mut stmt = conn.prepare(
        "SELECT rental_end_date FROM rentals WHERE returned_at IS NULL AND rental_end_date < ?",
    )?;

    let dates = stmt.query_map(params![today.to_string()], |row| row.get::<_, String>(0))?;

    let mut total_debt = 0;
    for date_result in dates {
        let end_date_str = date_result?;
        if let Ok(end_date) = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d") {
            let debt = calculate_debt_from_date(end_date);
            total_debt += debt;
        }
    }

    Ok(total_debt)
}

fn calculate_debt_from_date(end_date: NaiveDate) -> i32 {
    let today = Utc::now().date_naive();
    if today <= end_date {
        return 0;
    }
    let days_overdue = (today - end_date).num_days();
    let years_overdue = (days_overdue as f64 / 365.25).ceil() as i32;
    years_overdue * 1000
}

/// Returns payment summary for a given date range.
pub fn get_payment_summary(
    conn: &Connection,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<PaymentSummary> {
    // Use a safe default date - January 1, 2000
    const DEFAULT_START_YEAR: i32 = 2000;
    const DEFAULT_START_MONTH: u32 = 1;
    const DEFAULT_START_DAY: u32 = 1;
    
    let start = start_date.unwrap_or_else(|| {
        NaiveDate::from_ymd_opt(DEFAULT_START_YEAR, DEFAULT_START_MONTH, DEFAULT_START_DAY)
            .expect("2000-01-01 is a valid date")
    });
    let end = end_date.unwrap_or_else(|| Utc::now().date_naive());

    let mut stmt = conn.prepare(
        r#"
        SELECT
            payment_type,
            SUM(amount_cents) as total_cents,
            COUNT(*) as count
        FROM payments
        WHERE payment_date >= ?1 AND payment_date <= ?2
        GROUP BY payment_type
        "#,
    )?;

    let rows = stmt.query_map(params![start.to_string(), end.to_string()], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, i32>(2)?,
        ))
    })?;

    let mut deposits = 0;
    let mut extensions = 0;
    let mut deposit_returns = 0;
    let mut deposit_count = 0;
    let mut extension_count = 0;
    let mut deposit_return_count = 0;

    for result in rows {
        let (payment_type, amount, count) = result?;
        match PaymentType::from_str(&payment_type) {
            Some(PaymentType::Deposit) => {
                deposits = amount;
                deposit_count = count;
            }
            Some(PaymentType::Extension) => {
                extensions = amount;
                extension_count = count;
            }
            Some(PaymentType::DepositReturn) => {
                deposit_returns = amount;
                deposit_return_count = count;
            }
            None => {}
        }
    }

    Ok(PaymentSummary {
        deposits_cents: deposits,
        extensions_cents: extensions,
        deposit_returns_cents: deposit_returns,
        total_cents: deposits + extensions + deposit_returns,
        net_cents: deposits + extensions + deposit_returns,
        deposit_count,
        extension_count,
        deposit_return_count,
    })
}

/// Returns information about all debtors.
pub fn get_debtors(conn: &Connection) -> Result<Vec<DebtorInfo>> {
    let today = Utc::now().date_naive();

    let mut stmt = conn.prepare(
        r#"
        SELECT
            tenant_username,
            tenant_type,
            rental_end_date
        FROM rentals
        WHERE returned_at IS NULL AND rental_end_date < ?
        ORDER BY rental_end_date ASC
        "#,
    )?;

    let rentals = stmt.query_map(params![today.to_string()], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    use std::collections::HashMap;
    let mut debtors: HashMap<String, (TenantType, i32, i64)> = HashMap::new();

    for result in rentals {
        let (username, tenant_type_str, end_date_str) = result?;
        if let Ok(end_date) = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d") {
            let debt = calculate_debt_from_date(end_date);
            let days_overdue = (today - end_date).num_days();

            let tenant_type = TenantType::from_str(&tenant_type_str).unwrap_or(TenantType::Schüler);

            debtors
                .entry(username)
                .and_modify(|(_, total, max_days)| {
                    *total += debt;
                    *max_days = (*max_days).max(days_overdue);
                })
                .or_insert((tenant_type, debt, days_overdue));
        }
    }

    let mut result: Vec<DebtorInfo> = debtors
        .into_iter()
        .map(|(username, (tenant_type, debt, days))| DebtorInfo {
            email: format!("{}@athenetz.de", username),
            username,
            total_debt_cents: debt,
            tenant_type,
            days_overdue: days,
        })
        .collect();

    result.sort_by(|a, b| b.total_debt_cents.cmp(&a.total_debt_cents));

    Ok(result)
}

fn row_to_rental_with_locker(row: &rusqlite::Row) -> rusqlite::Result<RentalWithLocker> {
    use crate::models::{Locker, Rental};

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

    let l_created_at_str: String = row.get("l_created_at")?;
    let l_created_at = DateTime::parse_from_rfc3339(&l_created_at_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let rental = Rental {
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
    };

    let locker = Locker {
        id: row.get("l_id")?,
        label: row.get("label")?,
        location: row.get("location")?,
        height: row.get("height")?,
        is_damaged: row.get("is_damaged")?,
        created_at: l_created_at,
    };

    Ok(RentalWithLocker { rental, locker })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{lockers, migrations::apply_migrations, payments, rentals};
    use crate::models::{Locker, Payment, Rental};
    use chrono::Duration;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_get_dashboard_stats() -> Result<()> {
        let conn = setup_db();

        // Create lockers
        let l1 = lockers::create_locker(&conn, &Locker::new("A-01", "Location1", 50))?;
        let l2 = lockers::create_locker(&conn, &Locker::new("A-02", "Location1", 100))?;
        let _l3 = lockers::create_locker(&conn, &Locker::new("B-01", "Location2", 150))?;

        // Create rentals
        let today = Utc::now().date_naive();
        let rental = Rental::new(
            l1,
            "user1",
            TenantType::Schüler,
            today,
            today + Duration::days(365),
        );
        rentals::create_rental(&conn, &rental)?;

        let rental2 = Rental::new(
            l2,
            "user2",
            TenantType::Lehrer,
            today,
            today + Duration::days(365),
        );
        rentals::create_rental(&conn, &rental2)?;

        let stats = get_dashboard_stats(&conn)?;
        assert_eq!(stats.total_lockers, 3);
        assert_eq!(stats.occupied_lockers, 2);

        Ok(())
    }

    #[test]
    fn test_get_location_stats() -> Result<()> {
        let conn = setup_db();

        lockers::create_locker(&conn, &Locker::new("A-01", "Location1", 50))?;
        lockers::create_locker(&conn, &Locker::new("A-02", "Location1", 100))?;
        lockers::create_locker(&conn, &Locker::new("B-01", "Location2", 150))?;

        let stats = get_location_stats(&conn)?;
        assert_eq!(stats.len(), 2);

        let loc1 = stats.iter().find(|s| s.location == "Location1").unwrap();
        assert_eq!(loc1.total, 2);

        Ok(())
    }

    #[test]
    fn test_get_payment_summary() -> Result<()> {
        let conn = setup_db();

        let locker_id = lockers::create_locker(&conn, &Locker::new("A-01", "Test", 100))?;
        let today = Utc::now().date_naive();
        let rental = Rental::new(
            locker_id,
            "user",
            TenantType::Schüler,
            today,
            today + Duration::days(365),
        );
        let rental_id = rentals::create_rental(&conn, &rental)?;

        payments::create_payment(
            &conn,
            &Payment::new(rental_id, 1000, PaymentType::Deposit, today),
        )?;
        payments::create_payment(
            &conn,
            &Payment::new(rental_id, 1000, PaymentType::Extension, today),
        )?;

        let summary = get_payment_summary(&conn, None, None)?;
        assert_eq!(summary.deposits_cents, 1000);
        assert_eq!(summary.extensions_cents, 1000);
        assert_eq!(summary.total_cents, 2000);

        Ok(())
    }

    #[test]
    fn test_get_overdue_rentals() -> Result<()> {
        let conn = setup_db();

        let locker_id = lockers::create_locker(&conn, &Locker::new("A-01", "Test", 100))?;
        let today = Utc::now().date_naive();

        // Create an overdue rental
        let rental = Rental::new(
            locker_id,
            "user",
            TenantType::Schüler,
            today - Duration::days(400),
            today - Duration::days(35),
        );
        rentals::create_rental(&conn, &rental)?;

        let overdue = get_overdue_rentals(&conn)?;
        assert_eq!(overdue.len(), 1);

        Ok(())
    }
}
