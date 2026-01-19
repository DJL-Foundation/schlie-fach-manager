/**
 * Management Screen
 * @module ui/screens/management
 */

import { Theme, colorize } from '../theme';
import { getLocationStatistics } from '../../db/locations';
import { getDamagedLockers, getLockerCount } from '../../db/lockers';
import { getAuditLogCount } from '../../db/audit';

/**
 * Management menu options
 */
export type ManagementOption = 'locations' | 'settings' | 'audit' | 'import' | 'export';

/**
 * Management screen component
 */
export class ManagementScreen {
  private theme: Theme;
  private selectedOption: ManagementOption = 'locations';

  constructor() {
    this.theme = Theme.defaultDark();
  }

  /**
   * Set selected menu option
   * @param option Option to select
   */
  setOption(option: ManagementOption): void {
    this.selectedOption = option;
  }

  /**
   * Get current selected option
   */
  getOption(): ManagementOption {
    return this.selectedOption;
  }

  /**
   * Render the management screen
   * @param width Available width
   * @param height Available height
   * @returns Rendered lines
   */
  render(width: number, height: number): string[] {
    const lines: string[] = [];

    // Menu
    lines.push(...this.renderMenu(width));

    // Statistics overview
    lines.push(...this.renderStatistics(width));

    // Location details
    lines.push(...this.renderLocationDetails(width));

    // Fill remaining space
    while (lines.length < height) {
      lines.push(' '.repeat(width));
    }

    return lines.slice(0, height);
  }

  /**
   * Render menu section
   */
  private renderMenu(width: number): string[] {
    const lines: string[] = [];
    const options: Array<{ id: ManagementOption; label: string; key: string }> = [
      { id: 'locations', label: 'Standorte', key: 'l' },
      { id: 'settings', label: 'Einstellungen', key: 's' },
      { id: 'audit', label: 'Audit-Log', key: 'a' },
      { id: 'import', label: 'Import', key: 'i' },
      { id: 'export', label: 'Export', key: 'x' },
    ];

    lines.push(colorize('╔═══ Verwaltung ' + '═'.repeat(Math.max(0, width - 17)) + '╗', this.theme.get('border')));

    let menuLine = '';
    for (const option of options) {
      const isSelected = option.id === this.selectedOption;
      const label = `[${option.key}] ${option.label}`;

      if (isSelected) {
        menuLine += colorize(` ${label} `, this.theme.get('primary'));
      } else {
        menuLine += colorize(` ${label} `, this.theme.get('textDim'));
      }
    }

    const visibleLength = menuLine.replace(/\x1b\[[0-9;]*m/g, '').length;
    const padding = ' '.repeat(Math.max(0, width - 2 - visibleLength));

    lines.push(
      colorize('║', this.theme.get('border')) +
      menuLine + padding +
      colorize('║', this.theme.get('border'))
    );

    lines.push(colorize('╠' + '═'.repeat(width - 2) + '╣', this.theme.get('border')));

    return lines;
  }

  /**
   * Render statistics section
   */
  private renderStatistics(width: number): string[] {
    const lines: string[] = [];
    const totalLockers = getLockerCount();
    const damagedLockers = getDamagedLockers().length;
    const auditEntries = getAuditLogCount();

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Statistiken:`.padEnd(width - 2), this.theme.get('textDim')) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(`   Schließfächer gesamt: ${totalLockers}`.padEnd(width - 2), this.theme.get('text')) +
      colorize('║', this.theme.get('border'))
    );

    const damagedColor = damagedLockers > 0 ? this.theme.get('warning') : this.theme.get('success');
    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(`   Beschädigt: ${damagedLockers}`.padEnd(width - 2), damagedColor) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(`   Audit-Einträge: ${auditEntries}`.padEnd(width - 2), this.theme.get('text')) +
      colorize('║', this.theme.get('border'))
    );

    lines.push(
      colorize('║', this.theme.get('border')) +
      ' '.repeat(width - 2) +
      colorize('║', this.theme.get('border'))
    );

    return lines;
  }

  /**
   * Render location details section
   */
  private renderLocationDetails(width: number): string[] {
    const lines: string[] = [];
    const locationStats = getLocationStatistics();

    lines.push(
      colorize('║', this.theme.get('border')) +
      colorize(` Standorte:`.padEnd(width - 2), this.theme.get('textDim')) +
      colorize('║', this.theme.get('border'))
    );

    if (locationStats.length === 0) {
      lines.push(
        colorize('║', this.theme.get('border')) +
        colorize(`   Keine Standorte vorhanden`.padEnd(width - 2), this.theme.get('textDim')) +
        colorize('║', this.theme.get('border'))
      );
    } else {
      for (const loc of locationStats.slice(0, 5)) {
        const line = `   ${loc.name}: ${loc.occupiedLockers}/${loc.totalLockers} belegt`;
        lines.push(
          colorize('║', this.theme.get('border')) +
          colorize(line.padEnd(width - 2), this.theme.get('text')) +
          colorize('║', this.theme.get('border'))
        );
      }
    }

    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }
}
