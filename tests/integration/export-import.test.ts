/**
 * Export/Import Tests
 * @module tests/integration/export-import.test
 */

import { describe, it, expect, beforeEach, afterEach } from 'bun:test';
import { DatabaseConnection } from '../../src/db/connection';
import { runMigrations } from '../../src/db/migrations';
import { createLocker, getAllLockers } from '../../src/db/lockers';
import { exportToJson, exportSummaryToJson } from '../../src/export/json';
import { exportLockersToCsv, exportRentalsToCsv } from '../../src/export/csv';
import { exportToToml, exportSettingsToToml } from '../../src/export/toml';
import { generateMarkdownReport, generateShortSummary } from '../../src/export/markdown';
import { importFromJson, validateJsonImport } from '../../src/import/json';
import { importFromToml } from '../../src/import/toml';

describe('Export/Import', () => {
  beforeEach(() => {
    DatabaseConnection.enableTestMode();
    const db = DatabaseConnection.getConnection();
    runMigrations(db);
  });

  afterEach(() => {
    DatabaseConnection.close();
    DatabaseConnection.disableTestMode();
  });

  describe('JSON Export', () => {
    it('should export data to JSON', () => {
      createLocker('L001', 'Building A', 'M');
      createLocker('L002', 'Building B', 'L');

      const json = exportToJson();
      const data = JSON.parse(json);

      expect(data.version).toBe('2.1.0');
      expect(data.lockers).toHaveLength(2);
      expect(data.exportDate).toBeDefined();
    });

    it('should export summary to JSON', () => {
      createLocker('L001', 'Building A', 'M');

      const json = exportSummaryToJson();
      const data = JSON.parse(json);

      expect(data.totals.lockers).toBe(1);
      expect(data.locationStats).toBeDefined();
    });
  });

  describe('CSV Export', () => {
    it('should export lockers to CSV', () => {
      createLocker('L001', 'Building A', 'M', 'Test notes');

      const csv = exportLockersToCsv();
      
      expect(csv).toContain('Nummer');
      expect(csv).toContain('L001');
      expect(csv).toContain('Building A');
    });

    it('should export rentals to CSV', () => {
      const csv = exportRentalsToCsv();
      
      expect(csv).toContain('Schließfach');
      expect(csv).toContain('Mieter');
    });
  });

  describe('TOML Export', () => {
    it('should export data to TOML', () => {
      createLocker('L001', 'Building A', 'M');

      const toml = exportToToml();
      
      expect(toml).toContain('[meta]');
      expect(toml).toContain('version = "2.1.0"');
      expect(toml).toContain('[[lockers]]');
      expect(toml).toContain('number = "L001"');
    });

    it('should export settings to TOML', () => {
      const toml = exportSettingsToToml();
      
      expect(toml).toContain('# Schließfach-Manager Settings');
      expect(toml).toContain('deposit_cents');
    });
  });

  describe('Markdown Export', () => {
    it('should generate full markdown report', () => {
      createLocker('L001', 'Building A', 'M');

      const md = generateMarkdownReport();
      
      expect(md).toContain('# Schließfach-Manager Bericht');
      expect(md).toContain('## Übersicht');
      expect(md).toContain('Schließfächer gesamt');
    });

    it('should generate short summary', () => {
      createLocker('L001', 'Building A', 'M');

      const md = generateShortSummary();
      
      expect(md).toContain('# Kurzübersicht');
      expect(md).toContain('Belegung:');
    });
  });

  describe('JSON Import', () => {
    it('should import lockers from JSON', () => {
      const importData = {
        version: '2.1.0',
        lockers: [
          { number: 'L100', location: 'Test Location', size: 'S' },
          { number: 'L101', location: 'Test Location', size: 'M' },
        ],
      };

      const result = importFromJson(JSON.stringify(importData));

      expect(result.success).toBe(true);
      expect(result.lockers.imported).toBe(2);

      const lockers = getAllLockers();
      expect(lockers.length).toBe(2);
    });

    it('should skip existing lockers', () => {
      createLocker('L001', 'Building A', 'M');

      const importData = {
        lockers: [{ number: 'L001', location: 'Building A', size: 'M' }],
      };

      const result = importFromJson(JSON.stringify(importData), { skipExisting: true });

      expect(result.success).toBe(true);
      expect(result.lockers.skipped).toBe(1);
    });

    it('should validate JSON import data', () => {
      const validData = { lockers: [{ number: 'L001', location: 'Test', size: 'M' }] };
      const invalidData = { lockers: [{ number: 'L001' }] }; // Missing required fields

      const validResult = validateJsonImport(JSON.stringify(validData));
      expect(validResult.valid).toBe(true);

      const invalidResult = validateJsonImport(JSON.stringify(invalidData));
      expect(invalidResult.valid).toBe(false);
    });
  });

  describe('TOML Import', () => {
    it('should import lockers from TOML', () => {
      const toml = `
[meta]
version = "2.1.0"

[[lockers]]
number = "L200"
location = "TOML Location"
size = "L"

[[lockers]]
number = "L201"
location = "TOML Location"
size = "XL"
`;

      const result = importFromToml(toml);

      expect(result.success).toBe(true);
      expect(result.lockers.imported).toBe(2);

      const lockers = getAllLockers();
      expect(lockers.length).toBe(2);
    });
  });

  describe('Round-trip Export/Import', () => {
    it('should preserve data through JSON round-trip', () => {
      // Create test data
      createLocker('L001', 'Building A', 'M', 'Test note');
      createLocker('L002', 'Building B', 'L');

      // Export
      const exported = exportToJson();

      // Clear database
      const db = DatabaseConnection.getConnection();
      db.exec('DELETE FROM lockers');

      expect(getAllLockers().length).toBe(0);

      // Import
      const result = importFromJson(exported);

      expect(result.success).toBe(true);
      expect(result.lockers.imported).toBe(2);

      const lockers = getAllLockers();
      expect(lockers.length).toBe(2);
      expect(lockers.find((l) => l.number === 'L001')?.notes).toBe('Test note');
    });
  });
});
