/**
 * Finances Screen
 * @module ui/screens/finances
 */

import { Theme, colorize } from '../theme';
import { getRevenueForPeriod, getPaymentSummaryByType, getPendingDeposits } from '../../db/payments';

/**
 * Finances screen component
 */
export class FinancesScreen {
  private theme: Theme;

  constructor() {
    this.theme = Theme.defaultDark();
  }

  /**
   * Render the finances screen
   * @param width Available width
   * @param height Available height
   * @returns Rendered lines
   */
  render(width: number, height: number): string[] {
    const lines: string[] = [];

    // Revenue summary
    lines.push(...this.renderRevenueSummary(width));

    // Payment breakdown
    lines.push(...this.renderPaymentBreakdown(width));

    // Pending deposits
    lines.push(...this.renderPendingDeposits(width));

    // Fill remaining space
    while (lines.length < height) {
      lines.push(' '.repeat(width));
    }

    return lines.slice(0, height);
  }

  /**
   * Render revenue summary section
   */
  private renderRevenueSummary(width: number): string[] {
    const lines: string[] = [];
    const revenue7d = getRevenueForPeriod(7);
    const revenue30d = getRevenueForPeriod(30);
    const revenue365d = getRevenueForPeriod(365);

    lines.push(colorize('╔═══ Einnahmen ' + '═'.repeat(Math.max(0, width - 16)) + '╗', this.theme.get('border')));

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Letzte 7 Tage:   ${this.formatCurrency(revenue7d)}`.padEnd(width - 2), this.theme.get('text')) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Letzte 30 Tage:  ${this.formatCurrency(revenue30d)}`.padEnd(width - 2), this.theme.get('success')) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Letztes Jahr:    ${this.formatCurrency(revenue365d)}`.padEnd(width - 2), this.theme.get('text')) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Render payment breakdown section
   */
  private renderPaymentBreakdown(width: number): string[] {
    const lines: string[] = [];
    const summary = getPaymentSummaryByType(30);

    lines.push(colorize('╔═══ Zahlungsarten (30 Tage) ' + '═'.repeat(Math.max(0, width - 30)) + '╗', this.theme.get('border')));

    const types = [
      { key: 'cash', label: 'Bargeld' },
      { key: 'card', label: 'Karte' },
      { key: 'transfer', label: 'Überweisung' },
      { key: 'other', label: 'Sonstige' },
    ] as const;

    for (const type of types) {
      const data = summary[type.key];
      const line = ` ${type.label.padEnd(12)}: ${data.count.toString().padStart(3)} × ${this.formatCurrency(data.total)}`;

      lines.push(
        colorize('║', this.theme.get('border')) +
        colorize(line.padEnd(width - 2), this.theme.get('text')) +
        colorize('║', this.theme.get('border'))
      );
    }

    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Render pending deposits section
   */
  private renderPendingDeposits(width: number): string[] {
    const lines: string[] = [];
    const pending = getPendingDeposits();

    lines.push(colorize('╔═══ Offene Pfandbeträge ' + '═'.repeat(Math.max(0, width - 26)) + '╗', this.theme.get('border')));

    const color = pending > 0 ? this.theme.get('warning') : this.theme.get('success');
    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Gesamt: ${this.formatCurrency(pending)}`.padEnd(width - 2), color) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Format cents as currency string
   * @param cents Amount in cents
   * @returns Formatted currency string
   */
  private formatCurrency(cents: number): string {
    const euros = cents / 100;
    return `€${euros.toFixed(2)}`;
  }
}
