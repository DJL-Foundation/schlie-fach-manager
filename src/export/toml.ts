/**
 * TOML Export Module
 * @module export/toml
 */

import { getAllLockers } from '../db/lockers';
import { getAllLocations } from '../db/locations';
import { DatabaseConnection } from '../db/connection';

/**
 * Escape string value for TOML
 * @param str String to escape
 * @returns Escaped string
 */
function escapeTomlString(str: string): string {
  return str.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/\n/g, '\\n');
}

/**
 * Convert value to TOML format
 * @param value Value to convert
 * @returns TOML formatted string
 */
function toTomlValue(value: unknown): string {
  if (value === null || value === undefined) return '""';
  if (typeof value === 'string') return `"${escapeTomlString(value)}"`;
  if (typeof value === 'boolean') return value ? 'true' : 'false';
  if (typeof value === 'number') return value.toString();
  if (Array.isArray(value)) {
    return '[' + value.map(toTomlValue).join(', ') + ']';
  }
  return `"${String(value)}"`;
}

/**
 * Export all data to TOML format
 * @returns TOML string
 */
export function exportToToml(): string {
  const db = DatabaseConnection.getConnection();
  const lines: string[] = [];

  // Header
  lines.push('# Schließfach-Manager Export');
  lines.push(`# Generated: ${new Date().toISOString()}`);
  lines.push('');

  // Version
  lines.push('[meta]');
  lines.push('version = "2.1.0"');
  lines.push(`export_date = "${new Date().toISOString()}"`);
  lines.push('');

  // Settings
  lines.push('[settings]');
  const settings = db.prepare('SELECT key, value FROM settings').all() as Array<{
    key: string;
    value: string;
  }>;
  for (const setting of settings) {
    lines.push(`${setting.key} = ${toTomlValue(setting.value)}`);
  }
  lines.push('');

  // Locations
  const locations = getAllLocations();
  lines.push('# Standorte');
  for (const location of locations) {
    lines.push(`[[locations]]`);
    lines.push(`id = ${location.id}`);
    lines.push(`name = ${toTomlValue(location.name)}`);
    lines.push('');
  }

  // Lockers
  const lockers = getAllLockers();
  lines.push('# Schließfächer');
  for (const locker of lockers) {
    lines.push(`[[lockers]]`);
    lines.push(`id = ${locker.id}`);
    lines.push(`number = ${toTomlValue(locker.number)}`);
    lines.push(`location = ${toTomlValue(locker.location)}`);
    lines.push(`size = ${toTomlValue(locker.size)}`);
    lines.push(`is_damaged = ${locker.is_damaged}`);
    if (locker.notes) {
      lines.push(`notes = ${toTomlValue(locker.notes)}`);
    }
    lines.push('');
  }

  return lines.join('\n');
}

/**
 * Export settings only to TOML format
 * @returns TOML string
 */
export function exportSettingsToToml(): string {
  const db = DatabaseConnection.getConnection();
  const lines: string[] = [];

  lines.push('# Schließfach-Manager Settings');
  lines.push(`# Generated: ${new Date().toISOString()}`);
  lines.push('');

  const settings = db.prepare('SELECT key, value, description FROM settings').all() as Array<{
    key: string;
    value: string;
    description: string | null;
  }>;

  for (const setting of settings) {
    if (setting.description) {
      lines.push(`# ${setting.description}`);
    }
    lines.push(`${setting.key} = ${toTomlValue(setting.value)}`);
    lines.push('');
  }

  return lines.join('\n');
}
