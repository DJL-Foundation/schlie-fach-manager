use crate::{
    app::ManagementTab,
    ui::theme::Theme,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Widget},
};

/// Management screen with 4 tabs: Lockers, Locations, Settings, AuditLog
/// Per v2.1 spec section 8
pub struct ManagementScreen {
    current_tab: ManagementTab,
}

impl ManagementScreen {
    pub fn new(tab: ManagementTab) -> Self {
        Self { current_tab: tab }
    }

    pub fn current_tab(&self) -> ManagementTab {
        self.current_tab
    }

    pub fn set_tab(&mut self, tab: ManagementTab) {
        self.current_tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            ManagementTab::Lockers => ManagementTab::Locations,
            ManagementTab::Locations => ManagementTab::Settings,
            ManagementTab::Settings => ManagementTab::AuditLog,
            ManagementTab::AuditLog => ManagementTab::Lockers,
        };
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            ManagementTab::Lockers => ManagementTab::AuditLog,
            ManagementTab::Locations => ManagementTab::Lockers,
            ManagementTab::Settings => ManagementTab::Locations,
            ManagementTab::AuditLog => ManagementTab::Settings,
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
            ManagementTab::Lockers.name(),
            ManagementTab::Locations.name(),
            ManagementTab::Settings.name(),
            ManagementTab::AuditLog.name(),
        ];

        let selected_index = match self.current_tab {
            ManagementTab::Lockers => 0,
            ManagementTab::Locations => 1,
            ManagementTab::Settings => 2,
            ManagementTab::AuditLog => 3,
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
            ManagementTab::Lockers => (
                "Schließfächer",
                "Verwalten Sie alle Schließfächer im System.\n\n\
                 Funktionen:\n\
                 • Schließfächer hinzufügen/bearbeiten/löschen\n\
                 • Bulk-Import/Export\n\
                 • Status ändern (Verfügbar/Wartung/Defekt)\n\
                 • Standort zuweisen",
            ),
            ManagementTab::Locations => (
                "Standorte",
                "Verwalten Sie alle Standorte und deren Eigenschaften.\n\n\
                 Funktionen:\n\
                 • Standorte hinzufügen/bearbeiten/löschen\n\
                 • Adressverwaltung\n\
                 • Zuordnung zu Schließfächern\n\
                 • Kapazitätsübersicht",
            ),
            ManagementTab::Settings => (
                "Einstellungen",
                "Systemeinstellungen und Konfiguration.\n\n\
                 Optionen:\n\
                 • Screensaver-Timeout\n\
                 • Standard-Mietdauer\n\
                 • Preiskonfiguration\n\
                 • Währung und Sprache\n\
                 • Backup-Einstellungen",
            ),
            ManagementTab::AuditLog => (
                "Audit Log",
                "Protokollierung aller Systemaktivitäten.\n\n\
                 Anzeige:\n\
                 • Benutzeraktionen\n\
                 • Datenänderungen\n\
                 • Zeitstempel\n\
                 • Filtermöglichkeiten\n\
                 • Export-Funktion",
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
        let mut screen = ManagementScreen::new(ManagementTab::Lockers);
        assert_eq!(screen.current_tab(), ManagementTab::Lockers);

        screen.next_tab();
        assert_eq!(screen.current_tab(), ManagementTab::Locations);

        screen.next_tab();
        assert_eq!(screen.current_tab(), ManagementTab::Settings);

        screen.next_tab();
        assert_eq!(screen.current_tab(), ManagementTab::AuditLog);

        screen.next_tab();
        assert_eq!(screen.current_tab(), ManagementTab::Lockers); // Wrap around

        screen.previous_tab();
        assert_eq!(screen.current_tab(), ManagementTab::AuditLog);
    }

    #[test]
    fn test_set_tab() {
        let mut screen = ManagementScreen::new(ManagementTab::Lockers);
        screen.set_tab(ManagementTab::AuditLog);
        assert_eq!(screen.current_tab(), ManagementTab::AuditLog);
    }
}
