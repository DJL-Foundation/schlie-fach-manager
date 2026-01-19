/**
 * Status Bar Widget with Escape Indicator
 * @module ui/widgets/status-bar
 */

import type { StatusLevel, StatusMessage } from '../../types/app';
import { Theme, colorize, ANSI } from '../theme';

/**
 * Status bar component for displaying messages and escape indicator
 */
export class StatusBar {
  private message: StatusMessage | null = null;
  private escapeCount = 0;
  private lastEscapeTime = 0;
  private readonly resetDelay = 1000; // 1 second
  private theme: Theme;

  constructor() {
    this.theme = Theme.defaultDark();
  }

  /**
   * Set the status message
   * @param text Message text
   * @param level Message level (affects color)
   */
  setMessage(text: string, level: StatusLevel = 'info'): void {
    this.message = {
      text,
      level,
      timestamp: Date.now(),
    };
  }

  /**
   * Clear the status message
   */
  clearMessage(): void {
    this.message = null;
  }

  /**
   * Get current message
   */
  getMessage(): StatusMessage | null {
    return this.message;
  }

  /**
   * Increment escape counter
   * Auto-resets after timeout
   */
  incrementEscapeCount(): void {
    const now = Date.now();

    // Reset if more than 1 second passed
    if (now - this.lastEscapeTime > this.resetDelay) {
      this.escapeCount = 0;
    }

    this.escapeCount++;
    this.lastEscapeTime = now;
  }

  /**
   * Reset escape counter
   */
  resetEscapeCount(): void {
    this.escapeCount = 0;
    this.lastEscapeTime = 0;
  }

  /**
   * Get current escape count
   */
  getEscapeCount(): number {
    // Check if should auto-reset
    if (Date.now() - this.lastEscapeTime > this.resetDelay && this.lastEscapeTime > 0) {
      this.escapeCount = 0;
      this.lastEscapeTime = 0;
    }
    return this.escapeCount;
  }

  /**
   * Check if should return to dashboard (3 escapes)
   */
  shouldReturnToDashboard(): boolean {
    return this.getEscapeCount() >= 3;
  }

  /**
   * Show screensaver countdown message
   * @param seconds Seconds until screensaver
   */
  showScreensaverCountdown(seconds: number): void {
    this.setMessage(`⏱ Screensaver in ${seconds} Sekunden...`, 'info');
  }

  /**
   * Render the status bar to string
   * @param width Available width
   * @returns Rendered status bar line
   */
  render(width: number): string {
    // Build message part
    let messageText = '';
    let messageColor = this.theme.get('text');

    if (this.message) {
      messageText = this.message.text;

      switch (this.message.level) {
        case 'success':
          messageColor = this.theme.get('statusSuccess');
          break;
        case 'warning':
          messageColor = this.theme.get('statusWarning');
          break;
        case 'error':
          messageColor = this.theme.get('statusError');
          break;
        default:
          messageColor = this.theme.get('statusInfo');
      }
    }

    // Build escape indicator
    const indicator = this.renderEscapeIndicator();
    const indicatorWidth = 5; // [│││]

    // Calculate message max width
    const maxMessageWidth = width - indicatorWidth - 2;
    const truncatedMessage = messageText.substring(0, maxMessageWidth);
    const paddedMessage = truncatedMessage.padEnd(maxMessageWidth);

    // Combine parts
    let result = colorize(paddedMessage, messageColor);
    result += ' ' + indicator;

    return result;
  }

  /**
   * Render the escape indicator [│││] or [║║║]
   */
  private renderEscapeIndicator(): string {
    const count = this.getEscapeCount();
    const pipes: string[] = [];

    for (let i = 0; i < 3; i++) {
      if (i < count) {
        // Filled pipe (active)
        pipes.push(colorize('║', this.theme.get('escapeIndicatorActive')));
      } else {
        // Empty pipe (inactive)
        pipes.push(colorize('│', this.theme.get('escapeIndicatorInactive')));
      }
    }

    return `[${pipes.join('')}]`;
  }
}

/**
 * Format a success message
 * @param text Message text
 * @returns Formatted message with icon
 */
export function successMessage(text: string): string {
  return `✓ ${text}`;
}

/**
 * Format a warning message
 * @param text Message text
 * @returns Formatted message with icon
 */
export function warningMessage(text: string): string {
  return `⚠ ${text}`;
}

/**
 * Format an error message
 * @param text Message text
 * @returns Formatted message with icon
 */
export function errorMessage(text: string): string {
  return `✗ ${text}`;
}

/**
 * Format an info message
 * @param text Message text
 * @returns Formatted message with icon
 */
export function infoMessage(text: string): string {
  return `ℹ ${text}`;
}
