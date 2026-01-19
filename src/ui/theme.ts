/**
 * Theme System for UI Components
 * @module ui/theme
 */

/**
 * Theme color definitions
 */
export interface ThemeColors {
  // Base
  background: string;
  foreground: string;
  text: string;
  textDim: string;

  // Accent
  primary: string;
  secondary: string;
  accent: string;

  // Status
  success: string;
  warning: string;
  error: string;
  info: string;

  // UI Elements
  border: string;
  borderActive: string;
  headerBg: string;
  headerFg: string;

  // Keybinds
  keybindGlobal: string;
  keybindContext: string;

  // Window Switcher
  windowCurrent: string;
  windowAdjacent: string;

  // Wizard
  wizardSystem: string;
  wizardUser: string;
  wizardOptionSelected: string;
  wizardOptionNormal: string;

  // Status Bar
  statusInfo: string;
  statusSuccess: string;
  statusWarning: string;
  statusError: string;

  // Escape Indicator
  escapeIndicatorActive: string;
  escapeIndicatorInactive: string;

  // Screensaver
  screensaverBg: string;
  screensaverFg: string;
}

/**
 * Theme class for managing UI colors
 */
export class Theme {
  private constructor(private colors: ThemeColors) {}

  /**
   * Get the default dark theme (Catppuccin-inspired)
   */
  static defaultDark(): Theme {
    return new Theme({
      // Base
      background: '#1e1e2e',
      foreground: '#cdd6f4',
      text: '#cdd6f4',
      textDim: '#6c7086',

      // Accent
      primary: '#89b4fa',
      secondary: '#94e2d5',
      accent: '#f38ba8',

      // Status
      success: '#a6e3a1',
      warning: '#f9e2af',
      error: '#f38ba8',
      info: '#89dceb',

      // UI Elements
      border: '#45475a',
      borderActive: '#89b4fa',
      headerBg: '#313244',
      headerFg: '#cdd6f4',

      // Keybinds
      keybindGlobal: '#89b4fa',
      keybindContext: '#f9e2af',

      // Window Switcher
      windowCurrent: '#a6e3a1',
      windowAdjacent: '#6c7086',

      // Wizard
      wizardSystem: '#89dceb',
      wizardUser: '#f5c2e7',
      wizardOptionSelected: '#a6e3a1',
      wizardOptionNormal: '#cdd6f4',

      // Status Bar
      statusInfo: '#89dceb',
      statusSuccess: '#a6e3a1',
      statusWarning: '#f9e2af',
      statusError: '#f38ba8',

      // Escape Indicator
      escapeIndicatorActive: '#f38ba8',
      escapeIndicatorInactive: '#45475a',

      // Screensaver
      screensaverBg: '#000000',
      screensaverFg: '#a6e3a1',
    });
  }

  /**
   * Get a color by key
   * @param key Color key
   * @returns Color value (hex string)
   */
  get<K extends keyof ThemeColors>(key: K): string {
    return this.colors[key];
  }

  /**
   * Get all colors
   * @returns All theme colors
   */
  getAll(): ThemeColors {
    return { ...this.colors };
  }
}

/**
 * ANSI color codes for terminal output
 */
export const ANSI = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  italic: '\x1b[3m',
  underline: '\x1b[4m',
  blink: '\x1b[5m',
  reverse: '\x1b[7m',
  hidden: '\x1b[8m',

  // Foreground colors
  black: '\x1b[30m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  white: '\x1b[37m',

  // Background colors
  bgBlack: '\x1b[40m',
  bgRed: '\x1b[41m',
  bgGreen: '\x1b[42m',
  bgYellow: '\x1b[43m',
  bgBlue: '\x1b[44m',
  bgMagenta: '\x1b[45m',
  bgCyan: '\x1b[46m',
  bgWhite: '\x1b[47m',

  // Bright foreground colors
  brightBlack: '\x1b[90m',
  brightRed: '\x1b[91m',
  brightGreen: '\x1b[92m',
  brightYellow: '\x1b[93m',
  brightBlue: '\x1b[94m',
  brightMagenta: '\x1b[95m',
  brightCyan: '\x1b[96m',
  brightWhite: '\x1b[97m',
} as const;

/**
 * Convert hex color to ANSI 256 color code
 * @param hex Hex color string
 * @returns ANSI escape sequence for the color
 */
export function hexToAnsi256(hex: string): string {
  // Remove # if present
  hex = hex.replace('#', '');

  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);

  // Convert to 256 color
  const code = 16 + 36 * Math.round((r / 255) * 5) + 6 * Math.round((g / 255) * 5) + Math.round((b / 255) * 5);

  return `\x1b[38;5;${code}m`;
}

/**
 * Convert hex color to ANSI 256 background color code
 * @param hex Hex color string
 * @returns ANSI escape sequence for the background color
 */
export function hexToAnsi256Bg(hex: string): string {
  // Remove # if present
  hex = hex.replace('#', '');

  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);

  // Convert to 256 color
  const code = 16 + 36 * Math.round((r / 255) * 5) + 6 * Math.round((g / 255) * 5) + Math.round((b / 255) * 5);

  return `\x1b[48;5;${code}m`;
}

/**
 * Apply color to text using ANSI codes
 * @param text Text to colorize
 * @param fg Foreground hex color
 * @param bg Optional background hex color
 * @returns Colorized text with ANSI codes
 */
export function colorize(text: string, fg: string, bg?: string): string {
  let result = hexToAnsi256(fg);
  if (bg) {
    result += hexToAnsi256Bg(bg);
  }
  return result + text + ANSI.reset;
}
