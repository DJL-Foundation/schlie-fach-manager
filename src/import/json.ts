/**
 * JSON Import Module
 * @module import/json
 */

import { z } from 'zod';
import { createLocker, getLockerByNumber } from '../db/lockers';
import { getOrCreateLocation } from '../db/locations';
import { logAction } from '../db/audit';
import { DatabaseConnection } from '../db/connection';
import type { LockerSize } from '../types/database';

/**
 * Import locker schema
 */
const ImportLockerSchema = z.object({
  number: z.string(),
  location: z.string(),
  size: z.enum(['S', 'M', 'L', 'XL']),
  is_damaged: z.boolean().optional(),
  notes: z.string().nullable().optional(),
});

/**
 * Import location schema
 */
const ImportLocationSchema = z.object({
  name: z.string(),
});

/**
 * Import settings schema
 */
const ImportSettingSchema = z.object({
  key: z.string(),
  value: z.string(),
});

/**
 * Full import schema
 */
const ImportDataSchema = z.object({
  version: z.string().optional(),
  lockers: z.array(ImportLockerSchema).optional(),
  locations: z.array(ImportLocationSchema).optional(),
  settings: z.array(ImportSettingSchema).optional(),
});

/**
 * Import result
 */
export interface ImportResult {
  success: boolean;
  lockers: { imported: number; skipped: number; errors: string[] };
  locations: { imported: number; skipped: number; errors: string[] };
  settings: { imported: number; skipped: number; errors: string[] };
}

/**
 * Import data from JSON
 * @param jsonString JSON string to import
 * @param options Import options
 * @returns Import result
 */
export function importFromJson(
  jsonString: string,
  options: { skipExisting?: boolean; importSettings?: boolean } = {}
): ImportResult {
  const { skipExisting = true, importSettings = false } = options;

  const result: ImportResult = {
    success: false,
    lockers: { imported: 0, skipped: 0, errors: [] },
    locations: { imported: 0, skipped: 0, errors: [] },
    settings: { imported: 0, skipped: 0, errors: [] },
  };

  try {
    const rawData = JSON.parse(jsonString);
    const data = ImportDataSchema.parse(rawData);

    // Import locations first
    if (data.locations) {
      for (const loc of data.locations) {
        try {
          getOrCreateLocation(loc.name);
          result.locations.imported++;
        } catch (error) {
          result.locations.errors.push(`Location "${loc.name}": ${error}`);
        }
      }
    }

    // Import lockers
    if (data.lockers) {
      for (const locker of data.lockers) {
        try {
          // Check if locker already exists
          const existing = getLockerByNumber(locker.number);

          if (existing) {
            if (skipExisting) {
              result.lockers.skipped++;
              continue;
            }
          }

          // Create locker
          createLocker(
            locker.number,
            locker.location,
            locker.size as LockerSize,
            locker.notes || undefined
          );
          result.lockers.imported++;
        } catch (error) {
          result.lockers.errors.push(`Locker "${locker.number}": ${error}`);
        }
      }
    }

    // Import settings
    if (importSettings && data.settings) {
      const db = DatabaseConnection.getConnection();

      for (const setting of data.settings) {
        try {
          // Skip schema_version
          if (setting.key === 'schema_version') {
            result.settings.skipped++;
            continue;
          }

          db.prepare(`UPDATE settings SET value = ? WHERE key = ?`).run(setting.value, setting.key);
          result.settings.imported++;
        } catch (error) {
          result.settings.errors.push(`Setting "${setting.key}": ${error}`);
        }
      }
    }

    result.success = true;

    // Log import action
    logAction('import', 'locker', null, {
      format: 'json',
      lockers_imported: result.lockers.imported,
      locations_imported: result.locations.imported,
    });
  } catch (error) {
    if (error instanceof z.ZodError) {
      result.lockers.errors.push(`Validation error: ${error.message}`);
    } else if (error instanceof SyntaxError) {
      result.lockers.errors.push('Invalid JSON format');
    } else {
      result.lockers.errors.push(`Unknown error: ${error}`);
    }
  }

  return result;
}

/**
 * Validate JSON import data without importing
 * @param jsonString JSON string to validate
 * @returns Validation result
 */
export function validateJsonImport(jsonString: string): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  try {
    const rawData = JSON.parse(jsonString);
    ImportDataSchema.parse(rawData);
    return { valid: true, errors: [] };
  } catch (error) {
    if (error instanceof z.ZodError) {
      errors.push(...error.errors.map((e) => `${e.path.join('.')}: ${e.message}`));
    } else if (error instanceof SyntaxError) {
      errors.push('Invalid JSON syntax');
    } else {
      errors.push(`Unknown error: ${error}`);
    }
    return { valid: false, errors };
  }
}
