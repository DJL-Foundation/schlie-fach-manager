use ratatui::style::{Color, Modifier, Style};

/// Theme colors and styles for the application.
pub struct Theme;

impl Theme {
    // Base colors
    pub const PRIMARY: Color = Color::Cyan;
    pub const SECONDARY: Color = Color::Yellow;
    pub const SUCCESS: Color = Color::Green;
    pub const WARNING: Color = Color::Yellow;
    pub const ERROR: Color = Color::Red;
    pub const INFO: Color = Color::Cyan;
    pub const TEXT: Color = Color::White;
    pub const TEXT_DIM: Color = Color::Gray;
    pub const TEXT_DARK: Color = Color::DarkGray;
    pub const BG_HIGHLIGHT: Color = Color::DarkGray;
    pub const BG_SELECTED: Color = Color::Rgb(40, 40, 60);

    // v2.1 Keybind bar colors
    pub const KEYBIND_GLOBAL: Color = Color::Yellow;
    pub const KEYBIND_CONTEXT: Color = Color::Rgb(180, 180, 180);

    // v2.1 Window switcher colors
    pub const WINDOW_CURRENT: Color = Color::Cyan;
    pub const WINDOW_ADJACENT: Color = Color::DarkGray;

    // v2.1 Escape indicator colors
    pub const ESCAPE_INDICATOR_ACTIVE: Color = Color::Yellow;
    pub const ESCAPE_INDICATOR_INACTIVE: Color = Color::DarkGray;

    // v2.1 Screensaver colors
    pub const SCREENSAVER_BG: Color = Color::Black;
    pub const SCREENSAVER_FG: Color = Color::Green;

    // Status colors
    pub const FREE: Color = Color::Green;
    pub const OCCUPIED: Color = Color::Yellow;
    pub const DAMAGED: Color = Color::Red;
    pub const OVERDUE: Color = Color::Red;
    pub const EXPIRING: Color = Color::Yellow;

    // Styles
    pub fn title() -> Style {
        Style::default()
            .fg(Self::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header() -> Style {
        Style::default()
            .fg(Self::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn normal() -> Style {
        Style::default().fg(Self::TEXT)
    }

    pub fn dim() -> Style {
        Style::default().fg(Self::TEXT_DIM)
    }

    pub fn highlight() -> Style {
        Style::default()
            .bg(Self::BG_HIGHLIGHT)
            .fg(Self::TEXT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected() -> Style {
        Style::default()
            .bg(Self::BG_SELECTED)
            .fg(Self::TEXT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default().fg(Self::SUCCESS)
    }

    pub fn warning() -> Style {
        Style::default().fg(Self::WARNING)
    }

    pub fn error() -> Style {
        Style::default().fg(Self::ERROR)
    }

    pub fn tab_active() -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(Self::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn tab_inactive() -> Style {
        Style::default().fg(Self::TEXT_DIM)
    }

    pub fn button() -> Style {
        Style::default().fg(Self::TEXT).add_modifier(Modifier::BOLD)
    }

    pub fn button_selected() -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(Self::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn input() -> Style {
        Style::default()
            .fg(Self::TEXT)
            .add_modifier(Modifier::UNDERLINED)
    }

    pub fn input_focused() -> Style {
        Style::default()
            .fg(Self::PRIMARY)
            .add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
    }

    pub fn status_free() -> Style {
        Style::default().fg(Self::FREE).add_modifier(Modifier::BOLD)
    }

    pub fn status_occupied() -> Style {
        Style::default()
            .fg(Self::OCCUPIED)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_damaged() -> Style {
        Style::default()
            .fg(Self::DAMAGED)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_overdue() -> Style {
        Style::default()
            .fg(Self::OVERDUE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_expiring() -> Style {
        Style::default()
            .fg(Self::EXPIRING)
            .add_modifier(Modifier::BOLD)
    }

    /// Returns the status color for a locker.
    pub fn locker_status_style(is_occupied: bool, is_damaged: bool, is_overdue: bool) -> Style {
        if is_damaged {
            Self::status_damaged()
        } else if is_overdue {
            Self::status_overdue()
        } else if is_occupied {
            Self::status_occupied()
        } else {
            Self::status_free()
        }
    }

    // v2.1 Status bar styles
    /// Style for info status messages.
    pub fn status_info() -> Style {
        Style::default().fg(Self::INFO)
    }

    /// Style for success status messages.
    pub fn status_success() -> Style {
        Style::default()
            .fg(Self::SUCCESS)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for warning status messages.
    pub fn status_warning() -> Style {
        Style::default()
            .fg(Self::WARNING)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for error status messages.
    pub fn status_error() -> Style {
        Style::default()
            .fg(Self::ERROR)
            .add_modifier(Modifier::BOLD)
    }

    // v2.1 Keybind bar styles
    /// Style for global keybinds (accent color).
    pub fn keybind_global() -> Style {
        Style::default()
            .fg(Self::KEYBIND_GLOBAL)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for context-specific keybinds (neutral color).
    pub fn keybind_context() -> Style {
        Style::default().fg(Self::KEYBIND_CONTEXT)
    }

    // v2.1 Window switcher styles
    /// Style for the current window in the switcher.
    pub fn window_current() -> Style {
        Style::default()
            .fg(Self::WINDOW_CURRENT)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for adjacent windows in the switcher (dimmed).
    pub fn window_adjacent() -> Style {
        Style::default().fg(Self::WINDOW_ADJACENT)
    }

    // v2.1 Escape indicator styles
    /// Style for active escape indicator pipes.
    pub fn escape_indicator_active() -> Style {
        Style::default()
            .fg(Self::ESCAPE_INDICATOR_ACTIVE)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for inactive escape indicator pipes.
    pub fn escape_indicator_inactive() -> Style {
        Style::default().fg(Self::ESCAPE_INDICATOR_INACTIVE)
    }

    // v2.1 Screensaver styles
    /// Style for screensaver background.
    pub fn screensaver_bg() -> Style {
        Style::default().bg(Self::SCREENSAVER_BG)
    }

    /// Style for screensaver foreground (animation text).
    pub fn screensaver_fg() -> Style {
        Style::default().fg(Self::SCREENSAVER_FG)
    }

    /// Style for primary elements (accent color).
    pub fn primary_style() -> Style {
        Style::default()
            .fg(Self::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for info elements.
    pub fn info_style() -> Style {
        Style::default().fg(Self::INFO)
    }
}
