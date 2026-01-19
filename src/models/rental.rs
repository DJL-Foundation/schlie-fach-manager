use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Type of tenant renting a locker.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TenantType {
    Schüler,
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

/// Domain model for a rental agreement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rental {
    pub id: i64,
    pub locker_id: i64,
    pub tenant_username: String,
    pub tenant_type: TenantType,
    pub rental_start_date: NaiveDate,
    pub rental_end_date: NaiveDate,
    pub deposit_paid: bool,
    pub deposit_returned: bool,
    pub created_at: DateTime<Utc>,
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
        let years_overdue = (days_overdue as f64 / 365.25).ceil() as i32;
        years_overdue * 1000 // 10€ per year in cents
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
