/**
 * Occupancy History Functions
 * @module db/history
 */

import { DatabaseConnection } from './connection';
import type { OccupancyHistory } from '../types/database';
import { OccupancyHistorySchema } from './schema';
import { z } from 'zod';
import { format } from 'date-fns';

/**
 * Create a snapshot of current occupancy
 * @param notes Optional notes for the snapshot
 * @returns The created snapshot record
 */
export function createOccupancySnapshot(notes?: string): OccupancyHistory {
  const db = DatabaseConnection.getConnection();
  const snapshotDate = format(new Date(), 'yyyy-MM-dd');

  // Get current occupancy data
  const totalResult = db.prepare('SELECT COUNT(*) as count FROM lockers').get() as {
    count: number;
  };
  const totalLockers = totalResult.count;

  const occupiedResult = db
    .prepare(
      `
    SELECT COUNT(DISTINCT locker_id) as count
    FROM rentals
    WHERE end_date IS NULL OR date(end_date) >= date('now')
  `
    )
    .get() as { count: number };
  const occupiedLockers = occupiedResult.count;

  const occupancyPercent = totalLockers > 0 ? (occupiedLockers / totalLockers) * 100 : 0;

  // Insert or update snapshot for today
  db.prepare(
    `
    INSERT OR REPLACE INTO occupancy_history 
    (snapshot_date, total_lockers, occupied_lockers, occupancy_percent, notes)
    VALUES (?, ?, ?, ?, ?)
  `
  ).run(snapshotDate, totalLockers, occupiedLockers, occupancyPercent, notes || null);

  // Retrieve and return the created snapshot
  const result = db
    .prepare('SELECT * FROM occupancy_history WHERE snapshot_date = ?')
    .get(snapshotDate);

  return OccupancyHistorySchema.parse(result);
}

/**
 * Get occupancy history for a date range
 * @param days Number of days to look back
 * @returns Array of occupancy history entries
 */
export function getOccupancyHistory(days = 30): OccupancyHistory[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT * FROM occupancy_history
    WHERE date(snapshot_date) >= date('now', '-' || ? || ' days')
    ORDER BY snapshot_date ASC
  `
    )
    .all(days);

  return z.array(OccupancyHistorySchema).parse(results);
}

/**
 * Get the most recent occupancy snapshot
 * @returns Most recent snapshot or null if none exists
 */
export function getLatestOccupancySnapshot(): OccupancyHistory | null {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    SELECT * FROM occupancy_history
    ORDER BY snapshot_date DESC
    LIMIT 1
  `
    )
    .get();

  if (!result) return null;

  return OccupancyHistorySchema.parse(result);
}

/**
 * Get average occupancy for a period
 * @param days Number of days to calculate average for
 * @returns Average occupancy percentage
 */
export function getAverageOccupancy(days = 30): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    SELECT AVG(occupancy_percent) as avg
    FROM occupancy_history
    WHERE date(snapshot_date) >= date('now', '-' || ? || ' days')
  `
    )
    .get(days) as { avg: number | null };

  return result.avg ?? 0;
}

/**
 * Delete old occupancy history entries
 * @param keepDays Number of days to keep
 * @returns Number of deleted entries
 */
export function pruneOccupancyHistory(keepDays = 365): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    DELETE FROM occupancy_history
    WHERE date(snapshot_date) < date('now', '-' || ? || ' days')
  `
    )
    .run(keepDays);

  return result.changes;
}
