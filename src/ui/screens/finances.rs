use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Finances screen showing payments, revenue, and financial reports
/// Per v2.1 spec section 7
pub struct FinancesScreen;

impl FinancesScreen {
    pub fn new() -> Self {
        Self
    }

    /// Render the screen at the given area
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                "💰 Finanzen",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Übersicht über alle finanziellen Transaktionen:"),
            Line::from(""),
            Line::from(Span::styled("Funktionen:", Style::default().add_modifier(Modifier::UNDERLINED))),
            Line::from("• Zahlungsübersicht"),
            Line::from("• Umsatzberichte"),
            Line::from("• Offene Forderungen"),
            Line::from("• Rechnungserstellung"),
            Line::from("• Export-Funktionen (CSV, PDF)"),
            Line::from(""),
            Line::from(Span::styled(
                "[Work in Progress]",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Diese Ansicht zeigt später:",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                "• Aktuelle Einnahmen des Monats",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                "• Zahlungsverlauf (Tabelle)",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                "• Statistiken und Diagramme",
                Style::default().fg(Color::Gray),
            )),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Finanzverwaltung")
                    .style(Style::default().fg(theme.text)),
            )
            .alignment(Alignment::Left);

        paragraph.render(area, buf);
    }
}

impl Default for FinancesScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finances_screen_creation() {
        let _screen = FinancesScreen::new();
        // Just verify it can be created
    }
}
