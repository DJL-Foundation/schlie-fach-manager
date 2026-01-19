/**
 * Test Utilities for Schließfach-Manager
 * @module tests/helpers/test-utils
 */

import { beforeEach, afterEach } from 'bun:test';
import { DatabaseConnection } from '../../src/db/connection';
import { runMigrations } from '../../src/db/migrations';

/**
 * Setup test database (in-memory)
 * Call this in your test file to use an isolated database
 */
export function setupTestDatabase(): void {
  beforeEach(() => {
    // Enable test mode for in-memory database
    DatabaseConnection.enableTestMode();
    const db = DatabaseConnection.getConnection();
    runMigrations(db);
  });

  afterEach(() => {
    DatabaseConnection.close();
    DatabaseConnection.disableTestMode();
  });
}

/**
 * Create test lockers
 * @param count Number of lockers to create
 * @returns Array of created locker IDs
 */
export function createTestLockers(count: number): number[] {
  const db = DatabaseConnection.getConnection();
  const ids: number[] = [];

  const sizes = ['S', 'M', 'L', 'XL'];
  const locations = ['Building A', 'Building B'];

  for (let i = 1; i <= count; i++) {
    const result = db
      .prepare(
        `INSERT INTO lockers (number, location, size) 
         VALUES (?, ?, ?) 
         RETURNING id`
      )
      .get(`L${i.toString().padStart(3, '0')}`, locations[i % 2], sizes[i % 4]) as { id: number };

    ids.push(result.id);
  }

  return ids;
}

/**
 * Create test rental
 * @param lockerId Locker ID
 * @param renterName Renter name
 * @returns Created rental ID
 */
export function createTestRental(
  lockerId: number,
  renterName: string,
  options?: {
    startDate?: string;
    endDate?: string;
    depositPaid?: boolean;
  }
): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `INSERT INTO rentals (locker_id, renter_name, start_date, end_date, deposit_paid) 
       VALUES (?, ?, ?, ?, ?) 
       RETURNING id`
    )
    .get(
      lockerId,
      renterName,
      options?.startDate || new Date().toISOString().split('T')[0],
      options?.endDate || null,
      options?.depositPaid ? 1 : 0
    ) as { id: number };

  return result.id;
}

/**
 * Create test payment
 * @param rentalId Rental ID
 * @param amountCents Amount in cents
 * @returns Created payment ID
 */
export function createTestPayment(rentalId: number, amountCents: number): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `INSERT INTO payments (rental_id, amount_cents, payment_date, payment_type) 
       VALUES (?, ?, datetime('now'), 'cash') 
       RETURNING id`
    )
    .get(rentalId, amountCents) as { id: number };

  return result.id;
}

/**
 * Create test location
 * @param name Location name
 * @returns Created location ID
 */
export function createTestLocation(name: string): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(`INSERT INTO locations (name) VALUES (?) RETURNING id`)
    .get(name) as { id: number };

  return result.id;
}

/**
 * Get test database statistics
 */
export function getTestStats(): {
  lockers: number;
  rentals: number;
  payments: number;
  locations: number;
} {
  const db = DatabaseConnection.getConnection();

  const lockers = (db.prepare('SELECT COUNT(*) as count FROM lockers').get() as { count: number }).count;
  const rentals = (db.prepare('SELECT COUNT(*) as count FROM rentals').get() as { count: number }).count;
  const payments = (db.prepare('SELECT COUNT(*) as count FROM payments').get() as { count: number }).count;
  const locations = (db.prepare('SELECT COUNT(*) as count FROM locations').get() as { count: number }).count;

  return { lockers, rentals, payments, locations };
}
