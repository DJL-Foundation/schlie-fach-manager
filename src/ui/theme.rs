use ratatui::style::{Color, Modifier, Style};

/// Theme struct containing all colors and styles for the application
/// Based on v2.1 spec section 13
#[derive(Debug, Clone)]
pub struct Theme {
    // Base colors
    pub background: Color,
    pub foreground: Color,
    pub text: Color,
    pub text_dim: Color,
    
    // Accent colors
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    
    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    
    // UI elements
    pub border: Color,
    pub border_active: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    
    // Keybind bar
    pub keybind_global: Color,
    pub keybind_context: Color,
    
    // Window switcher
    pub window_current: Color,
    pub window_adjacent: Color,
    
    // Wizard
    pub wizard_system: Color,
    pub wizard_user: Color,
    pub wizard_option_selected: Color,
    pub wizard_option_normal: Color,
    
    // Status bar
    pub status_info: Style,
    pub status_success: Style,
    pub status_warning: Style,
    pub status_error: Style,
    
    // Escape indicator
    pub escape_indicator_active: Color,
    pub escape_indicator_inactive: Color,
    
    // Screensaver
    pub screensaver_bg: Color,
    pub screensaver_fg: Color,
}

impl Theme {
    /// Default dark theme
    pub fn default_dark() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            text: Color::Rgb(220, 220, 220),
            text_dim: Color::Rgb(128, 128, 128),
            
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Yellow,
            
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            
            border: Color::DarkGray,
            border_active: Color::Cyan,
            header_bg: Color::Rgb(30, 30, 30),
            header_fg: Color::White,
            
            keybind_global: Color::Yellow,
            keybind_context: Color::Rgb(180, 180, 180),
            
            window_current: Color::Cyan,
            window_adjacent: Color::DarkGray,
            
            wizard_system: Color::Cyan,
            wizard_user: Color::Green,
            wizard_option_selected: Color::Yellow,
            wizard_option_normal: Color::White,
            
            status_info: Style::default().fg(Color::Cyan),
            status_success: Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            status_warning: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            status_error: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            
            escape_indicator_active: Color::Yellow,
            escape_indicator_inactive: Color::DarkGray,
            
            screensaver_bg: Color::Black,
            screensaver_fg: Color::Green, // Matrix-style
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_dark()
    }
}
