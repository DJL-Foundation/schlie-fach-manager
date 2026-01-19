use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Domain model for a single locker entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Locker {
    pub id: i64,
    pub label: String,
    pub location: String,
    pub height: i32,
    pub is_damaged: bool,
    pub created_at: DateTime<Utc>,
}

impl Locker {
    /// Creates a new locker with the given parameters.
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
    pub fn mark_damaged(&mut self) {
        self.is_damaged = true;
    }

    /// Marks the locker as repaired.
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
