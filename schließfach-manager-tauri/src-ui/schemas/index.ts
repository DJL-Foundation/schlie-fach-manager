import { z } from 'zod';

// Locker schemas
export const lockerSizeSchema = z.enum(['S', 'M', 'L', 'XL']);

export const lockerSchema = z.object({
  id: z.number(),
  number: z.string().min(1, 'Schließfach-Nummer ist erforderlich'),
  location: z.string().min(1, 'Standort ist erforderlich'),
  size: lockerSizeSchema,
  is_damaged: z.boolean(),
  notes: z.string().nullable(),
  created_at: z.string(),
});

export const createLockerSchema = z.object({
  number: z.string().min(1, 'Schließfach-Nummer ist erforderlich'),
  location: z.string().min(1, 'Standort ist erforderlich'),
  size: lockerSizeSchema,
  notes: z.string().optional(),
});

export const updateLockerSchema = z.object({
  id: z.number(),
  number: z.string().min(1, 'Schließfach-Nummer ist erforderlich'),
  location: z.string().min(1, 'Standort ist erforderlich'),
  size: lockerSizeSchema,
  is_damaged: z.boolean(),
  notes: z.string().optional(),
});

// Rental schemas
export const rentalSchema = z.object({
  id: z.number(),
  locker_id: z.number(),
  locker_number: z.string(),
  renter_name: z.string().min(1, 'Name ist erforderlich'),
  renter_email: z.string().email('Ungültige E-Mail-Adresse').nullable(),
  renter_phone: z.string().nullable(),
  start_date: z.string(),
  end_date: z.string(),
  deposit_paid: z.boolean(),
  deposit_returned: z.boolean(),
  notes: z.string().nullable(),
  created_at: z.string(),
});

export const createRentalSchema = z.object({
  locker_id: z.number().min(1, 'Bitte wählen Sie ein Schließfach'),
  renter_name: z.string().min(1, 'Name ist erforderlich'),
  renter_email: z.string().email('Ungültige E-Mail-Adresse').optional().or(z.literal('')),
  renter_phone: z.string().optional(),
  start_date: z.string().min(1, 'Startdatum ist erforderlich'),
  duration: z.number().min(1, 'Mindestens 1 Monat'),
  deposit_paid: z.boolean(),
  notes: z.string().optional(),
});

// Payment schemas
export const paymentTypeSchema = z.enum(['deposit', 'yearly_fee', 'extension', 'refund', 'other']);

export const paymentSchema = z.object({
  id: z.number(),
  rental_id: z.number(),
  amount_cents: z.number(),
  payment_date: z.string().nullable(),
  payment_type: paymentTypeSchema,
  notes: z.string().nullable(),
  created_at: z.string(),
});

// Settings schemas
export const settingsSchema = z.object({
  deposit_cents: z.number().min(0),
  yearly_fee_cents: z.number().min(0),
  billing_period: z.enum(['monthly', 'yearly']),
  currency: z.string(),
  screensaver_timeout_seconds: z.number().min(0),
  app_version: z.string(),
});

export const updateSettingsSchema = z.object({
  deposit_cents: z.number().min(0, 'Pfand muss positiv sein'),
  yearly_fee_cents: z.number().min(0, 'Jahresgebühr muss positiv sein'),
  billing_period: z.enum(['monthly', 'yearly']),
  currency: z.string(),
  screensaver_timeout_seconds: z.number().min(0, 'Timeout muss positiv sein'),
});

// Export types
export type LockerSize = z.infer<typeof lockerSizeSchema>;
export type Locker = z.infer<typeof lockerSchema>;
export type CreateLockerInput = z.infer<typeof createLockerSchema>;
export type UpdateLockerInput = z.infer<typeof updateLockerSchema>;
export type Rental = z.infer<typeof rentalSchema>;
export type CreateRentalInput = z.infer<typeof createRentalSchema>;
export type PaymentType = z.infer<typeof paymentTypeSchema>;
export type Payment = z.infer<typeof paymentSchema>;
export type Settings = z.infer<typeof settingsSchema>;
export type UpdateSettingsInput = z.infer<typeof updateSettingsSchema>;
