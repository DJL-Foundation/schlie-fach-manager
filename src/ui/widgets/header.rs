//! Header widget for the application.
//!
//! Displays the application name and version on the left (1/3),
//! and the current screen name or window switcher on the right (2/3).

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::ui::theme::Theme;

/// The Header widget displays navigation information at the top of the screen.
///
/// In normal mode, it shows the app name/version and current screen name.
/// When the window switcher is active, it shows prev/current/next screens.
#[derive(Debug, Clone)]
pub struct Header {
    /// Application name (default: "Schließfach-Manager").
    app_name: String,
    /// Application version string.
    version: String,
    /// Current screen name.
    current_screen: String,
    /// Whether the window switcher is currently active.
    window_switcher_active: bool,
    /// List of all available windows/screens.
    window_list: Vec<String>,
    /// Index of the currently selected window in the window list.
    selected_window_index: usize,
}

/// Default application name.
const DEFAULT_APP_NAME: &str = "Schließfach-Manager";

impl Default for Header {
    fn default() -> Self {
        Self {
            app_name: DEFAULT_APP_NAME.to_string(),
            version: "2.1.0".to_string(),
            current_screen: "Dashboard".to_string(),
            window_switcher_active: false,
            window_list: vec![
                "Dashboard".to_string(),
                "Verleih-Management".to_string(),
                "Finanzen".to_string(),
                "Verwaltung".to_string(),
            ],
            selected_window_index: 0,
        }
    }
}

impl Header {
    /// Creates a new Header with the given version.
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            ..Default::default()
        }
    }

    /// Sets the application name.
    pub fn app_name(mut self, name: impl Into<String>) -> Self {
        self.app_name = name.into();
        self
    }

    /// Sets the current screen name.
    pub fn current_screen(mut self, screen: impl Into<String>) -> Self {
        self.current_screen = screen.into();
        self
    }

    /// Activates or deactivates the window switcher mode.
    pub fn window_switcher_active(mut self, active: bool) -> Self {
        self.window_switcher_active = active;
        self
    }

    /// Sets the list of available windows.
    pub fn window_list(mut self, list: Vec<String>) -> Self {
        self.window_list = list;
        self
    }

    /// Sets the currently selected window index.
    pub fn selected_window_index(mut self, index: usize) -> Self {
        self.selected_window_index = index;
        self
    }

    /// Returns the previous window name (wraps around).
    fn prev_window(&self) -> &str {
        if self.window_list.is_empty() {
            return "";
        }
        let idx = if self.selected_window_index == 0 {
            self.window_list.len() - 1
        } else {
            self.selected_window_index - 1
        };
        &self.window_list[idx]
    }

    /// Returns the current window name.
    fn current_window(&self) -> &str {
        if self.window_list.is_empty() {
            return "";
        }
        &self.window_list[self.selected_window_index.min(self.window_list.len() - 1)]
    }

    /// Returns the next window name (wraps around).
    fn next_window(&self) -> &str {
        if self.window_list.is_empty() {
            return "";
        }
        let idx = (self.selected_window_index + 1) % self.window_list.len();
        &self.window_list[idx]
    }

    /// Renders the simple screen name (when window switcher is not active).
    fn render_simple_screen_name(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("[{}]", self.current_screen);
        let paragraph = Paragraph::new(Line::from(Span::styled(
            text,
            Style::default()
                .fg(Theme::PRIMARY)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(ratatui::layout::Alignment::Right);
        paragraph.render(area, buf);
    }

    /// Renders the window switcher (prev | [current] | next).
    fn render_window_switcher(&self, area: Rect, buf: &mut Buffer) {
        // Split into 3 equal parts
        let thirds = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area);

        // Previous window (dimmed)
        let prev_text = self.prev_window();
        let prev_paragraph = Paragraph::new(Line::from(Span::styled(
            prev_text,
            Theme::window_adjacent(),
        )))
        .alignment(ratatui::layout::Alignment::Center);
        prev_paragraph.render(thirds[0], buf);

        // Current window (highlighted with box)
        let current_text = format!("[{}]", self.current_window());
        let current_paragraph = Paragraph::new(Line::from(Span::styled(
            current_text,
            Theme::window_current(),
        )))
        .alignment(ratatui::layout::Alignment::Center);
        current_paragraph.render(thirds[1], buf);

        // Next window (dimmed)
        let next_text = self.next_window();
        let next_paragraph = Paragraph::new(Line::from(Span::styled(
            next_text,
            Theme::window_adjacent(),
        )))
        .alignment(ratatui::layout::Alignment::Center);
        next_paragraph.render(thirds[2], buf);
    }
}

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear the area with header background
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Theme::TEXT_DIM));
        let inner = block.inner(area);
        block.render(area, buf);

        // Split into left (1/3) and right (2/3)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
            .split(inner);

        // Left side: App name + version
        let left_text = format!("{} v{}", self.app_name, self.version);
        let left_paragraph = Paragraph::new(Line::from(Span::styled(
            left_text,
            Style::default()
                .fg(Theme::TEXT)
                .add_modifier(Modifier::BOLD),
        )));
        left_paragraph.render(chunks[0], buf);

        // Right side: Window switcher or simple screen name
        if self.window_switcher_active {
            self.render_window_switcher(chunks[1], buf);
        } else {
            self.render_simple_screen_name(chunks[1], buf);
        }
    }
}

/// Helper struct to manage window switcher state.
#[derive(Debug, Clone, Default)]
pub struct WindowSwitcherState {
    /// Whether the window switcher is currently active.
    pub active: bool,
    /// Index of the currently selected window.
    pub selected_index: usize,
    /// List of available window names.
    pub window_names: Vec<String>,
}

impl WindowSwitcherState {
    /// Creates a new WindowSwitcherState with default windows.
    pub fn new() -> Self {
        Self {
            active: false,
            selected_index: 0,
            window_names: vec![
                "Dashboard".to_string(),
                "Verleih-Management".to_string(),
                "Finanzen".to_string(),
                "Verwaltung".to_string(),
            ],
        }
    }

    /// Activates the window switcher at the given screen index.
    pub fn activate(&mut self, current_screen_index: usize) {
        self.active = true;
        self.selected_index = current_screen_index.min(self.window_names.len().saturating_sub(1));
    }

    /// Deactivates the window switcher.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Selects the next window (wraps around).
    pub fn select_next(&mut self) {
        if !self.window_names.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.window_names.len();
        }
    }

    /// Selects the previous window (wraps around).
    pub fn select_prev(&mut self) {
        if !self.window_names.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.window_names.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    /// Returns the currently selected window name.
    pub fn selected_window(&self) -> Option<&str> {
        self.window_names
            .get(self.selected_index)
            .map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_default() {
        let header = Header::default();
        assert_eq!(header.version, "2.1.0");
        assert_eq!(header.current_screen, "Dashboard");
        assert!(!header.window_switcher_active);
        assert_eq!(header.window_list.len(), 4);
    }

    #[test]
    fn test_header_builder() {
        let header = Header::new("2.1.0")
            .current_screen("Finanzen")
            .window_switcher_active(true)
            .selected_window_index(2);

        assert_eq!(header.version, "2.1.0");
        assert_eq!(header.current_screen, "Finanzen");
        assert!(header.window_switcher_active);
        assert_eq!(header.selected_window_index, 2);
    }

    #[test]
    fn test_window_navigation() {
        let header = Header::default().selected_window_index(0);
        assert_eq!(header.current_window(), "Dashboard");
        assert_eq!(header.next_window(), "Verleih-Management");
        assert_eq!(header.prev_window(), "Verwaltung"); // wraps around
    }

    #[test]
    fn test_window_switcher_state() {
        let mut state = WindowSwitcherState::new();
        assert!(!state.active);
        assert_eq!(state.selected_index, 0);

        state.activate(1);
        assert!(state.active);
        assert_eq!(state.selected_index, 1);

        state.select_next();
        assert_eq!(state.selected_index, 2);

        state.select_prev();
        assert_eq!(state.selected_index, 1);

        // Test wrap around
        state.selected_index = 0;
        state.select_prev();
        assert_eq!(state.selected_index, 3);

        state.select_next();
        assert_eq!(state.selected_index, 0);
    }
}
