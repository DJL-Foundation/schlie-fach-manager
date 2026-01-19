/**
 * Payment Database Operations
 * @module db/payments
 */

import { DatabaseConnection } from './connection';
import type { Payment, PaymentType } from '../types/database';
import { PaymentSchema } from './schema';
import { z } from 'zod';
import { logAction } from './audit';

/**
 * Create a new payment
 * @param rentalId Associated rental ID
 * @param amountCents Amount in cents
 * @param paymentDate Payment date (ISO string)
 * @param paymentType Type of payment
 * @param notes Optional notes
 * @returns The created payment
 */
export function createPayment(
  rentalId: number,
  amountCents: number,
  paymentDate: string,
  paymentType: PaymentType,
  notes?: string
): Payment {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    INSERT INTO payments (rental_id, amount_cents, payment_date, payment_type, notes)
    VALUES (?, ?, ?, ?, ?)
    RETURNING *
  `
    )
    .get(rentalId, amountCents, paymentDate, paymentType, notes || null);

  const payment = PaymentSchema.parse(result);

  logAction('create', 'payment', payment.id, {
    rental_id: rentalId,
    amount_cents: amountCents,
    payment_type: paymentType,
  });

  return payment;
}

/**
 * Get a payment by ID
 * @param id Payment ID
 * @returns Payment or null if not found
 */
export function getPaymentById(id: number): Payment | null {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('SELECT * FROM payments WHERE id = ?').get(id);

  if (!result) return null;

  return PaymentSchema.parse(result);
}

/**
 * Get all payments for a rental
 * @param rentalId Rental ID
 * @returns Array of payments for the rental
 */
export function getPaymentsByRentalId(rentalId: number): Payment[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare('SELECT * FROM payments WHERE rental_id = ? ORDER BY payment_date DESC')
    .all(rentalId);

  return z.array(PaymentSchema).parse(results);
}

/**
 * Get total payments for a rental
 * @param rentalId Rental ID
 * @returns Total amount in cents
 */
export function getTotalPaymentsForRental(rentalId: number): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare('SELECT COALESCE(SUM(amount_cents), 0) as total FROM payments WHERE rental_id = ?')
    .get(rentalId) as { total: number };

  return result.total;
}

/**
 * Get revenue for a period
 * @param days Number of days to look back
 * @returns Total revenue in cents
 */
export function getRevenueForPeriod(days = 30): number {
  const db = DatabaseConnection.getConnection();

  const result = db
    .prepare(
      `
    SELECT COALESCE(SUM(amount_cents), 0) as total
    FROM payments
    WHERE date(payment_date) >= date('now', '-' || ? || ' days')
  `
    )
    .get(days) as { total: number };

  return result.total;
}

/**
 * Get payments within a date range
 * @param startDate Start date (ISO string)
 * @param endDate End date (ISO string)
 * @returns Array of payments within the range
 */
export function getPaymentsByDateRange(startDate: string, endDate: string): Payment[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT * FROM payments
    WHERE date(payment_date) >= date(?) AND date(payment_date) <= date(?)
    ORDER BY payment_date DESC
  `
    )
    .all(startDate, endDate);

  return z.array(PaymentSchema).parse(results);
}

/**
 * Get payments by type
 * @param paymentType Payment type to filter by
 * @param limit Maximum number of results
 * @returns Array of payments of the specified type
 */
export function getPaymentsByType(paymentType: PaymentType, limit = 100): Payment[] {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT * FROM payments
    WHERE payment_type = ?
    ORDER BY payment_date DESC
    LIMIT ?
  `
    )
    .all(paymentType, limit);

  return z.array(PaymentSchema).parse(results);
}

/**
 * Delete a payment
 * @param id Payment ID
 * @returns True if deleted, false if not found
 */
export function deletePayment(id: number): boolean {
  const db = DatabaseConnection.getConnection();

  const result = db.prepare('DELETE FROM payments WHERE id = ?').run(id);

  if (result.changes > 0) {
    logAction('delete', 'payment', id);
    return true;
  }

  return false;
}

/**
 * Get payment summary by type for a period
 * @param days Number of days to look back
 * @returns Object with totals by payment type
 */
export function getPaymentSummaryByType(
  days = 30
): Record<PaymentType, { count: number; total: number }> {
  const db = DatabaseConnection.getConnection();

  const results = db
    .prepare(
      `
    SELECT 
      payment_type,
      COUNT(*) as count,
      COALESCE(SUM(amount_cents), 0) as total
    FROM payments
    WHERE date(payment_date) >= date('now', '-' || ? || ' days')
    GROUP BY payment_type
  `
    )
    .all(days) as Array<{ payment_type: PaymentType; count: number; total: number }>;

  const summary: Record<PaymentType, { count: number; total: number }> = {
    cash: { count: 0, total: 0 },
    card: { count: 0, total: 0 },
    transfer: { count: 0, total: 0 },
    other: { count: 0, total: 0 },
  };

  for (const row of results) {
    summary[row.payment_type] = { count: row.count, total: row.total };
  }

  return summary;
}

/**
 * Get pending deposit payments (rentals without deposit paid)
 * @returns Total pending deposits in cents
 */
export function getPendingDeposits(): number {
  const db = DatabaseConnection.getConnection();

  const depositSetting = db
    .prepare("SELECT value FROM settings WHERE key = 'deposit_cents'")
    .get() as { value: string } | undefined;

  const depositCents = depositSetting ? parseInt(depositSetting.value, 10) : 1000;

  const result = db
    .prepare(
      `
    SELECT COUNT(*) as count
    FROM rentals
    WHERE deposit_paid = 0 AND (end_date IS NULL OR date(end_date) >= date('now'))
  `
    )
    .get() as { count: number };

  return result.count * depositCents;
}
