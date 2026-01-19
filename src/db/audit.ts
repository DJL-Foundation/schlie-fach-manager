/**
 * Audit Logging Functions
 * @module db/audit
 */

import { DatabaseConnection } from './connection';
import type { AuditLog, AuditAction, AuditEntityType } from '../types/database';
import { AuditLogSchema } from './schema';
import { z } from 'zod';

/**
 * Log an action to the audit log
 * @param action Type of action performed
 * @param entityType Type of entity affected
 * @param entityId ID of the affected entity (can be null)
 * @param details Additional details as key-value pairs
 * @param username User who performed the action
 */
export function logAction(
  action: AuditAction,
  entityType: AuditEntityType,
  entityId: number | null,
  details?: Record<string, unknown>,
  username = 'system'
): void {
  const db = DatabaseConnection.getConnection();

  db.prepare(
    `
    INSERT INTO audit_log (action, entity_type, entity_id, details, username)
    VALUES (?, ?, ?, ?, ?)
  `
  ).run(action, entityType, entityId, details ? JSON.stringify(details) : null, username);
}

/**
 * Get recent audit log entries
 * @param limit Maximum number of entries to return
 * @returns Array of audit log entries
 */
export function getAuditLog(limit = 100): AuditLog[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT * FROM audit_log
    ORDER BY timestamp DESC
    LIMIT ?
  `
    )
    .all(limit);

  return z.array(AuditLogSchema).parse(results);
}

/**
 * Get audit log entries for a specific entity
 * @param entityType Type of entity
 * @param entityId ID of the entity
 * @returns Array of audit log entries for the entity
 */
export function getAuditLogForEntity(entityType: AuditEntityType, entityId: number): AuditLog[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT * FROM audit_log
    WHERE entity_type = ? AND entity_id = ?
    ORDER BY timestamp DESC
  `
    )
    .all(entityType, entityId);

  return z.array(AuditLogSchema).parse(results);
}

/**
 * Get audit log entries filtered by action type
 * @param action Action type to filter by
 * @param limit Maximum number of entries to return
 * @returns Array of filtered audit log entries
 */
export function getAuditLogByAction(action: AuditAction, limit = 100): AuditLog[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT * FROM audit_log
    WHERE action = ?
    ORDER BY timestamp DESC
    LIMIT ?
  `
    )
    .all(action, limit);

  return z.array(AuditLogSchema).parse(results);
}

/**
 * Get audit log entries within a date range
 * @param startDate Start date (ISO string)
 * @param endDate End date (ISO string)
 * @returns Array of audit log entries within range
 */
export function getAuditLogByDateRange(startDate: string, endDate: string): AuditLog[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT * FROM audit_log
    WHERE timestamp >= ? AND timestamp <= ?
    ORDER BY timestamp DESC
  `
    )
    .all(startDate, endDate);

  return z.array(AuditLogSchema).parse(results);
}

/**
 * Count total audit log entries
 * @returns Total number of audit log entries
 */
export function getAuditLogCount(): number {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT COUNT(*) as count FROM audit_log').get() as { count: number };

  return result.count;
}
