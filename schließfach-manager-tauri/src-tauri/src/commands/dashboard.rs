//! Dashboard-related Tauri commands

use tauri::State;
use crate::db::{DbConnection, DashboardData, StatusBarData, SizeStats, LocationStats, OccupancyHistoryPoint};
use std::collections::HashMap;
use chrono::{Utc, Duration};

/// Get dashboard data including stats and charts
#[tauri::command]
pub async fn get_dashboard_data(db: State<'_, DbConnection>) -> Result<DashboardData, String> {
    let conn = db.0.lock().await;
    
    // Get total lockers
    let total_lockers: i64 = conn
        .query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    // Get occupied lockers (those with active rentals)
    let occupied_lockers: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT locker_id) FROM rentals 
             WHERE date(end_date) >= date('now') AND deposit_returned = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    // Calculate occupancy percentage
    let occupancy_percent = if total_lockers > 0 {
        (occupied_lockers as f64 / total_lockers as f64) * 100.0
    } else {
        0.0
    };
    
    // Get stats by size
    let mut by_size: HashMap<String, SizeStats> = HashMap::new();
    {
        let mut stmt = conn
            .prepare(
                "SELECT l.size, COUNT(*) as total,
                 SUM(CASE WHEN r.id IS NOT NULL THEN 1 ELSE 0 END) as occupied
                 FROM lockers l
                 LEFT JOIN rentals r ON l.id = r.locker_id 
                   AND date(r.end_date) >= date('now') 
                   AND r.deposit_returned = 0
                 GROUP BY l.size",
            )
            .map_err(|e| e.to_string())?;
        
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            })
            .map_err(|e| e.to_string())?;
        
        for row in rows {
            let (size, total, occupied) = row.map_err(|e| e.to_string())?;
            by_size.insert(size, SizeStats { total, occupied });
        }
    }
    
    // Get stats by location
    let mut by_location: HashMap<String, LocationStats> = HashMap::new();
    {
        let mut stmt = conn
            .prepare(
                "SELECT l.location, COUNT(*) as total,
                 SUM(CASE WHEN r.id IS NOT NULL THEN 1 ELSE 0 END) as occupied
                 FROM lockers l
                 LEFT JOIN rentals r ON l.id = r.locker_id 
                   AND date(r.end_date) >= date('now') 
                   AND r.deposit_returned = 0
                 GROUP BY l.location",
            )
            .map_err(|e| e.to_string())?;
        
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            })
            .map_err(|e| e.to_string())?;
        
        for row in rows {
            let (location, total, occupied) = row.map_err(|e| e.to_string())?;
            by_location.insert(location, LocationStats { total, occupied });
        }
    }
    
    // Get overdue returns
    let overdue_returns: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM rentals 
             WHERE date(end_date) < date('now') AND deposit_returned = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    // Get expiring soon (within 7 days)
    let expiring_soon: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM rentals 
             WHERE date(end_date) >= date('now') 
               AND date(end_date) <= date('now', '+7 days')
               AND deposit_returned = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    // Get damaged lockers
    let damaged_lockers: i64 = conn
        .query_row("SELECT COUNT(*) FROM lockers WHERE is_damaged = 1", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    // Get pending payments (simplified - would need more complex logic in real app)
    let pending_payments_cents: i64 = 0;
    
    // Get revenue in last 30 days
    let revenue_30d_cents: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(amount_cents), 0) FROM payments 
             WHERE date(payment_date) >= date('now', '-30 days')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    // Generate occupancy history (mock for now - would need historical data table)
    let mut occupancy_history: Vec<OccupancyHistoryPoint> = Vec::new();
    let now = Utc::now();
    for i in (0..30).rev() {
        let date = now - Duration::days(i);
        occupancy_history.push(OccupancyHistoryPoint {
            date: date.format("%Y-%m-%d").to_string(),
            percent: occupancy_percent + (((i % 5) as f64 - 2.0) * 2.0), // Simulated variation
        });
    }
    
    Ok(DashboardData {
        total_lockers,
        occupied_lockers,
        occupancy_percent,
        by_size,
        by_location,
        overdue_returns,
        expiring_soon,
        damaged_lockers,
        pending_payments_cents,
        revenue_30d_cents,
        occupancy_history,
    })
}

/// Get status bar data
#[tauri::command]
pub async fn get_status_bar_data(db: State<'_, DbConnection>) -> Result<StatusBarData, String> {
    let conn = db.0.lock().await;
    
    let total_lockers: i64 = conn
        .query_row("SELECT COUNT(*) FROM lockers", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    let occupied_lockers: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT locker_id) FROM rentals 
             WHERE date(end_date) >= date('now') AND deposit_returned = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    
    let occupancy_percent = if total_lockers > 0 {
        (occupied_lockers as f64 / total_lockers as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(StatusBarData {
        db_status: "connected".to_string(),
        total_lockers,
        occupied_lockers,
        occupancy_percent,
    })
}
