use crate::{
    db::Database,
    model::{Locker, LockerStatus},
    ui::{
        theme::Theme,
        widgets::{Header, KeybindBar, StatusBar, StatusLevel},
        state::InactivityState,
        screens::screensaver::ScreensaverScreen,
    },
};
use color_eyre::eyre::{Result, eyre};
use unicode_width::UnicodeWidthStr;
use std::time::{Duration, Instant};

/// Describes in which context the TUI currently processes keyboard input.
/// * `Normal`: navigation / command keys
/// * `Searching`: keystrokes are appended to the search query
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Searching,
}

/// The main application screen enumeration
/// Based on v2.1 spec section 4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    Dashboard,
    RentalManagement(RentalTab),
    Finances,
    Management(ManagementTab),
    Screensaver,
}

/// Tabs within the Rental Management screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RentalTab {
    Search,
    List,
    Extend,
    Return,
    Damage,
}

impl RentalTab {
    pub fn name(&self) -> &str {
        match self {
            RentalTab::Search => "Suche",
            RentalTab::List => "Liste",
            RentalTab::Extend => "Verlängern",
            RentalTab::Return => "Rückgabe",
            RentalTab::Damage => "Defekt",
        }
    }
}

/// Tabs within the Management screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagementTab {
    Lockers,
    Locations,
    Settings,
    AuditLog,
}

impl ManagementTab {
    pub fn name(&self) -> &str {
        match self {
            ManagementTab::Lockers => "Schließfächer",
            ManagementTab::Locations => "Standorte",
            ManagementTab::Settings => "Einstellungen",
            ManagementTab::AuditLog => "Audit Log",
        }
    }
}

/// Application state container that manages screens, UI components, and state.
/// Based on v2.1 spec sections 4-6.
pub struct App {
    // Database connection
    db: Database,
    
    // Current screen and previous screen tracking
    screen: AppScreen,
    previous_screen: Option<AppScreen>,
    
    // UI components
    pub header: Header,
    pub keybind_bar: KeybindBar,
    pub status_bar: StatusBar,
    pub theme: Theme,
    
    // Screensaver state
    last_activity: Instant,
    screensaver_timeout: Duration,
    countdown_duration: Duration,
    screensaver_active: bool,
    screensaver_screen: Option<ScreensaverScreen>,
    
    // Legacy locker management state (backward compatibility)
    lockers: Vec<Locker>,
    filtered_indices: Vec<usize>,
    selected: usize,
    pub input_mode: InputMode,
    pub search_query: String,
}

impl App {
    /// Creates a new application instance using the provided `Database`.
    /// Lockers are loaded immediately so the UI can render meaningful data.
    pub fn new(db: Database) -> Result<Self> {
        let lockers = db.list_lockers()?;
        let filtered_indices = (0..lockers.len()).collect();
        
        // Load screensaver timeout from database settings (default: 60 seconds)
        let screensaver_timeout = Self::load_screensaver_timeout(&db)?;
        
        Ok(Self {
            db,
            screen: AppScreen::Dashboard,
            previous_screen: None,
            header: Header::new(),
            keybind_bar: KeybindBar::new(),
            status_bar: StatusBar::new(),
            theme: Theme::default(),
            last_activity: Instant::now(),
            screensaver_timeout,
            countdown_duration: Duration::from_secs(15),
            screensaver_active: false,
            screensaver_screen: None,
            lockers,
            filtered_indices,
            selected: 0,
            input_mode: InputMode::Normal,
            search_query: String::new(),
        })
    }
    
    /// Load screensaver timeout from database settings
    fn load_screensaver_timeout(db: &Database) -> Result<Duration> {
        // Query settings table for screensaver_timeout_seconds
        // Default to 60 seconds if not found
        let conn = db.connection();
        let timeout_secs: i64 = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'screensaver_timeout_seconds'",
                [],
                |row| {
                    let val: String = row.get(0)?;
                    val.parse::<i64>().map_err(|_| rusqlite::Error::InvalidQuery)
                },
            )
            .unwrap_or(60);
        
        Ok(Duration::from_secs(timeout_secs as u64))
    }
    
    /// Get the current screen
    pub fn screen(&self) -> &AppScreen {
        &self.screen
    }
    
    /// Check if screensaver is active
    pub fn is_screensaver_active(&self) -> bool {
        self.screensaver_active
    }
    
    /// Get the screensaver screen (if active)
    pub fn screensaver_screen(&self) -> Option<&ScreensaverScreen> {
        self.screensaver_screen.as_ref()
    }
    
    /// Get mutable reference to screensaver screen (if active)
    pub fn screensaver_screen_mut(&mut self) -> Option<&mut ScreensaverScreen> {
        self.screensaver_screen.as_mut()
    }
    
    // ========================================================================
    // Screen Management (v2.1 spec section 4 & 6)
    // ========================================================================
    
    /// Switch to a new screen
    pub fn switch_screen(&mut self, screen: AppScreen) {
        // Don't track screensaver as previous screen
        if self.screen != AppScreen::Screensaver {
            self.previous_screen = Some(self.screen.clone());
        }
        self.screen = screen;
        self.update_header_and_keybinds();
    }
    
    /// Update header and keybind bar for current screen
    fn update_header_and_keybinds(&mut self) {
        self.header.set_current_screen(self.screen_name());
        // Context keybinds will be set by individual screen implementations
    }
    
    /// Get the display name for the current screen
    fn screen_name(&self) -> String {
        match &self.screen {
            AppScreen::Dashboard => "Dashboard".to_string(),
            AppScreen::RentalManagement(tab) => format!("Verleih-Management: {}", tab.name()),
            AppScreen::Finances => "Finanzen".to_string(),
            AppScreen::Management(tab) => format!("Verwaltung: {}", tab.name()),
            AppScreen::Screensaver => "Screensaver".to_string(),
        }
    }
    
    // ========================================================================
    // Inactivity Tracking & Screensaver (v2.1 spec section 5)
    // ========================================================================
    
    /// Update the last activity timestamp (resets inactivity timer)
    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
        
        // If screensaver was active, exit it
        if self.screensaver_active {
            self.exit_screensaver();
        }
    }
    
    /// Check current inactivity state
    pub fn check_inactivity(&self) -> InactivityState {
        let inactive_duration = self.last_activity.elapsed();
        
        if inactive_duration >= self.screensaver_timeout + self.countdown_duration {
            InactivityState::ScreensaverActive
        } else if inactive_duration >= self.screensaver_timeout {
            let remaining = self.countdown_duration
                .saturating_sub(inactive_duration - self.screensaver_timeout);
            InactivityState::Countdown(remaining.as_secs())
        } else {
            InactivityState::Active
        }
    }
    
    /// Activate the screensaver
    pub fn activate_screensaver(&mut self) {
        if !self.screensaver_active {
            self.previous_screen = Some(self.screen.clone());
            self.screensaver_active = true;
            self.screen = AppScreen::Screensaver;
            self.screensaver_screen = Some(ScreensaverScreen::new());
        }
    }
    
    /// Exit the screensaver and return to Dashboard
    /// Per spec section 5.1: "Rückkehr direkt zum Dashboard"
    pub fn exit_screensaver(&mut self) {
        if self.screensaver_active {
            self.screensaver_active = false;
            self.screensaver_screen = None;
            
            // Always return to Dashboard after screensaver (not previous screen)
            self.screen = AppScreen::Dashboard;
            self.previous_screen = None;
            
            self.update_activity();
            self.update_header_and_keybinds();
        }
    }
    
    // ========================================================================
    // Key Handling (v2.1 spec section 4.4)
    // ========================================================================
    
    /// Handle a key press
    /// Returns Ok(true) if the app should quit, Ok(false) otherwise
    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) -> Result<bool> {
        use crossterm::event::KeyCode;
        
        // If screensaver is active, any key exits it
        if self.screensaver_active {
            self.exit_screensaver();
            return Ok(false);
        }
        
        // Handle window switcher if active (before other key handling)
        if self.header.is_window_switcher_active() {
            return self.handle_window_switcher_key(key);
        }
        
        // Handle Escape key with triple-escape logic
        if key == KeyCode::Esc {
            return self.handle_escape();
        }
        
        // Any other key resets escape counter
        self.status_bar.reset_escape_count();
        
        // Global keybinds
        match key {
            KeyCode::Char('Q') => return Ok(true), // Shift+Q to quit (detected as uppercase Q)
            KeyCode::Char('^') => {
                self.header.toggle_window_switcher();
                return Ok(false);
            }
            _ => {}
        }
        
        // Screen-specific key handling
        self.handle_screen_key(key)
    }
    
    /// Handle escape key with 3x escape = Dashboard logic
    /// Per spec section 4.4
    fn handle_escape(&mut self) -> Result<bool> {
        self.status_bar.increment_escape_count();
        
        if self.status_bar.should_return_to_dashboard() {
            self.status_bar.reset_escape_count();
            self.switch_screen(AppScreen::Dashboard);
            self.status_bar.set_message(
                "Zurück zum Dashboard".to_string(),
                StatusLevel::Info,
            );
            return Ok(false);
        }
        
        // Normal escape behavior (close dialog, go back, etc.)
        self.handle_escape_normal()
    }
    
    /// Handle normal escape behavior (not triple-escape)
    fn handle_escape_normal(&mut self) -> Result<bool> {
        // Screen-specific escape handling
        match &self.screen {
            AppScreen::Dashboard => {
                // On Dashboard, escape clears search or does nothing
                if self.input_mode == InputMode::Searching {
                    self.set_input_mode(InputMode::Normal);
                    self.clear_search();
                }
            }
            _ => {
                // On other screens, escape goes back to Dashboard
                self.switch_screen(AppScreen::Dashboard);
            }
        }
        
        Ok(false)
    }
    
    /// Handle key when window switcher is active
    fn handle_window_switcher_key(&mut self, key: crossterm::event::KeyCode) -> Result<bool> {
        use crossterm::event::KeyCode;
        
        match key {
            KeyCode::Tab => {
                self.header.select_next_window();
            }
            KeyCode::BackTab => {
                self.header.select_previous_window();
            }
            KeyCode::Enter => {
                if let Some(screen) = self.header.confirm_window_selection() {
                    self.switch_screen(screen);
                }
            }
            KeyCode::Esc => {
                self.header.cancel_window_switcher();
            }
            _ => {}
        }
        
        Ok(false)
    }
    
    /// Handle key for the current screen
    fn handle_screen_key(&mut self, key: crossterm::event::KeyCode) -> Result<bool> {
        use crossterm::event::KeyCode;
        
        match &self.screen {
            AppScreen::Dashboard => self.handle_dashboard_key(key),
            AppScreen::RentalManagement(tab) => self.handle_rental_management_key(key, *tab),
            AppScreen::Finances => self.handle_finances_key(key),
            AppScreen::Management(tab) => self.handle_management_key(key, *tab),
            AppScreen::Screensaver => Ok(false), // Already handled above
        }
    }
    
    /// Handle key on Dashboard screen
    /// Per spec section 6.3: Keys 1-5 jump to specific rental management tabs
    fn handle_dashboard_key(&mut self, key: crossterm::event::KeyCode) -> Result<bool> {
        use crossterm::event::KeyCode;
        
        match key {
            KeyCode::Char('1') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Search));
            }
            KeyCode::Char('2') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::List));
            }
            KeyCode::Char('3') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Extend));
            }
            KeyCode::Char('4') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Return));
            }
            KeyCode::Char('5') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Damage));
            }
            // Legacy navigation keys
            KeyCode::Down => self.next(),
            KeyCode::Up => self.previous(),
            KeyCode::Char('/') => {
                self.set_input_mode(InputMode::Searching);
            }
            KeyCode::Char(ch) if self.input_mode == InputMode::Searching => {
                self.push_search_char(ch);
            }
            KeyCode::Backspace if self.input_mode == InputMode::Searching => {
                self.pop_search_char();
            }
            _ => {}
        }
        
        Ok(false)
    }
    
    /// Handle key on Rental Management screen
    fn handle_rental_management_key(&mut self, key: crossterm::event::KeyCode, _tab: RentalTab) -> Result<bool> {
        use crossterm::event::KeyCode;
        
        match key {
            KeyCode::Tab => {
                // Switch to next tab (not implemented yet)
            }
            KeyCode::BackTab => {
                // Switch to previous tab (not implemented yet)
            }
            _ => {}
        }
        
        Ok(false)
    }
    
    /// Handle key on Finances screen
    fn handle_finances_key(&mut self, _key: crossterm::event::KeyCode) -> Result<bool> {
        // Not implemented yet
        Ok(false)
    }
    
    /// Handle key on Management screen
    fn handle_management_key(&mut self, _key: crossterm::event::KeyCode, _tab: ManagementTab) -> Result<bool> {
        // Not implemented yet
        Ok(false)
    }
    
    // ========================================================================
    // Legacy Locker Management Methods (Backward Compatibility)
    // ========================================================================

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
        self.status_bar.set_message(message.into(), StatusLevel::Info);
    }

    /// Clears the currently visible status message.
    pub fn clear_status(&mut self) {
        self.status_bar.message = None;
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
    
    // ========================================================================
    // v2.1 Tests: Screen Management, Escape Handling, Inactivity, Screensaver
    // ========================================================================
    
    #[test]
    fn test_screen_switching() {
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        assert_eq!(app.screen(), &AppScreen::Dashboard);
        
        app.switch_screen(AppScreen::RentalManagement(RentalTab::List));
        assert_eq!(app.screen(), &AppScreen::RentalManagement(RentalTab::List));
        
        app.switch_screen(AppScreen::Finances);
        assert_eq!(app.screen(), &AppScreen::Finances);
        
        app.switch_screen(AppScreen::Dashboard);
        assert_eq!(app.screen(), &AppScreen::Dashboard);
    }
    
    #[test]
    fn test_triple_escape_to_dashboard() {
        use crossterm::event::KeyCode;
        
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        app.switch_screen(AppScreen::RentalManagement(RentalTab::List));
        
        // Press Esc 3 times
        app.handle_key(KeyCode::Esc).expect("handle key failed");
        assert_eq!(app.status_bar.escape_count(), 1);
        
        app.handle_key(KeyCode::Esc).expect("handle key failed");
        assert_eq!(app.status_bar.escape_count(), 2);
        
        app.handle_key(KeyCode::Esc).expect("handle key failed");
        assert_eq!(app.status_bar.escape_count(), 0); // Reset after return to dashboard
        assert_eq!(app.screen(), &AppScreen::Dashboard);
    }
    
    #[test]
    fn test_escape_counter_reset_on_other_key() {
        use crossterm::event::KeyCode;
        
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        app.handle_key(KeyCode::Esc).expect("handle key failed");
        assert_eq!(app.status_bar.escape_count(), 1);
        
        app.handle_key(KeyCode::Char('a')).expect("handle key failed");
        assert_eq!(app.status_bar.escape_count(), 0);
    }
    
    #[test]
    fn test_dashboard_keys_1_to_5() {
        use crossterm::event::KeyCode;
        
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        // Key 1: Search
        app.handle_key(KeyCode::Char('1')).expect("handle key failed");
        assert_eq!(app.screen(), &AppScreen::RentalManagement(RentalTab::Search));
        
        app.switch_screen(AppScreen::Dashboard);
        
        // Key 2: List
        app.handle_key(KeyCode::Char('2')).expect("handle key failed");
        assert_eq!(app.screen(), &AppScreen::RentalManagement(RentalTab::List));
        
        app.switch_screen(AppScreen::Dashboard);
        
        // Key 3: Extend
        app.handle_key(KeyCode::Char('3')).expect("handle key failed");
        assert_eq!(app.screen(), &AppScreen::RentalManagement(RentalTab::Extend));
        
        app.switch_screen(AppScreen::Dashboard);
        
        // Key 4: Return
        app.handle_key(KeyCode::Char('4')).expect("handle key failed");
        assert_eq!(app.screen(), &AppScreen::RentalManagement(RentalTab::Return));
        
        app.switch_screen(AppScreen::Dashboard);
        
        // Key 5: Damage (NEW in v2.1)
        app.handle_key(KeyCode::Char('5')).expect("handle key failed");
        assert_eq!(app.screen(), &AppScreen::RentalManagement(RentalTab::Damage));
    }
    
    #[test]
    fn test_inactivity_detection() {
        use std::thread;
        
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        // Set very short timeout for testing
        app.screensaver_timeout = Duration::from_millis(50);
        app.countdown_duration = Duration::from_millis(50);
        
        // Initially active
        assert_eq!(app.check_inactivity(), InactivityState::Active);
        
        // Wait for timeout
        thread::sleep(Duration::from_millis(60));
        
        // Should be in countdown
        match app.check_inactivity() {
            InactivityState::Countdown(_) => {}, // Expected
            other => panic!("Expected Countdown, got {:?}", other),
        }
        
        // Wait for countdown to expire
        thread::sleep(Duration::from_millis(60));
        
        // Should be screensaver active
        assert_eq!(app.check_inactivity(), InactivityState::ScreensaverActive);
    }
    
    #[test]
    fn test_screensaver_activation() {
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        app.switch_screen(AppScreen::RentalManagement(RentalTab::List));
        
        assert!(!app.is_screensaver_active());
        
        app.activate_screensaver();
        
        assert!(app.is_screensaver_active());
        assert_eq!(app.screen(), &AppScreen::Screensaver);
        assert!(app.screensaver_screen().is_some());
    }
    
    #[test]
    fn test_screensaver_exit_returns_to_dashboard() {
        use crossterm::event::KeyCode;
        
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        app.switch_screen(AppScreen::RentalManagement(RentalTab::List));
        app.activate_screensaver();
        
        assert!(app.is_screensaver_active());
        
        // Any key exits screensaver
        app.handle_key(KeyCode::Char('a')).expect("handle key failed");
        
        assert!(!app.is_screensaver_active());
        // Should return to Dashboard, NOT previous screen
        assert_eq!(app.screen(), &AppScreen::Dashboard);
    }
    
    #[test]
    fn test_window_switcher_navigation() {
        use crossterm::event::KeyCode;
        
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        assert!(!app.header.is_window_switcher_active());
        
        // Activate window switcher with ^
        app.handle_key(KeyCode::Char('^')).expect("handle key failed");
        assert!(app.header.is_window_switcher_active());
        
        // Navigate with Tab
        app.handle_key(KeyCode::Tab).expect("handle key failed");
        
        // Confirm with Enter
        app.handle_key(KeyCode::Enter).expect("handle key failed");
        assert!(!app.header.is_window_switcher_active());
    }
    
    #[test]
    fn test_window_switcher_cancel() {
        use crossterm::event::KeyCode;
        
        let lockers = [Locker::new(1, "A-01")];
        let mut app = build_app(&lockers);
        
        let original_screen = app.screen().clone();
        
        // Activate window switcher
        app.handle_key(KeyCode::Char('^')).expect("handle key failed");
        assert!(app.header.is_window_switcher_active());
        
        // Cancel with Esc (this increments escape counter but doesn't trigger triple-escape)
        app.handle_key(KeyCode::Esc).expect("handle key failed");
        
        // Window switcher should be closed
        assert!(!app.header.is_window_switcher_active());
        // Screen should remain the same
        assert_eq!(app.screen(), &original_screen);
    }
    
    #[test]
    fn test_rental_tab_names() {
        assert_eq!(RentalTab::Search.name(), "Suche");
        assert_eq!(RentalTab::List.name(), "Liste");
        assert_eq!(RentalTab::Extend.name(), "Verlängern");
        assert_eq!(RentalTab::Return.name(), "Rückgabe");
        assert_eq!(RentalTab::Damage.name(), "Defekt");
    }
    
    #[test]
    fn test_management_tab_names() {
        assert_eq!(ManagementTab::Lockers.name(), "Schließfächer");
        assert_eq!(ManagementTab::Locations.name(), "Standorte");
        assert_eq!(ManagementTab::Settings.name(), "Einstellungen");
        assert_eq!(ManagementTab::AuditLog.name(), "Audit Log");
    }
}
