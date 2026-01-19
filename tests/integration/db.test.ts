/**
 * Database Tests
 * @module tests/integration/db.test
 */

import { describe, it, expect, beforeEach, afterEach } from 'bun:test';
import { DatabaseConnection } from '../../src/db/connection';
import { runMigrations, getCurrentSchemaVersion } from '../../src/db/migrations';
import { createLocker, getLockerById, getAllLockers, getAvailableLockers } from '../../src/db/lockers';
import { createRental, getRentalById, getActiveRentals, endRental } from '../../src/db/rentals';
import { createPayment, getPaymentsByRentalId, getRevenueForPeriod } from '../../src/db/payments';
import { logAction, getAuditLog } from '../../src/db/audit';

describe('Database', () => {
  beforeEach(() => {
    DatabaseConnection.enableTestMode();
    const db = DatabaseConnection.getConnection();
    runMigrations(db);
  });

  afterEach(() => {
    DatabaseConnection.close();
    DatabaseConnection.disableTestMode();
  });

  describe('Migrations', () => {
    it('should run migrations successfully', () => {
      const db = DatabaseConnection.getConnection();
      const version = getCurrentSchemaVersion(db);
      expect(version).toBe(3);
    });

    it('should create all required tables', () => {
      const db = DatabaseConnection.getConnection();
      
      const tables = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .all() as Array<{ name: string }>;
      
      const tableNames = tables.map(t => t.name);
      
      expect(tableNames).toContain('lockers');
      expect(tableNames).toContain('rentals');
      expect(tableNames).toContain('payments');
      expect(tableNames).toContain('locations');
      expect(tableNames).toContain('settings');
      expect(tableNames).toContain('audit_log');
      expect(tableNames).toContain('occupancy_history');
    });
  });

  describe('Lockers', () => {
    it('should create a locker', () => {
      const locker = createLocker('L001', 'Building A', 'M', 'Test notes');
      
      expect(locker.id).toBeGreaterThan(0);
      expect(locker.number).toBe('L001');
      expect(locker.location).toBe('Building A');
      expect(locker.size).toBe('M');
      expect(locker.notes).toBe('Test notes');
      expect(locker.is_damaged).toBe(false);
    });

    it('should get locker by ID', () => {
      const created = createLocker('L002', 'Building B', 'L');
      const fetched = getLockerById(created.id);
      
      expect(fetched).not.toBeNull();
      expect(fetched!.number).toBe('L002');
    });

    it('should list all lockers', () => {
      createLocker('L001', 'Building A', 'S');
      createLocker('L002', 'Building A', 'M');
      createLocker('L003', 'Building B', 'L');
      
      const lockers = getAllLockers();
      
      expect(lockers.length).toBe(3);
    });

    it('should get available lockers', () => {
      const locker1 = createLocker('L001', 'Building A', 'S');
      const locker2 = createLocker('L002', 'Building A', 'M');
      
      // Rent locker1
      createRental(locker1.id, 'Test User', new Date().toISOString().split('T')[0]);
      
      const available = getAvailableLockers();
      
      expect(available.length).toBe(1);
      expect(available[0].id).toBe(locker2.id);
    });
  });

  describe('Rentals', () => {
    it('should create a rental', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      const rental = createRental(
        locker.id,
        'John Doe',
        '2024-01-01',
        '2024-12-31',
        'john@example.com',
        '+1234567890',
        true
      );
      
      expect(rental.id).toBeGreaterThan(0);
      expect(rental.locker_id).toBe(locker.id);
      expect(rental.renter_name).toBe('John Doe');
      expect(rental.deposit_paid).toBe(true);
    });

    it('should get rental by ID', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      const created = createRental(locker.id, 'Jane Doe', '2024-01-01');
      const fetched = getRentalById(created.id);
      
      expect(fetched).not.toBeNull();
      expect(fetched!.renter_name).toBe('Jane Doe');
    });

    it('should get active rentals', () => {
      const locker1 = createLocker('L001', 'Building A', 'S');
      const locker2 = createLocker('L002', 'Building A', 'M');
      
      createRental(locker1.id, 'Active Renter', '2024-01-01'); // No end date = active
      createRental(locker2.id, 'Past Renter', '2023-01-01', '2023-06-01'); // Past end date
      
      const active = getActiveRentals();
      
      expect(active.length).toBe(1);
      expect(active[0].renter_name).toBe('Active Renter');
    });

    it('should end a rental', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      const rental = createRental(locker.id, 'Test Renter', '2024-01-01');
      
      const ended = endRental(rental.id, true);
      
      expect(ended).not.toBeNull();
      expect(ended!.end_date).not.toBeNull();
      expect(ended!.deposit_returned).toBe(true);
    });
  });

  describe('Payments', () => {
    it('should create a payment', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      const rental = createRental(locker.id, 'Test Renter', '2024-01-01');
      const payment = createPayment(rental.id, 1000, new Date().toISOString().split('T')[0], 'cash');
      
      expect(payment.id).toBeGreaterThan(0);
      expect(payment.amount_cents).toBe(1000);
    });

    it('should get payments for rental', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      const rental = createRental(locker.id, 'Test Renter', '2024-01-01');
      
      createPayment(rental.id, 500, new Date().toISOString().split('T')[0], 'cash');
      createPayment(rental.id, 1000, new Date().toISOString().split('T')[0], 'card');
      
      const payments = getPaymentsByRentalId(rental.id);
      
      expect(payments.length).toBe(2);
    });

    it('should calculate revenue for period', () => {
      const locker = createLocker('L001', 'Building A', 'M');
      const rental = createRental(locker.id, 'Test Renter', '2024-01-01');
      
      createPayment(rental.id, 1000, new Date().toISOString().split('T')[0], 'cash');
      createPayment(rental.id, 2000, new Date().toISOString().split('T')[0], 'card');
      
      const revenue = getRevenueForPeriod(30);
      
      expect(revenue).toBe(3000);
    });
  });

  describe('Audit Log', () => {
    it('should log actions', () => {
      logAction('create', 'locker', 1, { number: 'L001' }, 'test_user');
      
      const logs = getAuditLog(10);
      
      expect(logs.length).toBeGreaterThan(0);
      expect(logs[0].action).toBe('create');
      expect(logs[0].entity_type).toBe('locker');
    });

    it('should automatically log locker creation', () => {
      createLocker('L001', 'Building A', 'M');
      
      const logs = getAuditLog(10);
      
      const lockerLogs = logs.filter(l => l.entity_type === 'locker' && l.action === 'create');
      expect(lockerLogs.length).toBeGreaterThan(0);
    });
  });
});
