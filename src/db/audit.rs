use crate::model::AuditEntry;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row, params};

/// Inserts an audit entry into the log.
pub fn log_action(
    conn: &Connection,
    action: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    details: &str,
    username: &str,
) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO audit_log (action, entity_type, entity_id, details, username)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![action, entity_type, entity_id, details, username],
    )?;
    Ok(())
}

/// Returns audit entries ordered by newest first.
pub fn list_audit_entries(conn: &Connection) -> Result<Vec<AuditEntry>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, timestamp, action, entity_type, entity_id, details, username
        FROM audit_log
        ORDER BY timestamp DESC
        "#,
    )?;
    let rows = stmt.query_map([], row_to_audit)?;
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}

/// Maps a database row to an audit entry.
fn row_to_audit(row: &Row<'_>) -> rusqlite::Result<AuditEntry> {
    let timestamp: String = row.get("timestamp")?;
    Ok(AuditEntry {
        id: row.get("id")?,
        timestamp: parse_datetime(&timestamp),
        action: row.get("action")?,
        entity_type: row.get("entity_type")?,
        entity_id: row.get("entity_id")?,
        details: row.get("details")?,
        username: row.get("username")?,
    })
}

/// Parses a RFC3339 timestamp or returns the current time.
fn parse_datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::Database;

    #[test]
    fn audit_log_inserts_entries() -> Result<()> {
        let db = Database::open_in_memory()?;
        log_action(db.connection(), "create", "locker", Some(1), "{}", "test")?;
        let entries = list_audit_entries(db.connection())?;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].action, "create");
        Ok(())
    }
}
