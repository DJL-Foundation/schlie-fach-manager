use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::time::{Duration, Instant};

/// Message displayed in the status bar.
#[derive(Debug, Clone)]
pub struct StatusMessage {
    pub text: String,
    pub level: StatusLevel,
    pub timestamp: Instant,
}

/// Severity level for status messages.
#[derive(Debug, Clone, Copy)]
pub enum StatusLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Status bar widget with escape indicator.
#[derive(Debug, Default)]
pub struct StatusBar {
    pub message: Option<StatusMessage>,
    pub escape_count: u8,
    pub last_escape_time: Option<Instant>,
}

impl StatusBar {
    /// Sets a status message with the provided severity.
    pub fn set_message(&mut self, text: String, level: StatusLevel) {
        self.message = Some(StatusMessage {
            text,
            level,
            timestamp: Instant::now(),
        });
    }

    /// Increments the escape counter used for quick navigation.
    pub fn increment_escape_count(&mut self) {
        let now = Instant::now();
        if let Some(last_time) = self.last_escape_time {
            if now.duration_since(last_time) > Duration::from_secs(1) {
                self.escape_count = 0;
            }
        }

        self.escape_count = (self.escape_count + 1).min(3);
        self.last_escape_time = Some(now);
    }

    /// Resets the escape counter and timestamp.
    pub fn reset_escape_count(&mut self) {
        self.escape_count = 0;
        self.last_escape_time = None;
    }

    /// Returns `true` when the escape counter reached the maximum.
    pub fn should_return_to_dashboard(&self) -> bool {
        self.escape_count >= 3
    }

    /// Shows the screensaver countdown message.
    pub fn show_screensaver_countdown(&mut self, seconds: u64) {
        self.set_message(
            format!("⏱ Screensaver in {} Sekunden...", seconds),
            StatusLevel::Info,
        );
    }

    /// Render the status bar into the provided buffer area.
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);

        if let Some(msg) = &self.message {
            if msg.timestamp.elapsed() < Duration::from_secs(5) {
                let style = match msg.level {
                    StatusLevel::Info => theme.status_info,
                    StatusLevel::Success => theme.status_success,
                    StatusLevel::Warning => theme.status_warning,
                    StatusLevel::Error => theme.status_error,
                };

                let text = Paragraph::new(msg.text.as_str())
                    .style(style)
                    .alignment(Alignment::Left)
                    .block(Block::default().borders(Borders::NONE));
                text.render(chunks[0], buf);
            }
        }

        self.render_escape_indicator(chunks[1], buf, theme);
    }

    /// Renders the escape indicator pipes.
    fn render_escape_indicator(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let pipes = ['|', '|', '|'];
        let mut spans = vec![Span::raw("[")];

        for (idx, pipe) in pipes.iter().enumerate() {
            let style = if idx < self.escape_count as usize {
                Style::default().fg(theme.escape_indicator_active)
            } else {
                Style::default().fg(theme.escape_indicator_inactive)
            };
            spans.push(Span::styled(pipe.to_string(), style));
        }

        spans.push(Span::raw("]"));
        let indicator = Paragraph::new(Line::from(spans)).alignment(Alignment::Right);
        indicator.render(area, buf);
    }
}
