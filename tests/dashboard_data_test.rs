use schliessfach_manager::db::Database;
use tempfile::tempdir;

/// Test that verifies dashboard data can be loaded from database
#[test]
fn test_dashboard_data_loading() {
    // Create a temporary database
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();
    
    // The database should be initialized with migrations
    let conn = db.connection();
    
    // Verify tables exist
    let table_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='lockers')",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert!(table_exists, "Lockers table should exist");
    
    // Add some test data
    conn.execute(
        "INSERT INTO lockers (number, location, size) VALUES ('001', 'Hauptgebäude', 'Klein')",
        [],
    ).unwrap();
    
    conn.execute(
        "INSERT INTO lockers (number, location, size) VALUES ('002', 'Hauptgebäude', 'Mittel')",
        [],
    ).unwrap();
    
    conn.execute(
        "INSERT INTO lockers (number, location, size, is_damaged) VALUES ('003', 'Nebengebäude', 'Groß', 1)",
        [],
    ).unwrap();
    
    // Verify we can count lockers
    let total_lockers: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lockers",
        [],
        |row| row.get(0),
    ).unwrap();
    assert_eq!(total_lockers, 3, "Should have 3 lockers");
    
    // Verify we can count damaged lockers
    let damaged_lockers: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lockers WHERE is_damaged = 1",
        [],
        |row| row.get(0),
    ).unwrap();
    assert_eq!(damaged_lockers, 1, "Should have 1 damaged locker");
}

/// Test that verifies the dashboard can handle an empty database
#[test]
fn test_dashboard_empty_database() {
    // Create a temporary database
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test_empty.db");
    let db = Database::open(&db_path).unwrap();
    
    let conn = db.connection();
    
    // Verify empty database returns 0 for counts
    let total_lockers: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lockers",
        [],
        |row| row.get(0),
    ).unwrap();
    assert_eq!(total_lockers, 0, "Empty database should have 0 lockers");
    
    let occupied_lockers: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT locker_id) FROM rentals 
         WHERE date('now') BETWEEN start_date AND end_date",
        [],
        |row| row.get(0),
    ).unwrap();
    assert_eq!(occupied_lockers, 0, "Empty database should have 0 occupied lockers");
}

/// Test revenue calculation over 30 days
#[test]
fn test_dashboard_revenue_calculation() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test_revenue.db");
    let db = Database::open(&db_path).unwrap();
    
    let conn = db.connection();
    
    // Add a locker
    conn.execute(
        "INSERT INTO lockers (number, location, size) VALUES ('001', 'Test', 'Klein')",
        [],
    ).unwrap();
    let locker_id = conn.last_insert_rowid();
    
    // Add a rental
    conn.execute(
        "INSERT INTO rentals (locker_id, renter_name, start_date, end_date) 
         VALUES (?1, 'Test User', date('now', '-60 days'), date('now', '+30 days'))",
        [locker_id],
    ).unwrap();
    let rental_id = conn.last_insert_rowid();
    
    // Add a payment from 15 days ago
    conn.execute(
        "INSERT INTO payments (rental_id, amount_cents, payment_date, payment_type) 
         VALUES (?1, 5000, date('now', '-15 days'), 'Deposit')",
        [rental_id],
    ).unwrap();
    
    // Add a payment from 45 days ago (outside 30-day window)
    conn.execute(
        "INSERT INTO payments (rental_id, amount_cents, payment_date, payment_type) 
         VALUES (?1, 3000, date('now', '-45 days'), 'Fee')",
        [rental_id],
    ).unwrap();
    
    // Calculate revenue for last 30 days
    let revenue_30d: i64 = conn.query_row(
        "SELECT COALESCE(SUM(amount_cents), 0) FROM payments 
         WHERE payment_date >= date('now', '-30 days')",
        [],
        |row| row.get(0),
    ).unwrap();
    
    // Should only include the 5000 cents payment from 15 days ago
    assert_eq!(revenue_30d, 5000, "Revenue should be 5000 cents (50 EUR)");
}
