import { invoke } from '@tauri-apps/api/core';
import type {
  DashboardData,
  StatusBarData,
  Locker,
  CreateLockerInput,
  UpdateLockerInput,
  Rental,
  CreateRentalInput,
  Payment,
  Settings,
  AuditLogEntry,
  ExportInput,
} from '@/types';

// Dashboard Commands
export async function getDashboardData(): Promise<DashboardData> {
  return invoke<DashboardData>('get_dashboard_data');
}

export async function getStatusBarData(): Promise<StatusBarData> {
  return invoke<StatusBarData>('get_status_bar_data');
}

// Locker Commands
export async function getAllLockers(): Promise<Locker[]> {
  return invoke<Locker[]>('get_all_lockers');
}

export async function getAvailableLockers(): Promise<Locker[]> {
  return invoke<Locker[]>('get_available_lockers');
}

export async function getLocker(id: number): Promise<Locker> {
  return invoke<Locker>('get_locker', { id });
}

export async function createLocker(input: CreateLockerInput): Promise<number> {
  return invoke<number>('create_locker', { locker: input });
}

export async function updateLocker(input: UpdateLockerInput): Promise<void> {
  return invoke<void>('update_locker', { locker: input });
}

export async function deleteLocker(id: number): Promise<void> {
  return invoke<void>('delete_locker', { id });
}

// Rental Commands
export async function getAllRentals(): Promise<Rental[]> {
  return invoke<Rental[]>('get_all_rentals');
}

export async function getActiveRentals(): Promise<Rental[]> {
  return invoke<Rental[]>('get_active_rentals');
}

export async function getOverdueRentals(): Promise<Rental[]> {
  return invoke<Rental[]>('get_overdue_rentals');
}

export async function getRental(id: number): Promise<Rental> {
  return invoke<Rental>('get_rental', { id });
}

export async function createRental(input: CreateRentalInput): Promise<number> {
  return invoke<number>('create_rental', { rental: input });
}

export async function extendRental(id: number, months: number): Promise<void> {
  return invoke<void>('extend_rental', { id, months });
}

export async function returnRental(id: number): Promise<void> {
  return invoke<void>('return_rental', { id });
}

// Payment Commands
export async function getAllPayments(): Promise<Payment[]> {
  return invoke<Payment[]>('get_all_payments');
}

export async function getPaymentsByRental(rentalId: number): Promise<Payment[]> {
  return invoke<Payment[]>('get_payments_by_rental', { rentalId });
}

export async function createPayment(payment: Omit<Payment, 'id' | 'created_at'>): Promise<number> {
  return invoke<number>('create_payment', { payment });
}

// Settings Commands
export async function getSettings(): Promise<Settings> {
  return invoke<Settings>('get_settings');
}

export async function updateSetting(key: string, value: string): Promise<void> {
  return invoke<void>('update_setting', { key, value });
}

// Audit Commands
export async function getAuditLog(limit?: number): Promise<AuditLogEntry[]> {
  return invoke<AuditLogEntry[]>('get_audit_log', { limit: limit ?? 100 });
}

// Export/Import Commands
export async function exportData(input: ExportInput): Promise<void> {
  return invoke<void>('export_data', { input });
}

export async function importData(path: string): Promise<void> {
  return invoke<void>('import_data', { path });
}

// Location Commands
export async function getAllLocations(): Promise<string[]> {
  return invoke<string[]>('get_all_locations');
}

export async function createLocation(name: string): Promise<number> {
  return invoke<number>('create_location', { name });
}

export async function deleteLocation(id: number): Promise<void> {
  return invoke<void>('delete_location', { id });
}

// Database Commands
export async function runMigrations(): Promise<void> {
  return invoke<void>('run_database_migrations');
}
