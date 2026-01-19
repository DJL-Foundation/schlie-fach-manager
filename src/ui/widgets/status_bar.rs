use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use std::time::{Duration, Instant};

/// Status bar component with message display and escape indicator
/// Based on v2.1 spec section 4.4
#[derive(Debug)]
pub struct StatusBar {
    pub message: Option<StatusMessage>,
    escape_count: u8,
    last_escape_time: Option<Instant>,
}

/// A status message with level and timestamp
#[derive(Debug, Clone)]
pub struct StatusMessage {
    pub text: String,
    pub level: StatusLevel,
    pub timestamp: Instant,
}

/// Status message level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            message: None,
            escape_count: 0,
            last_escape_time: None,
        }
    }
    
    /// Set a status message
    pub fn set_message(&mut self, text: String, level: StatusLevel) {
        self.message = Some(StatusMessage {
            text,
            level,
            timestamp: Instant::now(),
        });
    }
    
    /// Increment the escape count (called when Esc is pressed)
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
    
    /// Reset the escape count
    pub fn reset_escape_count(&mut self) {
        self.escape_count = 0;
        self.last_escape_time = None;
    }
    
    /// Check if we should return to dashboard (3 escapes pressed)
    pub fn should_return_to_dashboard(&self) -> bool {
        self.escape_count >= 3
    }
    
    /// Get the current escape count
    pub fn escape_count(&self) -> u8 {
        self.escape_count
    }
    
    /// Check if the escape count should be reset based on time
    pub fn check_escape_timeout(&mut self) {
        if let Some(last_time) = self.last_escape_time {
            if last_time.elapsed() > Duration::from_secs(1) {
                self.reset_escape_count();
            }
        }
    }
    
    /// Render the status bar
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Split area: left for message, right for escape indicator
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ])
            .split(area);
        
        // Render message
        if let Some(msg) = &self.message {
            // Auto-clear after 5 seconds
            if msg.timestamp.elapsed() < Duration::from_secs(5) {
                let style = match msg.level {
                    StatusLevel::Info => theme.status_info,
                    StatusLevel::Success => theme.status_success,
                    StatusLevel::Warning => theme.status_warning,
                    StatusLevel::Error => theme.status_error,
                };
                
                let text = Paragraph::new(&*msg.text)
                    .style(style)
                    .alignment(Alignment::Left);
                
                text.render(chunks[0], buf);
            }
        }
        
        // Render escape indicator [|||]
        self.render_escape_indicator(chunks[1], buf, theme);
    }
    
    fn render_escape_indicator(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Format: [|||]
        // Each pipe changes color based on escape_count
        let pipes = ['|', '|', '|'];
        let mut spans = vec![
            Span::raw("["),
        ];
        
        for (i, pipe) in pipes.iter().enumerate() {
            let style = if i < self.escape_count as usize {
                Style::default().fg(theme.escape_indicator_active)
            } else {
                Style::default().fg(theme.escape_indicator_inactive)
            };
            
            spans.push(Span::styled(pipe.to_string(), style));
        }
        
        spans.push(Span::raw("]"));
        
        let indicator = Paragraph::new(Line::from(spans))
            .alignment(Alignment::Right);
        
        indicator.render(area, buf);
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_escape_count_increment() {
        let mut status_bar = StatusBar::new();
        
        assert_eq!(status_bar.escape_count(), 0);
        
        status_bar.increment_escape_count();
        assert_eq!(status_bar.escape_count(), 1);
        
        status_bar.increment_escape_count();
        assert_eq!(status_bar.escape_count(), 2);
        
        status_bar.increment_escape_count();
        assert_eq!(status_bar.escape_count(), 3);
        
        // Should cap at 3
        status_bar.increment_escape_count();
        assert_eq!(status_bar.escape_count(), 3);
    }
    
    #[test]
    fn test_should_return_to_dashboard() {
        let mut status_bar = StatusBar::new();
        
        assert!(!status_bar.should_return_to_dashboard());
        
        status_bar.increment_escape_count();
        assert!(!status_bar.should_return_to_dashboard());
        
        status_bar.increment_escape_count();
        assert!(!status_bar.should_return_to_dashboard());
        
        status_bar.increment_escape_count();
        assert!(status_bar.should_return_to_dashboard());
    }
    
    #[test]
    fn test_escape_count_reset() {
        let mut status_bar = StatusBar::new();
        
        status_bar.increment_escape_count();
        status_bar.increment_escape_count();
        assert_eq!(status_bar.escape_count(), 2);
        
        status_bar.reset_escape_count();
        assert_eq!(status_bar.escape_count(), 0);
    }
    
    #[test]
    fn test_escape_count_timeout() {
        let mut status_bar = StatusBar::new();
        
        status_bar.increment_escape_count();
        assert_eq!(status_bar.escape_count(), 1);
        
        // Simulate waiting more than 1 second
        thread::sleep(Duration::from_millis(1100));
        
        status_bar.increment_escape_count();
        // Should have reset to 0, then incremented to 1
        assert_eq!(status_bar.escape_count(), 1);
    }
    
    #[test]
    fn test_escape_count_timeout_check() {
        let mut status_bar = StatusBar::new();
        
        status_bar.increment_escape_count();
        status_bar.increment_escape_count();
        assert_eq!(status_bar.escape_count(), 2);
        
        // Simulate waiting more than 1 second
        thread::sleep(Duration::from_millis(1100));
        
        status_bar.check_escape_timeout();
        assert_eq!(status_bar.escape_count(), 0);
    }
}
