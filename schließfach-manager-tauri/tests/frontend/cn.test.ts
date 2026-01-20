import { describe, it, expect } from 'vitest';
import { cn } from '@/lib/cn';

describe('cn utility', () => {
  it('merges class names', () => {
    const result = cn('class1', 'class2');
    expect(result).toBe('class1 class2');
  });

  it('handles conditional classes', () => {
    const result = cn('base', true && 'active', false && 'inactive');
    expect(result).toBe('base active');
  });

  it('handles undefined values', () => {
    const result = cn('base', undefined, 'other');
    expect(result).toBe('base other');
  });

  it('handles null values', () => {
    const result = cn('base', null, 'other');
    expect(result).toBe('base other');
  });

  it('merges Tailwind classes correctly', () => {
    const result = cn('p-4', 'p-6');
    expect(result).toBe('p-6');
  });

  it('handles object syntax', () => {
    const result = cn({ 'active': true, 'disabled': false });
    expect(result).toBe('active');
  });

  it('handles array syntax', () => {
    const result = cn(['class1', 'class2']);
    expect(result).toBe('class1 class2');
  });
});
