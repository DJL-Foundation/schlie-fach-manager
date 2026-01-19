/**
 * JSON Export Module
 * @module export/json
 */

import { getAllLockers } from '../db/lockers';
import { getActiveRentals, getOverdueRentals } from '../db/rentals';
import { getPaymentsByDateRange, getRevenueForPeriod } from '../db/payments';
import { getAuditLog } from '../db/audit';
import { getOccupancyHistory } from '../db/history';
import { getAllLocations, getLocationStatistics } from '../db/locations';
import { DatabaseConnection } from '../db/connection';

/**
 * Export data structure
 */
export interface ExportData {
  version: string;
  exportDate: string;
  lockers: ReturnType<typeof getAllLockers>;
  rentals: ReturnType<typeof getActiveRentals>;
  locations: ReturnType<typeof getAllLocations>;
  settings: Array<{ key: string; value: string }>;
}

/**
 * Export all data to JSON
 * @returns JSON string of exported data
 */
export function exportToJson(): string {
  const db = DatabaseConnection.getConnection();

  // Get settings
  const settings = db.prepare('SELECT key, value FROM settings').all() as Array<{
    key: string;
    value: string;
  }>;

  const data: ExportData = {
    version: '2.1.0',
    exportDate: new Date().toISOString(),
    lockers: getAllLockers(),
    rentals: getActiveRentals(),
    locations: getAllLocations(),
    settings,
  };

  return JSON.stringify(data, null, 2);
}

/**
 * Export summary statistics to JSON
 * @returns JSON string of summary data
 */
export function exportSummaryToJson(): string {
  const lockers = getAllLockers();
  const activeRentals = getActiveRentals();
  const overdueRentals = getOverdueRentals();
  const locationStats = getLocationStatistics();
  const revenue30d = getRevenueForPeriod(30);
  const history = getOccupancyHistory(30);

  const summary = {
    exportDate: new Date().toISOString(),
    totals: {
      lockers: lockers.length,
      activeRentals: activeRentals.length,
      overdueRentals: overdueRentals.length,
      damagedLockers: lockers.filter((l) => l.is_damaged).length,
    },
    revenue: {
      last30Days: revenue30d,
    },
    locationStats,
    occupancyHistory: history,
  };

  return JSON.stringify(summary, null, 2);
}

/**
 * Export audit log to JSON
 * @param limit Maximum number of entries
 * @returns JSON string of audit log
 */
export function exportAuditLogToJson(limit = 1000): string {
  const auditLog = getAuditLog(limit);

  return JSON.stringify(
    {
      exportDate: new Date().toISOString(),
      entries: auditLog,
    },
    null,
    2
  );
}
