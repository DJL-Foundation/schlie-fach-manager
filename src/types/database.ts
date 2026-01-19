/**
 * Database Types for Schließfach-Manager v2.1
 * @module types/database
 */

/**
 * Locker size options
 */
export type LockerSize = 'S' | 'M' | 'L' | 'XL';

/**
 * Payment types
 */
export type PaymentType = 'cash' | 'card' | 'transfer' | 'other';

/**
 * Audit log actions
 */
export type AuditAction = 'create' | 'update' | 'delete' | 'export' | 'import';

/**
 * Audit log entity types
 */
export type AuditEntityType = 'locker' | 'rental' | 'payment' | 'location' | 'settings';

/**
 * Locker entity
 */
export interface Locker {
  id: number;
  number: string;
  location: string;
  size: LockerSize;
  is_damaged: boolean;
  notes: string | null;
  created_at: string;
}

/**
 * Rental entity
 */
export interface Rental {
  id: number;
  locker_id: number;
  renter_name: string;
  renter_email: string | null;
  renter_phone: string | null;
  start_date: string;
  end_date: string | null;
  deposit_paid: boolean;
  deposit_returned: boolean;
  notes: string | null;
  created_at: string;
}

/**
 * Payment entity
 */
export interface Payment {
  id: number;
  rental_id: number;
  amount_cents: number;
  payment_date: string;
  payment_type: PaymentType;
  notes: string | null;
  created_at: string;
}

/**
 * Location entity
 */
export interface Location {
  id: number;
  name: string;
  created_at: string;
}

/**
 * Settings entity
 */
export interface Setting {
  key: string;
  value: string;
  description: string | null;
  updated_at: string;
}

/**
 * Audit log entity
 */
export interface AuditLog {
  id: number;
  timestamp: string;
  action: AuditAction;
  entity_type: AuditEntityType;
  entity_id: number | null;
  details: string | null;
  username: string;
}

/**
 * Occupancy history entity
 */
export interface OccupancyHistory {
  id: number;
  snapshot_date: string;
  total_lockers: number;
  occupied_lockers: number;
  occupancy_percent: number;
  notes: string | null;
}

/**
 * Rental with associated locker information
 */
export interface RentalWithLocker extends Rental {
  locker_number: string;
  locker_location: string;
  locker_size: LockerSize;
}
