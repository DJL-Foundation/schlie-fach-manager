use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
};

/// Configuration for a table column.
pub struct ColumnDef {
    pub header: String,
    pub width: Constraint,
}

impl ColumnDef {
    pub fn new(header: impl Into<String>, width: Constraint) -> Self {
        Self {
            header: header.into(),
            width,
        }
    }
}

/// Renders a table with a detail panel on the right.
pub fn render_table_with_detail<T>(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    columns: &[ColumnDef],
    items: &[T],
    selected: Option<usize>,
    row_mapper: impl Fn(&T, usize) -> Row<'static>,
    detail_renderer: impl Fn(Option<&T>) -> Text<'static>,
    _footer_text: &str,
) {
    // Split into table and detail panel
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Render table
    render_table(
        frame,
        chunks[0],
        title,
        columns,
        items,
        selected,
        &row_mapper,
    );

    // Render detail panel
    let selected_item = selected.and_then(|idx| items.get(idx));
    let detail_text = detail_renderer(selected_item);

    let detail_title = if selected_item.is_some() {
        "Details"
    } else {
        "Keine Auswahl"
    };

    let detail_block = Block::default()
        .title(format!(" {} ", detail_title))
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let detail_paragraph = Paragraph::new(detail_text)
        .block(detail_block)
        .wrap(Wrap { trim: true });

    frame.render_widget(detail_paragraph, chunks[1]);
}

/// Renders just the table portion.
pub fn render_table<T>(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    columns: &[ColumnDef],
    items: &[T],
    selected: Option<usize>,
    row_mapper: &impl Fn(&T, usize) -> Row<'static>,
) {
    let block = Block::default()
        .title(format!(" {} ({} Einträge) ", title, items.len()))
        .borders(Borders::ALL)
        .border_style(Theme::normal());

    // Create header row
    let header_cells: Vec<Cell> = columns
        .iter()
        .map(|col| Cell::from(Span::styled(col.header.clone(), Theme::header())))
        .collect();
    let header = Row::new(header_cells).height(1);

    // Create data rows
    let rows: Vec<Row> = items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let mut row = row_mapper(item, idx);
            if Some(idx) == selected {
                row = row.style(Theme::selected());
            }
            row
        })
        .collect();

    // Extract widths from column definitions
    let widths: Vec<Constraint> = columns.iter().map(|col| col.width).collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .column_spacing(1);

    frame.render_widget(table, area);
}

/// Creates a detail text block with key-value pairs.
pub fn detail_lines(pairs: Vec<(&str, String)>) -> Text<'static> {
    let lines: Vec<Line> = pairs
        .into_iter()
        .map(|(key, value)| {
            Line::from(vec![
                Span::styled(format!("{}: ", key), Theme::dim()),
                Span::styled(value, Theme::normal()),
            ])
        })
        .collect();
    Text::from(lines)
}
