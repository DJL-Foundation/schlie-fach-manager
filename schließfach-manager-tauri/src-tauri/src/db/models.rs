//! Database models for Schließfach-Manager.

use serde::{Deserialize, Serialize};

/// Locker size enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LockerSize {
    S,
    M,
    L,
    XL,
}

impl From<String> for LockerSize {
    fn from(s: String) -> Self {
        match s.as_str() {
            "S" => LockerSize::S,
            "M" => LockerSize::M,
            "L" => LockerSize::L,
            "XL" => LockerSize::XL,
            _ => LockerSize::M,
        }
    }
}

impl From<LockerSize> for String {
    fn from(size: LockerSize) -> Self {
        match size {
            LockerSize::S => "S".to_string(),
            LockerSize::M => "M".to_string(),
            LockerSize::L => "L".to_string(),
            LockerSize::XL => "XL".to_string(),
        }
    }
}

/// Locker model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locker {
    pub id: i64,
    pub number: String,
    pub location: String,
    pub size: String,
    pub is_damaged: bool,
    pub notes: Option<String>,
    pub created_at: String,
}

/// Input for creating a new locker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLockerInput {
    pub number: String,
    pub location: String,
    pub size: String,
    pub notes: Option<String>,
}

/// Input for updating a locker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLockerInput {
    pub id: i64,
    pub number: String,
    pub location: String,
    pub size: String,
    pub is_damaged: bool,
    pub notes: Option<String>,
}

/// Rental model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rental {
    pub id: i64,
    pub locker_id: i64,
    pub locker_number: String,
    pub renter_name: String,
    pub renter_email: Option<String>,
    pub renter_phone: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub deposit_paid: bool,
    pub deposit_returned: bool,
    pub notes: Option<String>,
    pub created_at: String,
}

/// Input for creating a new rental
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRentalInput {
    pub locker_id: i64,
    pub renter_name: String,
    pub renter_email: Option<String>,
    pub renter_phone: Option<String>,
    pub start_date: String,
    pub duration_months: i32,
    pub deposit_paid: bool,
    pub notes: Option<String>,
}

/// Payment type enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    Deposit,
    YearlyFee,
    Extension,
    Refund,
    Other,
}

impl From<String> for PaymentType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "deposit" => PaymentType::Deposit,
            "yearly_fee" => PaymentType::YearlyFee,
            "extension" => PaymentType::Extension,
            "refund" => PaymentType::Refund,
            _ => PaymentType::Other,
        }
    }
}

/// Payment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub rental_id: i64,
    pub amount_cents: i64,
    pub payment_date: Option<String>,
    pub payment_type: String,
    pub notes: Option<String>,
    pub created_at: String,
}

/// Settings model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub deposit_cents: i64,
    pub yearly_fee_cents: i64,
    pub billing_period: String,
    pub currency: String,
    pub screensaver_timeout_seconds: i64,
    pub app_version: String,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: i64,
    pub timestamp: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub details: Option<String>,
    pub username: String,
}

/// Dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub total_lockers: i64,
    pub occupied_lockers: i64,
    pub occupancy_percent: f64,
    pub by_size: std::collections::HashMap<String, SizeStats>,
    pub by_location: std::collections::HashMap<String, LocationStats>,
    pub overdue_returns: i64,
    pub expiring_soon: i64,
    pub damaged_lockers: i64,
    pub pending_payments_cents: i64,
    pub revenue_30d_cents: i64,
    pub occupancy_history: Vec<OccupancyHistoryPoint>,
}

/// Size statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeStats {
    pub total: i64,
    pub occupied: i64,
}

/// Location statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationStats {
    pub total: i64,
    pub occupied: i64,
}

/// Occupancy history point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupancyHistoryPoint {
    pub date: String,
    pub percent: f64,
}

/// Status bar data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBarData {
    pub db_status: String,
    pub total_lockers: i64,
    pub occupied_lockers: i64,
    pub occupancy_percent: f64,
}

/// Export input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportInput {
    pub format: String,
    pub path: String,
}

/// Location model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}
