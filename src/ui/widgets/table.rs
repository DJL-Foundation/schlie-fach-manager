use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};

/// Simple helper widget for rendering tables with themed headers.
pub struct TableWidget<'a> {
    pub title: &'a str,
    pub headers: Vec<&'a str>,
    pub rows: Vec<Vec<String>>,
    pub widths: Vec<Constraint>,
}

impl<'a> TableWidget<'a> {
    /// Render the table widget to the given buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let header = Row::new(self.headers.iter().map(|header| {
            Cell::from(Span::styled(
                *header,
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ))
        }));

        let rows = self.rows.iter().map(|row| {
            let cells = row.iter().map(|cell| Cell::from(cell.clone()));
            Row::new(cells)
        });

        let table = Table::new(rows, self.widths.clone())
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(self.title))
            .column_spacing(2);
        table.render(area, buf);
    }
}
