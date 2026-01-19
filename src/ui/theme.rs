use ratatui::style::{Color, Modifier, Style};

/// Theme colors and styles for the application.
pub struct Theme;

impl Theme {
    // Colors
    pub const PRIMARY: Color = Color::Cyan;
    pub const SECONDARY: Color = Color::Yellow;
    pub const SUCCESS: Color = Color::Green;
    pub const WARNING: Color = Color::Yellow;
    pub const ERROR: Color = Color::Red;
    pub const TEXT: Color = Color::White;
    pub const TEXT_DIM: Color = Color::Gray;
    pub const TEXT_DARK: Color = Color::DarkGray;
    pub const BG_HIGHLIGHT: Color = Color::DarkGray;
    pub const BG_SELECTED: Color = Color::Rgb(40, 40, 60);

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
        Style::default()
            .fg(Self::TEXT)
            .add_modifier(Modifier::BOLD)
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
        Style::default()
            .fg(Self::FREE)
            .add_modifier(Modifier::BOLD)
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
}
