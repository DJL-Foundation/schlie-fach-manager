//! Payment domain model and related types.
//!
//! This module contains the [`Payment`] struct for recording financial transactions
//! and the [`PaymentType`] enum for categorizing payments.
//!
//! # Payment Types
//!
//! | Type | German | Amount | Description |
//! |------|--------|--------|-------------|
//! | Deposit | Pfand | +10€ | Initial deposit when renting |
//! | Extension | Verlängerung | +10€ | Yearly extension fee |
//! | DepositReturn | Pfand-Rückgabe | -10€ | Deposit refund on return |
//!
//! # Example
//!
//! ```rust
//! use schliessfach_manager::models::{Payment, PaymentType};
//! use chrono::Utc;
//!
//! // Create a deposit payment (10€)
//! let deposit = Payment::new(1, 1000, PaymentType::Deposit, Utc::now().date_naive());
//! assert_eq!(deposit.format_amount(), "10.00 €");
//!
//! // Create a deposit return (-10€)
//! let refund = Payment::new(1, -1000, PaymentType::DepositReturn, Utc::now().date_naive());
//! assert_eq!(refund.amount_euros(), -10.0);
//! ```

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of payment transaction.
///
/// Categorizes financial transactions in the locker management system:
///
/// - **Deposit**: Initial security deposit (10€)
/// - **Extension**: Yearly extension fee (10€/year)
/// - **DepositReturn**: Refund of deposit on locker return (-10€)
///
/// # Example
///
/// ```rust
/// use schliessfach_manager::models::PaymentType;
///
/// let payment_type = PaymentType::Deposit;
/// assert_eq!(payment_type.display_name(), "Pfand");
/// assert_eq!(format!("{}", payment_type), "Pfand");
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentType {
    /// Initial security deposit (German: Pfand). Typically 10€.
    Deposit,
    /// Yearly extension fee (German: Verlängerung). 10€ per year.
    Extension,
    /// Deposit refund on locker return (German: Pfand-Rückgabe). -10€.
    DepositReturn,
}

impl PaymentType {
    /// Converts to a string representation for serialization.
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentType::Deposit => "Deposit",
            PaymentType::Extension => "Extension",
            PaymentType::DepositReturn => "DepositReturn",
        }
    }

    /// Parses from a string representation.
    ///
    /// # Returns
    ///
    /// * `Some(PaymentType)` if the string matches
    /// * `None` if the string doesn't match any payment type
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Deposit" => Some(PaymentType::Deposit),
            "Extension" => Some(PaymentType::Extension),
            "DepositReturn" => Some(PaymentType::DepositReturn),
            _ => None,
        }
    }

    /// Returns the German display name for this payment type.
    ///
    /// Used for user-facing display in the German-language UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            PaymentType::Deposit => "Pfand",
            PaymentType::Extension => "Verlängerung",
            PaymentType::DepositReturn => "Pfand-Rückgabe",
        }
    }
}

impl std::fmt::Display for PaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// A financial transaction record.
///
/// Payments track all monetary transactions related to locker rentals,
/// including deposits, extensions, and refunds. Amounts are stored in cents
/// to avoid floating-point precision issues.
///
/// # Fields
///
/// * `id` - Database identifier
/// * `rental_id` - Foreign key to the associated rental
/// * `amount_cents` - Amount in cents (positive for income, negative for refunds)
/// * `payment_type` - Type of payment
/// * `payment_date` - Date the payment was made
/// * `notes` - Optional notes about the payment
///
/// # Example
///
/// ```rust
/// use schliessfach_manager::models::{Payment, PaymentType};
/// use chrono::Utc;
///
/// let payment = Payment::new(
///     1,                      // rental_id
///     1000,                   // amount_cents (10€)
///     PaymentType::Deposit,   // payment type
///     Utc::now().date_naive() // payment date
/// );
///
/// assert_eq!(payment.amount_euros(), 10.0);
/// assert_eq!(payment.format_amount(), "10.00 €");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// Database identifier.
    pub id: i64,
    /// Foreign key to the associated rental.
    pub rental_id: i64,
    /// Amount in cents (positive for income, negative for refunds).
    pub amount_cents: i32,
    /// Type of payment.
    pub payment_type: PaymentType,
    /// Date the payment was made.
    pub payment_date: NaiveDate,
    /// Optional notes about the payment.
    pub notes: Option<String>,
}

impl Payment {
    /// Creates a new payment record.
    ///
    /// # Arguments
    ///
    /// * `rental_id` - Foreign key to the associated rental
    /// * `amount_cents` - Amount in cents
    /// * `payment_type` - Type of payment
    /// * `payment_date` - Date the payment was made
    ///
    /// # Example
    ///
    /// ```rust
    /// use schliessfach_manager::models::{Payment, PaymentType};
    /// use chrono::Utc;
    ///
    /// let payment = Payment::new(1, 1000, PaymentType::Deposit, Utc::now().date_naive());
    /// ```
    pub fn new(
        rental_id: i64,
        amount_cents: i32,
        payment_type: PaymentType,
        payment_date: NaiveDate,
    ) -> Self {
        Self {
            id: 0,
            rental_id,
            amount_cents,
            payment_type,
            payment_date,
            notes: None,
        }
    }

    /// Returns the amount formatted as euros.
    ///
    /// Converts from cents to euros by dividing by 100.
    pub fn amount_euros(&self) -> f64 {
        self.amount_cents as f64 / 100.0
    }

    /// Formats the amount as a string with euro sign.
    ///
    /// # Example
    ///
    /// ```rust
    /// use schliessfach_manager::models::{Payment, PaymentType};
    /// use chrono::Utc;
    ///
    /// let payment = Payment::new(1, 1000, PaymentType::Deposit, Utc::now().date_naive());
    /// assert_eq!(payment.format_amount(), "10.00 €");
    /// ```
    pub fn format_amount(&self) -> String {
        format!("{:.2} €", self.amount_euros())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_payment_creation() {
        let payment = Payment::new(1, 1000, PaymentType::Deposit, Utc::now().date_naive());
        assert_eq!(payment.rental_id, 1);
        assert_eq!(payment.amount_cents, 1000);
        assert_eq!(payment.payment_type, PaymentType::Deposit);
    }

    #[test]
    fn test_amount_formatting() {
        let payment = Payment::new(1, 1000, PaymentType::Deposit, Utc::now().date_naive());
        assert_eq!(payment.amount_euros(), 10.0);
        assert_eq!(payment.format_amount(), "10.00 €");
    }

    #[test]
    fn test_payment_type() {
        assert_eq!(PaymentType::Deposit.as_str(), "Deposit");
        assert_eq!(PaymentType::Deposit.display_name(), "Pfand");
        assert_eq!(PaymentType::from_str("Deposit"), Some(PaymentType::Deposit));
        assert_eq!(PaymentType::from_str("Invalid"), None);
    }
}
