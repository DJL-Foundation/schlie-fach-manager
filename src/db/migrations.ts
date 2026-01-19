/**
 * Database Schema Migrations
 * @module db/migrations
 */

import type { Database } from 'bun:sqlite';

/**
 * Current schema version
 */
const CURRENT_SCHEMA_VERSION = 3;

/**
 * Run all pending migrations
 * @param db Database connection
 */
export function runMigrations(db: Database): void {
  const currentVersion = getSchemaVersion(db);

  if (currentVersion < 1) {
    createInitialSchema(db);
  }

  if (currentVersion < 2) {
    addSettingsTable(db);
    addAuditLogTable(db);
  }

  if (currentVersion < 3) {
    addOccupancyHistoryTable(db);
  }

  setSchemaVersion(db, CURRENT_SCHEMA_VERSION);
}

/**
 * Get the current schema version from database
 * @param db Database connection
 * @returns Current schema version number
 */
function getSchemaVersion(db: Database): number {
  try {
    const result = db.prepare('SELECT value FROM settings WHERE key = ?').get('schema_version') as
      | { value: string }
      | undefined;
    return result ? parseInt(result.value, 10) : 0;
  } catch {
    return 0;
  }
}

/**
 * Set the schema version in database
 * @param db Database connection
 * @param version Version number to set
 */
function setSchemaVersion(db: Database, version: number): void {
  db.prepare(
    `
    INSERT OR REPLACE INTO settings (key, value, description, updated_at)
    VALUES (?, ?, ?, datetime('now'))
  `
  ).run('schema_version', version.toString(), 'Database schema version');
}

/**
 * Create initial schema (migration v1)
 * @param db Database connection
 */
function createInitialSchema(db: Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS locations (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL UNIQUE,
      created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    
    CREATE TABLE IF NOT EXISTS lockers (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      number TEXT NOT NULL UNIQUE,
      location TEXT NOT NULL,
      size TEXT NOT NULL CHECK(size IN ('S', 'M', 'L', 'XL')),
      is_damaged BOOLEAN NOT NULL DEFAULT 0,
      notes TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    
    CREATE TABLE IF NOT EXISTS rentals (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      locker_id INTEGER NOT NULL,
      renter_name TEXT NOT NULL,
      renter_email TEXT,
      renter_phone TEXT,
      start_date TEXT NOT NULL,
      end_date TEXT,
      deposit_paid BOOLEAN NOT NULL DEFAULT 0,
      deposit_returned BOOLEAN NOT NULL DEFAULT 0,
      notes TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      FOREIGN KEY (locker_id) REFERENCES lockers(id)
    );
    
    CREATE TABLE IF NOT EXISTS payments (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      rental_id INTEGER NOT NULL,
      amount_cents INTEGER NOT NULL,
      payment_date TEXT NOT NULL,
      payment_type TEXT NOT NULL CHECK(payment_type IN ('cash', 'card', 'transfer', 'other')),
      notes TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      FOREIGN KEY (rental_id) REFERENCES rentals(id)
    );
    
    CREATE INDEX IF NOT EXISTS idx_rentals_locker ON rentals(locker_id);
    CREATE INDEX IF NOT EXISTS idx_rentals_dates ON rentals(start_date, end_date);
    CREATE INDEX IF NOT EXISTS idx_payments_rental ON payments(rental_id);
  `);
}

/**
 * Add settings table (migration v2 part 1)
 * @param db Database connection
 */
function addSettingsTable(db: Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS settings (
      key TEXT PRIMARY KEY NOT NULL,
      value TEXT NOT NULL,
      description TEXT,
      updated_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    
    INSERT OR IGNORE INTO settings (key, value, description) VALUES
      ('deposit_cents', '1000', 'Pfandbetrag in Cents (Standard: 10.00€)'),
      ('yearly_fee_cents', '1000', 'Jahresgebühr in Cents (Standard: 10.00€)'),
      ('billing_period', 'yearly', 'Berechnungszeitraum: monthly oder yearly'),
      ('currency', 'EUR', 'Währung (ISO 4217 Code)'),
      ('screensaver_timeout_seconds', '60', 'Sekunden Inaktivität bis Screensaver'),
      ('app_version', '2.1.0', 'Anwendungsversion');
  `);
}

/**
 * Add audit log table (migration v2 part 2)
 * @param db Database connection
 */
function addAuditLogTable(db: Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS audit_log (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      timestamp TEXT NOT NULL DEFAULT (datetime('now')),
      action TEXT NOT NULL CHECK(action IN ('create', 'update', 'delete', 'export', 'import')),
      entity_type TEXT NOT NULL CHECK(entity_type IN ('locker', 'rental', 'payment', 'location', 'settings')),
      entity_id INTEGER,
      details TEXT,
      username TEXT NOT NULL DEFAULT 'system'
    );
    
    CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp);
    CREATE INDEX IF NOT EXISTS idx_audit_entity ON audit_log(entity_type, entity_id);
  `);
}

/**
 * Add occupancy history table (migration v3)
 * @param db Database connection
 */
function addOccupancyHistoryTable(db: Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS occupancy_history (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      snapshot_date TEXT NOT NULL UNIQUE,
      total_lockers INTEGER NOT NULL,
      occupied_lockers INTEGER NOT NULL,
      occupancy_percent REAL NOT NULL,
      notes TEXT
    );
    
    CREATE INDEX IF NOT EXISTS idx_occupancy_date ON occupancy_history(snapshot_date);
  `);
}

/**
 * Get the current schema version (public API)
 * @param db Database connection
 * @returns Current schema version
 */
export function getCurrentSchemaVersion(db: Database): number {
  return getSchemaVersion(db);
}
