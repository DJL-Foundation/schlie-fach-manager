/**
 * Keybind Bar Widget
 * @module ui/widgets/keybind-bar
 */

import type { Keybind } from '../../types/ui';
import { Theme, colorize } from '../theme';
import { truncateText } from '../index';

/**
 * Keybind bar component for displaying keyboard shortcuts
 * Two-line display: global keybinds on top, context keybinds below
 */
export class KeybindBar {
  private globalBinds: Keybind[];
  private contextBinds: Keybind[];
  private contextMessage?: string;
  private theme: Theme;

  constructor(globalBinds: Keybind[], contextBinds: Keybind[] = [], contextMessage?: string) {
    this.globalBinds = globalBinds;
    this.contextBinds = contextBinds;
    this.contextMessage = contextMessage;
    this.theme = Theme.defaultDark();
  }

  /**
   * Update context keybinds
   * @param binds New context keybinds
   * @param message Optional context message
   */
  updateContext(binds: Keybind[], message?: string): void {
    this.contextBinds = binds;
    this.contextMessage = message;
  }

  /**
   * Update global keybinds
   * @param binds New global keybinds
   */
  updateGlobal(binds: Keybind[]): void {
    this.globalBinds = binds;
  }

  /**
   * Render the keybind bar to string array
   * @param width Available width
   * @returns Two lines of rendered keybinds
   */
  render(width: number): string[] {
    const lines: string[] = [];

    // Line 1: Global keybinds
    const globalText = this.formatKeybinds(this.globalBinds, 'global');
    lines.push(truncateText(globalText, width).padEnd(width));

    // Line 2: Context keybinds with optional message
    let contextLine = '';
    if (this.contextMessage) {
      contextLine = colorize(`[${this.contextMessage}] `, this.theme.get('textDim'));
    }

    const contextText = this.formatKeybinds(this.contextBinds, 'context');
    const availableWidth = width - (this.contextMessage ? this.contextMessage.length + 3 : 0);
    contextLine += truncateText(contextText, availableWidth);

    // Pad to full width
    const visibleLength = contextLine.replace(/\x1b\[[0-9;]*m/g, '').length;
    contextLine += ' '.repeat(Math.max(0, width - visibleLength));

    lines.push(contextLine);

    return lines;
  }

  /**
   * Format keybinds into display string
   * @param binds Keybinds to format
   * @param scope Scope for coloring
   * @returns Formatted string
   */
  private formatKeybinds(binds: Keybind[], scope: 'global' | 'context'): string {
    const color = scope === 'global' ? this.theme.get('keybindGlobal') : this.theme.get('keybindContext');

    return binds
      .map((b) => colorize(`[${b.key}]`, color) + ` ${b.description}`)
      .join('  ');
  }

  /**
   * Get global keybinds
   */
  getGlobalBinds(): Keybind[] {
    return [...this.globalBinds];
  }

  /**
   * Get context keybinds
   */
  getContextBinds(): Keybind[] {
    return [...this.contextBinds];
  }
}

/**
 * Default global keybinds for the application
 */
export const DEFAULT_GLOBAL_KEYBINDS: Keybind[] = [
  { key: '^', description: 'Fenster wechseln', scope: 'global' },
  { key: 'Esc×3', description: 'Dashboard', scope: 'global' },
  { key: 'q', description: 'Beenden', scope: 'global' },
];

/**
 * Dashboard context keybinds
 */
export const DASHBOARD_KEYBINDS: Keybind[] = [
  { key: '1', description: 'Verleih', scope: 'context' },
  { key: '2', description: 'Finanzen', scope: 'context' },
  { key: '3', description: 'Verwaltung', scope: 'context' },
];

/**
 * Rental management keybinds
 */
export const RENTAL_KEYBINDS: Keybind[] = [
  { key: 'n', description: 'Neu', scope: 'context' },
  { key: 'e', description: 'Verlängern', scope: 'context' },
  { key: 'r', description: 'Rückgabe', scope: 'context' },
  { key: 'd', description: 'Schaden', scope: 'context' },
];

/**
 * Finances keybinds
 */
export const FINANCES_KEYBINDS: Keybind[] = [
  { key: 'p', description: 'Zahlung', scope: 'context' },
  { key: 'e', description: 'Export', scope: 'context' },
];

/**
 * Management keybinds
 */
export const MANAGEMENT_KEYBINDS: Keybind[] = [
  { key: 'l', description: 'Standorte', scope: 'context' },
  { key: 's', description: 'Einstellungen', scope: 'context' },
  { key: 'a', description: 'Audit-Log', scope: 'context' },
  { key: 'i', description: 'Import', scope: 'context' },
  { key: 'x', description: 'Export', scope: 'context' },
];
