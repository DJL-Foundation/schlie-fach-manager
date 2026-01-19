use crate::{
    config,
    model::{
        Debtor, Lease, Location, LocationStatistics, Locker, LockerHeight, LockerStatistics,
        LockerStatus, TenantType,
    },
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};
use rusqlite::types::Type;
use rusqlite::{Connection, OptionalExtension, Row, params};
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::path::Path;

const DB_FILENAME: &str = "schliessfach-manager.db";

/// High-level handle around the SQLite connection that powers the locker manager.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens (and creates if necessary) a database inside the platform specific data directory.
    pub fn open_default() -> Result<Self> {
        let path = config::database_path(DB_FILENAME)?;
        Self::open(path)
    }

    /// Opens a database located at `path`, creating parent directories and running migrations.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    /// Opens an ephemeral in-memory database, primarily intended for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.apply_migrations()?;
        Ok(db)
    }

    fn apply_migrations(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            -- Locations table
            CREATE TABLE IF NOT EXISTS locations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );

            -- Lockers table (physical locker units)
            CREATE TABLE IF NOT EXISTS lockers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                display_number TEXT NOT NULL,
                location_id INTEGER NOT NULL,
                height TEXT NOT NULL DEFAULT 'Middle',
                status TEXT NOT NULL DEFAULT 'Free',
                is_damaged INTEGER NOT NULL DEFAULT 0,
                note TEXT,
                FOREIGN KEY (location_id) REFERENCES locations(id)
            );

            -- Leases table (rental history)
            CREATE TABLE IF NOT EXISTS leases (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                locker_id INTEGER NOT NULL,
                tenant_username TEXT NOT NULL,
                tenant_type TEXT NOT NULL,
                start_date TEXT NOT NULL,
                end_date TEXT NOT NULL,
                deposit_paid INTEGER NOT NULL DEFAULT 0,
                yearly_fee_paid_until TEXT NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 1,
                FOREIGN KEY (locker_id) REFERENCES lockers(id)
            );

            -- Create indexes for common queries
            CREATE INDEX IF NOT EXISTS idx_lockers_location ON lockers(location_id);
            CREATE INDEX IF NOT EXISTS idx_lockers_status ON lockers(status);
            CREATE INDEX IF NOT EXISTS idx_leases_locker ON leases(locker_id);
            CREATE INDEX IF NOT EXISTS idx_leases_active ON leases(is_active);
            CREATE INDEX IF NOT EXISTS idx_leases_username ON leases(tenant_username);
            "#,
        )?;
        Ok(())
    }

    // ========== Location Operations ==========

    /// Insert a new location
    pub fn insert_location(&self, name: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO locations (name) VALUES (?1)",
            params![name],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Get all locations
    pub fn list_locations(&self) -> Result<Vec<Location>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM locations ORDER BY name ASC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Location {
                id: row.get("id")?,
                name: row.get("name")?,
            })
        })?;
        let mut locations = Vec::new();
        for loc in rows {
            locations.push(loc?);
        }
        Ok(locations)
    }

    /// Get a location by ID
    pub fn get_location(&self, id: i64) -> Result<Option<Location>> {
        let location = self
            .conn
            .query_row(
                "SELECT id, name FROM locations WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Location {
                        id: row.get("id")?,
                        name: row.get("name")?,
                    })
                },
            )
            .optional()?;
        Ok(location)
    }

    /// Update a location's name
    pub fn update_location(&self, id: i64, name: &str) -> Result<bool> {
        let affected = self.conn.execute(
            "UPDATE locations SET name = ?1 WHERE id = ?2",
            params![name, id],
        )?;
        Ok(affected > 0)
    }

    /// Delete a location (fails if lockers exist)
    pub fn delete_location(&self, id: i64) -> Result<bool> {
        // Check if lockers exist for this location
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM lockers WHERE location_id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        if count > 0 {
            return Err(eyre!("Cannot delete location with existing lockers"));
        }
        let affected = self.conn.execute("DELETE FROM locations WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    // ========== Locker Operations ==========

    /// Insert a new locker
    pub fn insert_locker(&self, locker: &Locker) -> Result<i64> {
        self.conn.execute(
            r#"
            INSERT INTO lockers (display_number, location_id, height, status, is_damaged, note)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                &locker.display_number,
                locker.location_id,
                locker.height.to_db_value(),
                locker.status.to_db_value(),
                locker.is_damaged as i32,
                locker.note.as_ref(),
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Update an existing locker
    pub fn update_locker(&self, locker: &Locker) -> Result<bool> {
        let affected = self.conn.execute(
            r#"
            UPDATE lockers SET 
                display_number = ?1,
                location_id = ?2,
                height = ?3,
                status = ?4,
                is_damaged = ?5,
                note = ?6
            WHERE id = ?7
            "#,
            params![
                &locker.display_number,
                locker.location_id,
                locker.height.to_db_value(),
                locker.status.to_db_value(),
                locker.is_damaged as i32,
                locker.note.as_ref(),
                locker.id,
            ],
        )?;
        Ok(affected > 0)
    }

    /// Get a locker by ID with location name and active tenant
    pub fn get_locker(&self, id: i64) -> Result<Option<Locker>> {
        let locker = self
            .conn
            .query_row(
                r#"
                SELECT l.id, l.display_number, l.location_id, l.height, l.status, 
                       l.is_damaged, l.note, loc.name as location_name,
                       (SELECT tenant_username FROM leases WHERE locker_id = l.id AND is_active = 1 LIMIT 1) as tenant_username
                FROM lockers l
                JOIN locations loc ON l.location_id = loc.id
                WHERE l.id = ?1
                "#,
                params![id],
                row_to_locker,
            )
            .optional()?;
        Ok(locker)
    }

    /// List all lockers with their location names and active tenants
    pub fn list_lockers(&self) -> Result<Vec<Locker>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT l.id, l.display_number, l.location_id, l.height, l.status, 
                   l.is_damaged, l.note, loc.name as location_name,
                   (SELECT tenant_username FROM leases WHERE locker_id = l.id AND is_active = 1 LIMIT 1) as tenant_username
            FROM lockers l
            JOIN locations loc ON l.location_id = loc.id
            ORDER BY loc.name ASC, l.display_number ASC
            "#,
        )?;
        let rows = stmt.query_map([], row_to_locker)?;
        let mut lockers = Vec::new();
        for locker in rows {
            lockers.push(locker?);
        }
        Ok(lockers)
    }

    /// List free lockers by location and optionally height
    pub fn list_free_lockers(&self, location_id: Option<i64>, height: Option<LockerHeight>) -> Result<Vec<Locker>> {
        let mut query = String::from(
            r#"
            SELECT l.id, l.display_number, l.location_id, l.height, l.status, 
                   l.is_damaged, l.note, loc.name as location_name,
                   NULL as tenant_username
            FROM lockers l
            JOIN locations loc ON l.location_id = loc.id
            WHERE l.status = 'Free' AND l.is_damaged = 0
            "#,
        );

        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(loc_id) = location_id {
            query.push_str(" AND l.location_id = ?");
            params_vec.push(Box::new(loc_id));
        }

        if let Some(h) = height {
            query.push_str(" AND l.height = ?");
            params_vec.push(Box::new(h.to_db_value().to_string()));
        }

        query.push_str(" ORDER BY l.display_number ASC");

        let mut stmt = self.conn.prepare(&query)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(params_refs.as_slice(), row_to_locker)?;
        
        let mut lockers = Vec::new();
        for locker in rows {
            lockers.push(locker?);
        }
        Ok(lockers)
    }

    /// Delete a locker (only if not occupied)
    pub fn delete_locker(&self, id: i64) -> Result<bool> {
        // Check for active leases
        let active_lease: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM leases WHERE locker_id = ?1 AND is_active = 1",
            params![id],
            |row| row.get(0),
        )?;
        if active_lease > 0 {
            return Err(eyre!("Cannot delete locker with active lease"));
        }
        let affected = self.conn.execute("DELETE FROM lockers WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    /// Bulk create lockers with a prefix and number range
    pub fn bulk_create_lockers(
        &self,
        location_id: i64,
        prefix: &str,
        start: i64,
        end: i64,
        height: LockerHeight,
    ) -> Result<i64> {
        let mut count = 0;
        for num in start..=end {
            let display_number = format!("{}{}", prefix, num);
            self.conn.execute(
                r#"
                INSERT INTO lockers (display_number, location_id, height, status, is_damaged)
                VALUES (?1, ?2, ?3, 'Free', 0)
                "#,
                params![&display_number, location_id, height.to_db_value()],
            )?;
            count += 1;
        }
        Ok(count)
    }

    // ========== Lease Operations ==========

    /// Create a new lease (rent a locker)
    pub fn create_lease(&self, lease: &Lease) -> Result<i64> {
        // First update the locker status
        self.conn.execute(
            "UPDATE lockers SET status = 'Occupied' WHERE id = ?1",
            params![lease.locker_id],
        )?;

        // Create the lease
        self.conn.execute(
            r#"
            INSERT INTO leases (locker_id, tenant_username, tenant_type, start_date, end_date, 
                               deposit_paid, yearly_fee_paid_until, is_active)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1)
            "#,
            params![
                lease.locker_id,
                &lease.tenant_username,
                lease.tenant_type.to_db_value(),
                lease.start_date.to_rfc3339(),
                lease.end_date.to_rfc3339(),
                lease.deposit_paid as i32,
                lease.yearly_fee_paid_until.to_rfc3339(),
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Get the active lease for a locker
    pub fn get_active_lease(&self, locker_id: i64) -> Result<Option<Lease>> {
        let lease = self
            .conn
            .query_row(
                r#"
                SELECT l.id, l.locker_id, l.tenant_username, l.tenant_type, l.start_date, 
                       l.end_date, l.deposit_paid, l.yearly_fee_paid_until, l.is_active,
                       loc.display_number as locker_display_number
                FROM leases l
                JOIN lockers loc ON l.locker_id = loc.id
                WHERE l.locker_id = ?1 AND l.is_active = 1
                "#,
                params![locker_id],
                row_to_lease,
            )
            .optional()?;
        Ok(lease)
    }

    /// Get a lease by username
    pub fn get_lease_by_username(&self, username: &str) -> Result<Option<Lease>> {
        let lease = self
            .conn
            .query_row(
                r#"
                SELECT l.id, l.locker_id, l.tenant_username, l.tenant_type, l.start_date, 
                       l.end_date, l.deposit_paid, l.yearly_fee_paid_until, l.is_active,
                       loc.display_number as locker_display_number
                FROM leases l
                JOIN lockers loc ON l.locker_id = loc.id
                WHERE l.tenant_username = ?1 AND l.is_active = 1
                "#,
                params![username],
                row_to_lease,
            )
            .optional()?;
        Ok(lease)
    }

    /// Extend a lease
    pub fn extend_lease(&self, lease_id: i64, new_end_date: DateTime<Utc>) -> Result<bool> {
        let affected = self.conn.execute(
            r#"
            UPDATE leases SET 
                end_date = ?1,
                yearly_fee_paid_until = ?1
            WHERE id = ?2
            "#,
            params![new_end_date.to_rfc3339(), lease_id],
        )?;
        Ok(affected > 0)
    }

    /// End a lease (return locker)
    pub fn end_lease(&self, lease_id: i64, locker_id: i64) -> Result<bool> {
        // Deactivate the lease
        let affected = self.conn.execute(
            "UPDATE leases SET is_active = 0 WHERE id = ?1",
            params![lease_id],
        )?;

        if affected > 0 {
            // Free the locker
            self.conn.execute(
                "UPDATE lockers SET status = 'Free' WHERE id = ?1",
                params![locker_id],
            )?;
        }

        Ok(affected > 0)
    }

    /// List all active leases
    pub fn list_active_leases(&self) -> Result<Vec<Lease>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT l.id, l.locker_id, l.tenant_username, l.tenant_type, l.start_date, 
                   l.end_date, l.deposit_paid, l.yearly_fee_paid_until, l.is_active,
                   loc.display_number as locker_display_number
            FROM leases l
            JOIN lockers loc ON l.locker_id = loc.id
            WHERE l.is_active = 1
            ORDER BY l.end_date ASC
            "#,
        )?;
        let rows = stmt.query_map([], row_to_lease)?;
        let mut leases = Vec::new();
        for lease in rows {
            leases.push(lease?);
        }
        Ok(leases)
    }

    /// List overdue leases (for debtor list)
    pub fn list_overdue_leases(&self, current_date: DateTime<Utc>) -> Result<Vec<Debtor>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT l.tenant_username, l.end_date, loc.display_number
            FROM leases l
            JOIN lockers loc ON l.locker_id = loc.id
            WHERE l.is_active = 1 AND l.end_date < ?1
            ORDER BY l.end_date ASC
            "#,
        )?;
        let rows = stmt.query_map(params![current_date.to_rfc3339()], |row| {
            let username: String = row.get("tenant_username")?;
            let end_date_str: String = row.get("end_date")?;
            let display_number: String = row.get("display_number")?;
            Ok((username, end_date_str, display_number))
        })?;

        let mut debtors = Vec::new();
        for result in rows {
            let (username, end_date_str, display_number) = result?;
            if let Ok(end_date) = DateTime::parse_from_rfc3339(&end_date_str) {
                let end_date = end_date.with_timezone(&Utc);
                let days_overdue = current_date.signed_duration_since(end_date).num_days();
                let years_overdue = (days_overdue as f64 / 365.0).ceil() as i64;
                let amount_owed = years_overdue * 10;

                debtors.push(Debtor {
                    email: format!("{}@athenetz.de", username),
                    username: username.clone(),
                    amount_owed,
                    locker_display_number: display_number,
                    days_overdue,
                });
            }
        }
        Ok(debtors)
    }

    // ========== Statistics ==========

    /// Get overall locker statistics
    pub fn get_locker_statistics(&self) -> Result<LockerStatistics> {
        let mut stats = LockerStatistics::default();

        let mut stmt = self.conn.prepare(
            "SELECT status, is_damaged, COUNT(*) as count FROM lockers GROUP BY status, is_damaged",
        )?;
        let rows = stmt.query_map([], |row| {
            let status: String = row.get("status")?;
            let is_damaged: i32 = row.get("is_damaged")?;
            let count: i64 = row.get("count")?;
            Ok((status, is_damaged, count))
        })?;

        for result in rows {
            let (status, is_damaged, count) = result?;
            let count = count as usize;
            stats.total += count;

            if is_damaged != 0 {
                stats.damaged += count;
            }

            match status.as_str() {
                "Free" => stats.free += count,
                "Occupied" => stats.occupied += count,
                "Maintenance" => stats.maintenance += count,
                _ => {}
            }
        }

        Ok(stats)
    }

    /// Get statistics per location
    pub fn get_location_statistics(&self) -> Result<Vec<LocationStatistics>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT loc.id, loc.name, l.status, COUNT(*) as count
            FROM locations loc
            LEFT JOIN lockers l ON loc.id = l.location_id
            GROUP BY loc.id, loc.name, l.status
            ORDER BY loc.name
            "#,
        )?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get("id")?;
            let name: String = row.get("name")?;
            let status: Option<String> = row.get("status")?;
            let count: i64 = row.get("count")?;
            Ok((id, name, status, count))
        })?;

        let mut stats_map: std::collections::HashMap<i64, LocationStatistics> =
            std::collections::HashMap::new();

        for result in rows {
            let (id, name, status, count) = result?;
            let entry = stats_map.entry(id).or_insert_with(|| LocationStatistics {
                location_id: id,
                location_name: name.clone(),
                total: 0,
                free: 0,
                occupied: 0,
            });

            let count = count as usize;
            if status.is_some() {
                entry.total += count;
                match status.as_deref() {
                    Some("Free") => entry.free += count,
                    Some("Occupied") => entry.occupied += count,
                    _ => {}
                }
            }
        }

        let mut stats: Vec<LocationStatistics> = stats_map.into_values().collect();
        stats.sort_by(|a, b| a.location_name.cmp(&b.location_name));
        Ok(stats)
    }

    /// Count lockers
    pub fn count_lockers(&self) -> Result<i64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(1) FROM lockers", [], |row| row.get(0))?;
        Ok(count)
    }

    /// Seed initial data if empty
    pub fn seed_if_empty(&self, locations: &[&str]) -> Result<()> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(1) FROM locations", [], |row| row.get(0))?;

        if count == 0 {
            for name in locations {
                self.insert_location(name)?;
            }
        }
        Ok(())
    }

    /// Access to the underlying connection for advanced operations
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

fn row_to_locker(row: &Row) -> rusqlite::Result<Locker> {
    let status_str: String = row.get("status")?;
    let status = LockerStatus::from_db_value(&status_str).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            Type::Text,
            Box::new(DecodeError(format!("invalid status: {}", status_str))),
        )
    })?;

    let height_str: String = row.get("height")?;
    let height = LockerHeight::from_db_value(&height_str).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            Type::Text,
            Box::new(DecodeError(format!("invalid height: {}", height_str))),
        )
    })?;

    let is_damaged: i32 = row.get("is_damaged")?;

    Ok(Locker {
        id: row.get("id")?,
        display_number: row.get("display_number")?,
        location_id: row.get("location_id")?,
        height,
        status,
        is_damaged: is_damaged != 0,
        note: row.get("note")?,
        tenant_username: row.get("tenant_username")?,
        location_name: row.get("location_name")?,
    })
}

fn row_to_lease(row: &Row) -> rusqlite::Result<Lease> {
    let tenant_type_str: String = row.get("tenant_type")?;
    let tenant_type = TenantType::from_db_value(&tenant_type_str).ok_or_else(|| {
        rusqlite::Error::FromSqlConversionFailure(
            0,
            Type::Text,
            Box::new(DecodeError(format!("invalid tenant type: {}", tenant_type_str))),
        )
    })?;

    let start_date_str: String = row.get("start_date")?;
    let start_date = DateTime::parse_from_rfc3339(&start_date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                Type::Text,
                Box::new(DecodeError(format!("invalid start_date: {}", e))),
            )
        })?;

    let end_date_str: String = row.get("end_date")?;
    let end_date = DateTime::parse_from_rfc3339(&end_date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                Type::Text,
                Box::new(DecodeError(format!("invalid end_date: {}", e))),
            )
        })?;

    let yearly_fee_str: String = row.get("yearly_fee_paid_until")?;
    let yearly_fee_paid_until = DateTime::parse_from_rfc3339(&yearly_fee_str)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                Type::Text,
                Box::new(DecodeError(format!("invalid yearly_fee_paid_until: {}", e))),
            )
        })?;

    let deposit_paid: i32 = row.get("deposit_paid")?;
    let is_active: i32 = row.get("is_active")?;

    Ok(Lease {
        id: row.get("id")?,
        locker_id: row.get("locker_id")?,
        tenant_username: row.get("tenant_username")?,
        tenant_type,
        start_date,
        end_date,
        deposit_paid: deposit_paid != 0,
        yearly_fee_paid_until,
        is_active: is_active != 0,
        locker_display_number: row.get("locker_display_number")?,
    })
}

#[derive(Debug)]
struct DecodeError(String);

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for DecodeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn setup_db() -> Database {
        let db = Database::open_in_memory().expect("in-memory db");
        db.insert_location("Hauptgebäude").expect("insert location");
        db.insert_location("Sporthalle").expect("insert location");
        db
    }

    #[test]
    fn location_crud() -> Result<()> {
        let db = Database::open_in_memory()?;
        
        let id = db.insert_location("Test Location")?;
        assert!(id > 0);

        let loc = db.get_location(id)?.expect("location should exist");
        assert_eq!(loc.name, "Test Location");

        db.update_location(id, "Updated Location")?;
        let loc = db.get_location(id)?.expect("location should exist");
        assert_eq!(loc.name, "Updated Location");

        db.delete_location(id)?;
        assert!(db.get_location(id)?.is_none());

        Ok(())
    }

    #[test]
    fn locker_crud() -> Result<()> {
        let db = setup_db();
        let locations = db.list_locations()?;
        let location_id = locations[0].id;

        let locker = Locker::new(0, "A-01", location_id, LockerHeight::Middle);
        let id = db.insert_locker(&locker)?;
        assert!(id > 0);

        let fetched = db.get_locker(id)?.expect("locker should exist");
        assert_eq!(fetched.display_number, "A-01");
        assert_eq!(fetched.status, LockerStatus::Free);

        let mut updated = fetched.clone();
        updated.note = Some("Test note".into());
        db.update_locker(&updated)?;

        let fetched = db.get_locker(id)?.expect("locker should exist");
        assert_eq!(fetched.note.as_deref(), Some("Test note"));

        Ok(())
    }

    #[test]
    fn bulk_create_lockers() -> Result<()> {
        let db = setup_db();
        let locations = db.list_locations()?;
        let location_id = locations[0].id;

        let count = db.bulk_create_lockers(location_id, "B-", 1, 5, LockerHeight::Top)?;
        assert_eq!(count, 5);

        let lockers = db.list_lockers()?;
        assert_eq!(lockers.len(), 5);

        Ok(())
    }

    #[test]
    fn lease_workflow() -> Result<()> {
        let db = setup_db();
        let locations = db.list_locations()?;
        let location_id = locations[0].id;

        // Create a locker
        let locker = Locker::new(0, "A-01", location_id, LockerHeight::Middle);
        let locker_id = db.insert_locker(&locker)?;

        // Create a lease
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let mut lease = Lease::new(0, locker_id, "max.mustermann", TenantType::Student, start, end);
        lease.deposit_paid = true;

        let lease_id = db.create_lease(&lease)?;
        assert!(lease_id > 0);

        // Check locker is now occupied
        let locker = db.get_locker(locker_id)?.expect("locker should exist");
        assert_eq!(locker.status, LockerStatus::Occupied);
        assert_eq!(locker.tenant_username.as_deref(), Some("max.mustermann"));

        // Get active lease
        let active = db.get_active_lease(locker_id)?.expect("lease should exist");
        assert_eq!(active.tenant_username, "max.mustermann");

        // Extend lease
        let new_end = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        db.extend_lease(lease_id, new_end)?;

        let active = db.get_active_lease(locker_id)?.expect("lease should exist");
        assert_eq!(active.end_date, new_end);

        // End lease
        db.end_lease(lease_id, locker_id)?;

        let locker = db.get_locker(locker_id)?.expect("locker should exist");
        assert_eq!(locker.status, LockerStatus::Free);

        Ok(())
    }

    #[test]
    fn statistics() -> Result<()> {
        let db = setup_db();
        let locations = db.list_locations()?;
        let loc1_id = locations[0].id;
        let loc2_id = locations[1].id;

        // Create lockers
        for i in 1..=5 {
            let locker = Locker::new(0, format!("A-{:02}", i), loc1_id, LockerHeight::Middle);
            db.insert_locker(&locker)?;
        }
        for i in 1..=3 {
            let locker = Locker::new(0, format!("B-{:02}", i), loc2_id, LockerHeight::Top);
            db.insert_locker(&locker)?;
        }

        // Create a lease for one locker
        let lockers = db.list_lockers()?;
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let lease = Lease::new(0, lockers[0].id, "test.user", TenantType::Student, start, end);
        db.create_lease(&lease)?;

        let stats = db.get_locker_statistics()?;
        assert_eq!(stats.total, 8);
        assert_eq!(stats.occupied, 1);
        assert_eq!(stats.free, 7);

        let loc_stats = db.get_location_statistics()?;
        assert_eq!(loc_stats.len(), 2);

        Ok(())
    }

    #[test]
    fn find_lease_by_username() -> Result<()> {
        let db = setup_db();
        let locations = db.list_locations()?;
        let location_id = locations[0].id;

        let locker = Locker::new(0, "A-01", location_id, LockerHeight::Middle);
        let locker_id = db.insert_locker(&locker)?;

        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let lease = Lease::new(0, locker_id, "anna.schmidt", TenantType::Student, start, end);
        db.create_lease(&lease)?;

        let found = db.get_lease_by_username("anna.schmidt")?.expect("lease should exist");
        assert_eq!(found.locker_display_number.as_deref(), Some("A-01"));

        assert!(db.get_lease_by_username("unknown.user")?.is_none());

        Ok(())
    }
}
