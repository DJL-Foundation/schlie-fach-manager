use crate::ui::state::{Notification, NotificationLevel};
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    prelude::Frame,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

/// Renders a notification in the top-right corner of the screen.
pub fn render_notification(frame: &mut Frame, area: Rect, notification: &Notification) {
    let width = (notification.message.len() as u16 + 8).min(area.width.saturating_sub(4));
    let height = 3;

    // Position in top-right corner
    let x = area.x + area.width.saturating_sub(width + 2);
    let y = area.y + 1;

    let notification_area = Rect::new(x, y, width, height);

    // Clear the area
    frame.render_widget(Clear, notification_area);

    // Choose style based on level
    let (icon, border_style) = match notification.level {
        NotificationLevel::Info => ("ℹ", Theme::info_style()),
        NotificationLevel::Success => ("✓", Theme::success()),
        NotificationLevel::Warning => ("⚠", Theme::warning()),
        NotificationLevel::Error => ("✗", Theme::error()),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(notification_area);
    frame.render_widget(block, notification_area);

    // Render message
    let message = Line::from(vec![
        Span::styled(format!("{} ", icon), border_style),
        Span::styled(&notification.message, Theme::normal()),
    ]);

    let paragraph = Paragraph::new(message);
    frame.render_widget(paragraph, inner);
}
