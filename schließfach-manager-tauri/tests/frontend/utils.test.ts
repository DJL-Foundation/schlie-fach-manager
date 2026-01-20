import { describe, it, expect } from 'vitest';
import {
  formatDate,
  formatDateTime,
  formatCurrency,
  formatPercent,
  isOverdue,
  isExpiringSoon,
  truncate,
  capitalize,
} from '@/lib/utils';

describe('formatDate', () => {
  it('formats valid date string', () => {
    const result = formatDate('2024-01-15');
    expect(result).toBe('15.01.2024');
  });

  it('returns dash for empty string', () => {
    const result = formatDate('');
    expect(result).toBe('-');
  });

  it('returns dash for invalid date', () => {
    const result = formatDate('invalid');
    expect(result).toBe('-');
  });
});

describe('formatDateTime', () => {
  it('formats date with time', () => {
    const result = formatDateTime('2024-01-15T14:30:00');
    expect(result).toContain('15.01.2024');
  });
});

describe('formatCurrency', () => {
  it('formats cents to euros', () => {
    const result = formatCurrency(1000);
    expect(result).toContain('10');
    expect(result).toContain('€');
  });

  it('handles zero', () => {
    const result = formatCurrency(0);
    expect(result).toContain('0');
    expect(result).toContain('€');
  });
});

describe('formatPercent', () => {
  it('formats percentage with one decimal', () => {
    const result = formatPercent(75.555);
    expect(result).toBe('75.6%');
  });

  it('formats round percentages', () => {
    const result = formatPercent(50);
    expect(result).toBe('50.0%');
  });
});

describe('isOverdue', () => {
  it('returns true for past date', () => {
    const result = isOverdue('2020-01-01');
    expect(result).toBe(true);
  });

  it('returns false for future date', () => {
    const result = isOverdue('2030-01-01');
    expect(result).toBe(false);
  });

  it('returns false for empty date', () => {
    const result = isOverdue('');
    expect(result).toBe(false);
  });
});

describe('isExpiringSoon', () => {
  it('returns false for past date', () => {
    const result = isExpiringSoon('2020-01-01');
    expect(result).toBe(false);
  });

  it('returns false for far future date', () => {
    const result = isExpiringSoon('2030-01-01');
    expect(result).toBe(false);
  });

  it('returns false for empty date', () => {
    const result = isExpiringSoon('');
    expect(result).toBe(false);
  });
});

describe('truncate', () => {
  it('truncates long text', () => {
    const result = truncate('Hello World', 5);
    expect(result).toBe('Hello...');
  });

  it('keeps short text unchanged', () => {
    const result = truncate('Hi', 5);
    expect(result).toBe('Hi');
  });
});

describe('capitalize', () => {
  it('capitalizes first letter', () => {
    const result = capitalize('hello');
    expect(result).toBe('Hello');
  });

  it('lowercases rest of string', () => {
    const result = capitalize('HELLO');
    expect(result).toBe('Hello');
  });

  it('handles empty string', () => {
    const result = capitalize('');
    expect(result).toBe('');
  });
});
