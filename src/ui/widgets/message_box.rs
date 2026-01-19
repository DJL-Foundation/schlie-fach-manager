use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Simple message box widget for confirmations and alerts.
pub struct MessageBox<'a> {
    pub title: &'a str,
    pub message: &'a str,
}

impl<'a> MessageBox<'a> {
    /// Render the message box inside the given area.
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title)
            .border_style(Style::default().fg(theme.border));
        let paragraph = Paragraph::new(self.message)
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.text));
        paragraph.render(area, buf);
    }
}
