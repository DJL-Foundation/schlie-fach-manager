use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Locker entity stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Locker {
    pub id: i64,
    pub number: String,
    pub location: String,
    pub size: String,
    pub is_damaged: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Locker {
    /// Creates a new locker record with sensible defaults.
    pub fn new(
        id: i64,
        number: impl Into<String>,
        location: impl Into<String>,
        size: impl Into<String>,
    ) -> Self {
        Self {
            id,
            number: number.into(),
            location: location.into(),
            size: size.into(),
            is_damaged: false,
            notes: None,
            created_at: Utc::now(),
        }
    }

    /// Marks the locker as damaged with an optional note.
    pub fn mark_damaged(&mut self, note: Option<String>) {
        self.is_damaged = true;
        if let Some(note) = note {
            self.notes = Some(note);
        }
    }

    /// Clears the damaged flag, keeping the existing notes.
    pub fn mark_repaired(&mut self) {
        self.is_damaged = false;
    }
}

/// Rental record representing a locker being assigned to a renter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rental {
    pub id: i64,
    pub locker_id: i64,
    pub renter_name: String,
    pub renter_email: Option<String>,
    pub renter_phone: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub deposit_paid: bool,
    pub deposit_returned: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Rental {
    /// Creates a new rental with required fields and sensible defaults.
    pub fn new(
        id: i64,
        locker_id: i64,
        renter_name: impl Into<String>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            id,
            locker_id,
            renter_name: renter_name.into(),
            renter_email: None,
            renter_phone: None,
            start_date,
            end_date,
            deposit_paid: false,
            deposit_returned: false,
            notes: None,
            created_at: Utc::now(),
        }
    }

    /// Checks whether the rental is active for the given date.
    pub fn is_active_on(&self, date: NaiveDate) -> bool {
        self.start_date <= date && self.end_date >= date
    }

    /// Extends the rental end date.
    pub fn extend_to(&mut self, new_end_date: NaiveDate) {
        if new_end_date > self.end_date {
            self.end_date = new_end_date;
        }
    }

    /// Marks the rental as returned on the specified date.
    pub fn close(&mut self, return_date: NaiveDate) {
        if return_date < self.end_date {
            self.end_date = return_date;
        }
        self.deposit_returned = true;
    }
}

/// Payment record linked to a rental.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Payment {
    pub id: i64,
    pub rental_id: i64,
    pub amount_cents: i64,
    pub payment_date: NaiveDate,
    pub payment_type: PaymentType,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Payment {
    /// Creates a payment record with required fields.
    pub fn new(
        id: i64,
        rental_id: i64,
        amount_cents: i64,
        payment_date: NaiveDate,
        payment_type: PaymentType,
    ) -> Self {
        Self {
            id,
            rental_id,
            amount_cents,
            payment_date,
            payment_type,
            notes: None,
            created_at: Utc::now(),
        }
    }
}

/// Location entry for grouping lockers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl Location {
    /// Creates a location with the current timestamp.
    pub fn new(id: i64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            created_at: Utc::now(),
        }
    }
}

/// Summary data used on the dashboard for each location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationSummary {
    pub name: String,
    pub occupied: usize,
    pub total: usize,
}

/// Billing period option.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BillingPeriod {
    Monthly,
    Yearly,
}

impl BillingPeriod {
    /// Returns the string representation used in settings and exports.
    pub fn as_str(&self) -> &'static str {
        match self {
            BillingPeriod::Monthly => "monthly",
            BillingPeriod::Yearly => "yearly",
        }
    }

    /// Parses a billing period from a stored string value.
    pub fn from_str(value: &str) -> Self {
        match value {
            "monthly" => BillingPeriod::Monthly,
            _ => BillingPeriod::Yearly,
        }
    }
}

/// Payment type value stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentType {
    Deposit,
    YearlyFee,
    Other(String),
}

impl PaymentType {
    /// Returns the database representation of the payment type.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentType::Deposit => "deposit",
            PaymentType::YearlyFee => "yearly_fee",
            PaymentType::Other(value) => value.as_str(),
        }
    }

    /// Parses a payment type from a stored string value.
    pub fn from_str(value: &str) -> Self {
        match value {
            "deposit" => PaymentType::Deposit,
            "yearly_fee" => PaymentType::YearlyFee,
            other => PaymentType::Other(other.to_string()),
        }
    }
}

/// Audit log entry persisted to the database.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEntry {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub details: String,
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rental_active_range_checks_dates() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).expect("start date");
        let end = NaiveDate::from_ymd_opt(2025, 1, 31).expect("end date");
        let rental = Rental::new(1, 2, "Max", start, end);

        assert!(rental.is_active_on(start));
        assert!(rental.is_active_on(end));
        assert!(!rental.is_active_on(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()));
        assert!(!rental.is_active_on(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap()));
    }

    #[test]
    fn billing_period_roundtrip() {
        assert_eq!(BillingPeriod::from_str("monthly"), BillingPeriod::Monthly);
        assert_eq!(BillingPeriod::from_str("yearly"), BillingPeriod::Yearly);
        assert_eq!(BillingPeriod::Yearly.as_str(), "yearly");
    }

    #[test]
    fn payment_type_roundtrip() {
        assert_eq!(PaymentType::from_str("deposit"), PaymentType::Deposit);
        assert_eq!(PaymentType::from_str("yearly_fee"), PaymentType::YearlyFee);
        assert_eq!(
            PaymentType::from_str("custom"),
            PaymentType::Other("custom".into())
        );
        assert_eq!(PaymentType::Deposit.as_str(), "deposit");
    }

    #[test]
    fn locker_damage_updates_fields() {
        let mut locker = Locker::new(1, "A-01", "Hauptgebäude", "Klein");
        assert!(!locker.is_damaged);

        locker.mark_damaged(Some("Schloss defekt".into()));
        assert!(locker.is_damaged);
        assert_eq!(locker.notes.as_deref(), Some("Schloss defekt"));

        locker.mark_repaired();
        assert!(!locker.is_damaged);
    }
}
