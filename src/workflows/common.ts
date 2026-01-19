/**
 * Common Workflow Utilities
 * @module workflows/common
 */

import { WizardRenderer } from '../ui/widgets/wizard';
import type { WizardOption, WizardMessage } from '../types/ui';

/**
 * Workflow state machine states
 */
export type WorkflowState = 'idle' | 'active' | 'completed' | 'cancelled';

/**
 * Base workflow context
 */
export interface WorkflowContext {
  state: WorkflowState;
  data: Record<string, unknown>;
  currentStep: number;
  totalSteps: number;
}

/**
 * Create a new workflow context
 */
export function createWorkflowContext(totalSteps: number): WorkflowContext {
  return {
    state: 'idle',
    data: {},
    currentStep: 0,
    totalSteps,
  };
}

/**
 * Workflow step definition
 */
export interface WorkflowStep {
  id: string;
  question: string;
  options: WizardOption[];
  validate?: (value: string) => boolean;
  transform?: (value: string) => unknown;
}

/**
 * Create options from array of strings
 * @param items Items to create options from
 * @returns Array of wizard options
 */
export function createOptionsFromStrings(items: string[]): WizardOption[] {
  return items.map((item) => ({
    label: item,
    value: item,
  }));
}

/**
 * Create options with custom labels
 * @param items Items with label and value
 * @returns Array of wizard options
 */
export function createOptions(
  items: Array<{ label: string; value: string; metadata?: Record<string, unknown> }>
): WizardOption[] {
  return items.map((item) => ({
    label: item.label,
    value: item.value,
    metadata: item.metadata,
  }));
}

/**
 * Create yes/no options
 * @returns Yes/No wizard options
 */
export function createYesNoOptions(): WizardOption[] {
  return [
    { label: 'Ja', value: 'yes' },
    { label: 'Nein', value: 'no' },
  ];
}

/**
 * Create confirm/cancel options
 * @returns Confirm/Cancel wizard options
 */
export function createConfirmCancelOptions(): WizardOption[] {
  return [
    { label: 'Bestätigen', value: 'confirm' },
    { label: 'Abbrechen', value: 'cancel' },
  ];
}

/**
 * Create size options
 * @returns Locker size options
 */
export function createSizeOptions(): WizardOption[] {
  return [
    { label: 'S - Klein', value: 'S' },
    { label: 'M - Mittel', value: 'M' },
    { label: 'L - Groß', value: 'L' },
    { label: 'XL - Extra Groß', value: 'XL' },
  ];
}

/**
 * Create payment type options
 * @returns Payment type options
 */
export function createPaymentTypeOptions(): WizardOption[] {
  return [
    { label: 'Bargeld', value: 'cash' },
    { label: 'Karte', value: 'card' },
    { label: 'Überweisung', value: 'transfer' },
    { label: 'Sonstige', value: 'other' },
  ];
}

/**
 * Format currency from cents
 * @param cents Amount in cents
 * @returns Formatted currency string
 */
export function formatCurrency(cents: number): string {
  return `€${(cents / 100).toFixed(2)}`;
}

/**
 * Format date for display
 * @param dateString ISO date string
 * @returns Formatted date string
 */
export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleDateString('de-DE', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  });
}

/**
 * Get current date as ISO string (date only)
 * @returns Current date as YYYY-MM-DD
 */
export function getCurrentDate(): string {
  return new Date().toISOString().split('T')[0];
}

/**
 * Calculate end date from start date and period
 * @param startDate Start date (ISO string)
 * @param months Number of months
 * @returns End date (ISO string)
 */
export function calculateEndDate(startDate: string, months: number): string {
  const date = new Date(startDate);
  date.setMonth(date.getMonth() + months);
  return date.toISOString().split('T')[0];
}
