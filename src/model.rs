use serde::{Deserialize, Serialize};

/// Operational state of a locker.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LockerStatus {
    Available,
    Occupied,
    Maintenance,
}

impl LockerStatus {
    /// Returns `true` when the locker can be immediately assigned.
    pub fn is_available(self) -> bool {
        matches!(self, LockerStatus::Available)
    }

    /// Returns `true` when the locker currently holds an occupant.
    pub fn is_occupied(self) -> bool {
        matches!(self, LockerStatus::Occupied)
    }

    /// Returns `true` when the locker is flagged for maintenance.
    pub fn is_blocked(self) -> bool {
        matches!(self, LockerStatus::Maintenance)
    }

    /// Maps the status to an integer we can store inside SQLite.
    pub fn to_db_value(self) -> i64 {
        match self {
            LockerStatus::Available => 0,
            LockerStatus::Occupied => 1,
            LockerStatus::Maintenance => 2,
        }
    }

    /// Recreates a status from the integer persisted in SQLite.
    pub fn from_db_value(value: i64) -> Option<Self> {
        match value {
            0 => Some(LockerStatus::Available),
            1 => Some(LockerStatus::Occupied),
            2 => Some(LockerStatus::Maintenance),
            _ => None,
        }
    }
}

/// Domain aggregate for a single locker entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Locker {
    pub id: i64,
    pub label: String,
    pub status: LockerStatus,
    pub occupant: Option<String>,
    pub note: Option<String>,
}

impl Locker {
    pub fn new(id: i64, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            status: LockerStatus::Available,
            occupant: None,
            note: None,
        }
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let needle = query.to_lowercase();
        self.label.to_lowercase().contains(&needle)
            || self
                .occupant
                .as_ref()
                .map(|name| name.to_lowercase().contains(&needle))
                .unwrap_or(false)
            || self
                .note
                .as_ref()
                .map(|note| note.to_lowercase().contains(&needle))
                .unwrap_or(false)
            || self.id.to_string().contains(&needle)
    }

    pub fn mark_available(&mut self) {
        self.status = LockerStatus::Available;
        self.occupant = None;
    }

    pub fn assign(&mut self, person: impl Into<String>) {
        self.status = LockerStatus::Occupied;
        self.occupant = Some(person.into());
    }

    pub fn release(&mut self) {
        self.status = LockerStatus::Available;
        self.occupant = None;
    }

    pub fn mark_maintenance(&mut self, note: impl Into<String>) {
        self.status = LockerStatus::Maintenance;
        self.occupant = None;
        self.note = Some(note.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_query_checks_label_occupant_note_and_id() {
        let mut locker = Locker::new(42, "B-12");
        locker.assign("Anna");
        locker.note = Some("Defekt".into());

        assert!(locker.matches_query("B-12"));
        assert!(locker.matches_query("Anna"));
        assert!(locker.matches_query("Defekt"));
        assert!(locker.matches_query("42"));
        assert!(!locker.matches_query("nicht da"));
    }

    #[test]
    fn assign_and_release_update_status_and_occupant() {
        let mut locker = Locker::new(1, "A-01");
        assert!(locker.status.is_available());
        assert!(locker.occupant.is_none());

        locker.assign("Karl");
        assert!(locker.status.is_occupied());
        assert_eq!(locker.occupant.as_deref(), Some("Karl"));

        locker.release();
        assert!(locker.status.is_available());
        assert!(locker.occupant.is_none());
    }

    #[test]
    fn mark_maintenance_sets_note_and_clears_occupant() {
        let mut locker = Locker::new(3, "C-03");
        locker.assign("Max");

        locker.mark_maintenance("Störung");
        assert!(locker.status.is_blocked());
        assert_eq!(locker.note.as_deref(), Some("Störung"));
        assert!(locker.occupant.is_none());
    }

    #[test]
    fn locker_status_db_roundtrip() {
        let cases = [
            (0, LockerStatus::Available),
            (1, LockerStatus::Occupied),
            (2, LockerStatus::Maintenance),
        ];

        for (value, status) in cases {
            assert_eq!(LockerStatus::from_db_value(value), Some(status));
            assert_eq!(status.to_db_value(), value);
        }

        assert_eq!(LockerStatus::from_db_value(99), None);
    }
}
