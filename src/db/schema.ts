/**
 * Database Schema Definitions with Zod Validation
 * @module db/schema
 */

import { z } from 'zod';

/**
 * Locker size enum values
 */
export const LockerSizeEnum = z.enum(['S', 'M', 'L', 'XL']);

/**
 * Payment type enum values
 */
export const PaymentTypeEnum = z.enum(['cash', 'card', 'transfer', 'other']);

/**
 * Audit action enum values
 */
export const AuditActionEnum = z.enum(['create', 'update', 'delete', 'export', 'import']);

/**
 * Audit entity type enum values
 */
export const AuditEntityTypeEnum = z.enum(['locker', 'rental', 'payment', 'location', 'settings']);

/**
 * Locker schema for runtime validation
 */
export const LockerSchema = z.object({
  id: z.number().int().positive(),
  number: z.string().min(1),
  location: z.string().min(1),
  size: LockerSizeEnum,
  is_damaged: z.union([z.boolean(), z.number()]).transform((val) => Boolean(val)),
  notes: z.string().nullable(),
  created_at: z.string(),
});

/**
 * Rental schema for runtime validation
 */
export const RentalSchema = z.object({
  id: z.number().int().positive(),
  locker_id: z.number().int().positive(),
  renter_name: z.string().min(1),
  renter_email: z.string().email().nullable().or(z.literal('')).transform((val) => val || null),
  renter_phone: z.string().nullable(),
  start_date: z.string(),
  end_date: z.string().nullable(),
  deposit_paid: z.union([z.boolean(), z.number()]).transform((val) => Boolean(val)),
  deposit_returned: z.union([z.boolean(), z.number()]).transform((val) => Boolean(val)),
  notes: z.string().nullable(),
  created_at: z.string(),
});

/**
 * Payment schema for runtime validation
 */
export const PaymentSchema = z.object({
  id: z.number().int().positive(),
  rental_id: z.number().int().positive(),
  amount_cents: z.number().int(),
  payment_date: z.string(),
  payment_type: PaymentTypeEnum,
  notes: z.string().nullable(),
  created_at: z.string(),
});

/**
 * Location schema for runtime validation
 */
export const LocationSchema = z.object({
  id: z.number().int().positive(),
  name: z.string().min(1),
  created_at: z.string(),
});

/**
 * Settings schema for runtime validation
 */
export const SettingSchema = z.object({
  key: z.string().min(1),
  value: z.string(),
  description: z.string().nullable(),
  updated_at: z.string(),
});

/**
 * Audit log schema for runtime validation
 */
export const AuditLogSchema = z.object({
  id: z.number().int().positive(),
  timestamp: z.string(),
  action: AuditActionEnum,
  entity_type: AuditEntityTypeEnum,
  entity_id: z.number().int().nullable(),
  details: z.string().nullable(),
  username: z.string(),
});

/**
 * Occupancy history schema for runtime validation
 */
export const OccupancyHistorySchema = z.object({
  id: z.number().int().positive(),
  snapshot_date: z.string(),
  total_lockers: z.number().int(),
  occupied_lockers: z.number().int(),
  occupancy_percent: z.number(),
  notes: z.string().nullable(),
});

/**
 * Count result schema for database queries
 */
export const CountResultSchema = z.object({
  count: z.number().int(),
});

/**
 * Total result schema for database queries
 */
export const TotalResultSchema = z.object({
  total: z.number(),
});

/**
 * Size statistics schema
 */
export const SizeStatsSchema = z.object({
  size: LockerSizeEnum,
  total: z.number().int(),
  occupied: z.number().int(),
});

/**
 * Location statistics schema
 */
export const LocationStatsSchema = z.object({
  location: z.string(),
  total: z.number().int(),
  occupied: z.number().int(),
});

/**
 * Occupancy history entry schema
 */
export const OccupancyHistoryEntrySchema = z.object({
  date: z.string(),
  percent: z.number(),
});

// Type exports from schemas
export type LockerRow = z.infer<typeof LockerSchema>;
export type RentalRow = z.infer<typeof RentalSchema>;
export type PaymentRow = z.infer<typeof PaymentSchema>;
export type LocationRow = z.infer<typeof LocationSchema>;
export type SettingRow = z.infer<typeof SettingSchema>;
export type AuditLogRow = z.infer<typeof AuditLogSchema>;
export type OccupancyHistoryRow = z.infer<typeof OccupancyHistorySchema>;
