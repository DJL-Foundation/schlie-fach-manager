/**
 * Database Connection Management
 * @module db/connection
 */

import { Database } from 'bun:sqlite';
import { join } from 'path';
import { mkdirSync, existsSync } from 'fs';

/**
 * Singleton database connection manager
 */
export class DatabaseConnection {
  private static instance: Database | null = null;
  private static testMode = false;

  /**
   * Get the database connection instance
   * Creates a new connection if one doesn't exist
   */
  static getConnection(): Database {
    if (!this.instance) {
      const dbPath = this.getDatabasePath();

      // Ensure directory exists (not needed for :memory:)
      if (dbPath !== ':memory:') {
        const dbDir = join(dbPath, '..');

        if (!existsSync(dbDir)) {
          mkdirSync(dbDir, { recursive: true });
        }
      }

      this.instance = new Database(dbPath);
      this.instance.exec('PRAGMA journal_mode = WAL');
      this.instance.exec('PRAGMA foreign_keys = ON');
    }

    return this.instance;
  }

  /**
   * Get the database file path based on platform
   */
  private static getDatabasePath(): string {
    // Use in-memory database for tests
    if (process.env.TEST_DB === ':memory:' || this.testMode) {
      return ':memory:';
    }

    // Allow custom path via environment variable
    if (process.env.DB_PATH) {
      return process.env.DB_PATH;
    }

    const home = process.env.HOME || process.env.USERPROFILE || '';

    if (process.platform === 'darwin') {
      return join(home, 'Library', 'Application Support', 'schliessfach-manager', 'data.db');
    } else if (process.platform === 'win32') {
      return join(process.env.APPDATA || '', 'schliessfach-manager', 'data.db');
    } else {
      return join(home, '.local', 'share', 'schliessfach-manager', 'data.db');
    }
  }

  /**
   * Enable test mode (uses in-memory database)
   */
  static enableTestMode(): void {
    this.testMode = true;
    this.close(); // Close existing connection
  }

  /**
   * Disable test mode
   */
  static disableTestMode(): void {
    this.testMode = false;
    this.close();
  }

  /**
   * Close the database connection
   */
  static close(): void {
    if (this.instance) {
      this.instance.close();
      this.instance = null;
    }
  }

  /**
   * Check if the database connection is open
   */
  static isOpen(): boolean {
    return this.instance !== null;
  }
}
