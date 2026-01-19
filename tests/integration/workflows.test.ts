/**
 * Workflow Tests
 * @module tests/integration/workflows.test
 */

import { describe, it, expect, beforeEach, afterEach } from 'bun:test';
import { DatabaseConnection } from '../../src/db/connection';
import { runMigrations } from '../../src/db/migrations';
import { createLocker, getAllLockers, getAvailableLockers } from '../../src/db/lockers';
import { createRental, getActiveRentals, endRental } from '../../src/db/rentals';
import {
  createWorkflowContext,
  createYesNoOptions,
  createSizeOptions,
  createPaymentTypeOptions,
  formatCurrency,
  formatDate,
  getCurrentDate,
  calculateEndDate,
} from '../../src/workflows/common';

describe('Workflows', () => {
  beforeEach(() => {
    DatabaseConnection.enableTestMode();
    const db = DatabaseConnection.getConnection();
    runMigrations(db);
  });

  afterEach(() => {
    DatabaseConnection.close();
    DatabaseConnection.disableTestMode();
  });

  describe('Common Utilities', () => {
    it('should create workflow context', () => {
      const context = createWorkflowContext(5);
      
      expect(context.state).toBe('idle');
      expect(context.currentStep).toBe(0);
      expect(context.totalSteps).toBe(5);
    });

    it('should create yes/no options', () => {
      const options = createYesNoOptions();
      
      expect(options).toHaveLength(2);
      expect(options[0].value).toBe('yes');
      expect(options[1].value).toBe('no');
    });

    it('should create size options', () => {
      const options = createSizeOptions();
      
      expect(options).toHaveLength(4);
      expect(options.map((o) => o.value)).toEqual(['S', 'M', 'L', 'XL']);
    });

    it('should create payment type options', () => {
      const options = createPaymentTypeOptions();
      
      expect(options).toHaveLength(4);
      expect(options.map((o) => o.value)).toEqual(['cash', 'card', 'transfer', 'other']);
    });

    it('should format currency correctly', () => {
      expect(formatCurrency(1000)).toBe('€10.00');
      expect(formatCurrency(1234)).toBe('€12.34');
      expect(formatCurrency(0)).toBe('€0.00');
    });

    it('should format date correctly', () => {
      const formatted = formatDate('2024-12-25');
      expect(formatted).toContain('25');
      expect(formatted).toContain('12');
      expect(formatted).toContain('2024');
    });

    it('should get current date in ISO format', () => {
      const date = getCurrentDate();
      expect(date).toMatch(/^\d{4}-\d{2}-\d{2}$/);
    });

    it('should calculate end date correctly', () => {
      const endDate = calculateEndDate('2024-01-15', 3);
      expect(endDate).toBe('2024-04-15');

      const yearLater = calculateEndDate('2024-01-15', 12);
      expect(yearLater).toBe('2025-01-15');
    });
  });

  describe('Rental Workflow Integration', () => {
    it('should have available lockers for rent workflow', () => {
      createLocker('L001', 'Building A', 'M');
      createLocker('L002', 'Building A', 'L');
      createLocker('L003', 'Building B', 'S');

      const available = getAvailableLockers();
      expect(available.length).toBe(3);
    });

    it('should reduce available lockers after rental', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      createLocker('L002', 'Building A', 'L');

      expect(getAvailableLockers().length).toBe(2);

      createRental(locker.id, 'Test User', getCurrentDate());

      expect(getAvailableLockers().length).toBe(1);
    });
  });

  describe('Return Workflow Integration', () => {
    it('should have active rentals for return workflow', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      createRental(locker.id, 'Test User', getCurrentDate());

      const active = getActiveRentals();
      expect(active.length).toBe(1);
    });

    it('should end rental and mark locker for future availability', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      const rental = createRental(locker.id, 'Test User', getCurrentDate());

      expect(getAvailableLockers().length).toBe(0);

      const ended = endRental(rental.id, true);

      // Rental is ended and marked as returned
      expect(ended).not.toBeNull();
      expect(ended!.deposit_returned).toBe(true);
      expect(ended!.end_date).toBe(getCurrentDate());
    });
  });

  describe('Extend Workflow Integration', () => {
    it('should have active rentals to extend', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      createRental(locker.id, 'Test User', getCurrentDate(), calculateEndDate(getCurrentDate(), 12));

      const active = getActiveRentals();
      expect(active.length).toBe(1);
      expect(active[0].end_date).toBeDefined();
    });
  });

  describe('Damage Workflow Integration', () => {
    it('should mark locker as damaged', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      expect(locker.is_damaged).toBe(false);

      const db = DatabaseConnection.getConnection();
      db.prepare('UPDATE lockers SET is_damaged = 1 WHERE id = ?').run(locker.id);

      const lockers = getAllLockers();
      expect(lockers[0].is_damaged).toBe(true);
    });

    it('should exclude damaged lockers from available', () => {
      createLocker('L001', 'Building A', 'M');
      const damaged = createLocker('L002', 'Building A', 'L');

      const db = DatabaseConnection.getConnection();
      db.prepare('UPDATE lockers SET is_damaged = 1 WHERE id = ?').run(damaged.id);

      const available = getAvailableLockers();
      expect(available.length).toBe(1);
      expect(available[0].number).toBe('L001');
    });
  });
});
