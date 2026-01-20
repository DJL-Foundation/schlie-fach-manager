// Locker Types
export interface Locker {
  id: number;
  number: string;
  location: string;
  size: LockerSize;
  is_damaged: boolean;
  notes: string | null;
  created_at: string;
}

export type LockerSize = 'S' | 'M' | 'L' | 'XL';

export interface CreateLockerInput {
  number: string;
  location: string;
  size: LockerSize;
  notes?: string;
}

export interface UpdateLockerInput {
  id: number;
  number: string;
  location: string;
  size: LockerSize;
  is_damaged: boolean;
  notes?: string;
}

// Rental Types
export interface Rental {
  id: number;
  locker_id: number;
  locker_number: string;
  renter_name: string;
  renter_email: string | null;
  renter_phone: string | null;
  start_date: string;
  end_date: string;
  deposit_paid: boolean;
  deposit_returned: boolean;
  notes: string | null;
  created_at: string;
}

export interface CreateRentalInput {
  locker_id: number;
  renter_name: string;
  renter_email?: string;
  renter_phone?: string;
  start_date: string;
  duration: number; // months
  deposit_paid: boolean;
  notes?: string;
}

// Payment Types
export interface Payment {
  id: number;
  rental_id: number;
  amount_cents: number;
  payment_date: string | null;
  payment_type: PaymentType;
  notes: string | null;
  created_at: string;
}

export type PaymentType = 'deposit' | 'yearly_fee' | 'extension' | 'refund' | 'other';

// Dashboard Types
export interface DashboardData {
  total_lockers: number;
  occupied_lockers: number;
  occupancy_percent: number;
  by_size: Record<string, SizeStats>;
  by_location: Record<string, LocationStats>;
  overdue_returns: number;
  expiring_soon: number;
  damaged_lockers: number;
  pending_payments_cents: number;
  revenue_30d_cents: number;
  occupancy_history: OccupancyHistoryPoint[];
}

export interface SizeStats {
  total: number;
  occupied: number;
}

export interface LocationStats {
  total: number;
  occupied: number;
}

export interface OccupancyHistoryPoint {
  date: string;
  percent: number;
}

export interface StatusBarData {
  db_status: 'connected' | 'error';
  total_lockers: number;
  occupied_lockers: number;
  occupancy_percent: number;
}

// Settings Types
export interface Settings {
  deposit_cents: number;
  yearly_fee_cents: number;
  billing_period: 'monthly' | 'yearly';
  currency: string;
  screensaver_timeout_seconds: number;
  app_version: string;
}

// Audit Types
export interface AuditLogEntry {
  id: number;
  timestamp: string;
  action: string;
  entity_type: string;
  entity_id: number | null;
  details: string | null;
  username: string;
}

// Export Types
export interface ExportInput {
  format: 'toml' | 'json' | 'csv';
  path: string;
}

// Location Types
export interface Location {
  id: number;
  name: string;
  created_at: string;
}

// Navigation Types
export interface NavItem {
  label: string;
  icon: React.ElementType;
  path?: string;
  children?: NavItem[];
}

export interface BreadcrumbItem {
  label: string;
  path?: string;
}

// Wizard Types
export interface WizardStep {
  id: string;
  title: string;
  description: string;
  component: React.ComponentType<WizardStepProps>;
}

export interface WizardStepProps {
  data: Record<string, unknown>;
  updateData: (data: Record<string, unknown>) => void;
  goNext: () => void;
  goPrev: () => void;
}
