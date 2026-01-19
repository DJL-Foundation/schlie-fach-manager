//! Audit logging for the Schließfach-Manager system.
//!
//! This module provides functions for logging and retrieving audit entries
//! that track all significant actions in the system.

use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use rusqlite::{params, Connection, Row};

/// Represents an entry in the audit log.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    /// Unique identifier for the audit entry.
    pub id: i64,
    /// When the action occurred.
    pub timestamp: DateTime<Utc>,
    /// The action that was performed (e.g., "CREATE", "UPDATE", "DELETE").
    pub action: String,
    /// The type of entity affected (e.g., "locker", "rental", "payment").
    pub entity_type: String,
    /// The ID of the affected entity (if applicable).
    pub entity_id: Option<i64>,
    /// Additional details about the action.
    pub details: Option<String>,
    /// The username who performed the action.
    pub username: Option<String>,
}

/// Logs an action to the audit log.
pub fn log_action(
    conn: &Connection,
    action: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    details: Option<&str>,
    username: Option<&str>,
) -> Result<()> {
    let timestamp = Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO audit_log (timestamp, action, entity_type, entity_id, details, username)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![timestamp, action, entity_type, entity_id, details, username],
    )?;

    Ok(())
}

/// Retrieves recent audit log entries, up to the specified limit.
pub fn get_audit_log(conn: &Connection, limit: u32) -> Result<Vec<AuditEntry>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, timestamp, action, entity_type, entity_id, details, username
        FROM audit_log
        ORDER BY timestamp DESC
        LIMIT ?1
        "#,
    )?;

    let rows = stmt.query_map(params![limit], row_to_audit_entry)?;

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }

    Ok(entries)
}

/// Retrieves audit entries filtered by entity type.
pub fn get_audit_log_by_entity_type(
    conn: &Connection,
    entity_type: &str,
    limit: u32,
) -> Result<Vec<AuditEntry>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, timestamp, action, entity_type, entity_id, details, username
        FROM audit_log
        WHERE entity_type = ?1
        ORDER BY timestamp DESC
        LIMIT ?2
        "#,
    )?;

    let rows = stmt.query_map(params![entity_type, limit], row_to_audit_entry)?;

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }

    Ok(entries)
}

/// Retrieves audit entries for a specific entity.
pub fn get_audit_log_for_entity(
    conn: &Connection,
    entity_type: &str,
    entity_id: i64,
) -> Result<Vec<AuditEntry>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, timestamp, action, entity_type, entity_id, details, username
        FROM audit_log
        WHERE entity_type = ?1 AND entity_id = ?2
        ORDER BY timestamp DESC
        "#,
    )?;

    let rows = stmt.query_map(params![entity_type, entity_id], row_to_audit_entry)?;

    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }

    Ok(entries)
}

/// Returns the count of audit log entries.
pub fn count_audit_entries(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row("SELECT COUNT(1) FROM audit_log", [], |row| row.get(0))?;
    Ok(count)
}

fn row_to_audit_entry(row: &Row) -> rusqlite::Result<AuditEntry> {
    let timestamp_str: String = row.get("timestamp")?;
    let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    Ok(AuditEntry {
        id: row.get("id")?,
        timestamp,
        action: row.get("action")?,
        entity_type: row.get("entity_type")?,
        entity_id: row.get("entity_id")?,
        details: row.get("details")?,
        username: row.get("username")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::apply_migrations;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_log_action() -> Result<()> {
        let conn = setup_db();
        log_action(
            &conn,
            "CREATE",
            "locker",
            Some(1),
            Some("Created locker A-01"),
            Some("admin"),
        )?;

        let count = count_audit_entries(&conn)?;
        assert_eq!(count, 1);
        Ok(())
    }

    #[test]
    fn test_get_audit_log() -> Result<()> {
        let conn = setup_db();
        log_action(&conn, "CREATE", "locker", Some(1), None, Some("admin"))?;
        log_action(&conn, "UPDATE", "locker", Some(1), None, Some("admin"))?;
        log_action(&conn, "DELETE", "rental", Some(5), None, Some("user"))?;

        let entries = get_audit_log(&conn, 10)?;
        assert_eq!(entries.len(), 3);
        // Most recent first
        assert_eq!(entries[0].action, "DELETE");
        assert_eq!(entries[1].action, "UPDATE");
        assert_eq!(entries[2].action, "CREATE");
        Ok(())
    }

    #[test]
    fn test_get_audit_log_limit() -> Result<()> {
        let conn = setup_db();
        for i in 0..10 {
            log_action(&conn, "TEST", "entity", Some(i), None, None)?;
        }

        let entries = get_audit_log(&conn, 5)?;
        assert_eq!(entries.len(), 5);
        Ok(())
    }

    #[test]
    fn test_get_audit_log_by_entity_type() -> Result<()> {
        let conn = setup_db();
        log_action(&conn, "CREATE", "locker", Some(1), None, None)?;
        log_action(&conn, "CREATE", "rental", Some(1), None, None)?;
        log_action(&conn, "CREATE", "locker", Some(2), None, None)?;

        let entries = get_audit_log_by_entity_type(&conn, "locker", 10)?;
        assert_eq!(entries.len(), 2);
        Ok(())
    }

    #[test]
    fn test_get_audit_log_for_entity() -> Result<()> {
        let conn = setup_db();
        log_action(&conn, "CREATE", "locker", Some(1), None, None)?;
        log_action(&conn, "UPDATE", "locker", Some(1), None, None)?;
        log_action(&conn, "CREATE", "locker", Some(2), None, None)?;

        let entries = get_audit_log_for_entity(&conn, "locker", 1)?;
        assert_eq!(entries.len(), 2);
        Ok(())
    }

    #[test]
    fn test_log_action_without_optional_fields() -> Result<()> {
        let conn = setup_db();
        log_action(&conn, "SYSTEM", "backup", None, None, None)?;

        let entries = get_audit_log(&conn, 1)?;
        assert_eq!(entries.len(), 1);
        assert!(entries[0].entity_id.is_none());
        assert!(entries[0].details.is_none());
        assert!(entries[0].username.is_none());
        Ok(())
    }
}
