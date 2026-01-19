use crate::db::{lockers, queries, Database};
use crate::models::{DashboardStats, DebtorInfo, Locker, PaymentSummary, RentalWithLocker};
use crate::screensaver::ScreensaverScreen;
use crate::ui::screens::create_screensaver_screen;
use crate::ui::screens::finance::FinanceState;
use crate::ui::screens::management::ManagementState;
use crate::ui::screens::rental::RentalState;
use crate::ui::state::{AppScreen, ConfirmDialog, InputMode, Notification, RentalManagementTab};
use crate::ui::widgets::{StatusBarState, StatusMessage};
use color_eyre::eyre::Result;
use std::time::{Duration, Instant};

/// Inactivity state for screensaver control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InactivityState {
    /// User is active.
    Active,
    /// Countdown to screensaver (seconds remaining).
    Countdown(u64),
    /// Screensaver is active.
    ScreensaverActive,
}

/// Main application state machine.
pub struct App {
    pub db: Database,
    pub screen: AppScreen,
    pub should_quit: bool,

    // Cached data
    pub lockers: Vec<Locker>,
    pub active_rentals: Vec<RentalWithLocker>,
    pub dashboard_stats: DashboardStats,
    pub payment_summary: PaymentSummary,
    pub debtors: Vec<DebtorInfo>,
    pub locations: Vec<String>,

    // Screen-specific states
    pub rental_state: RentalState,
    pub finance_state: FinanceState,
    pub management_state: ManagementState,

    // Input state
    pub input_mode: InputMode,
    pub search_query: String,

    // UI state
    pub notification: Option<Notification>,
    pub confirm_dialog: Option<ConfirmDialog>,

    // v2.1 features
    pub status_bar: StatusBarState,
    pub window_switcher_active: bool,
    pub selected_window_index: usize,

    // Screensaver
    pub last_activity: Instant,
    pub screensaver_timeout: Duration,
    pub countdown_duration: Duration,
    pub screensaver_active: bool,
    pub screensaver_screen: Option<ScreensaverScreen>,
    pub previous_status_message: Option<StatusMessage>,
}

impl App {
    /// Creates a new application instance.
    pub fn new(db: Database) -> Result<Self> {
        let mut app = Self {
            db,
            screen: AppScreen::Dashboard,
            should_quit: false,
            lockers: Vec::new(),
            active_rentals: Vec::new(),
            dashboard_stats: DashboardStats::default(),
            payment_summary: PaymentSummary::default(),
            debtors: Vec::new(),
            locations: Vec::new(),
            rental_state: RentalState::new(),
            finance_state: FinanceState::new(),
            management_state: ManagementState::new(),
            input_mode: InputMode::Normal,
            search_query: String::new(),
            notification: None,
            confirm_dialog: None,
            // v2.1 features
            status_bar: StatusBarState::new(),
            window_switcher_active: false,
            selected_window_index: 0,
            // Screensaver
            last_activity: Instant::now(),
            screensaver_timeout: Duration::from_secs(60),
            countdown_duration: Duration::from_secs(15),
            screensaver_active: false,
            screensaver_screen: None,
            previous_status_message: None,
        };
        app.reload_data()?;
        Ok(app)
    }

    /// Reloads all data from the database.
    pub fn reload_data(&mut self) -> Result<()> {
        self.lockers = lockers::list_lockers(&self.db.conn)?;
        self.active_rentals = queries::get_active_rentals_with_lockers(&self.db.conn)?;
        self.dashboard_stats = queries::get_dashboard_stats(&self.db.conn)?;
        self.payment_summary = queries::get_payment_summary(&self.db.conn, None, None)?;
        self.debtors = queries::get_debtors(&self.db.conn)?;
        self.locations = lockers::list_distinct_locations(&self.db.conn)?;
        Ok(())
    }

    /// Switches to the given screen.
    pub fn switch_screen(&mut self, screen: AppScreen) {
        self.screen = screen;
        self.clear_search();
    }

    /// Switches to the next main screen.
    pub fn next_screen(&mut self) {
        self.screen = match &self.screen {
            AppScreen::Dashboard => {
                AppScreen::RentalManagement(RentalManagementTab::List)
            }
            AppScreen::RentalManagement(_) => {
                AppScreen::Finance(crate::ui::state::FinanceTab::Overview)
            }
            AppScreen::Finance(_) => {
                AppScreen::Management(crate::ui::state::ManagementTab::Lockers)
            }
            AppScreen::Management(_) => AppScreen::Dashboard,
            AppScreen::Screensaver => AppScreen::Dashboard,
        };
        self.clear_search();
    }

    /// Switches to the previous main screen.
    pub fn prev_screen(&mut self) {
        self.screen = match &self.screen {
            AppScreen::Dashboard => {
                AppScreen::Management(crate::ui::state::ManagementTab::Lockers)
            }
            AppScreen::RentalManagement(_) => AppScreen::Dashboard,
            AppScreen::Finance(_) => {
                AppScreen::RentalManagement(RentalManagementTab::List)
            }
            AppScreen::Management(_) => {
                AppScreen::Finance(crate::ui::state::FinanceTab::Overview)
            }
            AppScreen::Screensaver => AppScreen::Dashboard,
        };
        self.clear_search();
    }

    /// Navigates up in the current list.
    pub fn navigate_up(&mut self) {
        match &self.screen {
            AppScreen::RentalManagement(RentalManagementTab::List) => {
                if self.rental_state.list_selected > 0 {
                    self.rental_state.list_selected -= 1;
                } else if !self.lockers.is_empty() {
                    self.rental_state.list_selected = self.lockers.len() - 1;
                }
            }
            AppScreen::Finance(crate::ui::state::FinanceTab::Debtors) => {
                if self.finance_state.debtors_selected > 0 {
                    self.finance_state.debtors_selected -= 1;
                } else if !self.debtors.is_empty() {
                    self.finance_state.debtors_selected = self.debtors.len() - 1;
                }
            }
            AppScreen::Management(crate::ui::state::ManagementTab::Lockers) => {
                if self.management_state.lockers_selected > 0 {
                    self.management_state.lockers_selected -= 1;
                } else if !self.lockers.is_empty() {
                    self.management_state.lockers_selected = self.lockers.len() - 1;
                }
            }
            AppScreen::Management(crate::ui::state::ManagementTab::Locations) => {
                if self.management_state.locations_selected > 0 {
                    self.management_state.locations_selected -= 1;
                } else if !self.locations.is_empty() {
                    self.management_state.locations_selected = self.locations.len() - 1;
                }
            }
            _ => {}
        }
    }

    /// Navigates down in the current list.
    pub fn navigate_down(&mut self) {
        match &self.screen {
            AppScreen::RentalManagement(RentalManagementTab::List) => {
                if self.rental_state.list_selected < self.lockers.len().saturating_sub(1) {
                    self.rental_state.list_selected += 1;
                } else {
                    self.rental_state.list_selected = 0;
                }
            }
            AppScreen::Finance(crate::ui::state::FinanceTab::Debtors) => {
                if self.finance_state.debtors_selected < self.debtors.len().saturating_sub(1) {
                    self.finance_state.debtors_selected += 1;
                } else {
                    self.finance_state.debtors_selected = 0;
                }
            }
            AppScreen::Management(crate::ui::state::ManagementTab::Lockers) => {
                if self.management_state.lockers_selected < self.lockers.len().saturating_sub(1) {
                    self.management_state.lockers_selected += 1;
                } else {
                    self.management_state.lockers_selected = 0;
                }
            }
            AppScreen::Management(crate::ui::state::ManagementTab::Locations) => {
                if self.management_state.locations_selected < self.locations.len().saturating_sub(1)
                {
                    self.management_state.locations_selected += 1;
                } else {
                    self.management_state.locations_selected = 0;
                }
            }
            _ => {}
        }
    }

    /// Handles the Tab key to switch between sub-tabs.
    pub fn next_tab(&mut self) {
        match &self.screen {
            AppScreen::RentalManagement(_) => {
                self.rental_state.next_tab();
                self.screen = AppScreen::RentalManagement(self.rental_state.selected_tab);
            }
            AppScreen::Finance(_) => {
                self.finance_state.next_tab();
                self.screen = AppScreen::Finance(self.finance_state.selected_tab);
            }
            AppScreen::Management(_) => {
                self.management_state.next_tab();
                self.screen = AppScreen::Management(self.management_state.selected_tab);
            }
            AppScreen::Dashboard => {
                // Tab from dashboard goes to rental management
                self.next_screen();
            }
            AppScreen::Screensaver => {
                // Tab from screensaver does nothing
            }
        }
    }

    /// Clears the search query.
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.input_mode = InputMode::Normal;
    }

    /// Appends a character to the search query.
    pub fn push_search_char(&mut self, ch: char) {
        self.search_query.push(ch);
    }

    /// Removes the last character from the search query.
    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
    }

    /// Shows a notification.
    pub fn show_notification(&mut self, notification: Notification) {
        self.notification = Some(notification);
    }

    /// Shows a success notification.
    pub fn show_success(&mut self, message: impl Into<String>) {
        self.notification = Some(Notification::success(message));
    }

    /// Shows an error notification.
    pub fn show_error(&mut self, message: impl Into<String>) {
        self.notification = Some(Notification::error(message));
    }

    /// Clears expired notifications.
    pub fn clear_expired_notifications(&mut self) {
        if let Some(ref n) = self.notification {
            if n.is_expired() {
                self.notification = None;
            }
        }
    }

    /// Shows a confirmation dialog.
    pub fn show_confirm_dialog(&mut self, dialog: ConfirmDialog) {
        self.confirm_dialog = Some(dialog);
    }

    /// Clears the confirmation dialog.
    pub fn clear_confirm_dialog(&mut self) {
        self.confirm_dialog = None;
    }

    /// Marks a locker as damaged.
    pub fn mark_locker_damaged(&mut self, locker_id: i64) -> Result<()> {
        lockers::mark_locker_damaged(&self.db.conn, locker_id)?;
        self.reload_data()?;
        self.show_success("Schließfach als defekt markiert");
        Ok(())
    }

    /// Marks a locker as repaired.
    pub fn mark_locker_repaired(&mut self, locker_id: i64) -> Result<()> {
        lockers::mark_locker_repaired(&self.db.conn, locker_id)?;
        self.reload_data()?;
        self.show_success("Schließfach als repariert markiert");
        Ok(())
    }

    /// Returns the currently selected locker.
    pub fn selected_locker(&self) -> Option<&Locker> {
        match &self.screen {
            AppScreen::RentalManagement(RentalManagementTab::List) => {
                self.lockers.get(self.rental_state.list_selected)
            }
            AppScreen::Management(crate::ui::state::ManagementTab::Lockers) => {
                self.lockers.get(self.management_state.lockers_selected)
            }
            _ => None,
        }
    }

    /// Updates the last activity timestamp and exits screensaver if active.
    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
        if self.screensaver_active {
            self.exit_screensaver();
        }
    }

    /// Checks the current inactivity state.
    pub fn check_inactivity(&self) -> InactivityState {
        if self.screensaver_active {
            return InactivityState::ScreensaverActive;
        }

        let elapsed = self.last_activity.elapsed();
        let total_timeout = self.screensaver_timeout + self.countdown_duration;

        if elapsed >= total_timeout {
            InactivityState::ScreensaverActive
        } else if elapsed >= self.screensaver_timeout {
            let remaining = total_timeout.saturating_sub(elapsed);
            InactivityState::Countdown(remaining.as_secs())
        } else {
            InactivityState::Active
        }
    }

    /// Activates the screensaver, saving current state.
    pub fn activate_screensaver(&mut self) {
        if self.screensaver_active {
            return;
        }

        // Save the current status message
        self.previous_status_message = self.status_bar.to_widget().current_message().cloned();

        // Create screensaver screen
        self.screensaver_screen = Some(create_screensaver_screen());
        self.screensaver_active = true;
        self.screen = AppScreen::Screensaver;
    }

    /// Exits the screensaver and returns to Dashboard.
    pub fn exit_screensaver(&mut self) {
        if !self.screensaver_active {
            return;
        }

        self.screensaver_active = false;
        self.screensaver_screen = None;
        self.screen = AppScreen::Dashboard;
        self.last_activity = Instant::now();

        // Restore previous status message or show welcome back message
        if let Some(msg) = self.previous_status_message.take() {
            self.status_bar.set_message(msg.text, msg.level);
        } else {
            self.status_bar.info("Willkommen zurück!");
        }
    }

    /// Returns the current screen index for window switcher.
    pub fn current_screen_index(&self) -> usize {
        match &self.screen {
            AppScreen::Dashboard => 0,
            AppScreen::RentalManagement(_) => 1,
            AppScreen::Finance(_) => 2,
            AppScreen::Management(_) => 3,
            AppScreen::Screensaver => 0,
        }
    }

    /// Switches to screen by index (for window switcher).
    pub fn switch_to_screen_by_index(&mut self, index: usize) {
        let screen = match index {
            0 => AppScreen::Dashboard,
            1 => AppScreen::RentalManagement(RentalManagementTab::List),
            2 => AppScreen::Finance(crate::ui::state::FinanceTab::Overview),
            3 => AppScreen::Management(crate::ui::state::ManagementTab::Lockers),
            _ => AppScreen::Dashboard,
        };
        self.switch_screen(screen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::seed::seed_test_data;

    fn setup_app() -> App {
        let db = Database::open_in_memory().expect("in-memory db");
        seed_test_data(&db.conn).expect("seed data");
        App::new(db).expect("app init")
    }

    #[test]
    fn test_app_creation() {
        let app = setup_app();
        assert!(!app.lockers.is_empty());
        assert!(app.dashboard_stats.total_lockers > 0);
    }

    #[test]
    fn test_screen_navigation() {
        let mut app = setup_app();
        assert_eq!(app.screen, AppScreen::Dashboard);

        app.next_screen();
        assert!(matches!(app.screen, AppScreen::RentalManagement(_)));

        app.next_screen();
        assert!(matches!(app.screen, AppScreen::Finance(_)));

        app.next_screen();
        assert!(matches!(app.screen, AppScreen::Management(_)));

        app.next_screen();
        assert_eq!(app.screen, AppScreen::Dashboard);
    }

    #[test]
    fn test_list_navigation() {
        let mut app = setup_app();
        app.switch_screen(AppScreen::RentalManagement(RentalManagementTab::List));

        assert_eq!(app.rental_state.list_selected, 0);

        app.navigate_down();
        assert_eq!(app.rental_state.list_selected, 1);

        app.navigate_up();
        assert_eq!(app.rental_state.list_selected, 0);
    }
}
