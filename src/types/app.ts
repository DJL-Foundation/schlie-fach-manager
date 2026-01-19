/**
 * Application Types for Schließfach-Manager v2.1
 * @module types/app
 */

/**
 * Available screens in the application
 */
export type AppScreen =
  | 'dashboard'
  | 'rental'
  | 'finances'
  | 'management'
  | 'screensaver';

/**
 * Inactivity state for screensaver management
 */
export type InactivityState =
  | { type: 'active' }
  | { type: 'countdown'; seconds: number }
  | { type: 'screensaver' };

/**
 * Status message levels for the status bar
 */
export type StatusLevel = 'info' | 'success' | 'warning' | 'error';

/**
 * Status message displayed in the status bar
 */
export interface StatusMessage {
  text: string;
  level: StatusLevel;
  timestamp: number;
}

/**
 * Window information for window switcher
 */
export interface WindowInfo {
  name: string;
  screen: AppScreen;
}

/**
 * Application configuration
 */
export interface AppConfig {
  screensaverTimeoutSeconds: number;
  countdownDurationSeconds: number;
  depositCents: number;
  yearlyFeeCents: number;
  billingPeriod: 'monthly' | 'yearly';
  currency: string;
}

/**
 * Default application configuration
 */
export const DEFAULT_CONFIG: AppConfig = {
  screensaverTimeoutSeconds: 60,
  countdownDurationSeconds: 15,
  depositCents: 1000,
  yearlyFeeCents: 1000,
  billingPeriod: 'yearly',
  currency: 'EUR',
};
