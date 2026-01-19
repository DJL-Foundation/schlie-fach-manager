use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of payment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentType {
    Deposit,       // 10€ Pfand
    Extension,     // 10€ pro Jahr
    DepositReturn, // -10€ (Pfand zurück)
}

impl PaymentType {
    /// Converts to a string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentType::Deposit => "Deposit",
            PaymentType::Extension => "Extension",
            PaymentType::DepositReturn => "DepositReturn",
        }
    }

    /// Parses from a string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Deposit" => Some(PaymentType::Deposit),
            "Extension" => Some(PaymentType::Extension),
            "DepositReturn" => Some(PaymentType::DepositReturn),
            _ => None,
        }
    }

    /// Returns the German display name for this payment type.
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

/// Domain model for a payment record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub rental_id: i64,
    pub amount_cents: i32,
    pub payment_type: PaymentType,
    pub payment_date: NaiveDate,
    pub notes: Option<String>,
}

impl Payment {
    /// Creates a new payment record.
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
    pub fn amount_euros(&self) -> f64 {
        self.amount_cents as f64 / 100.0
    }

    /// Formats the amount as a string with euro sign.
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
