//! Status Bar widget for displaying status messages and escape indicator.
//!
//! The status bar shows status messages on the left (90%) and an escape
//! indicator [|||] on the right (10%) that tracks the 3x Escape to Dashboard feature.

use std::time::{Duration, Instant};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::ui::theme::Theme;

/// Status message severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    /// Informational message.
    Info,
    /// Success message.
    Success,
    /// Warning message.
    Warning,
    /// Error message.
    Error,
}

/// A status message with timestamp and level.
#[derive(Debug, Clone)]
pub struct StatusMessage {
    /// The message text.
    pub text: String,
    /// Severity level of the message.
    pub level: StatusLevel,
    /// When the message was created.
    pub timestamp: Instant,
}

impl StatusMessage {
    /// Creates a new status message.
    pub fn new(text: impl Into<String>, level: StatusLevel) -> Self {
        Self {
            text: text.into(),
            level,
            timestamp: Instant::now(),
        }
    }

    /// Returns the style for this message's level.
    pub fn style(&self) -> Style {
        match self.level {
            StatusLevel::Info => Theme::status_info(),
            StatusLevel::Success => Theme::status_success(),
            StatusLevel::Warning => Theme::status_warning(),
            StatusLevel::Error => Theme::status_error(),
        }
    }

    /// Checks if the message has expired (after 5 seconds by default).
    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed() >= Duration::from_secs(5)
    }
}

/// The Status Bar widget displays messages and the escape indicator.
///
/// Left side (90%): Status messages with level-based coloring.
/// Right side (10%): Escape indicator [|||] showing progress toward Dashboard.
#[derive(Debug, Clone, Default)]
pub struct StatusBar {
    /// Current status message (if any).
    message: Option<StatusMessage>,
    /// Number of consecutive escape presses (0-3).
    escape_count: u8,
    /// Time of the last escape press.
    last_escape_time: Option<Instant>,
}

impl StatusBar {
    /// Creates a new StatusBar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a status message.
    pub fn set_message(&mut self, text: impl Into<String>, level: StatusLevel) {
        self.message = Some(StatusMessage::new(text, level));
    }

    /// Sets an info message.
    pub fn info(&mut self, text: impl Into<String>) {
        self.set_message(text, StatusLevel::Info);
    }

    /// Sets a success message.
    pub fn success(&mut self, text: impl Into<String>) {
        self.set_message(text, StatusLevel::Success);
    }

    /// Sets a warning message.
    pub fn warning(&mut self, text: impl Into<String>) {
        self.set_message(text, StatusLevel::Warning);
    }

    /// Sets an error message.
    pub fn error(&mut self, text: impl Into<String>) {
        self.set_message(text, StatusLevel::Error);
    }

    /// Clears the current message.
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Clears expired messages (called on each tick).
    pub fn clear_expired_messages(&mut self) {
        if let Some(ref msg) = self.message {
            if msg.is_expired() {
                self.message = None;
            }
        }
    }

    /// Increments the escape counter when Esc is pressed.
    ///
    /// Resets the counter if more than 1 second has passed since the last Esc.
    pub fn increment_escape_count(&mut self) {
        let now = Instant::now();

        // Reset if more than 1 second since last escape
        if let Some(last_time) = self.last_escape_time {
            if now.duration_since(last_time) > Duration::from_secs(1) {
                self.escape_count = 0;
            }
        }

        self.escape_count = (self.escape_count + 1).min(3);
        self.last_escape_time = Some(now);
    }

    /// Resets the escape counter.
    pub fn reset_escape_count(&mut self) {
        self.escape_count = 0;
        self.last_escape_time = None;
    }

    /// Checks if the user should be returned to the Dashboard (3x Esc).
    pub fn should_return_to_dashboard(&self) -> bool {
        self.escape_count >= 3
    }

    /// Returns the current escape count.
    pub fn escape_count(&self) -> u8 {
        self.escape_count
    }

    /// Checks if the escape counter should be auto-reset due to timeout.
    ///
    /// Call this on each tick to handle the 1-second reset window.
    pub fn check_escape_timeout(&mut self) {
        if let Some(last_time) = self.last_escape_time {
            if last_time.elapsed() > Duration::from_secs(1) && self.escape_count > 0 {
                self.reset_escape_count();
            }
        }
    }

    /// Shows a screensaver countdown message.
    pub fn show_screensaver_countdown(&mut self, seconds: u64) {
        self.set_message(
            format!("⏱ Bildschirmschoner in {} Sekunden...", seconds),
            StatusLevel::Info,
        );
    }

    /// Returns the current message (if any and not expired).
    pub fn current_message(&self) -> Option<&StatusMessage> {
        self.message.as_ref().filter(|m| !m.is_expired())
    }

    /// Renders the escape indicator [|||].
    fn render_escape_indicator(&self, area: Rect, buf: &mut Buffer) {
        let mut spans: Vec<Span> = vec![Span::raw("[")];

        // Render 3 pipes with color based on escape_count
        for i in 0..3 {
            let style = if i < self.escape_count as usize {
                Theme::escape_indicator_active()
            } else {
                Theme::escape_indicator_inactive()
            };
            spans.push(Span::styled("|", style));
        }

        spans.push(Span::raw("]"));

        let indicator = Paragraph::new(Line::from(spans)).alignment(Alignment::Right);
        indicator.render(area, buf);
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split area: left for message (90%), right for escape indicator (10%)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);

        // Render message on the left
        if let Some(msg) = self.current_message() {
            let text = Paragraph::new(Line::from(Span::styled(&msg.text, msg.style())))
                .alignment(Alignment::Left);
            text.render(chunks[0], buf);
        }

        // Render escape indicator on the right
        self.render_escape_indicator(chunks[1], buf);
    }
}

/// Stateful version of StatusBar for use in App state.
#[derive(Debug, Clone, Default)]
pub struct StatusBarState {
    /// The underlying status bar.
    inner: StatusBar,
}

impl StatusBarState {
    /// Creates a new StatusBarState.
    pub fn new() -> Self {
        Self {
            inner: StatusBar::new(),
        }
    }

    /// Sets a status message.
    pub fn set_message(&mut self, text: impl Into<String>, level: StatusLevel) {
        self.inner.set_message(text, level);
    }

    /// Sets an info message.
    pub fn info(&mut self, text: impl Into<String>) {
        self.inner.info(text);
    }

    /// Sets a success message.
    pub fn success(&mut self, text: impl Into<String>) {
        self.inner.success(text);
    }

    /// Sets a warning message.
    pub fn warning(&mut self, text: impl Into<String>) {
        self.inner.warning(text);
    }

    /// Sets an error message.
    pub fn error(&mut self, text: impl Into<String>) {
        self.inner.error(text);
    }

    /// Clears the current message.
    pub fn clear_message(&mut self) {
        self.inner.clear_message();
    }

    /// Clears expired messages.
    pub fn clear_expired_messages(&mut self) {
        self.inner.clear_expired_messages();
    }

    /// Increments the escape counter.
    pub fn increment_escape_count(&mut self) {
        self.inner.increment_escape_count();
    }

    /// Resets the escape counter.
    pub fn reset_escape_count(&mut self) {
        self.inner.reset_escape_count();
    }

    /// Checks if should return to dashboard.
    pub fn should_return_to_dashboard(&self) -> bool {
        self.inner.should_return_to_dashboard()
    }

    /// Returns the current escape count.
    pub fn escape_count(&self) -> u8 {
        self.inner.escape_count()
    }

    /// Checks and handles escape timeout.
    pub fn check_escape_timeout(&mut self) {
        self.inner.check_escape_timeout();
    }

    /// Shows screensaver countdown.
    pub fn show_screensaver_countdown(&mut self, seconds: u64) {
        self.inner.show_screensaver_countdown(seconds);
    }

    /// Creates a StatusBar widget from this state.
    pub fn to_widget(&self) -> StatusBar {
        self.inner.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_status_bar_default() {
        let bar = StatusBar::new();
        assert!(bar.message.is_none());
        assert_eq!(bar.escape_count, 0);
        assert!(bar.last_escape_time.is_none());
    }

    #[test]
    fn test_status_message() {
        let mut bar = StatusBar::new();
        bar.set_message("Test message", StatusLevel::Info);

        assert!(bar.message.is_some());
        let msg = bar.message.as_ref().unwrap();
        assert_eq!(msg.text, "Test message");
        assert_eq!(msg.level, StatusLevel::Info);
        assert!(!msg.is_expired());
    }

    #[test]
    fn test_escape_counter() {
        let mut bar = StatusBar::new();
        assert_eq!(bar.escape_count(), 0);
        assert!(!bar.should_return_to_dashboard());

        bar.increment_escape_count();
        assert_eq!(bar.escape_count(), 1);
        assert!(!bar.should_return_to_dashboard());

        bar.increment_escape_count();
        assert_eq!(bar.escape_count(), 2);
        assert!(!bar.should_return_to_dashboard());

        bar.increment_escape_count();
        assert_eq!(bar.escape_count(), 3);
        assert!(bar.should_return_to_dashboard());

        // Should not exceed 3
        bar.increment_escape_count();
        assert_eq!(bar.escape_count(), 3);
    }

    #[test]
    fn test_escape_counter_reset() {
        let mut bar = StatusBar::new();
        bar.increment_escape_count();
        bar.increment_escape_count();
        assert_eq!(bar.escape_count(), 2);

        bar.reset_escape_count();
        assert_eq!(bar.escape_count(), 0);
        assert!(bar.last_escape_time.is_none());
    }

    #[test]
    fn test_escape_counter_timeout() {
        let mut bar = StatusBar::new();
        bar.increment_escape_count();
        assert_eq!(bar.escape_count(), 1);

        // Simulate timeout (wait slightly over 1 second)
        thread::sleep(Duration::from_millis(1100));

        // Next increment should reset first due to timeout
        bar.increment_escape_count();
        assert_eq!(bar.escape_count(), 1); // Reset to 0, then incremented to 1
    }

    #[test]
    fn test_status_levels() {
        let mut bar = StatusBar::new();

        bar.info("Info");
        assert_eq!(bar.message.as_ref().unwrap().level, StatusLevel::Info);

        bar.success("Success");
        assert_eq!(bar.message.as_ref().unwrap().level, StatusLevel::Success);

        bar.warning("Warning");
        assert_eq!(bar.message.as_ref().unwrap().level, StatusLevel::Warning);

        bar.error("Error");
        assert_eq!(bar.message.as_ref().unwrap().level, StatusLevel::Error);
    }

    #[test]
    fn test_status_bar_state() {
        let mut state = StatusBarState::new();

        state.info("Test");
        assert_eq!(state.escape_count(), 0);

        state.increment_escape_count();
        state.increment_escape_count();
        state.increment_escape_count();
        assert!(state.should_return_to_dashboard());

        state.reset_escape_count();
        assert!(!state.should_return_to_dashboard());
    }
}
