/**
 * UI Module - Layout and Terminal Management
 * @module ui
 */

import type { Rect, Layout } from '../types/ui';

/**
 * Fixed heights for layout sections
 */
const HEADER_HEIGHT = 3;
const KEYBIND_BAR_HEIGHT = 2;
const STATUS_BAR_HEIGHT = 1;

/**
 * Calculate the main layout based on terminal dimensions
 * @param termWidth Terminal width in columns
 * @param termHeight Terminal height in rows
 * @returns Layout with all section rectangles
 */
export function calculateLayout(termWidth: number, termHeight: number): Layout {
  const contentHeight = termHeight - HEADER_HEIGHT - KEYBIND_BAR_HEIGHT - STATUS_BAR_HEIGHT;

  return {
    header: {
      x: 0,
      y: 0,
      width: termWidth,
      height: HEADER_HEIGHT,
    },
    content: {
      x: 0,
      y: HEADER_HEIGHT,
      width: termWidth,
      height: contentHeight,
    },
    keybindBar: {
      x: 0,
      y: HEADER_HEIGHT + contentHeight,
      width: termWidth,
      height: KEYBIND_BAR_HEIGHT,
    },
    statusBar: {
      x: 0,
      y: termHeight - STATUS_BAR_HEIGHT,
      width: termWidth,
      height: STATUS_BAR_HEIGHT,
    },
  };
}

/**
 * Calculate dashboard content layout (fixed heights)
 * @param contentRect Content area rectangle
 * @returns Object with section heights
 */
export function calculateDashboardLayout(contentRect: Rect): {
  overview: number;
  statistics: number;
  alerts: number;
  graph: number;
} {
  const OVERVIEW_HEIGHT = 5;
  const STATISTICS_HEIGHT = 8;
  const ALERTS_HEIGHT = 4;
  const MIN_GRAPH_HEIGHT = 10;

  const remainingHeight = contentRect.height - OVERVIEW_HEIGHT - STATISTICS_HEIGHT - ALERTS_HEIGHT;
  const graphHeight = Math.max(MIN_GRAPH_HEIGHT, remainingHeight);

  return {
    overview: OVERVIEW_HEIGHT,
    statistics: STATISTICS_HEIGHT,
    alerts: ALERTS_HEIGHT,
    graph: graphHeight,
  };
}

/**
 * Terminal abstraction for rendering
 * Provides a simple interface for terminal operations
 */
export class TerminalRenderer {
  private buffer: string[][] = [];
  private width: number;
  private height: number;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.clear();
  }

  /**
   * Clear the buffer
   */
  clear(): void {
    this.buffer = Array.from({ length: this.height }, () => Array(this.width).fill(' '));
  }

  /**
   * Write text at position
   * @param x X coordinate (column)
   * @param y Y coordinate (row)
   * @param text Text to write
   */
  write(x: number, y: number, text: string): void {
    if (y < 0 || y >= this.height) return;

    const chars = [...text];
    for (let i = 0; i < chars.length && x + i < this.width; i++) {
      if (x + i >= 0) {
        this.buffer[y][x + i] = chars[i];
      }
    }
  }

  /**
   * Draw a horizontal line
   * @param x Start X coordinate
   * @param y Y coordinate
   * @param length Line length
   * @param char Character to use (default: ─)
   */
  hline(x: number, y: number, length: number, char = '─'): void {
    for (let i = 0; i < length && x + i < this.width; i++) {
      if (y >= 0 && y < this.height && x + i >= 0) {
        this.buffer[y][x + i] = char;
      }
    }
  }

  /**
   * Draw a vertical line
   * @param x X coordinate
   * @param y Start Y coordinate
   * @param length Line length
   * @param char Character to use (default: │)
   */
  vline(x: number, y: number, length: number, char = '│'): void {
    for (let i = 0; i < length && y + i < this.height; i++) {
      if (x >= 0 && x < this.width && y + i >= 0) {
        this.buffer[y + i][x] = char;
      }
    }
  }

  /**
   * Draw a box border
   * @param rect Rectangle to draw border around
   */
  box(rect: Rect): void {
    const { x, y, width, height } = rect;

    // Corners
    if (y >= 0 && y < this.height && x >= 0 && x < this.width) {
      this.buffer[y][x] = '╔';
    }
    if (y >= 0 && y < this.height && x + width - 1 >= 0 && x + width - 1 < this.width) {
      this.buffer[y][x + width - 1] = '╗';
    }
    if (y + height - 1 >= 0 && y + height - 1 < this.height && x >= 0 && x < this.width) {
      this.buffer[y + height - 1][x] = '╚';
    }
    if (y + height - 1 >= 0 && y + height - 1 < this.height && x + width - 1 >= 0 && x + width - 1 < this.width) {
      this.buffer[y + height - 1][x + width - 1] = '╝';
    }

    // Horizontal lines
    this.hline(x + 1, y, width - 2, '═');
    this.hline(x + 1, y + height - 1, width - 2, '═');

    // Vertical lines
    this.vline(x, y + 1, height - 2, '║');
    this.vline(x + width - 1, y + 1, height - 2, '║');
  }

  /**
   * Get the rendered buffer as string
   * @returns Rendered content
   */
  render(): string {
    return this.buffer.map((row) => row.join('')).join('\n');
  }

  /**
   * Get terminal dimensions
   */
  getWidth(): number {
    return this.width;
  }

  getHeight(): number {
    return this.height;
  }

  /**
   * Resize the buffer
   * @param width New width
   * @param height New height
   */
  resize(width: number, height: number): void {
    this.width = width;
    this.height = height;
    this.clear();
  }
}

/**
 * Get terminal size from environment
 * @returns Object with columns and rows
 */
export function getTerminalSize(): { columns: number; rows: number } {
  return {
    columns: process.stdout.columns || 80,
    rows: process.stdout.rows || 24,
  };
}

/**
 * Move cursor to position
 * @param x Column (0-indexed)
 * @param y Row (0-indexed)
 * @returns ANSI escape sequence
 */
export function moveCursor(x: number, y: number): string {
  return `\x1b[${y + 1};${x + 1}H`;
}

/**
 * Clear the screen
 * @returns ANSI escape sequence
 */
export function clearScreen(): string {
  return '\x1b[2J\x1b[H';
}

/**
 * Hide cursor
 * @returns ANSI escape sequence
 */
export function hideCursor(): string {
  return '\x1b[?25l';
}

/**
 * Show cursor
 * @returns ANSI escape sequence
 */
export function showCursor(): string {
  return '\x1b[?25h';
}

/**
 * Enter alternate screen buffer
 * @returns ANSI escape sequence
 */
export function enterAlternateScreen(): string {
  return '\x1b[?1049h';
}

/**
 * Exit alternate screen buffer
 * @returns ANSI escape sequence
 */
export function exitAlternateScreen(): string {
  return '\x1b[?1049l';
}

/**
 * Pad text to specified width
 * @param text Text to pad
 * @param width Target width
 * @param align Alignment (left, center, right)
 * @returns Padded text
 */
export function padText(text: string, width: number, align: 'left' | 'center' | 'right' = 'left'): string {
  // Get visible length (excluding ANSI codes)
  const visibleLength = text.replace(/\x1b\[[0-9;]*m/g, '').length;

  if (visibleLength >= width) {
    return text.slice(0, width);
  }

  const padding = width - visibleLength;

  switch (align) {
    case 'center': {
      const leftPad = Math.floor(padding / 2);
      const rightPad = padding - leftPad;
      return ' '.repeat(leftPad) + text + ' '.repeat(rightPad);
    }
    case 'right':
      return ' '.repeat(padding) + text;
    default:
      return text + ' '.repeat(padding);
  }
}

/**
 * Truncate text to max width with ellipsis
 * @param text Text to truncate
 * @param maxWidth Maximum width
 * @returns Truncated text
 */
export function truncateText(text: string, maxWidth: number): string {
  if (text.length <= maxWidth) return text;
  if (maxWidth <= 3) return text.slice(0, maxWidth);
  return text.slice(0, maxWidth - 3) + '...';
}
