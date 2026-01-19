use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Operational state of a locker.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LockerStatus {
    Free,
    Occupied,
    Maintenance,
}

impl LockerStatus {
    /// Returns `true` when the locker can be immediately assigned.
    pub fn is_free(self) -> bool {
        matches!(self, LockerStatus::Free)
    }

    /// Returns `true` when the locker currently holds an occupant.
    pub fn is_occupied(self) -> bool {
        matches!(self, LockerStatus::Occupied)
    }

    /// Returns `true` when the locker is flagged for maintenance.
    pub fn is_maintenance(self) -> bool {
        matches!(self, LockerStatus::Maintenance)
    }

    /// Maps the status to a string we can store inside SQLite.
    pub fn to_db_value(self) -> &'static str {
        match self {
            LockerStatus::Free => "Free",
            LockerStatus::Occupied => "Occupied",
            LockerStatus::Maintenance => "Maintenance",
        }
    }

    /// Recreates a status from the string persisted in SQLite.
    pub fn from_db_value(value: &str) -> Option<Self> {
        match value {
            "Free" => Some(LockerStatus::Free),
            "Occupied" => Some(LockerStatus::Occupied),
            "Maintenance" => Some(LockerStatus::Maintenance),
            _ => None,
        }
    }

    /// German display name
    pub fn display_name(&self) -> &'static str {
        match self {
            LockerStatus::Free => "Frei",
            LockerStatus::Occupied => "Belegt",
            LockerStatus::Maintenance => "Wartung",
        }
    }
}

/// Height position of the locker (for "Kein Dachboden für 5. Klässler" rule)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LockerHeight {
    Top,
    Middle,
    Bottom,
}

impl LockerHeight {
    pub fn to_db_value(self) -> &'static str {
        match self {
            LockerHeight::Top => "Top",
            LockerHeight::Middle => "Middle",
            LockerHeight::Bottom => "Bottom",
        }
    }

    pub fn from_db_value(value: &str) -> Option<Self> {
        match value {
            "Top" => Some(LockerHeight::Top),
            "Middle" => Some(LockerHeight::Middle),
            "Bottom" => Some(LockerHeight::Bottom),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            LockerHeight::Top => "Oben",
            LockerHeight::Middle => "Mitte",
            LockerHeight::Bottom => "Unten",
        }
    }

    pub fn all() -> &'static [LockerHeight] {
        &[LockerHeight::Top, LockerHeight::Middle, LockerHeight::Bottom]
    }
}

/// Type of tenant
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TenantType {
    Student,
    Teacher,
}

impl TenantType {
    pub fn to_db_value(self) -> &'static str {
        match self {
            TenantType::Student => "Student",
            TenantType::Teacher => "Teacher",
        }
    }

    pub fn from_db_value(value: &str) -> Option<Self> {
        match value {
            "Student" => Some(TenantType::Student),
            "Teacher" => Some(TenantType::Teacher),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TenantType::Student => "Schüler",
            TenantType::Teacher => "Lehrer",
        }
    }

    pub fn all() -> &'static [TenantType] {
        &[TenantType::Student, TenantType::Teacher]
    }
}

/// A physical location where lockers are located.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Location {
    pub id: i64,
    pub name: String,
}

impl Location {
    pub fn new(id: i64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}

/// Domain aggregate for a single locker entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Locker {
    pub id: i64,
    pub display_number: String,
    pub location_id: i64,
    pub height: LockerHeight,
    pub status: LockerStatus,
    pub is_damaged: bool,
    pub note: Option<String>,
    // Denormalized fields for display (from active lease)
    pub tenant_username: Option<String>,
    pub location_name: Option<String>,
}

impl Locker {
    pub fn new(id: i64, display_number: impl Into<String>, location_id: i64, height: LockerHeight) -> Self {
        Self {
            id,
            display_number: display_number.into(),
            location_id,
            height,
            status: LockerStatus::Free,
            is_damaged: false,
            note: None,
            tenant_username: None,
            location_name: None,
        }
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let needle = query.to_lowercase();
        self.display_number.to_lowercase().contains(&needle)
            || self
                .tenant_username
                .as_ref()
                .map(|name| name.to_lowercase().contains(&needle))
                .unwrap_or(false)
            || self
                .note
                .as_ref()
                .map(|note| note.to_lowercase().contains(&needle))
                .unwrap_or(false)
            || self
                .location_name
                .as_ref()
                .map(|loc| loc.to_lowercase().contains(&needle))
                .unwrap_or(false)
            || self.id.to_string().contains(&needle)
    }

    pub fn mark_free(&mut self) {
        self.status = LockerStatus::Free;
        self.tenant_username = None;
    }

    pub fn mark_occupied(&mut self, username: impl Into<String>) {
        self.status = LockerStatus::Occupied;
        self.tenant_username = Some(username.into());
    }

    pub fn mark_maintenance(&mut self, note: impl Into<String>) {
        self.status = LockerStatus::Maintenance;
        self.note = Some(note.into());
    }

    pub fn set_damaged(&mut self, is_damaged: bool, note: Option<String>) {
        self.is_damaged = is_damaged;
        if note.is_some() {
            self.note = note;
        }
    }

    /// Returns the IServ email address for the tenant
    pub fn tenant_email(&self) -> Option<String> {
        self.tenant_username.as_ref().map(|u| format!("{}@athenetz.de", u))
    }
}

/// A lease/rental agreement for a locker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lease {
    pub id: i64,
    pub locker_id: i64,
    pub tenant_username: String,
    pub tenant_type: TenantType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub deposit_paid: bool,
    pub yearly_fee_paid_until: DateTime<Utc>,
    pub is_active: bool,
    // Denormalized for display
    pub locker_display_number: Option<String>,
}

impl Lease {
    pub fn new(
        id: i64,
        locker_id: i64,
        tenant_username: impl Into<String>,
        tenant_type: TenantType,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            locker_id,
            tenant_username: tenant_username.into(),
            tenant_type,
            start_date,
            end_date,
            deposit_paid: false,
            yearly_fee_paid_until: end_date,
            is_active: true,
            locker_display_number: None,
        }
    }

    /// Returns the IServ email address
    pub fn tenant_email(&self) -> String {
        format!("{}@athenetz.de", self.tenant_username)
    }

    /// Calculate the debt amount in euros
    /// Returns positive value if the tenant owes money (lease expired)
    pub fn calculate_debt(&self, current_date: DateTime<Utc>) -> i64 {
        if self.end_date >= current_date {
            return 0;
        }

        // Calculate years overdue (each year is 10€)
        let duration = current_date.signed_duration_since(self.end_date);
        let days_overdue = duration.num_days();
        if days_overdue <= 0 {
            return 0;
        }

        // Use DAYS_PER_YEAR for billing, charge 10€ per started year
        let years_overdue = (days_overdue as f64 / DAYS_PER_YEAR as f64).ceil() as i64;
        years_overdue * 10
    }

    /// Extend the lease by adding years based on payment (10€ per year)
    pub fn extend(&mut self, amount_paid: i64) {
        if amount_paid <= 0 || amount_paid % 10 != 0 {
            return;
        }

        let years_to_add = amount_paid / 10;
        // Add years to end_date
        self.end_date = self.end_date + chrono::Duration::days(DAYS_PER_YEAR * years_to_add);
        self.yearly_fee_paid_until = self.end_date;
    }

    /// Check if the lease is overdue
    pub fn is_overdue(&self, current_date: DateTime<Utc>) -> bool {
        self.end_date < current_date
    }
}

/// Days per year (approximate, for billing purposes)
pub const DAYS_PER_YEAR: i64 = 365;

/// Financial constants
pub const DEPOSIT_AMOUNT: i64 = 10;
pub const YEARLY_FEE: i64 = 10;
pub const INITIAL_PAYMENT: i64 = DEPOSIT_AMOUNT + YEARLY_FEE; // 20€

/// Validate that a payment amount is valid (divisible by 10)
pub fn is_valid_payment(amount: i64) -> bool {
    amount > 0 && amount % 10 == 0
}

/// Validate IServ username format (alphanumeric with dots, no @)
pub fn is_valid_username(username: &str) -> bool {
    if username.is_empty() || username.contains('@') {
        return false;
    }
    username.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-')
}

/// Statistics for the dashboard
#[derive(Debug, Clone, Default)]
pub struct LockerStatistics {
    pub total: usize,
    pub free: usize,
    pub occupied: usize,
    pub maintenance: usize,
    pub damaged: usize,
}

impl LockerStatistics {
    pub fn free_percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.free as f64 / self.total as f64) * 100.0
        }
    }

    pub fn occupied_percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.occupied as f64 / self.total as f64) * 100.0
        }
    }
}

/// Statistics per location
#[derive(Debug, Clone)]
pub struct LocationStatistics {
    pub location_id: i64,
    pub location_name: String,
    pub total: usize,
    pub free: usize,
    pub occupied: usize,
}

/// Debtor information for finance reports
#[derive(Debug, Clone, Serialize)]
pub struct Debtor {
    pub email: String,
    pub username: String,
    pub amount_owed: i64,
    pub locker_display_number: String,
    pub days_overdue: i64,
}

/// Revenue entry for financial reports
#[derive(Debug, Clone, Serialize)]
pub struct RevenueEntry {
    pub date: DateTime<Utc>,
    pub amount: i64,
    pub description: String,
    pub tenant_username: String,
    pub locker_display_number: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn locker_status_db_roundtrip() {
        let cases = [
            ("Free", LockerStatus::Free),
            ("Occupied", LockerStatus::Occupied),
            ("Maintenance", LockerStatus::Maintenance),
        ];

        for (value, status) in cases {
            assert_eq!(LockerStatus::from_db_value(value), Some(status));
            assert_eq!(status.to_db_value(), value);
        }

        assert_eq!(LockerStatus::from_db_value("Invalid"), None);
    }

    #[test]
    fn locker_height_db_roundtrip() {
        let cases = [
            ("Top", LockerHeight::Top),
            ("Middle", LockerHeight::Middle),
            ("Bottom", LockerHeight::Bottom),
        ];

        for (value, height) in cases {
            assert_eq!(LockerHeight::from_db_value(value), Some(height));
            assert_eq!(height.to_db_value(), value);
        }
    }

    #[test]
    fn tenant_type_db_roundtrip() {
        let cases = [
            ("Student", TenantType::Student),
            ("Teacher", TenantType::Teacher),
        ];

        for (value, tenant_type) in cases {
            assert_eq!(TenantType::from_db_value(value), Some(tenant_type));
            assert_eq!(tenant_type.to_db_value(), value);
        }
    }

    #[test]
    fn locker_matches_query_checks_multiple_fields() {
        let mut locker = Locker::new(42, "B-12", 1, LockerHeight::Middle);
        locker.mark_occupied("anna.mueller");
        locker.note = Some("Defekt".into());
        locker.location_name = Some("Hauptgebäude".into());

        assert!(locker.matches_query("B-12"));
        assert!(locker.matches_query("anna"));
        assert!(locker.matches_query("Defekt"));
        assert!(locker.matches_query("42"));
        assert!(locker.matches_query("Haupt"));
        assert!(!locker.matches_query("nicht da"));
    }

    #[test]
    fn locker_mark_free_and_occupied() {
        let mut locker = Locker::new(1, "A-01", 1, LockerHeight::Top);
        assert!(locker.status.is_free());
        assert!(locker.tenant_username.is_none());

        locker.mark_occupied("karl.schmidt");
        assert!(locker.status.is_occupied());
        assert_eq!(locker.tenant_username.as_deref(), Some("karl.schmidt"));
        assert_eq!(locker.tenant_email(), Some("karl.schmidt@athenetz.de".to_string()));

        locker.mark_free();
        assert!(locker.status.is_free());
        assert!(locker.tenant_username.is_none());
    }

    #[test]
    fn lease_calculate_debt() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let lease = Lease::new(1, 1, "test.user", TenantType::Student, start, end);

        // Not overdue
        let current = Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap();
        assert_eq!(lease.calculate_debt(current), 0);

        // About 1 year overdue (10€) - 366 days after end_date
        let current = Utc.with_ymd_and_hms(2026, 1, 3, 0, 0, 0).unwrap();
        assert_eq!(lease.calculate_debt(current), 20); // ceil(367/365) = 2 years = 20€

        // About 2 years overdue
        let current = Utc.with_ymd_and_hms(2027, 1, 10, 0, 0, 0).unwrap();
        let debt = lease.calculate_debt(current);
        assert!(debt >= 20); // Should be at least 2 years
    }

    #[test]
    fn lease_extend_adds_years() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let mut lease = Lease::new(1, 1, "test.user", TenantType::Student, start, end);

        // Extend by 1 year (10€) - adds DAYS_PER_YEAR days
        lease.extend(10);
        let expected_date_1 = end + chrono::Duration::days(DAYS_PER_YEAR);
        assert_eq!(lease.end_date, expected_date_1);

        // Extend by 2 years (20€) - adds 2 * DAYS_PER_YEAR days
        let old_end = lease.end_date;
        lease.extend(20);
        let expected_date_2 = old_end + chrono::Duration::days(2 * DAYS_PER_YEAR);
        assert_eq!(lease.end_date, expected_date_2);
    }

    #[test]
    fn lease_extend_rejects_invalid_amounts() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let mut lease = Lease::new(1, 1, "test.user", TenantType::Student, start, end);

        let original_end = lease.end_date;

        // Invalid amounts should not change the lease
        lease.extend(0);
        assert_eq!(lease.end_date, original_end);

        lease.extend(-10);
        assert_eq!(lease.end_date, original_end);

        lease.extend(15); // Not divisible by 10
        assert_eq!(lease.end_date, original_end);
    }

    #[test]
    fn is_valid_payment_checks_divisibility() {
        assert!(is_valid_payment(10));
        assert!(is_valid_payment(20));
        assert!(is_valid_payment(100));
        assert!(!is_valid_payment(0));
        assert!(!is_valid_payment(-10));
        assert!(!is_valid_payment(15));
        assert!(!is_valid_payment(5));
    }

    #[test]
    fn is_valid_username_validation() {
        assert!(is_valid_username("max.mustermann"));
        assert!(is_valid_username("anna_mueller"));
        assert!(is_valid_username("karl-schmidt"));
        assert!(is_valid_username("user123"));
        assert!(!is_valid_username(""));
        assert!(!is_valid_username("user@domain"));
        assert!(!is_valid_username("user name")); // space not allowed
    }

    #[test]
    fn locker_statistics_percentages() {
        let stats = LockerStatistics {
            total: 100,
            free: 30,
            occupied: 60,
            maintenance: 10,
            damaged: 5,
        };

        assert!((stats.free_percentage() - 30.0).abs() < 0.01);
        assert!((stats.occupied_percentage() - 60.0).abs() < 0.01);

        let empty_stats = LockerStatistics::default();
        assert!((empty_stats.free_percentage() - 0.0).abs() < 0.01);
    }
}
