use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{BarChart, Widget},
};

/// Occupancy trend graph displayed on the dashboard.
pub struct OccupancyGraph {
    pub data: Vec<(String, f64)>,
}

impl OccupancyGraph {
    /// Render the graph into the given buffer area.
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let bars: Vec<(&str, u64)> = self
            .data
            .iter()
            .map(|(label, percent)| (label.as_str(), *percent as u64))
            .collect();

        let chart = BarChart::default()
            .data(&bars)
            .bar_width(3)
            .bar_gap(1)
            .bar_style(Style::default().fg(theme.accent))
            .value_style(Style::default().fg(theme.text));
        chart.render(area, buf);
    }
}
