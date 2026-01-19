pub mod connection;
pub mod migrations;
pub mod lockers;
pub mod rentals;
pub mod payments;
pub mod locations;
pub mod audit;
pub mod history;

pub use connection::Database;
pub use migrations::run_migrations;

// Legacy backwards compatibility layer for old Locker model
use crate::model::{Locker, LockerStatus};
use color_eyre::eyre::Result;

impl Database {
    /// Inserts or updates a locker entry (legacy method).
    /// Maps old Locker model to new schema.
    pub fn upsert_locker(&self, locker: &Locker) -> Result<()> {
        let number = &locker.label;
        let location = "Default";
        let size = "Medium";
        
        if let Some(existing) = lockers::get_locker_by_number(self.connection(), number)? {
            let mut updated = existing;
            updated.is_damaged = locker.status.is_blocked();
            updated.notes = locker.note.clone();
            lockers::update_locker(self.connection(), &updated)?;
            
            // If there's an occupant, create or update rental
            if let Some(occupant_name) = &locker.occupant {
                // Check if there's an active rental
                if let Some(mut rental) = rentals::get_current_rental_for_locker(self.connection(), updated.id)? {
                    rental.renter_name = occupant_name.clone();
                    rentals::update_rental(self.connection(), &rental)?;
                } else {
                    // Create new rental (dates that span current time)
                    rentals::create_rental(
                        self.connection(),
                        updated.id,
                        occupant_name,
                        None,
                        None,
                        "2026-01-01",
                        "2026-12-31",
                    )?;
                }
            }
        } else {
            let locker_id = lockers::create_locker(self.connection(), number, location, size)?;
            
            // Update the locker with notes and damaged status if needed
            if locker.note.is_some() || locker.status.is_blocked() {
                if let Some(mut new_locker) = lockers::get_locker(self.connection(), locker_id)? {
                    new_locker.notes = locker.note.clone();
                    new_locker.is_damaged = locker.status.is_blocked();
                    lockers::update_locker(self.connection(), &new_locker)?;
                }
            }
            
            // If there's an occupant, create rental (dates that span current time)
            if let Some(occupant_name) = &locker.occupant {
                rentals::create_rental(
                    self.connection(),
                    locker_id,
                    occupant_name,
                    None,
                    None,
                    "2026-01-01",
                    "2026-12-31",
                )?;
            }
        }
        
        Ok(())
    }

    /// Returns all lockers sorted by their label (legacy method).
    pub fn list_lockers(&self) -> Result<Vec<Locker>> {
        let new_lockers = lockers::list_lockers(self.connection())?;
        
        let mut old_lockers = Vec::new();
        for new_locker in new_lockers {
            let status = if new_locker.is_damaged {
                LockerStatus::Maintenance
            } else {
                if rentals::get_current_rental_for_locker(self.connection(), new_locker.id)?.is_some() {
                    LockerStatus::Occupied
                } else {
                    LockerStatus::Available
                }
            };
            
            let occupant = rentals::get_current_rental_for_locker(self.connection(), new_locker.id)?
                .map(|r| r.renter_name);
            
            old_lockers.push(Locker {
                id: new_locker.id,
                label: new_locker.number,
                status,
                occupant,
                note: new_locker.notes,
            });
        }
        
        Ok(old_lockers)
    }

    /// Returns a single locker by its id (legacy method).
    pub fn get_locker(&self, id: i64) -> Result<Option<Locker>> {
        if let Some(new_locker) = lockers::get_locker(self.connection(), id)? {
            let status = if new_locker.is_damaged {
                LockerStatus::Maintenance
            } else {
                if rentals::get_current_rental_for_locker(self.connection(), id)?.is_some() {
                    LockerStatus::Occupied
                } else {
                    LockerStatus::Available
                }
            };
            
            let occupant = rentals::get_current_rental_for_locker(self.connection(), id)?
                .map(|r| r.renter_name);
            
            Ok(Some(Locker {
                id: new_locker.id,
                label: new_locker.number,
                status,
                occupant,
                note: new_locker.notes,
            }))
        } else {
            Ok(None)
        }
    }

    /// Deletes a locker and reports whether anything was removed (legacy method).
    pub fn delete_locker(&self, id: i64) -> Result<bool> {
        lockers::delete_locker(self.connection(), id)
    }

    /// Inserts the provided lockers only when the table is still empty (legacy method).
    pub fn seed_if_empty(&self, seed: &[Locker]) -> Result<()> {
        if self.count_lockers()? == 0 {
            for locker in seed {
                self.upsert_locker(locker)?;
            }
        }
        Ok(())
    }

    /// Returns the number of lockers stored in the database (legacy method).
    pub fn count_lockers(&self) -> Result<i64> {
        lockers::count_lockers(self.connection())
    }
}

#[cfg(test)]
mod legacy_tests {
    use super::*;
    use crate::model::Locker;

    #[test]
    fn test_legacy_upsert_and_fetch() -> Result<()> {
        let db = Database::open_in_memory()?;
        let mut locker = Locker::new(42, "Z-01");
        locker.assign("Alice");
        locker.note = Some("Test".into());

        db.upsert_locker(&locker)?;
        let fetched_list = db.list_lockers()?;
        let fetched = fetched_list.into_iter().find(|l| l.label == "Z-01").expect("locker missing");
        assert_eq!(fetched.label, "Z-01");
        assert_eq!(fetched.note, Some("Test".into()));

        Ok(())
    }

    #[test]
    fn test_legacy_delete() -> Result<()> {
        let db = Database::open_in_memory()?;
        let locker = Locker::new(7, "B-12");
        db.upsert_locker(&locker)?;

        let lockers = db.list_lockers()?;
        let id = lockers.iter().find(|l| l.label == "B-12").unwrap().id;

        assert!(db.delete_locker(id)?);
        assert!(!db.delete_locker(id)?);

        Ok(())
    }

    #[test]
    fn test_legacy_list_sorted() -> Result<()> {
        let db = Database::open_in_memory()?;
        let unsorted = vec![
            Locker::new(2, "B-02"),
            Locker::new(1, "A-01"),
            Locker::new(3, "C-03"),
        ];
        for locker in &unsorted {
            db.upsert_locker(locker)?;
        }

        let labels: Vec<String> = db
            .list_lockers()?
            .into_iter()
            .map(|locker| locker.label)
            .collect();

        assert_eq!(
            labels,
            vec!["A-01".to_string(), "B-02".to_string(), "C-03".to_string()]
        );

        Ok(())
    }

    #[test]
    fn test_legacy_seed_if_empty() -> Result<()> {
        let db = Database::open_in_memory()?;
        let initial = vec![Locker::new(1, "A-01"), Locker::new(2, "B-02")];
        db.seed_if_empty(initial.as_slice())?;
        assert_eq!(db.count_lockers()?, 2);

        let custom = Locker::new(3, "C-03");
        db.upsert_locker(&custom)?;

        let extra = vec![Locker::new(99, "Z-99")];
        db.seed_if_empty(extra.as_slice())?;

        let lockers = db.list_lockers()?;
        assert_eq!(lockers.len(), 3);
        let has_z99 = lockers.iter().any(|locker| locker.label == "Z-99");
        assert!(!has_z99);

        Ok(())
    }
}

