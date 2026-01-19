use crate::{
    app::RentalTab,
    ui::theme::Theme,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Widget},
};

/// Rental Management screen with 5 tabs: Search, List, Extend, Return, Damage
/// Per v2.1 spec section 6
pub struct RentalManagementScreen {
    current_tab: RentalTab,
}

impl RentalManagementScreen {
    pub fn new(tab: RentalTab) -> Self {
        Self { current_tab: tab }
    }

    pub fn current_tab(&self) -> RentalTab {
        self.current_tab
    }

    pub fn set_tab(&mut self, tab: RentalTab) {
        self.current_tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            RentalTab::Search => RentalTab::List,
            RentalTab::List => RentalTab::Extend,
            RentalTab::Extend => RentalTab::Return,
            RentalTab::Return => RentalTab::Damage,
            RentalTab::Damage => RentalTab::Search,
        };
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            RentalTab::Search => RentalTab::Damage,
            RentalTab::List => RentalTab::Search,
            RentalTab::Extend => RentalTab::List,
            RentalTab::Return => RentalTab::Extend,
            RentalTab::Damage => RentalTab::Return,
        };
    }

    /// Render the screen at the given area
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Tab bar
                Constraint::Min(5),    // Content
            ])
            .split(area);

        // Render tab bar
        self.render_tabs(chunks[0], buf, theme);

        // Render tab content
        self.render_tab_content(chunks[1], buf, theme);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let tab_titles = vec![
            RentalTab::Search.name(),
            RentalTab::List.name(),
            RentalTab::Extend.name(),
            RentalTab::Return.name(),
            RentalTab::Damage.name(),
        ];

        let selected_index = match self.current_tab {
            RentalTab::Search => 0,
            RentalTab::List => 1,
            RentalTab::Extend => 2,
            RentalTab::Return => 3,
            RentalTab::Damage => 4,
        };

        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL))
            .select(selected_index)
            .style(Style::default().fg(theme.text))
            .highlight_style(
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            );

        tabs.render(area, buf);
    }

    fn render_tab_content(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let (title, description) = match self.current_tab {
            RentalTab::Search => (
                "Suche",
                "Hier können Sie nach Schließfächern und Mietern suchen.\n\n\
                 Funktionen:\n\
                 • Freitextsuche\n\
                 • Filter nach Status\n\
                 • Erweiterte Suchoptionen",
            ),
            RentalTab::List => (
                "Liste",
                "Zeigt alle aktiven Mieten an.\n\n\
                 Funktionen:\n\
                 • Sortierbare Spalten\n\
                 • Filtermöglichkeiten\n\
                 • Export-Optionen",
            ),
            RentalTab::Extend => (
                "Verlängern",
                "Verlängern Sie bestehende Mietverträge.\n\n\
                 Workflow:\n\
                 1. Schließfach auswählen\n\
                 2. Neue Laufzeit eingeben\n\
                 3. Bestätigen",
            ),
            RentalTab::Return => (
                "Rückgabe",
                "Bearbeiten Sie Schließfach-Rückgaben.\n\n\
                 Workflow:\n\
                 1. Schließfach auswählen\n\
                 2. Zustand prüfen\n\
                 3. Rückgabe bestätigen",
            ),
            RentalTab::Damage => (
                "Defekt",
                "Melden und verwalten Sie defekte Schließfächer.\n\n\
                 Workflow:\n\
                 1. Schließfach auswählen\n\
                 2. Schadensbeschreibung eingeben\n\
                 3. Priorität festlegen",
            ),
        };

        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                title,
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(description),
            Line::from(""),
            Line::from(Span::styled(
                "[Work in Progress]",
                Style::default().fg(Color::Yellow),
            )),
        ];

        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Inhalt"))
            .style(Style::default().fg(theme.text));

        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_navigation() {
        let mut screen = RentalManagementScreen::new(RentalTab::Search);
        assert_eq!(screen.current_tab(), RentalTab::Search);

        screen.next_tab();
        assert_eq!(screen.current_tab(), RentalTab::List);

        screen.next_tab();
        assert_eq!(screen.current_tab(), RentalTab::Extend);

        screen.next_tab();
        assert_eq!(screen.current_tab(), RentalTab::Return);

        screen.next_tab();
        assert_eq!(screen.current_tab(), RentalTab::Damage);

        screen.next_tab();
        assert_eq!(screen.current_tab(), RentalTab::Search); // Wrap around

        screen.previous_tab();
        assert_eq!(screen.current_tab(), RentalTab::Damage);
    }

    #[test]
    fn test_set_tab() {
        let mut screen = RentalManagementScreen::new(RentalTab::Search);
        screen.set_tab(RentalTab::Return);
        assert_eq!(screen.current_tab(), RentalTab::Return);
    }
}
