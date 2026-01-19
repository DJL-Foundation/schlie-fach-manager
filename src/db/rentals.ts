/**
 * Rental Database Operations
 * @module db/rentals
 */

import { DatabaseConnection } from './connection';
import type { Rental, RentalWithLocker } from '../types/database';
import { RentalSchema } from './schema';
import { z } from 'zod';
import { logAction } from './audit';
import { format } from 'date-fns';

/**
 * Extended rental schema with locker information
 */
const RentalWithLockerSchema = RentalSchema.extend({
  locker_number: z.string(),
  locker_location: z.string(),
  locker_size: z.enum(['S', 'M', 'L', 'XL']),
});

/**
 * Create a new rental
 * @param lockerId Locker ID
 * @param renterName Renter's name
 * @param startDate Start date (ISO string)
 * @param endDate Optional end date (ISO string)
 * @param renterEmail Optional renter email
 * @param renterPhone Optional renter phone
 * @param depositPaid Whether deposit was paid
 * @param notes Optional notes
 * @returns The created rental
 */
export function createRental(
  lockerId: number,
  renterName: string,
  startDate: string,
  endDate?: string,
  renterEmail?: string,
  renterPhone?: string,
  depositPaid = false,
  notes?: string
): Rental {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    INSERT INTO rentals (locker_id, renter_name, renter_email, renter_phone, start_date, end_date, deposit_paid, notes)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
    RETURNING *
  `
    )
    .get(
      lockerId,
      renterName,
      renterEmail || null,
      renterPhone || null,
      startDate,
      endDate || null,
      depositPaid ? 1 : 0,
      notes || null
    );

  const rental = RentalSchema.parse(result);

  logAction('create', 'rental', rental.id, {
    locker_id: lockerId,
    renter_name: renterName,
    start_date: startDate,
  });

  return rental;
}

/**
 * Get a rental by ID
 * @param id Rental ID
 * @returns Rental or null if not found
 */
export function getRentalById(id: number): Rental | null {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT * FROM rentals WHERE id = ?').get(id);

  if (!result) return null;

  return RentalSchema.parse(result);
}

/**
 * Get a rental with locker information by ID
 * @param id Rental ID
 * @returns Rental with locker info or null if not found
 */
export function getRentalWithLockerById(id: number): RentalWithLocker | null {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    SELECT r.*, l.number as locker_number, l.location as locker_location, l.size as locker_size
    FROM rentals r
    JOIN lockers l ON r.locker_id = l.id
    WHERE r.id = ?
  `
    )
    .get(id);

  if (!result) return null;

  return RentalWithLockerSchema.parse(result);
}

/**
 * Get all active rentals (not ended)
 * @returns Array of active rentals with locker info
 */
export function getActiveRentals(): RentalWithLocker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT r.*, l.number as locker_number, l.location as locker_location, l.size as locker_size
    FROM rentals r
    JOIN lockers l ON r.locker_id = l.id
    WHERE r.end_date IS NULL OR date(r.end_date) >= date('now')
    ORDER BY r.start_date DESC
  `
    )
    .all();

  return z.array(RentalWithLockerSchema).parse(results);
}

/**
 * Get rentals by locker ID
 * @param lockerId Locker ID
 * @returns Array of rentals for the locker
 */
export function getRentalsByLockerId(lockerId: number): Rental[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare('SELECT * FROM rentals WHERE locker_id = ? ORDER BY start_date DESC')
    .all(lockerId);

  return z.array(RentalSchema).parse(results);
}

/**
 * Get current rental for a locker
 * @param lockerId Locker ID
 * @returns Current rental or null if locker is available
 */
export function getCurrentRentalForLocker(lockerId: number): Rental | null {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    SELECT * FROM rentals 
    WHERE locker_id = ? 
    AND (end_date IS NULL OR date(end_date) >= date('now'))
    ORDER BY start_date DESC
    LIMIT 1
  `
    )
    .get(lockerId);

  if (!result) return null;

  return RentalSchema.parse(result);
}

/**
 * Get overdue rentals (past end date but not returned)
 * @returns Array of overdue rentals with locker info
 */
export function getOverdueRentals(): RentalWithLocker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT r.*, l.number as locker_number, l.location as locker_location, l.size as locker_size
    FROM rentals r
    JOIN lockers l ON r.locker_id = l.id
    WHERE r.end_date IS NOT NULL AND date(r.end_date) < date('now')
    ORDER BY r.end_date ASC
  `
    )
    .all();

  return z.array(RentalWithLockerSchema).parse(results);
}

/**
 * Get rentals expiring soon
 * @param days Number of days to look ahead
 * @returns Array of expiring rentals with locker info
 */
export function getExpiringRentals(days = 30): RentalWithLocker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT r.*, l.number as locker_number, l.location as locker_location, l.size as locker_size
    FROM rentals r
    JOIN lockers l ON r.locker_id = l.id
    WHERE r.end_date IS NOT NULL 
    AND date(r.end_date) >= date('now')
    AND date(r.end_date) <= date('now', '+' || ? || ' days')
    ORDER BY r.end_date ASC
  `
    )
    .all(days);

  return z.array(RentalWithLockerSchema).parse(results);
}

/**
 * Extend a rental's end date
 * @param id Rental ID
 * @param newEndDate New end date (ISO string)
 * @returns Updated rental or null if not found
 */
export function extendRental(id: number, newEndDate: string): Rental | null {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    UPDATE rentals SET end_date = ? WHERE id = ?
    RETURNING *
  `
    )
    .get(newEndDate, id);

  if (!result) return null;

  const rental = RentalSchema.parse(result);

  logAction('update', 'rental', id, { action: 'extend', new_end_date: newEndDate });

  return rental;
}

/**
 * End a rental (return locker)
 * @param id Rental ID
 * @param depositReturned Whether the deposit was returned
 * @returns Updated rental or null if not found
 */
export function endRental(id: number, depositReturned = false): Rental | null {
  const db = DatabaseConnection.getConnection();
  const endDate = format(new Date(), 'yyyy-MM-dd');

  const result = db
    .prepare(
      `
    UPDATE rentals 
    SET end_date = ?, deposit_returned = ?
    WHERE id = ?
    RETURNING *
  `
    )
    .get(endDate, depositReturned ? 1 : 0, id);

  if (!result) return null;

  const rental = RentalSchema.parse(result);

  logAction('update', 'rental', id, { action: 'return', end_date: endDate, deposit_returned: depositReturned });

  return rental;
}

/**
 * Update deposit status
 * @param id Rental ID
 * @param depositPaid Whether deposit is paid
 * @param depositReturned Whether deposit is returned
 * @returns Updated rental or null if not found
 */
export function updateDepositStatus(
  id: number,
  depositPaid: boolean,
  depositReturned: boolean
): Rental | null {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    UPDATE rentals 
    SET deposit_paid = ?, deposit_returned = ?
    WHERE id = ?
    RETURNING *
  `
    )
    .get(depositPaid ? 1 : 0, depositReturned ? 1 : 0, id);

  if (!result) return null;

  const rental = RentalSchema.parse(result);

  logAction('update', 'rental', id, { deposit_paid: depositPaid, deposit_returned: depositReturned });

  return rental;
}

/**
 * Get count of occupied lockers
 * @returns Number of currently occupied lockers
 */
export function getOccupiedLockerCount(): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    SELECT COUNT(DISTINCT locker_id) as count
    FROM rentals
    WHERE end_date IS NULL OR date(end_date) >= date('now')
  `
    )
    .get() as { count: number };

  return result.count;
}

/**
 * Get count of overdue rentals
 * @returns Number of overdue rentals
 */
export function getOverdueCount(): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    SELECT COUNT(*) as count
    FROM rentals
    WHERE end_date IS NOT NULL AND date(end_date) < date('now')
  `
    )
    .get() as { count: number };

  return result.count;
}

/**
 * Search rentals by renter name
 * @param searchTerm Search term
 * @returns Array of matching rentals with locker info
 */
export function searchRentalsByName(searchTerm: string): RentalWithLocker[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT r.*, l.number as locker_number, l.location as locker_location, l.size as locker_size
    FROM rentals r
    JOIN lockers l ON r.locker_id = l.id
    WHERE r.renter_name LIKE ?
    ORDER BY r.start_date DESC
  `
    )
    .all(`%${searchTerm}%`);

  return z.array(RentalWithLockerSchema).parse(results);
}
