//! Locker domain model and related types.
//!
//! This module contains the [`Locker`] struct representing a physical storage locker
//! and related types like [`HeightCategory`] for categorizing lockers by height.
//!
//! # Example
//!
//! ```rust
//! use schliessfach_manager::models::Locker;
//!
//! // Create a new locker
//! let mut locker = Locker::new("A-001", "Hauptgebäude", 150);
//!
//! // Check properties
//! assert_eq!(locker.label, "A-001");
//! assert!(!locker.is_damaged);
//!
//! // Mark as damaged
//! locker.mark_damaged();
//! assert!(locker.is_damaged);
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A physical storage locker that can be rented by tenants.
///
/// Lockers are identified by their unique label (e.g., "A-001") and belong
/// to a specific location (e.g., "Hauptgebäude", "Turnhalle").
///
/// # Fields
///
/// * `id` - Database identifier (auto-assigned on save)
/// * `label` - Unique human-readable identifier (e.g., "A-001")
/// * `location` - Physical location name
/// * `height` - Height in centimeters
/// * `is_damaged` - Whether the locker is currently marked as damaged
/// * `created_at` - Timestamp when the locker was created
///
/// # Example
///
/// ```rust
/// use schliessfach_manager::models::Locker;
///
/// let locker = Locker::new("B-042", "Turnhalle", 120);
/// println!("Locker {} at {} is {} cm tall",
///     locker.label, locker.location, locker.height);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Locker {
    /// Database identifier (auto-assigned on save).
    pub id: i64,
    /// Unique human-readable label (e.g., "A-001").
    pub label: String,
    /// Physical location name (e.g., "Hauptgebäude", "Turnhalle").
    pub location: String,
    /// Height in centimeters.
    pub height: i32,
    /// Whether the locker is currently marked as damaged.
    pub is_damaged: bool,
    /// Timestamp when the locker was created.
    pub created_at: DateTime<Utc>,
}

impl Locker {
    /// Creates a new locker with the given parameters.
    ///
    /// The locker is created with `is_damaged` set to `false` and
    /// `created_at` set to the current time.
    ///
    /// # Arguments
    ///
    /// * `label` - Unique identifier for the locker (e.g., "A-001")
    /// * `location` - Physical location name
    /// * `height` - Height in centimeters
    ///
    /// # Example
    ///
    /// ```rust
    /// use schliessfach_manager::models::Locker;
    ///
    /// let locker = Locker::new("C-010", "Neubau", 180);
    /// assert_eq!(locker.label, "C-010");
    /// ```
    pub fn new(label: impl Into<String>, location: impl Into<String>, height: i32) -> Self {
        Self {
            id: 0,
            label: label.into(),
            location: location.into(),
            height,
            is_damaged: false,
            created_at: Utc::now(),
        }
    }

    /// Marks the locker as damaged.
    ///
    /// Damaged lockers should not be rented out until repaired.
    pub fn mark_damaged(&mut self) {
        self.is_damaged = true;
    }

    /// Marks the locker as repaired.
    ///
    /// The locker becomes available for rental again.
    pub fn mark_repaired(&mut self) {
        self.is_damaged = false;
    }

    /// Returns the height category for this locker.
    pub fn height_category(&self) -> HeightCategory {
        match self.height {
            0..=50 => HeightCategory::Low,
            51..=150 => HeightCategory::Middle,
            _ => HeightCategory::High,
        }
    }

    /// Checks if the locker matches the given query string.
    pub fn matches_query(&self, query: &str) -> bool {
        let needle = query.to_lowercase();
        self.label.to_lowercase().contains(&needle)
            || self.location.to_lowercase().contains(&needle)
            || self.id.to_string().contains(&needle)
    }
}

/// Height preference for locker selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeightCategory {
    Low,    // 0-50cm
    Middle, // 50-150cm
    High,   // 150-300cm
}

impl HeightCategory {
    /// Returns the range for this height category.
    pub fn range(&self) -> (i32, i32) {
        match self {
            HeightCategory::Low => (0, 50),
            HeightCategory::Middle => (51, 150),
            HeightCategory::High => (151, 300),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locker_creation() {
        let locker = Locker::new("A-042", "Hauptgebäude", 100);
        assert_eq!(locker.label, "A-042");
        assert_eq!(locker.location, "Hauptgebäude");
        assert_eq!(locker.height, 100);
        assert!(!locker.is_damaged);
    }

    #[test]
    fn test_locker_damage_flag() {
        let mut locker = Locker::new("A-042", "Test", 100);
        assert!(!locker.is_damaged);

        locker.mark_damaged();
        assert!(locker.is_damaged);

        locker.mark_repaired();
        assert!(!locker.is_damaged);
    }

    #[test]
    fn test_height_category() {
        let low = Locker::new("A-1", "Test", 30);
        assert_eq!(low.height_category(), HeightCategory::Low);

        let mid = Locker::new("A-2", "Test", 100);
        assert_eq!(mid.height_category(), HeightCategory::Middle);

        let high = Locker::new("A-3", "Test", 200);
        assert_eq!(high.height_category(), HeightCategory::High);
    }

    #[test]
    fn test_matches_query() {
        let locker = Locker::new("A-042", "Hauptgebäude", 100);
        assert!(locker.matches_query("A-042"));
        assert!(locker.matches_query("haupt"));
        assert!(!locker.matches_query("xyz"));
    }
}
