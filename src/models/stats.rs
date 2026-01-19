use super::{Locker, Rental, TenantType};
use serde::{Deserialize, Serialize};

/// Statistics for the dashboard.
#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    pub total_lockers: i32,
    pub occupied_lockers: i32,
    pub damaged_lockers: i32,
    pub damaged_and_occupied: i32,
    pub locations: Vec<LocationStats>,
    pub expiring_soon: Vec<RentalWithLocker>,
    pub overdue_rentals: Vec<RentalWithLocker>,
    pub total_revenue_cents: i32,
    pub outstanding_payments_cents: i32,
}

impl DashboardStats {
    /// Returns the percentage of occupied lockers.
    pub fn occupancy_percentage(&self) -> f32 {
        if self.total_lockers == 0 {
            return 0.0;
        }
        (self.occupied_lockers as f32 / self.total_lockers as f32) * 100.0
    }

    /// Returns the number of free lockers.
    pub fn free_lockers(&self) -> i32 {
        self.total_lockers - self.occupied_lockers
    }

    /// Returns the damage percentage.
    pub fn damage_percentage(&self) -> f32 {
        if self.total_lockers == 0 {
            return 0.0;
        }
        (self.damaged_lockers as f32 / self.total_lockers as f32) * 100.0
    }
}

/// Statistics for a single location.
#[derive(Debug, Clone, Default)]
pub struct LocationStats {
    pub location: String,
    pub total: i32,
    pub occupied: i32,
    pub damaged: i32,
}

impl LocationStats {
    /// Returns the percentage of occupied lockers at this location.
    pub fn occupancy_percentage(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        (self.occupied as f32 / self.total as f32) * 100.0
    }

    /// Returns the number of free lockers at this location.
    pub fn free(&self) -> i32 {
        self.total - self.occupied
    }
}

/// A rental with its associated locker.
#[derive(Debug, Clone)]
pub struct RentalWithLocker {
    pub rental: Rental,
    pub locker: Locker,
}

/// Information about a debtor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtorInfo {
    pub username: String,
    pub email: String,
    pub total_debt_cents: i32,
    pub tenant_type: TenantType,
    pub days_overdue: i64,
}

impl DebtorInfo {
    /// Returns the debt formatted as euros.
    pub fn debt_euros(&self) -> f64 {
        self.total_debt_cents as f64 / 100.0
    }

    /// Formats the debt as a string with euro sign.
    pub fn format_debt(&self) -> String {
        format!("{:.2} €", self.debt_euros())
    }
}

/// Summary of payments for a given period.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentSummary {
    pub deposits_cents: i32,
    pub extensions_cents: i32,
    pub deposit_returns_cents: i32,
    pub total_cents: i32,
    pub net_cents: i32,
    pub deposit_count: i32,
    pub extension_count: i32,
    pub deposit_return_count: i32,
}

impl PaymentSummary {
    /// Returns the total revenue formatted as euros.
    pub fn total_euros(&self) -> f64 {
        self.total_cents as f64 / 100.0
    }

    /// Returns the net revenue formatted as euros.
    pub fn net_euros(&self) -> f64 {
        self.net_cents as f64 / 100.0
    }

    /// Formats an amount in cents as a euro string.
    pub fn format_cents(cents: i32) -> String {
        format!("{:.2} €", cents as f64 / 100.0)
    }
}

/// Location entity for location management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Location {
    /// Creates a new location.
    pub fn new(name: impl Into<String>, description: Option<String>) -> Self {
        Self {
            id: 0,
            name: name.into(),
            description,
            created_at: chrono::Utc::now(),
        }
    }
}

/// Audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: i64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub details: Option<String>,
    pub username: Option<String>,
}

/// Full backup data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullBackup {
    pub version: String,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub lockers: Vec<Locker>,
    pub rentals: Vec<Rental>,
    pub payments: Vec<super::Payment>,
    pub locations: Vec<Location>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_stats() {
        let stats = DashboardStats {
            total_lockers: 100,
            occupied_lockers: 75,
            damaged_lockers: 5,
            ..Default::default()
        };

        assert_eq!(stats.occupancy_percentage(), 75.0);
        assert_eq!(stats.free_lockers(), 25);
        assert_eq!(stats.damage_percentage(), 5.0);
    }

    #[test]
    fn test_location_stats() {
        let stats = LocationStats {
            location: "Test".to_string(),
            total: 50,
            occupied: 40,
            damaged: 2,
        };

        assert_eq!(stats.occupancy_percentage(), 80.0);
        assert_eq!(stats.free(), 10);
    }

    #[test]
    fn test_debtor_info() {
        let debtor = DebtorInfo {
            username: "test".to_string(),
            email: "test@athenetz.de".to_string(),
            total_debt_cents: 2000,
            tenant_type: TenantType::Schüler,
            days_overdue: 30,
        };

        assert_eq!(debtor.debt_euros(), 20.0);
        assert_eq!(debtor.format_debt(), "20.00 €");
    }

    #[test]
    fn test_payment_summary() {
        let summary = PaymentSummary {
            deposits_cents: 10000,
            extensions_cents: 5000,
            deposit_returns_cents: -2000,
            total_cents: 13000,
            net_cents: 13000,
            deposit_count: 10,
            extension_count: 5,
            deposit_return_count: 2,
        };

        assert_eq!(summary.total_euros(), 130.0);
        assert_eq!(PaymentSummary::format_cents(1000), "10.00 €");
    }
}
