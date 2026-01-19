use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Header widget that renders the application title and window switcher.
pub struct Header {
    pub version: String,
    pub current_screen: String,
    pub window_switcher_active: bool,
    pub window_list: Vec<String>,
    pub selected_window_index: usize,
}

impl Header {
    /// Render the header into the provided buffer area.
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
            .split(area);

        let left_text = format!("Schließfach-Manager v{}", self.version);
        let left = Paragraph::new(left_text)
            .style(Style::default().fg(theme.header_fg).bg(theme.header_bg))
            .block(Block::default().borders(Borders::NONE));
        left.render(chunks[0], buf);

        if self.window_switcher_active {
            self.render_window_switcher(chunks[1], buf, theme);
        } else {
            self.render_simple_screen_name(chunks[1], buf, theme);
        }
    }

    /// Renders the simple screen name variant when the switcher is inactive.
    fn render_simple_screen_name(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let text = Line::from(vec![Span::styled(
            format!("[{}]", self.current_screen),
            Style::default().fg(theme.header_fg).bg(theme.header_bg),
        )]);
        let paragraph = Paragraph::new(text).alignment(Alignment::Right);
        paragraph.render(area, buf);
    }

    /// Renders the 3-part window switcher when active.
    fn render_window_switcher(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        if self.window_list.is_empty() {
            self.render_simple_screen_name(area, buf, theme);
            return;
        }

        let thirds = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area);

        let len = self.window_list.len();
        let current = self.selected_window_index % len;
        let prev = if current == 0 { len - 1 } else { current - 1 };
        let next = (current + 1) % len;

        let prev_text = Line::from(vec![Span::styled(
            format!("│{}│", self.window_list[prev]),
            Style::default()
                .fg(theme.window_adjacent)
                .bg(theme.header_bg),
        )]);
        let current_text = Line::from(vec![Span::styled(
            format!("│[{}]│", self.window_list[current]),
            Style::default()
                .fg(theme.window_current)
                .bg(theme.header_bg),
        )]);
        let next_text = Line::from(vec![Span::styled(
            format!("│{}│", self.window_list[next]),
            Style::default()
                .fg(theme.window_adjacent)
                .bg(theme.header_bg),
        )]);

        Paragraph::new(prev_text)
            .alignment(Alignment::Center)
            .render(thirds[0], buf);
        Paragraph::new(current_text)
            .alignment(Alignment::Center)
            .render(thirds[1], buf);
        Paragraph::new(next_text)
            .alignment(Alignment::Center)
            .render(thirds[2], buf);
    }
}
