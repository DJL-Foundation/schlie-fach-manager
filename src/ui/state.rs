use std::time::{Duration, Instant};

/// UI state management for the application
#[derive(Debug, Clone)]
pub struct UiState {
    /// Whether the window switcher is currently active
    pub window_switcher_active: bool,
    
    /// Index of the selected window in the switcher
    pub selected_window_index: usize,
    
    /// Last time user activity was detected
    pub last_activity: Instant,
    
    /// Whether the screensaver is currently active
    pub screensaver_active: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            window_switcher_active: false,
            selected_window_index: 0,
            last_activity: Instant::now(),
            screensaver_active: false,
        }
    }
    
    /// Update the last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
    }
    
    /// Get the duration since the last activity
    pub fn time_since_activity(&self) -> Duration {
        self.last_activity.elapsed()
    }
    
    /// Activate the window switcher
    pub fn activate_window_switcher(&mut self) {
        self.window_switcher_active = true;
    }
    
    /// Deactivate the window switcher
    pub fn deactivate_window_switcher(&mut self) {
        self.window_switcher_active = false;
    }
    
    /// Activate the screensaver
    pub fn activate_screensaver(&mut self) {
        self.screensaver_active = true;
    }
    
    /// Deactivate the screensaver
    pub fn deactivate_screensaver(&mut self) {
        self.screensaver_active = false;
        self.update_activity();
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

/// Inactivity state for screensaver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InactivityState {
    /// User is active
    Active,
    /// Countdown to screensaver (remaining seconds)
    Countdown(u64),
    /// Screensaver is active
    ScreensaverActive,
}
