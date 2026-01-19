use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Header component with window switcher support
/// Based on v2.1 spec section 4.2
#[derive(Debug, Clone)]
pub struct Header {
    version: String,
    current_screen: String,
    window_switcher_active: bool,
    window_list: Vec<String>,
    selected_window_index: usize,
}

impl Header {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
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
    
    /// Set the current screen name
    pub fn set_current_screen(&mut self, screen: String) {
        self.current_screen = screen;
    }
    
    /// Check if window switcher is active
    pub fn is_window_switcher_active(&self) -> bool {
        self.window_switcher_active
    }
    
    /// Toggle the window switcher
    pub fn toggle_window_switcher(&mut self) {
        if self.window_switcher_active {
            self.deactivate_switcher();
        } else {
            self.activate_switcher();
        }
    }
    
    /// Activate the window switcher
    pub fn activate_switcher(&mut self) {
        self.window_switcher_active = true;
        // Find the index of the current screen
        if let Some(index) = self.window_list.iter().position(|s| {
            s.starts_with(&self.current_screen.split(':').next().unwrap_or(""))
        }) {
            self.selected_window_index = index;
        }
    }
    
    /// Deactivate the window switcher
    pub fn deactivate_switcher(&mut self) {
        self.window_switcher_active = false;
    }
    
    /// Select the next window in the switcher
    pub fn select_next_window(&mut self) {
        self.selected_window_index = (self.selected_window_index + 1) % self.window_list.len();
    }
    
    /// Select the previous window in the switcher
    pub fn select_previous_window(&mut self) {
        if self.selected_window_index == 0 {
            self.selected_window_index = self.window_list.len() - 1;
        } else {
            self.selected_window_index -= 1;
        }
    }
    
    /// Confirm window selection and return the selected screen
    pub fn confirm_window_selection(&mut self) -> Option<crate::app::AppScreen> {
        use crate::app::{AppScreen, RentalTab, ManagementTab};
        
        let screen = match self.selected_window_index {
            0 => AppScreen::Dashboard,
            1 => AppScreen::RentalManagement(RentalTab::List),
            2 => AppScreen::Finances,
            3 => AppScreen::Management(ManagementTab::Lockers),
            _ => return None,
        };
        
        self.deactivate_switcher();
        Some(screen)
    }
    
    /// Cancel window switcher
    pub fn cancel_window_switcher(&mut self) {
        self.deactivate_switcher();
    }
    
    /// Get the currently selected window
    pub fn selected_window(&self) -> &str {
        &self.window_list[self.selected_window_index]
    }
    
    /// Render the header
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Split into left (1/3) and right (2/3)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(67),
            ])
            .split(area);
        
        // Render left: app name + version
        self.render_app_name(chunks[0], buf, theme);
        
        // Render right: window switcher or simple screen name
        if self.window_switcher_active {
            self.render_window_switcher(chunks[1], buf, theme);
        } else {
            self.render_simple_screen_name(chunks[1], buf, theme);
        }
    }
    
    fn render_app_name(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let text = format!("Schließfach-Manager v{}", self.version);
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(theme.header_fg).bg(theme.header_bg))
            .alignment(Alignment::Left);
        
        paragraph.render(area, buf);
    }
    
    fn render_simple_screen_name(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let text = format!("[{}]", self.current_screen);
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(theme.header_fg).bg(theme.header_bg))
            .alignment(Alignment::Right);
        
        paragraph.render(area, buf);
    }
    
    fn render_window_switcher(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Split right area into 3 equal parts
        let thirds = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area);
        
        // Calculate prev, current, next indices
        let prev_idx = if self.selected_window_index == 0 {
            self.window_list.len() - 1
        } else {
            self.selected_window_index - 1
        };
        
        let current_idx = self.selected_window_index;
        
        let next_idx = (self.selected_window_index + 1) % self.window_list.len();
        
        // Render prev (dimmed)
        let prev_text = format!("|{}|", self.window_list[prev_idx]);
        let prev = Paragraph::new(prev_text)
            .style(Style::default().fg(theme.window_adjacent).bg(theme.header_bg))
            .alignment(Alignment::Center);
        prev.render(thirds[0], buf);
        
        // Render current (highlighted)
        let current_text = format!("[{}]", self.window_list[current_idx]);
        let current = Paragraph::new(current_text)
            .style(Style::default().fg(theme.window_current).bg(theme.header_bg))
            .alignment(Alignment::Center);
        current.render(thirds[1], buf);
        
        // Render next (dimmed)
        let next_text = format!("|{}|", self.window_list[next_idx]);
        let next = Paragraph::new(next_text)
            .style(Style::default().fg(theme.window_adjacent).bg(theme.header_bg))
            .alignment(Alignment::Center);
        next.render(thirds[2], buf);
    }
}
