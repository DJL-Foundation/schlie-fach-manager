use crate::models::Locker;
use crate::ui::state::ManagementTab;
use crate::ui::theme::Theme;
use crate::ui::widgets::{render_table_with_detail, ColumnDef};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Tabs},
};

/// State for management screens.
#[derive(Debug, Default)]
pub struct ManagementState {
    pub selected_tab: ManagementTab,
    pub lockers_selected: usize,
    pub locations_selected: usize,
}

impl Default for ManagementTab {
    fn default() -> Self {
        ManagementTab::Lockers
    }
}

impl ManagementState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_tab(&mut self) {
        let idx = self.selected_tab.index();
        let next = ManagementTab::from_index((idx + 1) % ManagementTab::all().len());
        self.selected_tab = next.unwrap_or(ManagementTab::Lockers);
    }

    pub fn prev_tab(&mut self) {
        let idx = self.selected_tab.index();
        let prev = if idx == 0 {
            ManagementTab::all().len() - 1
        } else {
            idx - 1
        };
        self.selected_tab = ManagementTab::from_index(prev).unwrap_or(ManagementTab::Lockers);
    }
}

/// Renders the management screen.
pub fn render_management(
    frame: &mut Frame,
    area: Rect,
    state: &ManagementState,
    lockers: &[Locker],
    locations: &[String],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(10),   // Content
            Constraint::Length(2), // Footer
        ])
        .split(area);

    // Render tabs
    render_tabs(frame, chunks[0], state.selected_tab);

    // Render content based on selected tab
    match state.selected_tab {
        ManagementTab::Lockers => {
            render_lockers_tab(frame, chunks[1], state, lockers);
        }
        ManagementTab::Locations => {
            render_locations_tab(frame, chunks[1], state, locations, lockers);
        }
        ManagementTab::Settings => {
            render_settings_tab(frame, chunks[1]);
        }
        ManagementTab::AuditLog => {
            render_audit_tab(frame, chunks[1]);
        }
        ManagementTab::Backup => {
            render_backup_tab(frame, chunks[1]);
        }
    }

    // Render footer
    render_footer(frame, chunks[2], state.selected_tab);
}

fn render_tabs(frame: &mut Frame, area: Rect, selected: ManagementTab) {
    let titles: Vec<Line> = ManagementTab::all()
        .iter()
        .map(|tab| {
            let style = if *tab == selected {
                Theme::tab_active()
            } else {
                Theme::tab_inactive()
            };
            Line::from(Span::styled(format!(" {} ", tab.label()), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title(" Verwaltung ")
                .borders(Borders::ALL)
                .border_style(Theme::title()),
        )
        .select(selected.index())
        .divider("|");

    frame.render_widget(tabs, area);
}

fn render_lockers_tab(frame: &mut Frame, area: Rect, state: &ManagementState, lockers: &[Locker]) {
    let columns = vec![
        ColumnDef::new("ID", Constraint::Length(10)),
        ColumnDef::new("Standort", Constraint::Percentage(35)),
        ColumnDef::new("Höhe", Constraint::Length(8)),
        ColumnDef::new("Defekt", Constraint::Length(8)),
    ];

    let row_mapper = |locker: &Locker, _idx: usize| -> Row<'static> {
        let defekt_style = if locker.is_damaged {
            Theme::status_damaged()
        } else {
            Theme::status_free()
        };
        let defekt_text = if locker.is_damaged {
            "Ja ⚠".to_string()
        } else {
            "Nein".to_string()
        };

        Row::new(vec![
            Cell::from(locker.label.clone()),
            Cell::from(locker.location.clone()),
            Cell::from(format!("{} cm", locker.height)),
            Cell::from(Span::styled(defekt_text, defekt_style)),
        ])
    };

    let detail_renderer = |locker: Option<&Locker>| -> Text<'static> {
        if let Some(l) = locker {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("Label: ".to_string(), Theme::dim()),
                    Span::styled(l.label.clone(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Standort: ".to_string(), Theme::dim()),
                    Span::styled(l.location.clone(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Höhe: ".to_string(), Theme::dim()),
                    Span::styled(format!("{} cm", l.height), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Defekt: ".to_string(), Theme::dim()),
                    if l.is_damaged {
                        Span::styled("Ja ⚠".to_string(), Theme::status_damaged())
                    } else {
                        Span::styled("Nein".to_string(), Theme::status_free())
                    },
                ]),
                Line::from(vec![
                    Span::styled("Erstellt: ".to_string(), Theme::dim()),
                    Span::styled(l.created_at.format("%d.%m.%Y").to_string(), Theme::normal()),
                ]),
                Line::from("".to_string()),
            ];

            lines.push(Line::from(vec![
                Span::styled("[E]".to_string(), Theme::primary_style()),
                Span::raw(" Bearbeiten".to_string()),
            ]));

            if l.is_damaged {
                lines.push(Line::from(vec![
                    Span::styled("[R]".to_string(), Theme::primary_style()),
                    Span::raw(" Als repariert markieren".to_string()),
                ]));
            }

            lines.push(Line::from(vec![
                Span::styled("[D]".to_string(), Theme::primary_style()),
                Span::raw(" Löschen".to_string()),
            ]));

            Text::from(lines)
        } else {
            Text::from(Line::from(Span::styled(
                "Kein Schließfach ausgewählt".to_string(),
                Theme::dim(),
            )))
        }
    };

    let damaged_count = lockers.iter().filter(|l| l.is_damaged).count();

    render_table_with_detail(
        frame,
        area,
        "Schließfächer",
        &columns,
        lockers,
        Some(state.lockers_selected),
        row_mapper,
        detail_renderer,
        &format!("{} Schließfächer ({} defekt)", lockers.len(), damaged_count),
    );
}

fn render_locations_tab(
    frame: &mut Frame,
    area: Rect,
    state: &ManagementState,
    locations: &[String],
    lockers: &[Locker],
) {
    // Build location stats
    struct LocationStat {
        name: String,
        total: usize,
        damaged: usize,
    }

    let location_stats: Vec<LocationStat> = locations
        .iter()
        .map(|loc| {
            let loc_lockers: Vec<_> = lockers.iter().filter(|l| &l.location == loc).collect();
            LocationStat {
                name: loc.clone(),
                total: loc_lockers.len(),
                damaged: loc_lockers.iter().filter(|l| l.is_damaged).count(),
            }
        })
        .collect();

    let columns = vec![
        ColumnDef::new("Name", Constraint::Percentage(50)),
        ColumnDef::new("Schließfächer", Constraint::Length(15)),
        ColumnDef::new("Defekt", Constraint::Length(10)),
    ];

    let row_mapper = |stat: &LocationStat, _idx: usize| -> Row<'static> {
        Row::new(vec![
            Cell::from(stat.name.clone()),
            Cell::from(stat.total.to_string()),
            Cell::from(stat.damaged.to_string()),
        ])
    };

    let detail_renderer = |stat: Option<&LocationStat>| -> Text<'static> {
        if let Some(s) = stat {
            let lines = vec![
                Line::from(vec![
                    Span::styled("Name: ".to_string(), Theme::dim()),
                    Span::styled(s.name.clone(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Schließfächer: ".to_string(), Theme::dim()),
                    Span::styled(s.total.to_string(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Defekt: ".to_string(), Theme::dim()),
                    Span::styled(s.damaged.to_string(), Theme::warning()),
                ]),
                Line::from("".to_string()),
                Line::from(vec![
                    Span::styled("[E]".to_string(), Theme::primary_style()),
                    Span::raw(" Bearbeiten".to_string()),
                ]),
                Line::from(vec![
                    Span::styled("[D]".to_string(), Theme::primary_style()),
                    Span::raw(" Löschen".to_string()),
                ]),
            ];
            Text::from(lines)
        } else {
            Text::from(Line::from(Span::styled(
                "Kein Standort ausgewählt".to_string(),
                Theme::dim(),
            )))
        }
    };

    render_table_with_detail(
        frame,
        area,
        "Standorte",
        &columns,
        &location_stats,
        Some(state.locations_selected),
        row_mapper,
        detail_renderer,
        &format!("{} Standorte", locations.len()),
    );
}

fn render_backup_tab(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Backup & Datenmanagement ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // DB info
            Constraint::Length(10), // Export
            Constraint::Length(6),  // Import
            Constraint::Min(1),     // Spacer
        ])
        .split(inner);

    // DB info
    let db_info = vec![Line::from(vec![
        Span::styled("Datenbank-Pfad: ", Theme::dim()),
        Span::styled(
            "~/.local/share/schliessfach-manager/schliessfach.db",
            Theme::normal(),
        ),
    ])];
    frame.render_widget(Paragraph::new(db_info), chunks[0]);

    // Export section
    let export_block = Block::default()
        .title(" Export ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let export_inner = export_block.inner(chunks[1]);
    frame.render_widget(export_block, chunks[1]);

    let export_lines = vec![
        Line::from(vec![
            Span::styled("[1]", Theme::primary_style()),
            Span::raw(" Vollständiger Export (JSON)"),
        ]),
        Line::from(Span::styled(
            "    Alle Daten (Schließfächer, Verleih, Zahlungen, Standorte)",
            Theme::dim(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("[2]", Theme::primary_style()),
            Span::raw(" Schließfächer (CSV)"),
        ]),
        Line::from(vec![
            Span::styled("[3]", Theme::primary_style()),
            Span::raw(" Aktive Verleih (CSV)"),
        ]),
        Line::from(vec![
            Span::styled("[4]", Theme::primary_style()),
            Span::raw(" Zahlungshistorie (CSV)"),
        ]),
    ];
    frame.render_widget(Paragraph::new(export_lines), export_inner);

    // Import section
    let import_block = Block::default()
        .title(" Import ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let import_inner = import_block.inner(chunks[2]);
    frame.render_widget(import_block, chunks[2]);

    let import_lines = vec![
        Line::from(vec![
            Span::styled("[5]", Theme::primary_style()),
            Span::raw(" Vollständiger Import (JSON)"),
        ]),
        Line::from(Span::styled(
            "    ⚠ WARNUNG: Überschreibt alle vorhandenen Daten!",
            Theme::warning(),
        )),
    ];
    frame.render_widget(Paragraph::new(import_lines), import_inner);
}

/// Renders the settings tab.
fn render_settings_tab(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Einstellungen ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8), // Financial settings
            Constraint::Length(6), // UI settings
            Constraint::Length(4), // App info
            Constraint::Min(1),    // Spacer
        ])
        .split(inner);

    // Financial settings section
    let financial_block = Block::default()
        .title(" Finanzielle Einstellungen ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let financial_inner = financial_block.inner(chunks[0]);
    frame.render_widget(financial_block, chunks[0]);

    let financial_lines = vec![
        Line::from(vec![
            Span::styled("Pfandbetrag:              ", Theme::dim()),
            Span::styled("10,00 €", Theme::normal()),
        ]),
        Line::from(vec![
            Span::styled("Jahresgebühr:             ", Theme::dim()),
            Span::styled("10,00 €", Theme::normal()),
        ]),
        Line::from(vec![
            Span::styled("Berechnungszeitraum:      ", Theme::dim()),
            Span::styled("Jährlich", Theme::normal()),
        ]),
        Line::from(vec![
            Span::styled("Währung:                  ", Theme::dim()),
            Span::styled("EUR", Theme::normal()),
        ]),
    ];
    frame.render_widget(Paragraph::new(financial_lines), financial_inner);

    // UI settings section
    let ui_block = Block::default()
        .title(" Benutzeroberfläche ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let ui_inner = ui_block.inner(chunks[1]);
    frame.render_widget(ui_block, chunks[1]);

    let ui_lines = vec![Line::from(vec![
        Span::styled("Screensaver Timeout:      ", Theme::dim()),
        Span::styled("60 Sekunden", Theme::normal()),
    ])];
    frame.render_widget(Paragraph::new(ui_lines), ui_inner);

    // App info section
    let info_lines = vec![Line::from(vec![
        Span::styled("Anwendungsversion:        ", Theme::dim()),
        Span::styled("2.1.0", Theme::normal()),
    ])];
    frame.render_widget(Paragraph::new(info_lines), chunks[2]);
}

/// Renders the audit log tab.
fn render_audit_tab(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Audit-Protokoll ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // Filter info
            Constraint::Min(10),   // Log entries
        ])
        .split(inner);

    // Filter info
    let filter_info = Line::from(vec![
        Span::styled("Filter: ", Theme::dim()),
        Span::styled("Alle Einträge", Theme::normal()),
        Span::styled(" | ", Theme::dim()),
        Span::styled("Letzte 100 Einträge", Theme::dim()),
    ]);
    frame.render_widget(Paragraph::new(filter_info), chunks[0]);

    // Log entries placeholder
    let log_block = Block::default()
        .title(" Aktionen ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let log_inner = log_block.inner(chunks[1]);
    frame.render_widget(log_block, chunks[1]);

    let log_lines = vec![
        Line::from(Span::styled(
            "Keine Audit-Einträge vorhanden.",
            Theme::dim(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Aktionen werden hier protokolliert sobald sie ausgeführt werden.",
            Theme::dim(),
        )),
    ];
    frame.render_widget(Paragraph::new(log_lines), log_inner);
}

fn render_footer(frame: &mut Frame, area: Rect, tab: ManagementTab) {
    let help_text = match tab {
        ManagementTab::Lockers => {
            "[N] Neu | [B] Bulk-Erstellung | [E] Bearbeiten | [R] Repariert | [D] Löschen | [Tab] Nächster Tab"
        }
        ManagementTab::Locations => "[N] Neu | [E] Bearbeiten | [D] Löschen | [Tab] Nächster Tab",
        ManagementTab::Settings => "[E] Bearbeiten | [S] Speichern | [Tab] Nächster Tab",
        ManagementTab::AuditLog => "[↑/↓] Navigieren | [F] Filtern | [Tab] Nächster Tab",
        ManagementTab::Backup => "[1-5] Aktion wählen | [Tab] Nächster Tab",
    };

    let footer = Paragraph::new(Line::from(Span::styled(help_text, Theme::dim())));
    frame.render_widget(footer, area);
}
