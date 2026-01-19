/**
 * CSV Export Module
 * @module export/csv
 */

import { getAllLockers } from '../db/lockers';
import { getActiveRentals, getOverdueRentals, getExpiringRentals } from '../db/rentals';
import { getAuditLog } from '../db/audit';

/**
 * Convert array of objects to CSV
 * @param data Array of objects
 * @param columns Column definitions
 * @returns CSV string
 */
function arrayToCsv<T>(
  data: T[],
  columns: Array<{ key: keyof T; header: string }>
): string {
  // Header row
  const header = columns.map((c) => `"${c.header}"`).join(';');

  // Data rows
  const rows = data.map((row) =>
    columns
      .map((c) => {
        const value = row[c.key];
        if (value === null || value === undefined) return '""';
        if (typeof value === 'string') return `"${value.replace(/"/g, '""')}"`;
        if (typeof value === 'boolean') return value ? '"Ja"' : '"Nein"';
        return `"${value}"`;
      })
      .join(';')
  );

  return [header, ...rows].join('\n');
}

/**
 * Export lockers to CSV
 * @returns CSV string
 */
export function exportLockersToCsv(): string {
  const lockers = getAllLockers();

  return arrayToCsv(lockers, [
    { key: 'number', header: 'Nummer' },
    { key: 'location', header: 'Standort' },
    { key: 'size', header: 'Größe' },
    { key: 'is_damaged', header: 'Beschädigt' },
    { key: 'notes', header: 'Notizen' },
    { key: 'created_at', header: 'Erstellt' },
  ]);
}

/**
 * Export active rentals to CSV
 * @returns CSV string
 */
export function exportRentalsToCsv(): string {
  const rentals = getActiveRentals();

  return arrayToCsv(rentals, [
    { key: 'locker_number', header: 'Schließfach' },
    { key: 'locker_location', header: 'Standort' },
    { key: 'locker_size', header: 'Größe' },
    { key: 'renter_name', header: 'Mieter' },
    { key: 'renter_email', header: 'E-Mail' },
    { key: 'renter_phone', header: 'Telefon' },
    { key: 'start_date', header: 'Start' },
    { key: 'end_date', header: 'Ende' },
    { key: 'deposit_paid', header: 'Pfand bezahlt' },
    { key: 'deposit_returned', header: 'Pfand zurück' },
    { key: 'notes', header: 'Notizen' },
  ]);
}

/**
 * Export overdue rentals to CSV
 * @returns CSV string
 */
export function exportOverdueRentalsToCsv(): string {
  const rentals = getOverdueRentals();

  return arrayToCsv(rentals, [
    { key: 'locker_number', header: 'Schließfach' },
    { key: 'renter_name', header: 'Mieter' },
    { key: 'renter_email', header: 'E-Mail' },
    { key: 'renter_phone', header: 'Telefon' },
    { key: 'end_date', header: 'Fällig seit' },
  ]);
}

/**
 * Export expiring rentals to CSV
 * @param days Days to look ahead
 * @returns CSV string
 */
export function exportExpiringRentalsToCsv(days = 30): string {
  const rentals = getExpiringRentals(days);

  return arrayToCsv(rentals, [
    { key: 'locker_number', header: 'Schließfach' },
    { key: 'renter_name', header: 'Mieter' },
    { key: 'renter_email', header: 'E-Mail' },
    { key: 'renter_phone', header: 'Telefon' },
    { key: 'end_date', header: 'Endet am' },
  ]);
}

/**
 * Export audit log to CSV
 * @param limit Maximum number of entries
 * @returns CSV string
 */
export function exportAuditLogToCsv(limit = 1000): string {
  const auditLog = getAuditLog(limit);

  return arrayToCsv(auditLog, [
    { key: 'timestamp', header: 'Zeitstempel' },
    { key: 'action', header: 'Aktion' },
    { key: 'entity_type', header: 'Typ' },
    { key: 'entity_id', header: 'ID' },
    { key: 'username', header: 'Benutzer' },
    { key: 'details', header: 'Details' },
  ]);
}
