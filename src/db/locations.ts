/**
 * Location Database Operations
 * @module db/locations
 */

import { DatabaseConnection } from './connection';
import type { Location } from '../types/database';
import { LocationSchema } from './schema';
import { z } from 'zod';
import { logAction } from './audit';

/**
 * Create a new location
 * @param name Location name
 * @returns The created location
 */
export function createLocation(name: string): Location {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    INSERT INTO locations (name)
    VALUES (?)
    RETURNING *
  `
    )
    .get(name);

  const location = LocationSchema.parse(result);

  logAction('create', 'location', location.id, { name });

  return location;
}

/**
 * Get a location by ID
 * @param id Location ID
 * @returns Location or null if not found
 */
export function getLocationById(id: number): Location | null {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT * FROM locations WHERE id = ?').get(id);

  if (!result) return null;

  return LocationSchema.parse(result);
}

/**
 * Get a location by name
 * @param name Location name
 * @returns Location or null if not found
 */
export function getLocationByName(name: string): Location | null {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT * FROM locations WHERE name = ?').get(name);

  if (!result) return null;

  return LocationSchema.parse(result);
}

/**
 * Get all locations
 * @returns Array of all locations
 */
export function getAllLocations(): Location[] {
  const db = DatabaseConnection.getConnection();

  const results = db.prepare('SELECT * FROM locations ORDER BY name').all();

  return z.array(LocationSchema).parse(results);
}

/**
 * Update a location name
 * @param id Location ID
 * @param name New name
 * @returns Updated location or null if not found
 */
export function updateLocationName(id: number, name: string): Location | null {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    UPDATE locations SET name = ? WHERE id = ?
    RETURNING *
  `
    )
    .get(name, id);

  if (!result) return null;

  const location = LocationSchema.parse(result);

  logAction('update', 'location', id, { name });

  return location;
}

/**
 * Delete a location
 * @param id Location ID
 * @returns True if deleted, false if not found
 */
export function deleteLocation(id: number): boolean {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('DELETE FROM locations WHERE id = ?').run(id);

  if (result.changes > 0) {
    logAction('delete', 'location', id);
    return true;
  }

  return false;
}

/**
 * Get or create a location by name
 * @param name Location name
 * @returns Existing or newly created location
 */
export function getOrCreateLocation(name: string): Location {
  const existing = getLocationByName(name);

  if (existing) return existing;

  return createLocation(name);
}

/**
 * Get location statistics
 * @returns Array of locations with locker counts
 */
export function getLocationStatistics(): Array<{
  name: string;
  totalLockers: number;
  occupiedLockers: number;
  availableLockers: number;
}> {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT 
      l.location as name,
      COUNT(*) as total_lockers,
      COUNT(CASE WHEN r.id IS NOT NULL THEN 1 END) as occupied_lockers
    FROM lockers l
    LEFT JOIN rentals r ON l.id = r.locker_id 
      AND (r.end_date IS NULL OR date(r.end_date) >= date('now'))
    GROUP BY l.location
    ORDER BY l.location
  `
    )
    .all() as Array<{ name: string; total_lockers: number; occupied_lockers: number }>;

  return results.map((row) => ({
    name: row.name,
    totalLockers: row.total_lockers,
    occupiedLockers: row.occupied_lockers,
    availableLockers: row.total_lockers - row.occupied_lockers,
  }));
}
