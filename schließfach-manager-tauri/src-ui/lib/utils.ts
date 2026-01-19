import { format, parseISO, isValid } from 'date-fns';
import { de } from 'date-fns/locale';

/**
 * Format a date string to German locale format
 */
export function formatDate(dateString: string, formatStr = 'dd.MM.yyyy'): string {
  if (!dateString) return '-';
  try {
    const date = parseISO(dateString);
    if (!isValid(date)) return '-';
    return format(date, formatStr, { locale: de });
  } catch {
    return '-';
  }
}

/**
 * Format a date string to include time
 */
export function formatDateTime(dateString: string): string {
  return formatDate(dateString, 'dd.MM.yyyy HH:mm');
}

/**
 * Format cents to currency string (EUR)
 */
export function formatCurrency(cents: number): string {
  const euros = cents / 100;
  return new Intl.NumberFormat('de-DE', {
    style: 'currency',
    currency: 'EUR',
  }).format(euros);
}

/**
 * Format percentage with one decimal place
 */
export function formatPercent(value: number): string {
  return `${value.toFixed(1)}%`;
}

/**
 * Check if a rental is overdue
 */
export function isOverdue(endDate: string): boolean {
  if (!endDate) return false;
  try {
    const date = parseISO(endDate);
    return isValid(date) && date < new Date();
  } catch {
    return false;
  }
}

/**
 * Check if a rental is expiring soon (within days)
 */
export function isExpiringSoon(endDate: string, days = 7): boolean {
  if (!endDate) return false;
  try {
    const date = parseISO(endDate);
    if (!isValid(date)) return false;
    const now = new Date();
    const future = new Date(now.getTime() + days * 24 * 60 * 60 * 1000);
    return date >= now && date <= future;
  } catch {
    return false;
  }
}

/**
 * Get current timestamp in ISO format
 */
export function getCurrentTimestamp(): string {
  return new Date().toISOString();
}

/**
 * Generate a unique ID
 */
export function generateId(): string {
  return crypto.randomUUID();
}

/**
 * Debounce function for inputs
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;
  
  return function executedFunction(...args: Parameters<T>) {
    if (timeout) {
      clearTimeout(timeout);
    }
    timeout = setTimeout(() => {
      func(...args);
    }, wait);
  };
}

/**
 * Truncate text with ellipsis
 */
export function truncate(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text;
  return `${text.slice(0, maxLength)}...`;
}

/**
 * Capitalize first letter of string
 */
export function capitalize(text: string): string {
  if (!text) return '';
  return text.charAt(0).toUpperCase() + text.slice(1).toLowerCase();
}
