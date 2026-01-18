use crate::{
    db::Database,
    model::{Locker, LockerStatus},
};
use color_eyre::eyre::{Result, eyre};
use unicode_width::UnicodeWidthStr;

/// Describes in which context the TUI currently processes keyboard input.
/// * `Normal`: navigation / command keys
/// * `Searching`: keystrokes are appended to the search query
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Searching,
}

/// Application state container that keeps track of all lockers,
/// the filtered selection, input handling flags, and transient UI messages.
pub struct App {
    db: Database,
    lockers: Vec<Locker>,
    filtered_indices: Vec<usize>,
    selected: usize,
    pub input_mode: InputMode,
    pub search_query: String,
    pub status_message: Option<String>,
}

impl App {
    /// Creates a new application instance using the provided `Database`.
    /// Lockers are loaded immediately so the UI can render meaningful data.
    pub fn new(db: Database) -> Result<Self> {
        let lockers = db.list_lockers()?;
        let filtered_indices = (0..lockers.len()).collect();
        Ok(Self {
            db,
            lockers,
            filtered_indices,
            selected: 0,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            status_message: None,
        })
    }

    /// Reloads locker data from storage (e.g. after an external modification).
    pub fn reload(&mut self) -> Result<()> {
        self.lockers = self.db.list_lockers()?;
        self.filtered_indices = (0..self.lockers.len()).collect();
        self.selected = 0;
        self.apply_filter();
        Ok(())
    }

    /// Returns the number of lockers visible after filtering.
    pub fn visible_count(&self) -> usize {
        self.filtered_indices.len()
    }

    /// Returns the currently highlighted index inside the filtered list.
    pub fn selected_index(&self) -> Option<usize> {
        if self.filtered_indices.is_empty() {
            None
        } else {
            Some(self.selected)
        }
    }

    /// Returns the width of the search query in terminal cells for layout calculations.
    pub fn search_width(&self) -> usize {
        UnicodeWidthStr::width(self.search_query.as_str())
    }

    /// Provides immutable access to the locker currently highlighted in the list.
    pub fn selected_locker(&self) -> Option<&Locker> {
        self.filtered_indices
            .get(self.selected)
            .and_then(|&idx| self.lockers.get(idx))
    }

    /// Returns immutable access to the locker at the provided visible index.
    pub fn locker_at(&self, visible_index: usize) -> Option<&Locker> {
        self.filtered_indices
            .get(visible_index)
            .and_then(|&idx| self.lockers.get(idx))
    }

    /// Iterates over all lockers that pass the current filter.
    pub fn visible_lockers(&self) -> impl Iterator<Item = &Locker> + '_ {
        self.filtered_indices
            .iter()
            .filter_map(|&idx| self.lockers.get(idx))
    }

    /// Provides mutable access to the selected locker for in-place updates.
    fn selected_locker_mut(&mut self) -> Option<&mut Locker> {
        let idx = *self.filtered_indices.get(self.selected)?;
        self.lockers.get_mut(idx)
    }

    /// Move the selection cursor to the next visible locker (wraps around).
    pub fn next(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.filtered_indices.len();
    }

    /// Move the selection cursor to the previous visible locker (wraps around).
    pub fn previous(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.filtered_indices.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    /// Appends a character to the search query and updates the filter set.
    pub fn push_search_char(&mut self, ch: char) {
        self.search_query.push(ch);
        self.apply_filter();
    }

    /// Removes the last character from the search query and updates the filter set.
    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
        self.apply_filter();
    }

    /// Clears the search query completely.
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.apply_filter();
    }

    /// Switches the current input handling mode.
    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    /// Assigns the selected locker to the provided `occupant`.
    pub fn assign_selected(&mut self, occupant: impl Into<String>) -> Result<()> {
        let name = occupant.into();
        if name.trim().is_empty() {
            return Err(eyre!("Name darf nicht leer sein"));
        }

        let locker = self
            .selected_locker_mut()
            .ok_or_else(|| eyre!("kein Schließfach ausgewählt"))?;

        let updated = {
            locker.assign(name);
            locker.clone()
        };

        self.db.upsert_locker(&updated)?;
        self.set_status("Schließfach belegt");
        Ok(())
    }

    /// Releases (frees) the selected locker.
    pub fn release_selected(&mut self) -> Result<()> {
        let locker = self
            .selected_locker_mut()
            .ok_or_else(|| eyre!("kein Schließfach ausgewählt"))?;

        let updated = {
            locker.release();
            locker.clone()
        };

        self.db.upsert_locker(&updated)?;
        self.set_status("Schließfach freigegeben");
        Ok(())
    }

    /// Toggles maintenance mode for the selected locker.
    /// When `note` is `Some`, the locker is set to maintenance with that note.
    /// When `None`, it switches back to available.
    pub fn toggle_maintenance(&mut self, note: Option<String>) -> Result<()> {
        let locker = self
            .selected_locker_mut()
            .ok_or_else(|| eyre!("kein Schließfach ausgewählt"))?;

        let updated = {
            match locker.status {
                LockerStatus::Maintenance => locker.mark_available(),
                _ => {
                    let text = note.unwrap_or_else(|| "Wartung".to_string());
                    locker.mark_maintenance(text);
                }
            }
            locker.clone()
        };

        self.db.upsert_locker(&updated)?;
        self.set_status("Wartungsstatus geändert");
        Ok(())
    }

    /// Sets a transient status message that can be rendered by the UI footer.
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    /// Clears the currently visible status message.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    fn apply_filter(&mut self) {
        if self.search_query.trim().is_empty() {
            self.filtered_indices = (0..self.lockers.len()).collect();
            self.selected = 0;
            return;
        }

        let needle = self.search_query.to_lowercase();
        self.filtered_indices = self
            .lockers
            .iter()
            .enumerate()
            .filter(|(_, locker)| locker.matches_query(&needle))
            .map(|(idx, _)| idx)
            .collect();

        self.selected = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app(lockers: &[Locker]) -> App {
        let db = Database::open_in_memory().expect("in-memory db");
        for locker in lockers {
            db.upsert_locker(locker).expect("seed locker");
        }
        App::new(db).expect("app init")
    }

    #[test]
    fn filtering_updates_visible_indices() {
        let lockers = [
            Locker::new(1, "A-01"),
            Locker::new(2, "B-02"),
            Locker::new(3, "B-03"),
        ];
        let mut app = build_app(&lockers);

        assert_eq!(app.visible_count(), 3);

        app.push_search_char('b');
        assert_eq!(app.visible_count(), 2);
        let labels: Vec<&str> = app
            .visible_lockers()
            .map(|locker| locker.label.as_str())
            .collect();
        assert_eq!(labels, vec!["B-02", "B-03"]);

        app.clear_search();
        assert_eq!(app.visible_count(), 3);
    }

    #[test]
    fn navigation_wraps_around() {
        let lockers = [Locker::new(1, "A-01"), Locker::new(2, "B-02")];
        let mut app = build_app(&lockers);

        assert_eq!(app.selected_index(), Some(0));
        app.previous();
        assert_eq!(app.selected_index(), Some(1));
        app.next();
        assert_eq!(app.selected_index(), Some(0));
    }

    #[test]
    fn actions_modify_selected_locker() {
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);

        app.assign_selected("Max Mustermann")
            .expect("assign selected failed");
        let locker = app.selected_locker().expect("locker missing");
        assert_eq!(locker.occupant.as_deref(), Some("Max Mustermann"));
        assert_eq!(locker.status, LockerStatus::Occupied);

        app.toggle_maintenance(Some("Defekt".into()))
            .expect("toggle maintenance on failed");
        let locker = app.selected_locker().expect("locker missing");
        assert_eq!(locker.status, LockerStatus::Maintenance);
        assert_eq!(locker.note.as_deref(), Some("Defekt"));
        assert!(locker.occupant.is_none());

        app.toggle_maintenance(None)
            .expect("toggle maintenance off failed");
        assert_eq!(
            app.selected_locker().expect("locker missing").status,
            LockerStatus::Available
        );

        app.assign_selected("Erna")
            .expect("assign second time failed");
        app.release_selected().expect("release selected failed");
        let locker = app.selected_locker().expect("locker missing");
        assert!(locker.occupant.is_none());
        assert_eq!(locker.status, LockerStatus::Available);
    }
}
