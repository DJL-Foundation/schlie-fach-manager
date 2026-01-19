/**
 * TOML Import Module
 * @module import/toml
 */

import { z } from 'zod';
import { createLocker, getLockerByNumber } from '../db/lockers';
import { getOrCreateLocation } from '../db/locations';
import { logAction } from '../db/audit';
import { DatabaseConnection } from '../db/connection';
import type { LockerSize } from '../types/database';

/**
 * Simple TOML parser (limited functionality for our use case)
 * @param tomlString TOML string to parse
 * @returns Parsed object
 */
function parseSimpleToml(tomlString: string): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  const lines = tomlString.split('\n');

  let currentSection = '';
  let currentArray: Array<Record<string, unknown>> | null = null;
  let currentItem: Record<string, unknown> | null = null;

  for (const rawLine of lines) {
    const line = rawLine.trim();

    // Skip comments and empty lines
    if (line.startsWith('#') || line === '') {
      continue;
    }

    // Array of tables [[name]]
    const arrayMatch = line.match(/^\[\[(\w+)\]\]$/);
    if (arrayMatch) {
      if (currentItem && currentArray) {
        currentArray.push(currentItem);
      }

      const arrayName = arrayMatch[1];
      if (!result[arrayName]) {
        result[arrayName] = [];
      }
      currentArray = result[arrayName] as Array<Record<string, unknown>>;
      currentItem = {};
      currentSection = '';
      continue;
    }

    // Single table [name]
    const sectionMatch = line.match(/^\[(\w+)\]$/);
    if (sectionMatch) {
      if (currentItem && currentArray) {
        currentArray.push(currentItem);
        currentItem = null;
        currentArray = null;
      }

      currentSection = sectionMatch[1];
      result[currentSection] = result[currentSection] || {};
      continue;
    }

    // Key = value
    const kvMatch = line.match(/^(\w+)\s*=\s*(.+)$/);
    if (kvMatch) {
      const key = kvMatch[1];
      let value: unknown = kvMatch[2].trim();

      // Parse value
      if (value === 'true') value = true;
      else if (value === 'false') value = false;
      else if (/^-?\d+$/.test(value as string)) value = parseInt(value as string, 10);
      else if (/^-?\d+\.\d+$/.test(value as string)) value = parseFloat(value as string);
      else if ((value as string).startsWith('"') && (value as string).endsWith('"')) {
        value = (value as string).slice(1, -1).replace(/\\"/g, '"').replace(/\\n/g, '\n');
      }

      if (currentItem) {
        currentItem[key] = value;
      } else if (currentSection) {
        (result[currentSection] as Record<string, unknown>)[key] = value;
      } else {
        result[key] = value;
      }
    }
  }

  // Push last item
  if (currentItem && currentArray) {
    currentArray.push(currentItem);
  }

  return result;
}

/**
 * Import locker schema for TOML
 */
const TomlLockerSchema = z.object({
  number: z.string(),
  location: z.string(),
  size: z.enum(['S', 'M', 'L', 'XL']),
  is_damaged: z.boolean().optional().default(false),
  notes: z.string().nullable().optional(),
});

/**
 * Import result
 */
export interface TomlImportResult {
  success: boolean;
  lockers: { imported: number; skipped: number; errors: string[] };
  locations: { imported: number; skipped: number; errors: string[] };
  settings: { imported: number; skipped: number; errors: string[] };
}

/**
 * Import data from TOML
 * @param tomlString TOML string to import
 * @param options Import options
 * @returns Import result
 */
export function importFromToml(
  tomlString: string,
  options: { skipExisting?: boolean; importSettings?: boolean } = {}
): TomlImportResult {
  const { skipExisting = true, importSettings = false } = options;

  const result: TomlImportResult = {
    success: false,
    lockers: { imported: 0, skipped: 0, errors: [] },
    locations: { imported: 0, skipped: 0, errors: [] },
    settings: { imported: 0, skipped: 0, errors: [] },
  };

  try {
    const data = parseSimpleToml(tomlString);

    // Import locations
    if (data.locations && Array.isArray(data.locations)) {
      for (const loc of data.locations) {
        try {
          if (typeof loc === 'object' && loc !== null && 'name' in loc) {
            getOrCreateLocation(loc.name as string);
            result.locations.imported++;
          }
        } catch (error) {
          result.locations.errors.push(`Location: ${error}`);
        }
      }
    }

    // Import lockers
    if (data.lockers && Array.isArray(data.lockers)) {
      for (const locker of data.lockers) {
        try {
          const parsed = TomlLockerSchema.parse(locker);

          // Check if locker already exists
          const existing = getLockerByNumber(parsed.number);

          if (existing) {
            if (skipExisting) {
              result.lockers.skipped++;
              continue;
            }
          }

          // Create locker
          createLocker(
            parsed.number,
            parsed.location,
            parsed.size as LockerSize,
            parsed.notes || undefined
          );
          result.lockers.imported++;
        } catch (error) {
          result.lockers.errors.push(`Locker: ${error}`);
        }
      }
    }

    // Import settings
    if (importSettings && data.settings && typeof data.settings === 'object') {
      const db = DatabaseConnection.getConnection();
      const settings = data.settings as Record<string, unknown>;

      for (const [key, value] of Object.entries(settings)) {
        try {
          // Skip schema_version
          if (key === 'schema_version') {
            result.settings.skipped++;
            continue;
          }

          db.prepare(`UPDATE settings SET value = ? WHERE key = ?`).run(String(value), key);
          result.settings.imported++;
        } catch (error) {
          result.settings.errors.push(`Setting "${key}": ${error}`);
        }
      }
    }

    result.success = true;

    // Log import action
    logAction('import', 'locker', null, {
      format: 'toml',
      lockers_imported: result.lockers.imported,
      locations_imported: result.locations.imported,
    });
  } catch (error) {
    result.lockers.errors.push(`Parse error: ${error}`);
  }

  return result;
}
