use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Message box widget for dialogs and confirmations
#[derive(Debug, Clone)]
pub struct MessageBox {
    title: String,
    message: String,
    box_type: MessageBoxType,
    buttons: Vec<String>,
    selected_button: usize,
}

/// Type of message box
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxType {
    Info,
    Warning,
    Error,
    Confirm,
}

impl MessageBox {
    pub fn new(title: String, message: String, box_type: MessageBoxType) -> Self {
        let buttons = match box_type {
            MessageBoxType::Info => vec!["OK".to_string()],
            MessageBoxType::Warning => vec!["OK".to_string()],
            MessageBoxType::Error => vec!["OK".to_string()],
            MessageBoxType::Confirm => vec!["Ja".to_string(), "Nein".to_string()],
        };
        
        Self {
            title,
            message,
            box_type,
            buttons,
            selected_button: 0,
        }
    }
    
    /// Create an info message box
    pub fn info(title: String, message: String) -> Self {
        Self::new(title, message, MessageBoxType::Info)
    }
    
    /// Create a warning message box
    pub fn warning(title: String, message: String) -> Self {
        Self::new(title, message, MessageBoxType::Warning)
    }
    
    /// Create an error message box
    pub fn error(title: String, message: String) -> Self {
        Self::new(title, message, MessageBoxType::Error)
    }
    
    /// Create a confirmation message box
    pub fn confirm(title: String, message: String) -> Self {
        Self::new(title, message, MessageBoxType::Confirm)
    }
    
    /// Set custom buttons
    pub fn with_buttons(mut self, buttons: Vec<String>) -> Self {
        self.buttons = buttons;
        self.selected_button = 0;
        self
    }
    
    /// Select the next button
    pub fn next_button(&mut self) {
        self.selected_button = (self.selected_button + 1) % self.buttons.len();
    }
    
    /// Select the previous button
    pub fn prev_button(&mut self) {
        if self.selected_button == 0 {
            self.selected_button = self.buttons.len() - 1;
        } else {
            self.selected_button -= 1;
        }
    }
    
    /// Get the currently selected button text
    pub fn selected_button_text(&self) -> &str {
        &self.buttons[self.selected_button]
    }
    
    /// Render the message box centered in the given area
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Calculate centered position
        let box_width = (area.width / 2).max(40);
        let box_height = (area.height / 2).max(10);
        
        let x = (area.width.saturating_sub(box_width)) / 2;
        let y = (area.height.saturating_sub(box_height)) / 2;
        
        let box_area = Rect {
            x: area.x + x,
            y: area.y + y,
            width: box_width,
            height: box_height,
        };
        
        // Determine border color based on type
        let border_color = match self.box_type {
            MessageBoxType::Info => theme.info,
            MessageBoxType::Warning => theme.warning,
            MessageBoxType::Error => theme.error,
            MessageBoxType::Confirm => theme.primary,
        };
        
        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));
        
        let inner = block.inner(box_area);
        block.render(box_area, buf);
        
        // Split into message area and button area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(inner);
        
        // Render message
        let message_para = Paragraph::new(self.message.as_str())
            .style(Style::default().fg(theme.text))
            .alignment(Alignment::Center);
        message_para.render(chunks[0], buf);
        
        // Render buttons
        self.render_buttons(chunks[1], buf, theme);
    }
    
    fn render_buttons(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Create button text
        let button_text: Vec<String> = self.buttons
            .iter()
            .enumerate()
            .map(|(i, btn)| {
                if i == self.selected_button {
                    format!("[ {} ]", btn)
                } else {
                    format!("  {}  ", btn)
                }
            })
            .collect();
        
        let buttons_line = button_text.join("  ");
        
        let style = if self.selected_button < self.buttons.len() {
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text)
        };
        
        let para = Paragraph::new(buttons_line)
            .style(style)
            .alignment(Alignment::Center);
        
        para.render(area, buf);
    }
}
