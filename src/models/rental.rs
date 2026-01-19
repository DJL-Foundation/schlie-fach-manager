//! Rental domain model and related types.
//!
//! This module contains the [`Rental`] struct representing a rental agreement
//! between a tenant and a locker, and the [`TenantType`] enum for categorizing tenants.
//!
//! # Rental Lifecycle
//!
//! 1. **Creation**: Tenant rents a locker, pays deposit
//! 2. **Active**: Rental is within its valid period
//! 3. **Expiring**: Rental end date is approaching (< 30 days)
//! 4. **Overdue**: Rental has passed its end date, debt accumulates
//! 5. **Returned**: Locker is returned, deposit handled based on debt
//!
//! # Debt Calculation
//!
//! Debt accumulates at 10€ per year (or partial year) that the rental is overdue:
//! - 1 day overdue = 10€ debt
//! - 366 days overdue = 20€ debt
//!
//! # Example
//!
//! ```rust
//! use schliessfach_manager::models::{Rental, TenantType};
//! use chrono::{Duration, Utc};
//!
//! let today = Utc::now().date_naive();
//! let rental = Rental::new(
//!     1,                              // locker_id
//!     "max.mustermann",               // username
//!     TenantType::Schüler,            // tenant type
//!     today,                          // start date
//!     today + Duration::days(365),    // end date (1 year)
//! );
//!
//! assert_eq!(rental.days_until_expiration(), 365);
//! assert!(!rental.is_overdue());
//! ```

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Average days per year (accounting for leap years).
const DAYS_PER_YEAR: f64 = 365.25;

/// Yearly extension fee in cents (10€).
const YEARLY_FEE_CENTS: i32 = 1000;

/// Type of tenant renting a locker.
///
/// In the German school system, lockers are rented by:
/// - **Schüler** (Students): Typically pay lower fees
/// - **Lehrer** (Teachers): Staff members
///
/// # Example
///
/// ```rust
/// use schliessfach_manager::models::TenantType;
///
/// let student = TenantType::Schüler;
/// let teacher = TenantType::Lehrer;
///
/// assert_eq!(student.as_str(), "Schüler");
/// assert_eq!(teacher.as_str(), "Lehrer");
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TenantType {
    /// Student tenant (German: Schüler).
    Schüler,
    /// Teacher tenant (German: Lehrer).
    Lehrer,
}

impl TenantType {
    /// Converts to a string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            TenantType::Schüler => "Schüler",
            TenantType::Lehrer => "Lehrer",
        }
    }

    /// Parses from a string representation.
    ///
    /// # Returns
    ///
    /// * `Some(TenantType)` if the string matches "Schüler" or "Lehrer"
    /// * `None` if the string doesn't match
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Schüler" => Some(TenantType::Schüler),
            "Lehrer" => Some(TenantType::Lehrer),
            _ => None,
        }
    }
}

impl std::fmt::Display for TenantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A rental agreement between a tenant and a locker.
///
/// Represents the contract for a tenant to use a specific locker for a defined period.
/// Tracks deposit payments, return status, and can calculate accumulated debt.
///
/// # Fields
///
/// * `id` - Database identifier
/// * `locker_id` - Foreign key to the rented locker
/// * `tenant_username` - Username of the tenant (e.g., "max.mustermann")
/// * `tenant_type` - Type of tenant (Schüler or Lehrer)
/// * `rental_start_date` - When the rental period begins
/// * `rental_end_date` - When the rental period ends
/// * `deposit_paid` - Whether the 10€ deposit has been paid
/// * `deposit_returned` - Whether the deposit has been returned
/// * `created_at` - When the rental record was created
/// * `returned_at` - When the locker was returned (if applicable)
///
/// # Example
///
/// ```rust
/// use schliessfach_manager::models::{Rental, TenantType};
/// use chrono::{Duration, Utc};
///
/// let today = Utc::now().date_naive();
/// let mut rental = Rental::new(
///     1, "anna.schmidt", TenantType::Lehrer,
///     today, today + Duration::days(365)
/// );
///
/// // Mark deposit as paid
/// rental.deposit_paid = true;
///
/// // Get email address
/// assert_eq!(rental.email(), "anna.schmidt@athenetz.de");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rental {
    /// Database identifier.
    pub id: i64,
    /// Foreign key to the rented locker.
    pub locker_id: i64,
    /// Username of the tenant (e.g., "max.mustermann").
    pub tenant_username: String,
    /// Type of tenant (Schüler or Lehrer).
    pub tenant_type: TenantType,
    /// When the rental period begins.
    pub rental_start_date: NaiveDate,
    /// When the rental period ends.
    pub rental_end_date: NaiveDate,
    /// Whether the 10€ deposit has been paid.
    pub deposit_paid: bool,
    /// Whether the deposit has been returned.
    pub deposit_returned: bool,
    /// When the rental record was created.
    pub created_at: DateTime<Utc>,
    /// When the locker was returned (if applicable).
    pub returned_at: Option<DateTime<Utc>>,
}

impl Rental {
    /// Creates a new rental.
    pub fn new(
        locker_id: i64,
        tenant_username: impl Into<String>,
        tenant_type: TenantType,
        rental_start_date: NaiveDate,
        rental_end_date: NaiveDate,
    ) -> Self {
        Self {
            id: 0,
            locker_id,
            tenant_username: tenant_username.into(),
            tenant_type,
            rental_start_date,
            rental_end_date,
            deposit_paid: false,
            deposit_returned: false,
            created_at: Utc::now(),
            returned_at: None,
        }
    }

    /// Checks if the rental is active (not returned).
    pub fn is_active(&self) -> bool {
        self.returned_at.is_none()
    }

    /// Checks if the rental is overdue.
    pub fn is_overdue(&self) -> bool {
        self.is_active() && Utc::now().date_naive() > self.rental_end_date
    }

    /// Calculates the number of days until expiration (negative if overdue).
    pub fn days_until_expiration(&self) -> i64 {
        let today = Utc::now().date_naive();
        (self.rental_end_date - today).num_days()
    }

    /// Calculates the debt for this rental in cents.
    pub fn calculate_debt(&self) -> i32 {
        let today = Utc::now().date_naive();
        if today <= self.rental_end_date {
            return 0;
        }

        let days_overdue = (today - self.rental_end_date).num_days();
        let years_overdue = (days_overdue as f64 / DAYS_PER_YEAR).ceil() as i32;
        years_overdue * YEARLY_FEE_CENTS
    }

    /// Returns the full email address for this tenant.
    pub fn email(&self) -> String {
        format!("{}@athenetz.de", self.tenant_username)
    }

    /// Checks if the rental matches the given query string.
    pub fn matches_query(&self, query: &str) -> bool {
        let needle = query.to_lowercase();
        self.tenant_username.to_lowercase().contains(&needle)
            || self.id.to_string().contains(&needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_rental_creation() {
        let today = Utc::now().date_naive();
        let rental = Rental::new(
            1,
            "max.mustermann",
            TenantType::Schüler,
            today,
            today + Duration::days(365),
        );

        assert_eq!(rental.locker_id, 1);
        assert_eq!(rental.tenant_username, "max.mustermann");
        assert_eq!(rental.tenant_type, TenantType::Schüler);
        assert!(rental.is_active());
        assert!(!rental.is_overdue());
    }

    #[test]
    fn test_rental_debt_calculation() {
        let today = Utc::now().date_naive();

        // Not overdue
        let rental = Rental::new(
            1,
            "user",
            TenantType::Schüler,
            today - Duration::days(100),
            today + Duration::days(100),
        );
        assert_eq!(rental.calculate_debt(), 0);

        // Overdue by 40 days (less than a year)
        let rental = Rental::new(
            1,
            "user",
            TenantType::Schüler,
            today - Duration::days(400),
            today - Duration::days(40),
        );
        assert_eq!(rental.calculate_debt(), 1000); // 10€
    }

    #[test]
    fn test_tenant_type() {
        assert_eq!(TenantType::Schüler.as_str(), "Schüler");
        assert_eq!(TenantType::Lehrer.as_str(), "Lehrer");
        assert_eq!(TenantType::from_str("Schüler"), Some(TenantType::Schüler));
        assert_eq!(TenantType::from_str("Invalid"), None);
    }

    #[test]
    fn test_email() {
        let today = Utc::now().date_naive();
        let rental = Rental::new(1, "max.mustermann", TenantType::Schüler, today, today);
        assert_eq!(rental.email(), "max.mustermann@athenetz.de");
    }
}
