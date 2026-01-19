/**
 * Dashboard Screen
 * @module ui/screens/dashboard
 */

import { DatabaseConnection } from '../../db/connection';
import { Theme, colorize } from '../theme';
import { calculateDashboardLayout } from '../index';
import type { Rect } from '../../types/ui';

/**
 * Dashboard data structure
 */
export interface DashboardData {
  totalLockers: number;
  occupiedLockers: number;
  occupancyPercent: number;
  bySize: Record<string, { total: number; occupied: number }>;
  byLocation: Record<string, { total: number; occupied: number }>;
  overdueReturns: number;
  expiringSoon: number;
  damagedLockers: number;
  locations: string[];
  pendingPaymentsCents: number;
  revenue30dCents: number;
  occupancyHistory: Array<{ date: string; percent: number }>;
}

/**
 * Load all dashboard data from database
 * @returns Dashboard data
 */
export function loadDashboardData(): DashboardData {
  const db = DatabaseConnection.getConnection();

  // Total and occupied lockers
  const totalResult = db.prepare('SELECT COUNT(*) as count FROM lockers').get() as { count: number };
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

  // By size
  const bySizeResults = db
    .prepare(
      `
    SELECT 
      size,
      COUNT(*) as total,
      COUNT(CASE WHEN r.id IS NOT NULL THEN 1 END) as occupied
    FROM lockers l
    LEFT JOIN rentals r ON l.id = r.locker_id 
      AND (r.end_date IS NULL OR date(r.end_date) >= date('now'))
    GROUP BY size
  `
    )
    .all() as Array<{ size: string; total: number; occupied: number }>;

  const bySize: Record<string, { total: number; occupied: number }> = {};
  for (const row of bySizeResults) {
    bySize[row.size] = { total: row.total, occupied: row.occupied };
  }

  // By location
  const byLocationResults = db
    .prepare(
      `
    SELECT 
      location,
      COUNT(*) as total,
      COUNT(CASE WHEN r.id IS NOT NULL THEN 1 END) as occupied
    FROM lockers l
    LEFT JOIN rentals r ON l.id = r.locker_id 
      AND (r.end_date IS NULL OR date(r.end_date) >= date('now'))
    GROUP BY location
  `
    )
    .all() as Array<{ location: string; total: number; occupied: number }>;

  const byLocation: Record<string, { total: number; occupied: number }> = {};
  for (const row of byLocationResults) {
    byLocation[row.location] = { total: row.total, occupied: row.occupied };
  }

  // Overdue returns
  const overdueResult = db
    .prepare(
      `
    SELECT COUNT(*) as count
    FROM rentals
    WHERE end_date IS NOT NULL AND date(end_date) < date('now')
  `
    )
    .get() as { count: number };

  // Expiring soon (next 30 days)
  const expiringResult = db
    .prepare(
      `
    SELECT COUNT(*) as count
    FROM rentals
    WHERE end_date IS NOT NULL 
      AND date(end_date) >= date('now')
      AND date(end_date) <= date('now', '+30 days')
  `
    )
    .get() as { count: number };

  // Damaged lockers
  const damagedResult = db
    .prepare(
      `
    SELECT COUNT(*) as count
    FROM lockers
    WHERE is_damaged = 1
  `
    )
    .get() as { count: number };

  // Unique locations
  const locationResults = db
    .prepare('SELECT DISTINCT location FROM lockers ORDER BY location')
    .all() as Array<{ location: string }>;

  // Pending payments (deposit not paid)
  const depositSetting = db
    .prepare("SELECT value FROM settings WHERE key = 'deposit_cents'")
    .get() as { value: string } | undefined;
  const depositCents = depositSetting ? parseInt(depositSetting.value, 10) : 1000;

  const pendingDepositsResult = db
    .prepare(
      `
    SELECT COUNT(*) as count
    FROM rentals
    WHERE deposit_paid = 0 AND (end_date IS NULL OR date(end_date) >= date('now'))
  `
    )
    .get() as { count: number };
  const pendingPaymentsCents = pendingDepositsResult.count * depositCents;

  // Revenue last 30 days
  const revenueResult = db
    .prepare(
      `
    SELECT COALESCE(SUM(amount_cents), 0) as total
    FROM payments
    WHERE date(payment_date) >= date('now', '-30 days')
  `
    )
    .get() as { total: number };

  // Occupancy history (last 30 days)
  const historyResults = db
    .prepare(
      `
    SELECT snapshot_date as date, occupancy_percent as percent
    FROM occupancy_history
    WHERE date(snapshot_date) >= date('now', '-30 days')
    ORDER BY snapshot_date ASC
  `
    )
    .all() as Array<{ date: string; percent: number }>;

  return {
    totalLockers,
    occupiedLockers,
    occupancyPercent,
    bySize,
    byLocation,
    overdueReturns: overdueResult.count,
    expiringSoon: expiringResult.count,
    damagedLockers: damagedResult.count,
    locations: locationResults.map((l) => l.location),
    pendingPaymentsCents,
    revenue30dCents: revenueResult.total,
    occupancyHistory: historyResults,
  };
}

/**
 * Dashboard screen component
 */
export class DashboardScreen {
  private theme: Theme;
  private data: DashboardData | null = null;

  constructor() {
    this.theme = Theme.defaultDark();
  }

  /**
   * Refresh dashboard data
   */
  refresh(): void {
    this.data = loadDashboardData();
  }

  /**
   * Render the dashboard to string array
   * @param width Available width
   * @param height Available height
   * @returns Rendered lines
   */
  render(width: number, height: number): string[] {
    if (!this.data) {
      this.refresh();
    }

    const data = this.data!;
    const layout = calculateDashboardLayout({ x: 0, y: 0, width, height });
    const lines: string[] = [];

    // Overview section
    lines.push(...this.renderOverview(width, layout.overview, data));

    // Statistics section
    lines.push(...this.renderStatistics(width, layout.statistics, data));

    // Alerts section
    lines.push(...this.renderAlerts(width, layout.alerts, data));

    // Graph section
    lines.push(...this.renderGraph(width, layout.graph, data));

    return lines;
  }

  /**
   * Render overview section
   */
  private renderOverview(width: number, height: number, data: DashboardData): string[] {
    const lines: string[] = [];
    const freeLockers = data.totalLockers - data.occupiedLockers;

    // Top border
    lines.push(colorize('╔═══ Übersicht ' + '═'.repeat(Math.max(0, width - 16)) + '╗', this.theme.get('border')));

    // Content lines
    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Gesamt: ${data.totalLockers} Schließfächer`.padEnd(width - 2), this.theme.get('text')) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Belegt: ${data.occupiedLockers} (${data.occupancyPercent.toFixed(1)}%)`.padEnd(width - 2), this.theme.get('success')) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Frei: ${freeLockers}`.padEnd(width - 2), this.theme.get('info')) +
      colorize('║', this.theme.get('border'))
    );

    // Bottom border
    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Render statistics section
   */
  private renderStatistics(width: number, height: number, data: DashboardData): string[] {
    const lines: string[] = [];

    // Top border
    lines.push(colorize('╔═══ Statistiken ' + '═'.repeat(Math.max(0, width - 18)) + '╗', this.theme.get('border')));

    // By size header
    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(' Nach Größe:'.padEnd(width - 2), this.theme.get('textDim')) +
      colorize('║', this.theme.get('border'))
    );

    // Size statistics
    const sizes = ['S', 'M', 'L', 'XL'];
    for (const size of sizes) {
      const stats = data.bySize[size] || { total: 0, occupied: 0 };
      const line = `   ${size}: ${stats.occupied}/${stats.total}`;
      lines.push(
        colorize('║', this.theme.get('border')) +
        colorize(line.padEnd(width - 2), this.theme.get('text')) +
        colorize('║', this.theme.get('border'))
      );
    }

    // Fill remaining lines
    while (lines.length < height - 1) {
      lines.push(
        colorize('║', this.theme.get('border')) +
        ' '.repeat(width - 2) +
        colorize('║', this.theme.get('border'))
      );
    }

    // Bottom border
    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Render alerts section
   */
  private renderAlerts(width: number, height: number, data: DashboardData): string[] {
    const lines: string[] = [];

    // Top border
    lines.push(colorize('╔═══ Alarme ' + '═'.repeat(Math.max(0, width - 13)) + '╗', this.theme.get('border')));

    // Overdue
    const overdueColor = data.overdueReturns > 0 ? this.theme.get('error') : this.theme.get('success');
    const overdueIcon = data.overdueReturns > 0 ? '⚠' : '✓';
    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` ${overdueIcon} Überfällig: ${data.overdueReturns}`.padEnd(width - 2), overdueColor) +
      colorize('║', this.theme.get('border'))
    );

    // Expiring soon
    const expiringColor = data.expiringSoon > 0 ? this.theme.get('warning') : this.theme.get('success');
    const expiringIcon = data.expiringSoon > 0 ? '⏰' : '✓';
    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` ${expiringIcon} Bald fällig: ${data.expiringSoon}`.padEnd(width - 2), expiringColor) +
      colorize('║', this.theme.get('border'))
    );

    // Bottom border
    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Render trend graph section
   */
  private renderGraph(width: number, height: number, data: DashboardData): string[] {
    const lines: string[] = [];

    // Top border
    lines.push(colorize('╔═══ Belegungstrend (30 Tage) ' + '═'.repeat(Math.max(0, width - 32)) + '╗', this.theme.get('border')));

    // Graph content
    const graphHeight = height - 2;
    const maxPercent = 100;
    const barMaxWidth = width - 12;

    if (data.occupancyHistory.length === 0) {
      // No data message
      const message = 'Keine Verlaufsdaten verfügbar';
      const padding = Math.floor((width - 2 - message.length) / 2);
      lines.push(
        colorize('║', this.theme.get('border')) +
        ' '.repeat(padding) +
        colorize(message, this.theme.get('textDim')) +
        ' '.repeat(width - 2 - padding - message.length) +
        colorize('║', this.theme.get('border'))
      );
    } else {
      // Render bars
      const step = Math.max(1, Math.floor(data.occupancyHistory.length / graphHeight));
      for (let i = 0; i < graphHeight && i * step < data.occupancyHistory.length; i++) {
        const entry = data.occupancyHistory[i * step];
        const barWidth = Math.floor((entry.percent / maxPercent) * barMaxWidth);
        const bar = '█'.repeat(barWidth);
        const label = ` ${entry.percent.toFixed(0)}%`;

        lines.push(
          colorize('║', this.theme.get('border')) +
          colorize(' ' + bar, this.theme.get('accent')) +
          colorize(label.padEnd(width - 3 - barWidth), this.theme.get('text')) +
          colorize('║', this.theme.get('border'))
        );
      }
    }

    // Fill remaining lines
    while (lines.length < height - 1) {
      lines.push(
        colorize('║', this.theme.get('border')) +
        ' '.repeat(width - 2) +
        colorize('║', this.theme.get('border'))
      );
    }

    // Bottom border
    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Get dashboard data
   */
  getData(): DashboardData | null {
    return this.data;
  }
}
