//! Domain models for the Schließfach-Manager system.
//!
//! This module contains all the core domain types used throughout the application:
//!
//! - [`Locker`] - Physical storage lockers
//! - [`Rental`] - Rental contracts between tenants and lockers
//! - [`Payment`] - Financial transactions
//! - [`TenantType`] - Types of tenants (Schüler, Lehrer)
//! - [`PaymentType`] - Types of payments (Deposit, Extension, etc.)
//!
//! # Domain Overview
//!
//! ```text
//! ┌──────────┐    ┌──────────┐    ┌──────────┐
//! │  Locker  │◄───│  Rental  │───►│  Tenant  │
//! └──────────┘    └────┬─────┘    └──────────┘
//!                      │
//!                      ▼
//!                ┌──────────┐
//!                │ Payment  │
//!                └──────────┘
//! ```
//!
//! # Example
//!
//! ```rust
//! use schliessfach_manager::models::{Locker, Rental, TenantType, Payment, PaymentType};
//! use chrono::{Duration, Utc};
//!
//! // Create a locker
//! let locker = Locker::new("A-001", "Hauptgebäude", 150);
//!
//! // Create a rental for a student
//! let today = Utc::now().date_naive();
//! let rental = Rental::new(
//!     locker.id,
//!     "max.mustermann",
//!     TenantType::Schüler,
//!     today,
//!     today + Duration::days(365),
//! );
//!
//! // Create a deposit payment (rental_id, amount_cents, type, date)
//! let payment = Payment::new(rental.id, 1000, PaymentType::Deposit, today);
//! ```

pub mod locker;
pub mod payment;
pub mod rental;
pub mod stats;

pub use locker::Locker;
pub use payment::{Payment, PaymentType};
pub use rental::{Rental, TenantType};
pub use stats::{DashboardStats, DebtorInfo, LocationStats, PaymentSummary, RentalWithLocker};
