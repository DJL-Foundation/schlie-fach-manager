/**
 * Markdown Report Export Module
 * @module export/markdown
 */

import { getAllLockers } from '../db/lockers';
import { getActiveRentals, getOverdueRentals, getExpiringRentals } from '../db/rentals';
import { getRevenueForPeriod, getPaymentSummaryByType } from '../db/payments';
import { getOccupancyHistory } from '../db/history';
import { getLocationStatistics } from '../db/locations';

/**
 * Generate a full Markdown report
 * @returns Markdown string
 */
export function generateMarkdownReport(): string {
  const lines: string[] = [];
  const now = new Date();

  // Header
  lines.push('# Schließfach-Manager Bericht');
  lines.push('');
  lines.push(`**Generiert:** ${now.toLocaleString('de-DE')}`);
  lines.push('');

  // Summary
  lines.push('## Übersicht');
  lines.push('');

  const lockers = getAllLockers();
  const activeRentals = getActiveRentals();
  const overdueRentals = getOverdueRentals();
  const damagedLockers = lockers.filter((l) => l.is_damaged);

  lines.push(`| Metrik | Wert |`);
  lines.push(`|--------|------|`);
  lines.push(`| Schließfächer gesamt | ${lockers.length} |`);
  lines.push(`| Aktive Verleih | ${activeRentals.length} |`);
  lines.push(`| Überfällig | ${overdueRentals.length} |`);
  lines.push(`| Beschädigt | ${damagedLockers.length} |`);
  lines.push('');

  // Occupancy by location
  lines.push('## Belegung nach Standort');
  lines.push('');

  const locationStats = getLocationStatistics();
  lines.push(`| Standort | Gesamt | Belegt | Frei | Quote |`);
  lines.push(`|----------|--------|--------|------|-------|`);
  for (const loc of locationStats) {
    const percent = loc.totalLockers > 0 ? ((loc.occupiedLockers / loc.totalLockers) * 100).toFixed(1) : '0';
    lines.push(
      `| ${loc.name} | ${loc.totalLockers} | ${loc.occupiedLockers} | ${loc.availableLockers} | ${percent}% |`
    );
  }
  lines.push('');

  // Occupancy by size
  lines.push('## Belegung nach Größe');
  lines.push('');

  const sizes = ['S', 'M', 'L', 'XL'];
  lines.push(`| Größe | Gesamt | Belegt | Quote |`);
  lines.push(`|-------|--------|--------|-------|`);
  for (const size of sizes) {
    const total = lockers.filter((l) => l.size === size).length;
    const occupied = activeRentals.filter((r) => r.locker_size === size).length;
    const percent = total > 0 ? ((occupied / total) * 100).toFixed(1) : '0';
    lines.push(`| ${size} | ${total} | ${occupied} | ${percent}% |`);
  }
  lines.push('');

  // Financial summary
  lines.push('## Finanzübersicht');
  lines.push('');

  const revenue7d = getRevenueForPeriod(7);
  const revenue30d = getRevenueForPeriod(30);
  const revenue365d = getRevenueForPeriod(365);

  lines.push(`| Zeitraum | Einnahmen |`);
  lines.push(`|----------|-----------|`);
  lines.push(`| Letzte 7 Tage | €${(revenue7d / 100).toFixed(2)} |`);
  lines.push(`| Letzte 30 Tage | €${(revenue30d / 100).toFixed(2)} |`);
  lines.push(`| Letztes Jahr | €${(revenue365d / 100).toFixed(2)} |`);
  lines.push('');

  // Payment types
  lines.push('### Zahlungsarten (letzte 30 Tage)');
  lines.push('');

  const paymentSummary = getPaymentSummaryByType(30);
  const paymentTypeLabels: Record<string, string> = {
    cash: 'Bargeld',
    card: 'Karte',
    transfer: 'Überweisung',
    other: 'Sonstige',
  };

  lines.push(`| Zahlungsart | Anzahl | Summe |`);
  lines.push(`|-------------|--------|-------|`);
  for (const [type, data] of Object.entries(paymentSummary)) {
    lines.push(`| ${paymentTypeLabels[type] || type} | ${data.count} | €${(data.total / 100).toFixed(2)} |`);
  }
  lines.push('');

  // Overdue rentals
  if (overdueRentals.length > 0) {
    lines.push('## ⚠️ Überfällige Verleih');
    lines.push('');
    lines.push(`| Schließfach | Mieter | Fällig seit |`);
    lines.push(`|-------------|--------|-------------|`);
    for (const rental of overdueRentals.slice(0, 10)) {
      lines.push(`| ${rental.locker_number} | ${rental.renter_name} | ${rental.end_date} |`);
    }
    if (overdueRentals.length > 10) {
      lines.push(`| ... | *${overdueRentals.length - 10} weitere* | |`);
    }
    lines.push('');
  }

  // Expiring soon
  const expiring = getExpiringRentals(30);
  if (expiring.length > 0) {
    lines.push('## ⏰ Bald fällig (30 Tage)');
    lines.push('');
    lines.push(`| Schließfach | Mieter | Endet am |`);
    lines.push(`|-------------|--------|----------|`);
    for (const rental of expiring.slice(0, 10)) {
      lines.push(`| ${rental.locker_number} | ${rental.renter_name} | ${rental.end_date} |`);
    }
    if (expiring.length > 10) {
      lines.push(`| ... | *${expiring.length - 10} weitere* | |`);
    }
    lines.push('');
  }

  // Damaged lockers
  if (damagedLockers.length > 0) {
    lines.push('## 🔧 Beschädigte Schließfächer');
    lines.push('');
    lines.push(`| Nummer | Standort | Notizen |`);
    lines.push(`|--------|----------|---------|`);
    for (const locker of damagedLockers) {
      lines.push(`| ${locker.number} | ${locker.location} | ${locker.notes || '-'} |`);
    }
    lines.push('');
  }

  // Footer
  lines.push('---');
  lines.push('*Schließfach-Manager v2.1*');

  return lines.join('\n');
}

/**
 * Generate a short summary report
 * @returns Markdown string
 */
export function generateShortSummary(): string {
  const lines: string[] = [];

  const lockers = getAllLockers();
  const activeRentals = getActiveRentals();
  const overdueRentals = getOverdueRentals();
  const revenue30d = getRevenueForPeriod(30);

  const occupancy = lockers.length > 0 ? ((activeRentals.length / lockers.length) * 100).toFixed(1) : '0';

  lines.push('# Kurzübersicht');
  lines.push('');
  lines.push(`- **Belegung:** ${activeRentals.length}/${lockers.length} (${occupancy}%)`);
  lines.push(`- **Überfällig:** ${overdueRentals.length}`);
  lines.push(`- **Einnahmen (30 Tage):** €${(revenue30d / 100).toFixed(2)}`);
  lines.push('');
  lines.push(`*Stand: ${new Date().toLocaleString('de-DE')}*`);

  return lines.join('\n');
}
