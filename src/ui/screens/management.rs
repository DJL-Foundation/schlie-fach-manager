use crate::db::audit::AuditEntry;
use crate::db::settings::AppSettings;
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
    /// Selected setting index for editing.
    pub settings_selected: usize,
    /// Currently editing setting (if any).
    pub editing_setting: Option<String>,
    /// Audit log scroll position.
    pub audit_scroll: usize,
    /// Audit filter: None = all, Some(entity_type) = filtered.
    pub audit_filter: Option<String>,
    /// Export format selection (0=TOML, 1=JSON, 2=CSV, 3=MD).
    pub export_format: usize,
    /// Import dialog active.
    pub import_dialog_active: bool,
    /// Export dialog active.
    pub export_dialog_active: bool,
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
    settings: Option<&AppSettings>,
    audit_entries: &[AuditEntry],
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
            render_settings_tab(frame, chunks[1], state, settings);
        }
        ManagementTab::AuditLog => {
            render_audit_tab(frame, chunks[1], state, audit_entries);
        }
        ManagementTab::Backup => {
            render_backup_tab(frame, chunks[1], state);
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

fn render_backup_tab(frame: &mut Frame, area: Rect, state: &ManagementState) {
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
            Constraint::Length(12), // Export
            Constraint::Length(8),  // Import
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

    // Export section with format selection
    let export_block = Block::default()
        .title(" Export ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let export_inner = export_block.inner(chunks[1]);
    frame.render_widget(export_block, chunks[1]);

    let formats = ["TOML", "JSON", "CSV", "Markdown"];
    let format_line = Line::from(vec![
        Span::styled("Format: ", Theme::dim()),
        Span::styled(
            format!("[{}]", formats[state.export_format]),
            Theme::primary_style(),
        ),
        Span::styled(" (← / → zum Wechseln)", Theme::dim()),
    ]);

    let export_lines = vec![
        format_line,
        Line::from(""),
        Line::from(vec![
            Span::styled("[1]", Theme::primary_style()),
            Span::raw(" Vollständiger Export"),
        ]),
        Line::from(Span::styled(
            "    Alle Daten (Schließfächer, Verleih, Zahlungen, Standorte)",
            Theme::dim(),
        )),
        Line::from(vec![
            Span::styled("[2]", Theme::primary_style()),
            Span::raw(" Nur Schließfächer"),
        ]),
        Line::from(vec![
            Span::styled("[3]", Theme::primary_style()),
            Span::raw(" Nur aktive Verleih"),
        ]),
        Line::from(vec![
            Span::styled("[4]", Theme::primary_style()),
            Span::raw(" Zahlungshistorie"),
        ]),
    ];
    frame.render_widget(Paragraph::new(export_lines), export_inner);

    // Import section with format info
    let import_block = Block::default()
        .title(" Import ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let import_inner = import_block.inner(chunks[2]);
    frame.render_widget(import_block, chunks[2]);

    let import_lines = vec![
        Line::from(vec![
            Span::styled("[5]", Theme::primary_style()),
            Span::raw(" Vollständiger Import (TOML/JSON)"),
        ]),
        Line::from(Span::styled(
            "    ⚠ WARNUNG: Überschreibt alle vorhandenen Daten!",
            Theme::warning(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("[6]", Theme::primary_style()),
            Span::raw(" Schließfächer importieren (CSV)"),
        ]),
        Line::from(vec![
            Span::styled("[7]", Theme::primary_style()),
            Span::raw(" Verleih importieren (CSV)"),
        ]),
    ];
    frame.render_widget(Paragraph::new(import_lines), import_inner);
}

/// Renders the settings tab with editable values from AppSettings.
fn render_settings_tab(frame: &mut Frame, area: Rect, state: &ManagementState, settings: Option<&AppSettings>) {
    let settings = settings.cloned().unwrap_or_default();

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
            Constraint::Length(10), // Financial settings
            Constraint::Length(6),  // UI settings
            Constraint::Length(4),  // App info
            Constraint::Min(1),     // Spacer
        ])
        .split(inner);

    // Financial settings section with selection highlighting
    let financial_block = Block::default()
        .title(" Finanzielle Einstellungen ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let financial_inner = financial_block.inner(chunks[0]);
    frame.render_widget(financial_block, chunks[0]);

    // Settings with selection indicator
    let setting_items = [
        ("Pfandbetrag", format_cents(settings.deposit_cents)),
        ("Jahresgebühr", format_cents(settings.yearly_fee_cents)),
        ("Berechnungszeitraum", settings.billing_period.clone()),
        ("Währung", settings.currency.clone()),
    ];

    let financial_lines: Vec<Line> = setting_items
        .iter()
        .enumerate()
        .map(|(i, (label, value))| {
            let is_selected = state.settings_selected == i;
            let prefix = if is_selected { "▸ " } else { "  " };
            let label_style = if is_selected {
                Theme::primary_style()
            } else {
                Theme::dim()
            };
            let value_style = if is_selected {
                Theme::highlight()
            } else {
                Theme::normal()
            };
            Line::from(vec![
                Span::styled(prefix.to_string(), label_style),
                Span::styled(format!("{:<25}", label), label_style),
                Span::styled(value.clone(), value_style),
            ])
        })
        .collect();

    frame.render_widget(Paragraph::new(financial_lines), financial_inner);

    // UI settings section
    let ui_block = Block::default()
        .title(" Benutzeroberfläche ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let ui_inner = ui_block.inner(chunks[1]);
    frame.render_widget(ui_block, chunks[1]);

    let screensaver_selected = state.settings_selected == 4;
    let ss_prefix = if screensaver_selected { "▸ " } else { "  " };
    let ss_label_style = if screensaver_selected {
        Theme::primary_style()
    } else {
        Theme::dim()
    };
    let ss_value_style = if screensaver_selected {
        Theme::highlight()
    } else {
        Theme::normal()
    };

    let ui_lines = vec![
        Line::from(vec![
            Span::styled(ss_prefix.to_string(), ss_label_style),
            Span::styled("Screensaver Timeout       ".to_string(), ss_label_style),
            Span::styled(
                format!("{} Sekunden", settings.screensaver_timeout_seconds),
                ss_value_style,
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "[↑/↓] Auswählen | [Enter] Bearbeiten | [S] Speichern",
            Theme::dim(),
        )),
    ];
    frame.render_widget(Paragraph::new(ui_lines), ui_inner);

    // App info section
    let info_lines = vec![
        Line::from(vec![
            Span::styled("Anwendungsversion:        ", Theme::dim()),
            Span::styled(settings.app_version, Theme::normal()),
        ]),
    ];
    frame.render_widget(Paragraph::new(info_lines), chunks[2]);
}

/// Formats cents as Euro string (e.g., 1000 -> "10,00 €").
fn format_cents(cents: i32) -> String {
    let euros = cents / 100;
    let remainder = (cents % 100).abs();
    format!("{},{:02} €", euros, remainder)
}

/// Renders the audit log tab with actual entries.
fn render_audit_tab(frame: &mut Frame, area: Rect, state: &ManagementState, entries: &[AuditEntry]) {
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

    // Filter info with interactive filter options
    let filter_text = state
        .audit_filter
        .as_ref()
        .map(|f| f.as_str())
        .unwrap_or("Alle");

    let filter_info = Line::from(vec![
        Span::styled("Filter: ", Theme::dim()),
        Span::styled(format!("[{}]", filter_text), Theme::primary_style()),
        Span::styled(" | ", Theme::dim()),
        Span::styled(format!("{} Einträge", entries.len()), Theme::normal()),
        Span::styled(" | [F] Filter ändern | [↑/↓] Scrollen", Theme::dim()),
    ]);
    frame.render_widget(Paragraph::new(filter_info), chunks[0]);

    // Log entries list
    let log_block = Block::default()
        .title(" Aktionen ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let log_inner = log_block.inner(chunks[1]);
    frame.render_widget(log_block, chunks[1]);

    if entries.is_empty() {
        let empty_lines = vec![
            Line::from(""),
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
        frame.render_widget(Paragraph::new(empty_lines), log_inner);
    } else {
        // Show entries with scroll offset
        let visible_height = log_inner.height.saturating_sub(1) as usize;
        let start = state.audit_scroll;
        let end = (start + visible_height).min(entries.len());

        let log_lines: Vec<Line> = entries[start..end]
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let is_selected = i == 0 && state.audit_scroll == start;
                let timestamp = entry.timestamp.format("%d.%m.%Y %H:%M").to_string();
                let action_style = match entry.action.as_str() {
                    "CREATE" => Theme::status_free(),
                    "DELETE" => Theme::status_damaged(),
                    "UPDATE" => Theme::warning(),
                    _ => Theme::normal(),
                };

                let prefix = if is_selected { "▸ " } else { "  " };
                Line::from(vec![
                    Span::styled(prefix.to_string(), Theme::dim()),
                    Span::styled(format!("{} ", timestamp), Theme::dim()),
                    Span::styled(format!("{:<8}", entry.action), action_style),
                    Span::styled(format!(" {} ", entry.entity_type), Theme::normal()),
                    Span::styled(
                        entry.details.clone().unwrap_or_default(),
                        Theme::dim(),
                    ),
                ])
            })
            .collect();

        frame.render_widget(Paragraph::new(log_lines), log_inner);
    }
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
