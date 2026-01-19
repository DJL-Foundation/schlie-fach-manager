/**
 * Locker Database Operations
 * @module db/lockers
 */

import { DatabaseConnection } from './connection';
import type { Locker, LockerSize } from '../types/database';
import { LockerSchema, CountResultSchema } from './schema';
import { z } from 'zod';
import { logAction } from './audit';

/**
 * Create a new locker
 * @param number Locker number (unique identifier)
 * @param location Location name
 * @param size Locker size
 * @param notes Optional notes
 * @returns The created locker
 */
export function createLocker(
  number: string,
  location: string,
  size: LockerSize,
  notes?: string
): Locker {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    INSERT INTO lockers (number, location, size, notes)
    VALUES (?, ?, ?, ?)
    RETURNING *
  `
    )
    .get(number, location, size, notes || null);

  const locker = LockerSchema.parse(result);

  logAction('create', 'locker', locker.id, { number, location, size });

  return locker;
}

/**
 * Get a locker by ID
 * @param id Locker ID
 * @returns Locker or null if not found
 */
export function getLockerById(id: number): Locker | null {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT * FROM lockers WHERE id = ?').get(id);

  if (!result) return null;

  return LockerSchema.parse(result);
}

/**
 * Get a locker by number
 * @param number Locker number
 * @returns Locker or null if not found
 */
export function getLockerByNumber(number: string): Locker | null {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT * FROM lockers WHERE number = ?').get(number);

  if (!result) return null;

  return LockerSchema.parse(result);
}

/**
 * Get all lockers
 * @returns Array of all lockers
 */
export function getAllLockers(): Locker[] {
  const db = DatabaseConnection.getConnection();

  const results = db.prepare('SELECT * FROM lockers ORDER BY location, number').all();

  return z.array(LockerSchema).parse(results);
}

/**
 * Get lockers by location
 * @param location Location name
 * @returns Array of lockers at the location
 */
export function getLockersByLocation(location: string): Locker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare('SELECT * FROM lockers WHERE location = ? ORDER BY number')
    .all(location);

  return z.array(LockerSchema).parse(results);
}

/**
 * Get lockers by size
 * @param size Locker size
 * @returns Array of lockers of the specified size
 */
export function getLockersBySize(size: LockerSize): Locker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare('SELECT * FROM lockers WHERE size = ? ORDER BY location, number')
    .all(size);

  return z.array(LockerSchema).parse(results);
}

/**
 * Get available (not currently rented) lockers
 * @returns Array of available lockers
 */
export function getAvailableLockers(): Locker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT l.* FROM lockers l
    WHERE l.is_damaged = 0
    AND l.id NOT IN (
      SELECT locker_id FROM rentals
      WHERE end_date IS NULL OR date(end_date) >= date('now')
    )
    ORDER BY l.location, l.number
  `
    )
    .all();

  return z.array(LockerSchema).parse(results);
}

/**
 * Get damaged lockers
 * @returns Array of damaged lockers
 */
export function getDamagedLockers(): Locker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare('SELECT * FROM lockers WHERE is_damaged = 1 ORDER BY location, number')
    .all();

  return z.array(LockerSchema).parse(results);
}

/**
 * Update locker damaged status
 * @param id Locker ID
 * @param isDamaged New damaged status
 * @param notes Optional damage notes
 * @returns Updated locker or null if not found
 */
export function updateLockerDamageStatus(
  id: number,
  isDamaged: boolean,
  notes?: string
): Locker | null {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    UPDATE lockers 
    SET is_damaged = ?, notes = COALESCE(?, notes)
    WHERE id = ?
    RETURNING *
  `
    )
    .get(isDamaged ? 1 : 0, notes || null, id);

  if (!result) return null;

  const locker = LockerSchema.parse(result);

  logAction('update', 'locker', id, { is_damaged: isDamaged, notes });

  return locker;
}

/**
 * Delete a locker
 * @param id Locker ID
 * @returns True if deleted, false if not found
 */
export function deleteLocker(id: number): boolean {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('DELETE FROM lockers WHERE id = ?').run(id);

  if (result.changes > 0) {
    logAction('delete', 'locker', id);
    return true;
  }

  return false;
}

/**
 * Get total locker count
 * @returns Total number of lockers
 */
export function getLockerCount(): number {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT COUNT(*) as count FROM lockers').get();

  return CountResultSchema.parse(result).count;
}

/**
 * Get unique locations
 * @returns Array of unique location names
 */
export function getUniqueLocations(): string[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare('SELECT DISTINCT location FROM lockers ORDER BY location')
    .all() as { location: string }[];

  return results.map((r) => r.location);
}

/**
 * Create multiple lockers in bulk
 * @param lockers Array of locker data to create
 * @returns Array of created lockers
 */
export function createLockersBulk(
  lockers: Array<{
    number: string;
    location: string;
    size: LockerSize;
    notes?: string;
  }>
): Locker[] {
  const db = DatabaseConnection.getConnection();
  const created: Locker[] = [];

  const insertStmt = db.prepare(`
    INSERT INTO lockers (number, location, size, notes)
    VALUES (?, ?, ?, ?)
    RETURNING *
  `);

  const transaction = db.transaction(() => {
    for (const locker of lockers) {
      const result = insertStmt.get(locker.number, locker.location, locker.size, locker.notes || null);
      const parsed = LockerSchema.parse(result);
      created.push(parsed);
    }
  });

  transaction();

  logAction('create', 'locker', null, {
    bulk: true,
    count: created.length,
    numbers: created.map((l) => l.number),
  });

  return created;
}
