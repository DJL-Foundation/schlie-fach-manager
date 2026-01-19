use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{BarChart, Block, Borders, Widget},
};

/// Graph widget for occupancy trend visualization
/// Based on v2.1 spec section 6.5
#[derive(Debug)]
pub struct OccupancyGraph<'a> {
    data: Vec<(&'a str, u64)>,
    title: String,
}

impl<'a> OccupancyGraph<'a> {
    pub fn new(title: String) -> Self {
        Self {
            data: Vec::new(),
            title,
        }
    }
    
    /// Set the data points for the graph
    /// Each point is (label, value)
    pub fn set_data(&mut self, data: Vec<(&'a str, u64)>) {
        self.data = data;
    }
    
    /// Add a single data point
    pub fn add_data_point(&mut self, label: &'a str, value: u64) {
        self.data.push((label, value));
    }
    
    /// Render the graph
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        if self.data.is_empty() {
            // Render empty state
            let block = Block::default()
                .borders(Borders::ALL)
                .title(self.title.clone())
                .border_style(Style::default().fg(theme.border));
            block.render(area, buf);
            return;
        }
        
        let chart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.title.clone())
                    .border_style(Style::default().fg(theme.border))
            )
            .data(&self.data)
            .bar_width(3)
            .bar_gap(1)
            .bar_style(Style::default().fg(theme.accent))
            .value_style(Style::default().fg(theme.text));
        
        chart.render(area, buf);
    }
}

impl<'a> Default for OccupancyGraph<'a> {
    fn default() -> Self {
        Self::new("Belegungstrend".to_string())
    }
}
