/**
 * UI Types for Schließfach-Manager v2.1
 * @module types/ui
 */

/**
 * Rectangle dimensions for layout
 */
export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

/**
 * Main layout structure
 */
export interface Layout {
  header: Rect;
  content: Rect;
  keybindBar: Rect;
  statusBar: Rect;
}

/**
 * Keybind definition
 */
export interface Keybind {
  key: string;
  description: string;
  scope: 'global' | 'context';
}

/**
 * Header component properties
 */
export interface HeaderProps {
  version: string;
  currentScreen: string;
  windowSwitcherActive: boolean;
  windowList: WindowInfoUI[];
  selectedWindowIndex: number;
}

/**
 * Window information for UI display
 */
export interface WindowInfoUI {
  name: string;
  screen: string;
}

/**
 * Wizard message sender
 */
export type MessageSender = 'system' | 'user';

/**
 * Wizard option
 */
export interface WizardOption {
  label: string;
  value: string;
  metadata?: Record<string, unknown>;
}

/**
 * Wizard message content types
 */
export type MessageContent =
  | { type: 'question'; text: string; options: WizardOption[] }
  | { type: 'answer'; text: string }
  | { type: 'info'; text: string };

/**
 * Wizard message
 */
export interface WizardMessage {
  sender: MessageSender;
  content: MessageContent;
  timestamp: number;
}

/**
 * Wizard action results
 */
export type WizardAction =
  | { type: 'none' }
  | { type: 'next' }
  | { type: 'complete'; data: Record<string, unknown> }
  | { type: 'cancel' };

/**
 * Animation interface for screensaver
 */
export interface Animation {
  name: string;
  frames: string[];
  frameDelayMs: number;
  width: number;
  height: number;
}
