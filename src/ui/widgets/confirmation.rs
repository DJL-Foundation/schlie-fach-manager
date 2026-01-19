use crate::ui::state::ConfirmDialog;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Renders a confirmation dialog in the center of the screen.
pub fn render_confirmation_dialog(frame: &mut Frame, area: Rect, dialog: &ConfirmDialog) {
    // Calculate dialog size
    let dialog_width = 50.min(area.width.saturating_sub(4));
    let dialog_height = 10.min(area.height.saturating_sub(4));

    // Center the dialog
    let dialog_area = centered_rect(dialog_width, dialog_height, area);

    // Clear the area
    frame.render_widget(Clear, dialog_area);

    // Render the dialog block
    let block = Block::default()
        .title(format!(" {} ", dialog.title))
        .borders(Borders::ALL)
        .border_style(Theme::title());

    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    // Split inner area
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(2),    // Message
            Constraint::Length(1), // Subtitle (optional)
            Constraint::Length(2), // Buttons
        ])
        .split(inner);

    // Render message
    let message = Paragraph::new(Line::from(Span::styled(&dialog.message, Theme::normal())))
        .wrap(Wrap { trim: true });
    frame.render_widget(message, chunks[0]);

    // Render subtitle if present
    if let Some(subtitle) = &dialog.subtitle {
        let subtitle_paragraph = Paragraph::new(Line::from(Span::styled(subtitle, Theme::dim())));
        frame.render_widget(subtitle_paragraph, chunks[1]);
    }

    // Render buttons
    let confirm_style = if dialog.selected == 0 {
        Theme::button_selected()
    } else {
        Theme::button()
    };
    let cancel_style = if dialog.selected == 1 {
        Theme::button_selected()
    } else {
        Theme::button()
    };

    let buttons = Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("[ {} ]", dialog.confirm_label), confirm_style),
        Span::raw("    "),
        Span::styled(format!("[ {} ]", dialog.cancel_label), cancel_style),
        Span::raw("  "),
    ]);

    let buttons_paragraph = Paragraph::new(buttons);
    frame.render_widget(buttons_paragraph, chunks[2]);
}

/// Helper function to create a centered rect.
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
