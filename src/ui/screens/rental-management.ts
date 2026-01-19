/**
 * Rental Management Screen
 * @module ui/screens/rental-management
 */

import { Theme, colorize } from '../theme';
import { getActiveRentals, getOverdueRentals, getExpiringRentals } from '../../db/rentals';
import type { RentalWithLocker } from '../../types/database';

/**
 * Tab options for rental management
 */
export type RentalTab = 'active' | 'overdue' | 'expiring';

/**
 * Rental management screen component
 */
export class RentalManagementScreen {
  private theme: Theme;
  private currentTab: RentalTab = 'active';
  private selectedIndex = 0;

  constructor() {
    this.theme = Theme.defaultDark();
  }

  /**
   * Set current tab
   * @param tab Tab to switch to
   */
  setTab(tab: RentalTab): void {
    this.currentTab = tab;
    this.selectedIndex = 0;
  }

  /**
   * Get current tab
   */
  getTab(): RentalTab {
    return this.currentTab;
  }

  /**
   * Move selection up
   */
  selectPrevious(): void {
    if (this.selectedIndex > 0) {
      this.selectedIndex--;
    }
  }

  /**
   * Move selection down
   * @param maxItems Maximum number of items
   */
  selectNext(maxItems: number): void {
    if (this.selectedIndex < maxItems - 1) {
      this.selectedIndex++;
    }
  }

  /**
   * Get selected index
   */
  getSelectedIndex(): number {
    return this.selectedIndex;
  }

  /**
   * Render the rental management screen
   * @param width Available width
   * @param height Available height
   * @returns Rendered lines
   */
  render(width: number, height: number): string[] {
    const lines: string[] = [];

    // Render tabs
    lines.push(...this.renderTabs(width));

    // Render content based on current tab
    const contentHeight = height - 3; // Subtract tab height
    let rentals: RentalWithLocker[] = [];

    switch (this.currentTab) {
      case 'active':
        rentals = getActiveRentals();
        break;
      case 'overdue':
        rentals = getOverdueRentals();
        break;
      case 'expiring':
        rentals = getExpiringRentals();
        break;
    }

    lines.push(...this.renderRentalList(rentals, width, contentHeight));

    return lines;
  }

  /**
   * Render tab bar
   */
  private renderTabs(width: number): string[] {
    const tabs = [
      { id: 'active', label: 'Aktive Verleih' },
      { id: 'overdue', label: 'Überfällig' },
      { id: 'expiring', label: 'Bald fällig' },
    ];

    let tabLine = '';
    for (const tab of tabs) {
      const isSelected = tab.id === this.currentTab;
      const label = ` ${tab.label} `;

      if (isSelected) {
        tabLine += colorize(`[${label}]`, this.theme.get('primary'));
      } else {
        tabLine += colorize(` ${label} `, this.theme.get('textDim'));
      }
    }

    const padding = ' '.repeat(Math.max(0, width - tabLine.replace(/\x1b\[[0-9;]*m/g, '').length));

    return [
      colorize('╔' + '═'.repeat(width - 2) + '╗', this.theme.get('border')),
      colorize('║', this.theme.get('border')) + tabLine + padding + colorize('║', this.theme.get('border')),
      colorize('╠' + '═'.repeat(width - 2) + '╣', this.theme.get('border')),
    ];
  }

  /**
   * Render rental list
   */
  private renderRentalList(rentals: RentalWithLocker[], width: number, height: number): string[] {
    const lines: string[] = [];

    if (rentals.length === 0) {
      const message = 'Keine Einträge vorhanden';
      const padding = Math.floor((width - 2 - message.length) / 2);
      lines.push(
        colorize('║', this.theme.get('border')) +
        ' '.repeat(padding) +
        colorize(message, this.theme.get('textDim')) +
        ' '.repeat(width - 2 - padding - message.length) +
        colorize('║', this.theme.get('border'))
      );
    } else {
      // Header
      const header = ' Schließfach │ Mieter │ Start │ Ende ';
      lines.push(
        colorize('║', this.theme.get('border')) +
        colorize(header.padEnd(width - 2), this.theme.get('textDim')) +
        colorize('║', this.theme.get('border'))
      );

      // Separator
      lines.push(
        colorize('║', this.theme.get('border')) +
        '─'.repeat(width - 2) +
        colorize('║', this.theme.get('border'))
      );

      // Rental rows
      for (let i = 0; i < rentals.length && lines.length < height - 1; i++) {
        const rental = rentals[i];
        const isSelected = i === this.selectedIndex;
        const prefix = isSelected ? '▶' : ' ';

        const row = `${prefix} ${rental.locker_number.padEnd(10)} │ ${rental.renter_name.substring(0, 15).padEnd(15)} │ ${rental.start_date.substring(0, 10)} │ ${(rental.end_date || '—').substring(0, 10)}`;
        const color = isSelected ? this.theme.get('primary') : this.theme.get('text');

        lines.push(
          colorize('║', this.theme.get('border')) +
          colorize(row.substring(0, width - 2).padEnd(width - 2), color) +
          colorize('║', this.theme.get('border'))
        );
      }
    }

    // Fill remaining space
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
}
