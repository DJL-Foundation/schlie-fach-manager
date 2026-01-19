use color_eyre::eyre::Result;
use rusqlite::{Connection, params};

/// Represents an audit log entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditLogEntry {
    pub id: i64,
    pub timestamp: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub details: Option<String>,
    pub username: String,
}

/// Logs an action to the audit log.
pub fn log_action(
    conn: &Connection,
    action: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    details: Option<&str>,
    username: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO audit_log (action, entity_type, entity_id, details, username) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![action, entity_type, entity_id, details, username],
    )?;
    Ok(())
}

/// Returns all audit log entries.
pub fn list_audit_log(conn: &Connection) -> Result<Vec<AuditLogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, action, entity_type, entity_id, details, username 
         FROM audit_log 
         ORDER BY timestamp DESC"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(AuditLogEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            action: row.get(2)?,
            entity_type: row.get(3)?,
            entity_id: row.get(4)?,
            details: row.get(5)?,
            username: row.get(6)?,
        })
    })?;
    
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    
    Ok(entries)
}

/// Returns audit log entries for a specific entity.
pub fn list_audit_log_for_entity(
    conn: &Connection,
    entity_type: &str,
    entity_id: i64,
) -> Result<Vec<AuditLogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, action, entity_type, entity_id, details, username 
         FROM audit_log 
         WHERE entity_type = ?1 AND entity_id = ?2
         ORDER BY timestamp DESC"
    )?;
    
    let rows = stmt.query_map(params![entity_type, entity_id], |row| {
        Ok(AuditLogEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            action: row.get(2)?,
            entity_type: row.get(3)?,
            entity_id: row.get(4)?,
            details: row.get(5)?,
            username: row.get(6)?,
        })
    })?;
    
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    
    Ok(entries)
}

/// Returns audit log entries within a date range.
pub fn list_audit_log_in_range(
    conn: &Connection,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<AuditLogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, action, entity_type, entity_id, details, username 
         FROM audit_log 
         WHERE timestamp BETWEEN ?1 AND ?2
         ORDER BY timestamp DESC"
    )?;
    
    let rows = stmt.query_map(params![start_date, end_date], |row| {
        Ok(AuditLogEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            action: row.get(2)?,
            entity_type: row.get(3)?,
            entity_id: row.get(4)?,
            details: row.get(5)?,
            username: row.get(6)?,
        })
    })?;
    
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    
    Ok(entries)
}

/// Returns audit log entries by action type.
pub fn list_audit_log_by_action(conn: &Connection, action: &str) -> Result<Vec<AuditLogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, action, entity_type, entity_id, details, username 
         FROM audit_log 
         WHERE action = ?1
         ORDER BY timestamp DESC"
    )?;
    
    let rows = stmt.query_map(params![action], |row| {
        Ok(AuditLogEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            action: row.get(2)?,
            entity_type: row.get(3)?,
            entity_id: row.get(4)?,
            details: row.get(5)?,
            username: row.get(6)?,
        })
    })?;
    
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use rusqlite::Connection;

    #[test]
    fn test_log_action() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        log_action(
            &conn,
            "create",
            "locker",
            Some(42),
            Some("{\"number\": \"A-001\"}"),
            "admin",
        )?;

        let entries = list_audit_log(&conn)?;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].action, "create");
        assert_eq!(entries[0].entity_type, "locker");
        assert_eq!(entries[0].entity_id, Some(42));
        assert_eq!(entries[0].username, "admin");

        Ok(())
    }

    #[test]
    fn test_list_audit_log_for_entity() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        log_action(&conn, "create", "locker", Some(1), None, "system")?;
        log_action(&conn, "update", "locker", Some(1), None, "system")?;
        log_action(&conn, "delete", "locker", Some(1), None, "system")?;
        log_action(&conn, "create", "rental", Some(2), None, "system")?;

        let locker_entries = list_audit_log_for_entity(&conn, "locker", 1)?;
        assert_eq!(locker_entries.len(), 3);

        Ok(())
    }

    #[test]
    fn test_list_audit_log_by_action() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn)?;

        log_action(&conn, "create", "locker", Some(1), None, "system")?;
        log_action(&conn, "update", "locker", Some(1), None, "system")?;
        log_action(&conn, "create", "rental", Some(2), None, "system")?;

        let create_entries = list_audit_log_by_action(&conn, "create")?;
        assert_eq!(create_entries.len(), 2);

        Ok(())
    }
}
