//! Keybind Bar widget for displaying keyboard shortcuts.
//!
//! Displays global keybinds on line 1 (accent color) and
//! context-specific keybinds on line 2+ (neutral color).

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::ui::theme::Theme;

/// Scope of a keybind (determines its color).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeybindScope {
    /// Global keybinds shown in accent color.
    Global,
    /// Context-specific keybinds shown in neutral color.
    Context,
}

/// A single keybind definition.
#[derive(Debug, Clone)]
pub struct Keybind {
    /// The key or key combination (e.g., "Tab", "^", "1").
    pub key: String,
    /// Description of what the key does.
    pub description: String,
    /// Whether this is a global or context-specific keybind.
    pub scope: KeybindScope,
}

impl Keybind {
    /// Creates a new keybind.
    pub fn new(
        key: impl Into<String>,
        description: impl Into<String>,
        scope: KeybindScope,
    ) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
            scope,
        }
    }

    /// Creates a global keybind.
    pub fn global(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(key, description, KeybindScope::Global)
    }

    /// Creates a context-specific keybind.
    pub fn context(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(key, description, KeybindScope::Context)
    }
}

/// The Keybind Bar widget displays available keyboard shortcuts.
///
/// Line 1: Global keybinds (Tab, Shift+Tab, ^, Shift+Q, Esc, Enter).
/// Line 2+: Context-specific keybinds that change per screen/mode.
#[derive(Debug, Clone, Default)]
pub struct KeybindBar {
    /// Global keybinds (always shown on line 1).
    global_binds: Vec<Keybind>,
    /// Context-specific keybinds (shown on line 2+).
    context_binds: Vec<Keybind>,
    /// Optional context message (e.g., "--- Wähle Fenster aus ---").
    context_message: Option<String>,
}

impl KeybindBar {
    /// Creates a new KeybindBar with default global keybinds.
    pub fn new() -> Self {
        Self {
            global_binds: Self::default_global_binds(),
            context_binds: Vec::new(),
            context_message: None,
        }
    }

    /// Returns the default global keybinds per v2.1 spec.
    fn default_global_binds() -> Vec<Keybind> {
        vec![
            Keybind::global("Tab", "Weiter"),
            Keybind::global("S-Tab", "Zurück"),
            Keybind::global("^", "Fenster"),
            Keybind::global("S-Q", "Beenden"),
            Keybind::global("Esc", "Abbrechen"),
            Keybind::global("Enter", "Bestätigen"),
        ]
    }

    /// Sets the global keybinds.
    pub fn global_binds(mut self, binds: Vec<Keybind>) -> Self {
        self.global_binds = binds;
        self
    }

    /// Sets the context-specific keybinds.
    pub fn context_binds(mut self, binds: Vec<Keybind>) -> Self {
        self.context_binds = binds;
        self
    }

    /// Sets a context message to display instead of context binds.
    pub fn context_message(mut self, message: impl Into<String>) -> Self {
        self.context_message = Some(message.into());
        self
    }

    /// Clears the context message.
    pub fn clear_context_message(mut self) -> Self {
        self.context_message = None;
        self
    }

    /// Formats keybinds into a Line with the specified style.
    fn format_keybinds(binds: &[Keybind], style: Style) -> Line<'static> {
        let mut spans: Vec<Span<'static>> = Vec::new();

        for (i, bind) in binds.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  "));
            }
            spans.push(Span::styled(format!("[{}]", bind.key), style));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(bind.description.to_string(), Theme::dim()));
        }

        Line::from(spans)
    }

    /// Returns keybinds for the Dashboard context.
    pub fn dashboard_context() -> Vec<Keybind> {
        vec![
            Keybind::context("1", "Suchen"),
            Keybind::context("2", "Liste"),
            Keybind::context("3", "Verlängern"),
            Keybind::context("4", "Rückgabe"),
            Keybind::context("5", "Defekt"),
        ]
    }

    /// Returns keybinds for the Window Switcher context.
    pub fn window_switcher_context() -> Vec<Keybind> {
        vec![
            Keybind::context("Tab", "Vor"),
            Keybind::context("S-Tab", "Zurück"),
            Keybind::context("Enter", "Öffnen"),
            Keybind::context("Esc", "Abbrechen"),
        ]
    }

    /// Returns keybinds for list navigation context.
    pub fn list_context() -> Vec<Keybind> {
        vec![
            Keybind::context("↑↓", "Navigation"),
            Keybind::context("Enter", "Auswählen"),
            Keybind::context("/", "Suchen"),
        ]
    }

    /// Returns keybinds for wizard/dialog context.
    pub fn wizard_context() -> Vec<Keybind> {
        vec![
            Keybind::context("↑↓", "Auswahl"),
            Keybind::context("Enter", "Bestätigen"),
            Keybind::context("Space", "Multi-Select"),
            Keybind::context("Esc", "Abbrechen"),
        ]
    }
}

impl Widget for KeybindBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split into 2 lines
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(area);

        // Line 1: Global keybinds in accent color
        let global_line = Self::format_keybinds(&self.global_binds, Theme::keybind_global());
        let global_paragraph = Paragraph::new(global_line).alignment(Alignment::Center);
        global_paragraph.render(chunks[0], buf);

        // Line 2: Context message or context keybinds
        if let Some(message) = &self.context_message {
            let message_line = Line::from(Span::styled(
                format!("--- {} ---", message),
                Theme::keybind_context(),
            ));
            let message_paragraph = Paragraph::new(message_line).alignment(Alignment::Center);
            message_paragraph.render(chunks[1], buf);
        } else {
            let context_line = Self::format_keybinds(&self.context_binds, Theme::keybind_context());
            let context_paragraph = Paragraph::new(context_line).alignment(Alignment::Center);
            context_paragraph.render(chunks[1], buf);
        }
    }
}

/// Helper struct for managing keybind bar state.
#[derive(Debug, Clone, Default)]
pub struct KeybindBarState {
    /// Current context keybinds.
    pub context_binds: Vec<Keybind>,
    /// Current context message (if any).
    pub context_message: Option<String>,
}

impl KeybindBarState {
    /// Creates a new KeybindBarState.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets context binds for the Dashboard.
    pub fn set_dashboard_context(&mut self) {
        self.context_binds = KeybindBar::dashboard_context();
        self.context_message = None;
    }

    /// Sets context for window switcher mode.
    pub fn set_window_switcher_context(&mut self) {
        self.context_binds = KeybindBar::window_switcher_context();
        self.context_message = Some("Wähle Fenster aus".to_string());
    }

    /// Sets context for list navigation.
    pub fn set_list_context(&mut self) {
        self.context_binds = KeybindBar::list_context();
        self.context_message = None;
    }

    /// Sets context for wizard/dialog mode.
    pub fn set_wizard_context(&mut self) {
        self.context_binds = KeybindBar::wizard_context();
        self.context_message = None;
    }

    /// Sets custom context binds.
    pub fn set_custom_context(&mut self, binds: Vec<Keybind>, message: Option<String>) {
        self.context_binds = binds;
        self.context_message = message;
    }

    /// Creates a KeybindBar widget from this state.
    pub fn to_widget(&self) -> KeybindBar {
        let mut bar = KeybindBar::new().context_binds(self.context_binds.clone());
        if let Some(msg) = &self.context_message {
            bar = bar.context_message(msg.clone());
        }
        bar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keybind_creation() {
        let kb = Keybind::global("Tab", "Next");
        assert_eq!(kb.key, "Tab");
        assert_eq!(kb.description, "Next");
        assert_eq!(kb.scope, KeybindScope::Global);

        let kb2 = Keybind::context("1", "Search");
        assert_eq!(kb2.scope, KeybindScope::Context);
    }

    #[test]
    fn test_keybind_bar_default() {
        let bar = KeybindBar::new();
        assert_eq!(bar.global_binds.len(), 6); // Tab, S-Tab, ^, S-Q, Esc, Enter
        assert!(bar.context_binds.is_empty());
        assert!(bar.context_message.is_none());
    }

    #[test]
    fn test_keybind_bar_builder() {
        let bar = KeybindBar::new()
            .context_binds(KeybindBar::dashboard_context())
            .context_message("Test message");

        assert_eq!(bar.context_binds.len(), 5);
        assert_eq!(bar.context_message, Some("Test message".to_string()));
    }

    #[test]
    fn test_keybind_bar_state() {
        let mut state = KeybindBarState::new();
        assert!(state.context_binds.is_empty());

        state.set_dashboard_context();
        assert_eq!(state.context_binds.len(), 5);
        assert!(state.context_message.is_none());

        state.set_window_switcher_context();
        assert!(state.context_message.is_some());
    }

    #[test]
    fn test_dashboard_context() {
        let binds = KeybindBar::dashboard_context();
        assert_eq!(binds.len(), 5);
        assert_eq!(binds[0].key, "1");
        assert_eq!(binds[4].key, "5");
    }
}
