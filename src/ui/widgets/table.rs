use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table as RatatuiTable, Widget},
};

/// Enhanced table widget for displaying data
#[derive(Debug)]
pub struct EnhancedTable<'a> {
    headers: Vec<&'a str>,
    rows: Vec<Vec<String>>,
    selected_row: Option<usize>,
    column_widths: Vec<u16>,
}

impl<'a> EnhancedTable<'a> {
    pub fn new(headers: Vec<&'a str>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
            selected_row: None,
            column_widths: Vec::new(),
        }
    }
    
    /// Add a row to the table
    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }
    
    /// Set the selected row
    pub fn set_selected_row(&mut self, index: Option<usize>) {
        self.selected_row = index;
    }
    
    /// Set column widths
    pub fn set_column_widths(&mut self, widths: Vec<u16>) {
        self.column_widths = widths;
    }
    
    /// Render the table
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Create header
        let header_cells: Vec<Cell> = self.headers
            .iter()
            .map(|h| {
                Cell::from(*h)
                    .style(Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD))
            })
            .collect();
        
        let header = Row::new(header_cells)
            .style(Style::default())
            .height(1);
        
        // Create rows
        let rows: Vec<Row> = self.rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let cells: Vec<Cell> = row
                    .iter()
                    .map(|cell| Cell::from(cell.as_str()))
                    .collect();
                
                let mut row_widget = Row::new(cells).height(1);
                
                if Some(i) == self.selected_row {
                    row_widget = row_widget.style(
                        Style::default()
                            .fg(theme.foreground)
                            .bg(theme.primary)
                            .add_modifier(Modifier::BOLD)
                    );
                }
                
                row_widget
            })
            .collect();
        
        // Determine constraints
        let constraints = if !self.column_widths.is_empty() {
            self.column_widths
                .iter()
                .map(|w| ratatui::layout::Constraint::Length(*w))
                .collect::<Vec<_>>()
        } else {
            // Equal width for all columns
            let width = 100 / self.headers.len() as u16;
            self.headers
                .iter()
                .map(|_| ratatui::layout::Constraint::Percentage(width))
                .collect::<Vec<_>>()
        };
        
        let table = RatatuiTable::new(rows, constraints)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border))
            )
            .column_spacing(2);
        
        table.render(area, buf);
    }
}
