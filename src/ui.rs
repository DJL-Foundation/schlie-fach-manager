use crate::{
    app::{
        AdminSubTab, App, AppTab, ExtendWizardData, ExtendWizardState, InputMode, ManagementSubTab,
        PopupType, RentalWizardData, RentalWizardState, ReturnWizardData, ReturnWizardState,
    },
    model::{LockerHeight, LockerStatus, TenantType},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, Gauge, List, ListItem, Paragraph, Row, Table,
        Tabs, Wrap,
    },
};

const HELP_TEXT_NORMAL: &str = "q: Beenden · Tab/Shift+Tab: Tab wechseln · ←→: Sub-Tab · ↑↓/jk: Navigation · /: Suchen · Enter: Auswahl · c: Chat-Modus · m: Formular-Modus";
const HELP_TEXT_SEARCH: &str = "ESC: Abbrechen · Enter: Bestätigen · Eingabe zum Filtern";
const HELP_TEXT_WIZARD: &str = "ESC: Abbrechen · Enter: Bestätigen · ↑↓: Auswahl";
const HELP_TEXT_POPUP: &str = "Enter/j: Bestätigen · ESC/n: Abbrechen";

// UI layout constants
const SEARCH_PROMPT_PREFIX: &str = " / zum Suchen: ";

/// Top-level render function
pub fn render(frame: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header/Tabs
            Constraint::Min(10),    // Content
            Constraint::Length(1),  // Status line
        ])
        .split(frame.size());

    render_tabs(frame, app, chunks[0]);
    render_content(frame, app, chunks[1]);
    render_status_line(frame, app, chunks[2]);

    // Render popup if active
    if let Some(popup) = &app.popup {
        render_popup(frame, popup);
    }
}

fn render_tabs(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let titles: Vec<Line<'_>> = AppTab::all()
        .iter()
        .map(|tab| {
            let style = if *tab == app.current_tab {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(Span::styled(tab.display_name(), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🔐 Schließfach Manager ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .select(app.current_tab.index())
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "));

    frame.render_widget(tabs, area);
}

fn render_content(frame: &mut Frame<'_>, app: &App, area: Rect) {
    match app.current_tab {
        AppTab::Dashboard => render_dashboard(frame, app, area),
        AppTab::Management => render_management(frame, app, area),
        AppTab::Finance => render_finance(frame, app, area),
        AppTab::Admin => render_admin(frame, app, area),
    }
}

// ========== Dashboard ==========

fn render_dashboard(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Stats
            Constraint::Min(5),     // Location overview
            Constraint::Length(10), // Action items
        ])
        .split(area);

    render_stats_widgets(frame, app, chunks[0]);
    render_location_overview(frame, app, chunks[1]);
    render_action_items(frame, app, chunks[2]);
}

fn render_stats_widgets(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    // Occupancy gauge
    let occupancy_pct = if app.statistics.total > 0 {
        (app.statistics.occupied as f64 / app.statistics.total as f64 * 100.0) as u16
    } else {
        0
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 📊 Belegung ")
                .border_type(BorderType::Rounded),
        )
        .gauge_style(
            Style::default()
                .fg(if occupancy_pct > 80 {
                    Color::Red
                } else if occupancy_pct > 60 {
                    Color::Yellow
                } else {
                    Color::Green
                })
                .bg(Color::DarkGray),
        )
        .percent(occupancy_pct)
        .label(format!(
            "{}/{} belegt ({}%)",
            app.statistics.occupied, app.statistics.total, occupancy_pct
        ));
    frame.render_widget(gauge, chunks[0]);

    // Free/Total stats
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Frei: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.statistics.free),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Belegt: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.statistics.occupied),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Wartung: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.statistics.maintenance),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Defekt: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.statistics.damaged),
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let stats_para = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" 📈 Übersicht ")
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(stats_para, chunks[1]);

    // Financial preview
    let potential_revenue = app.debtors.iter().map(|d| d.amount_owed).sum::<i64>();
    let finance_text = vec![
        Line::from(vec![
            Span::styled("Offene Forderungen: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}€", potential_revenue),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Schuldner: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.debtors.len()),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let finance_para = Paragraph::new(finance_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" 💰 Finanzen ")
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(finance_para, chunks[2]);
}

fn render_location_overview(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let rows: Vec<Row<'_>> = app
        .location_statistics
        .iter()
        .map(|stat| {
            let pct = if stat.total > 0 {
                (stat.occupied as f64 / stat.total as f64 * 100.0) as u16
            } else {
                0
            };
            let bar = create_bar(pct, 20);
            Row::new(vec![
                Cell::from(stat.location_name.clone()),
                Cell::from(format!("{}/{}", stat.occupied, stat.total)),
                Cell::from(bar),
                Cell::from(format!("{}%", pct)),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Standort",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Belegt",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Balken",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "%",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
    ]);

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Percentage(40),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" 🏢 Standort-Übersicht ")
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(table, area);
}

fn create_bar(pct: u16, width: usize) -> String {
    let filled = (pct as usize * width / 100).min(width);
    let empty = width - filled;
    let mut bar = String::with_capacity(width * 3); // UTF-8 chars can be up to 3 bytes
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..empty {
        bar.push('░');
    }
    bar
}

fn render_action_items(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Urgent repairs
    let damaged_items: Vec<ListItem<'_>> = app
        .lockers
        .iter()
        .filter(|l| l.is_damaged)
        .take(5)
        .map(|l| {
            ListItem::new(format!(
                "⚠️  {} - {}",
                l.display_number,
                l.note.as_deref().unwrap_or("Defekt")
            ))
            .style(Style::default().fg(Color::Red))
        })
        .collect();

    let repairs_list = List::new(damaged_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🔧 Dringende Reparaturen ")
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(repairs_list, chunks[0]);

    // Overdue payments
    let debtor_items: Vec<ListItem<'_>> = app
        .debtors
        .iter()
        .take(5)
        .map(|d| {
            ListItem::new(format!(
                "💸 {} - {}€ ({} Tage)",
                d.username, d.amount_owed, d.days_overdue
            ))
            .style(Style::default().fg(Color::Yellow))
        })
        .collect();

    let debtors_list = List::new(debtor_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 📋 Überfällige Zahlungen ")
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(debtors_list, chunks[1]);
}

// ========== Management ==========

fn render_management(frame: &mut Frame<'_>, app: &App, area: Rect) {
    // Check if wizard is active
    if app.rental_wizard.is_some() || app.extend_wizard.is_some() || app.return_wizard.is_some() {
        render_wizard(frame, app, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Sub-tabs
            Constraint::Length(3), // Search bar
            Constraint::Min(5),    // Content
        ])
        .split(area);

    render_subtabs(frame, app, chunks[0], ManagementSubTab::all(), app.management_subtab);
    render_search_bar(frame, app, chunks[1]);

    match app.management_subtab {
        ManagementSubTab::Search => render_management_search(frame, app, chunks[2]),
        ManagementSubTab::List => render_locker_list(frame, app, chunks[2]),
        ManagementSubTab::Extend => render_extend_return(frame, app, chunks[2]),
        ManagementSubTab::Damage => render_damage_report(frame, app, chunks[2]),
    }
}

fn render_subtabs<T: Copy + PartialEq>(
    frame: &mut Frame<'_>,
    _app: &App,
    area: Rect,
    subtabs: &[T],
    current: T,
) where
    T: SubTabDisplay,
{
    let titles: Vec<Line<'_>> = subtabs
        .iter()
        .map(|tab| {
            let style = if *tab == current {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(Span::styled(tab.display_name(), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .divider(Span::raw(" | "));

    frame.render_widget(tabs, area);
}

trait SubTabDisplay {
    fn display_name(&self) -> &'static str;
}

impl SubTabDisplay for ManagementSubTab {
    fn display_name(&self) -> &'static str {
        ManagementSubTab::display_name(self)
    }
}

impl SubTabDisplay for AdminSubTab {
    fn display_name(&self) -> &'static str {
        AdminSubTab::display_name(self)
    }
}

fn render_search_bar(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let (mode_label, mode_style) = match app.input_mode {
        InputMode::Normal => (" NORMAL ", Style::default().fg(Color::Black).bg(Color::Gray)),
        InputMode::Searching => (" SUCHE ", Style::default().fg(Color::Black).bg(Color::Yellow)),
        InputMode::Wizard => (" WIZARD ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        InputMode::FormInput => (" EINGABE ", Style::default().fg(Color::Black).bg(Color::Green)),
        InputMode::Popup => (" POPUP ", Style::default().fg(Color::Black).bg(Color::Magenta)),
    };

    let query_style = if app.input_mode == InputMode::Searching {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::UNDERLINED)
    } else {
        Style::default().fg(Color::White)
    };

    let search_para = Paragraph::new(Line::from(vec![
        Span::styled(mode_label, mode_style.add_modifier(Modifier::BOLD)),
        Span::raw(SEARCH_PROMPT_PREFIX),
        Span::styled(&app.search_query, query_style),
        if app.input_mode == InputMode::Searching {
            Span::styled("▌", Style::default().fg(Color::White))
        } else {
            Span::raw("")
        },
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Filter "));

    frame.render_widget(search_para, area);

    // Set cursor position for search mode
    if app.input_mode == InputMode::Searching {
        let cursor_x = area.x + 1 + mode_label.len() as u16 + SEARCH_PROMPT_PREFIX.len() as u16 + app.search_width() as u16;
        let cursor_y = area.y + 1;
        frame.set_cursor(cursor_x, cursor_y);
    }
}

fn render_management_search(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Schließfach Verleihen",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Drücke 'n' um einen neuen Verleihvorgang zu starten."),
        Line::from(""),
        Line::from("Der Wizard führt dich durch folgende Schritte:"),
        Line::from("  1. Standort auswählen"),
        Line::from("  2. Höhe auswählen (Oben/Mitte/Unten)"),
        Line::from("  3. Schließfach bestätigen"),
        Line::from("  4. IServ Benutzername eingeben"),
        Line::from("  5. Mietertyp wählen (Schüler/Lehrer)"),
        Line::from("  6. Bezahlung bestätigen (20€)"),
        Line::from(""),
        Line::from(vec![
            Span::raw("Aktuell freie Schließfächer: "),
            Span::styled(
                format!("{}", app.statistics.free),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let para = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🔑 Neuen Mietvertrag anlegen ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(para, area);
}

fn render_locker_list(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Nr.",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Standort",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Höhe",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Status",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Mieter",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Notiz",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
    ]);

    let selected_idx = app.selected_idx();

    let rows: Vec<Row<'_>> = app
        .visible_lockers()
        .enumerate()
        .map(|(idx, locker)| {
            let status_cell = match locker.status {
                LockerStatus::Free => Cell::from(Span::styled(
                    "Frei",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )),
                LockerStatus::Occupied => Cell::from(Span::styled(
                    "Belegt",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )),
                LockerStatus::Maintenance => Cell::from(Span::styled(
                    "Wartung",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
            };

            let damaged_marker = if locker.is_damaged { " ⚠️" } else { "" };

            let mut row = Row::new(vec![
                Cell::from(format!("{}{}", locker.display_number, damaged_marker)),
                Cell::from(locker.location_name.as_deref().unwrap_or("-").to_string()),
                Cell::from(locker.height.display_name()),
                status_cell,
                Cell::from(locker.tenant_username.as_deref().unwrap_or("-").to_string()),
                Cell::from(locker.note.as_deref().unwrap_or("-").to_string()),
            ]);

            if Some(idx) == selected_idx {
                row = row.style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                );
            }

            row
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(12),
            Constraint::Percentage(20),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" 📦 Schließfächer ({} Treffer) ", app.visible_count()))
            .border_type(BorderType::Rounded),
    )
    .column_spacing(1);

    frame.render_widget(table, area);
}

fn render_extend_return(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Extend section
    let extend_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Mietvertrag verlängern",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Drücke 'e' um einen Vertrag zu verlängern."),
        Line::from(""),
        Line::from("Preise:"),
        Line::from("  • 10€ = 1 Jahr Verlängerung"),
        Line::from("  • 20€ = 2 Jahre Verlängerung"),
        Line::from("  • usw."),
    ];

    let extend_para = Paragraph::new(extend_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 📅 Verlängern ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(extend_para, chunks[0]);

    // Return section
    let return_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Schließfach zurückgeben",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Drücke 'r' um ein Schließfach zurückzunehmen."),
        Line::from(""),
        Line::from("Wichtig:"),
        Line::from("  • Offene Schulden werden geprüft"),
        Line::from("  • Pfand (10€) zurückgeben!"),
    ];

    let return_para = Paragraph::new(return_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🔙 Rückgabe ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(return_para, chunks[1]);
}

fn render_damage_report(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Damaged lockers list
    let damaged_items: Vec<Row<'_>> = app
        .lockers
        .iter()
        .filter(|l| l.is_damaged)
        .map(|l| {
            Row::new(vec![
                Cell::from(l.display_number.clone()),
                Cell::from(l.location_name.as_deref().unwrap_or("-").to_string()),
                Cell::from(l.note.as_deref().unwrap_or("-").to_string()),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Nr.",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Standort",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Beschreibung",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
    ]);

    let table = Table::new(
        damaged_items,
        [
            Constraint::Length(12),
            Constraint::Percentage(30),
            Constraint::Percentage(50),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" ⚠️ Defekte Schließfächer ")
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(table, chunks[0]);

    // Actions
    let action_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Defekt melden",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("1. Wähle ein Schließfach in der Liste"),
        Line::from("2. Drücke 'd' für Defekt melden"),
        Line::from("3. Gib eine Beschreibung ein"),
        Line::from(""),
        Line::from(Span::styled(
            "Reparatur markieren",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Drücke 'f' um ein Schließfach"),
        Line::from("als repariert zu markieren."),
    ];

    let action_para = Paragraph::new(action_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🔧 Aktionen ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(action_para, chunks[1]);
}

// ========== Wizard Rendering ==========

fn render_wizard(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Chat area
    render_chat(frame, app, chunks[0]);

    // Input/Selection area
    render_wizard_input(frame, app, chunks[1]);
}

fn render_chat(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let messages: Vec<ListItem<'_>> = app
        .chat_messages
        .iter()
        .map(|msg| {
            let (prefix, style) = if msg.from_bot {
                ("🤖 ", Style::default().fg(Color::Cyan))
            } else {
                ("👤 ", Style::default().fg(Color::Green))
            };
            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(&msg.text, style),
            ]))
        })
        .collect();

    let list = List::new(messages).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" 💬 Chat ")
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(list, area);
}

fn render_wizard_input(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(area);

    // Selection or info based on current wizard state
    if let Some(wizard) = &app.rental_wizard {
        render_rental_wizard_options(frame, app, wizard, chunks[0]);
    } else if let Some(wizard) = &app.extend_wizard {
        render_extend_wizard_options(frame, app, wizard, chunks[0]);
    } else if let Some(wizard) = &app.return_wizard {
        render_return_wizard_options(frame, app, wizard, chunks[0]);
    }

    // Input field
    let input = Paragraph::new(Line::from(vec![
        Span::raw("> "),
        Span::styled(&app.form_input, Style::default().fg(Color::White)),
        Span::styled("▌", Style::default().fg(Color::White)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Eingabe ")
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(input, chunks[1]);

    // Set cursor for wizard input
    if app.input_mode == InputMode::Wizard {
        let cursor_x = chunks[1].x + 3 + app.form_input_width() as u16;
        let cursor_y = chunks[1].y + 1;
        frame.set_cursor(cursor_x, cursor_y);
    }
}

fn render_rental_wizard_options(frame: &mut Frame<'_>, app: &App, wizard: &RentalWizardData, area: Rect) {
    match wizard.state {
        RentalWizardState::AskLocation => {
            let items: Vec<ListItem<'_>> = app
                .locations
                .iter()
                .enumerate()
                .map(|(idx, loc)| {
                    let style = if idx == app.location_selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    let prefix = if idx == app.location_selected { "▶ " } else { "  " };
                    ListItem::new(format!("{}{}", prefix, loc.name)).style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Standort wählen (↑↓ + Enter) ")
                    .border_type(BorderType::Rounded),
            );

            frame.render_widget(list, area);
        }
        RentalWizardState::AskHeight => {
            let items: Vec<ListItem<'_>> = LockerHeight::all()
                .iter()
                .enumerate()
                .map(|(idx, height)| {
                    let style = if idx == app.selected_index {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    let prefix = if idx == app.selected_index { "▶ " } else { "  " };
                    ListItem::new(format!("{}{}", prefix, height.display_name())).style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Höhe wählen (↑↓ + Enter) ")
                    .border_type(BorderType::Rounded),
            );

            frame.render_widget(list, area);
        }
        RentalWizardState::AskTenantType => {
            let items: Vec<ListItem<'_>> = TenantType::all()
                .iter()
                .enumerate()
                .map(|(idx, tt)| {
                    let style = if idx == app.selected_index {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    let prefix = if idx == app.selected_index { "▶ " } else { "  " };
                    ListItem::new(format!("{}{}", prefix, tt.display_name())).style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Mietertyp wählen (↑↓ + Enter) ")
                    .border_type(BorderType::Rounded),
            );

            frame.render_widget(list, area);
        }
        _ => {
            let info = Paragraph::new("Bitte Eingabe im Textfeld machen und Enter drücken.")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Info ")
                        .border_type(BorderType::Rounded),
                )
                .wrap(Wrap { trim: true });

            frame.render_widget(info, area);
        }
    }
}

fn render_extend_wizard_options(frame: &mut Frame<'_>, _app: &App, wizard: &ExtendWizardData, area: Rect) {
    let info_text = match wizard.state {
        ExtendWizardState::AskIdentifier => "Gib die Schließfachnummer oder den Benutzernamen ein.",
        ExtendWizardState::ConfirmLease => "Bestätige mit 'j' oder 'n'.",
        ExtendWizardState::AskPayment => "Gib den erhaltenen Betrag ein (Standard: 10€).",
        ExtendWizardState::ConfirmExtend => "Bestätige die Verlängerung mit 'j' oder 'n'.",
        ExtendWizardState::Complete => "Verlängerung abgeschlossen.",
    };

    let info = Paragraph::new(info_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Verlängerung ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(info, area);
}

fn render_return_wizard_options(frame: &mut Frame<'_>, _app: &App, wizard: &ReturnWizardData, area: Rect) {
    let info_text = match wizard.state {
        ReturnWizardState::AskIdentifier => "Gib die Schließfachnummer oder den Benutzernamen ein.",
        ReturnWizardState::ConfirmLease => "Bestätige mit 'j' oder 'n'.",
        ReturnWizardState::CheckDebt => "Prüfe ob Schulden bezahlt wurden.",
        ReturnWizardState::ConfirmDepositReturn => "Bestätige Pfandrückgabe mit 'j' oder 'n'.",
        ReturnWizardState::Complete => "Rückgabe abgeschlossen.",
    };

    let mut lines = vec![Line::from(info_text)];
    
    if wizard.debt_amount > 0 {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("⚠️ Offene Schulden: {}€", wizard.debt_amount),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
    }

    let info = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Rückgabe ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(info, area);
}

// ========== Finance ==========

fn render_finance(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Debtors table
    render_debtors_table(frame, app, chunks[0]);

    // Finance summary
    render_finance_summary(frame, app, chunks[1]);
}

fn render_debtors_table(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from(Span::styled(
            "E-Mail",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Schließfach",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Betrag",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            "Tage überfällig",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
    ]);

    let rows: Vec<Row<'_>> = app
        .debtors
        .iter()
        .map(|d| {
            Row::new(vec![
                Cell::from(d.email.clone()),
                Cell::from(d.locker_display_number.clone()),
                Cell::from(Span::styled(
                    format!("{}€", d.amount_owed),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Cell::from(format!("{}", d.days_overdue)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Length(10),
            Constraint::Length(15),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" 💸 Schuldner-Liste ({}) ", app.debtors.len()))
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(table, area);
}

fn render_finance_summary(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let total_debt: i64 = app.debtors.iter().map(|d| d.amount_owed).sum();
    let active_leases = app.active_leases.len();

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Gesamt offene Forderungen: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}€", total_debt),
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Anzahl Schuldner: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.debtors.len()),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Aktive Mietverträge: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", active_leases),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Drücke 'x' für Export (JSON/CSV)",
            Style::default().fg(Color::Cyan),
        )),
    ];

    let para = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 📊 Zusammenfassung ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(para, area);
}

// ========== Admin ==========

fn render_admin(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Sub-tabs
            Constraint::Min(5),    // Content
        ])
        .split(area);

    render_subtabs(frame, app, chunks[0], AdminSubTab::all(), app.admin_subtab);

    match app.admin_subtab {
        AdminSubTab::Locations => render_admin_locations(frame, app, chunks[1]),
        AdminSubTab::Lockers => render_admin_lockers(frame, app, chunks[1]),
        AdminSubTab::Bulk => render_admin_bulk(frame, app, chunks[1]),
        AdminSubTab::Backup => render_admin_backup(frame, app, chunks[1]),
    }
}

fn render_admin_locations(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Locations list
    let items: Vec<ListItem<'_>> = app
        .locations
        .iter()
        .enumerate()
        .map(|(idx, loc)| {
            let style = if idx == app.location_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if idx == app.location_selected { "▶ " } else { "  " };
            ListItem::new(format!("{}{} (ID: {})", prefix, loc.name, loc.id)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" 🏢 Standorte ({}) ", app.locations.len()))
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(list, chunks[0]);

    // Actions
    let action_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Aktionen",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  a: Neuen Standort hinzufügen"),
        Line::from("  d: Ausgewählten Standort löschen"),
        Line::from("  ↑↓: Navigation"),
    ];

    let action_para = Paragraph::new(action_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hilfe ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(action_para, chunks[1]);
}

fn render_admin_lockers(frame: &mut Frame<'_>, app: &App, area: Rect) {
    // Reuse the locker list
    render_locker_list(frame, app, area);
}

fn render_admin_bulk(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Bulk-Erstellung von Schließfächern",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Mit dieser Funktion können mehrere Schließfächer"),
        Line::from("auf einmal erstellt werden."),
        Line::from(""),
        Line::from("Drücke 'b' um den Bulk-Wizard zu starten."),
        Line::from(""),
        Line::from("Der Wizard fragt nach:"),
        Line::from("  • Standort"),
        Line::from("  • Präfix (z.B. 'A-')"),
        Line::from("  • Startnummer"),
        Line::from("  • Endnummer"),
        Line::from("  • Höhe (Oben/Mitte/Unten)"),
        Line::from(""),
        Line::from(vec![
            Span::raw("Beispiel: Präfix 'B-', Start 1, Ende 50 "),
            Span::raw("→ B-1, B-2, ..., B-50"),
        ]),
    ];

    let para = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 📦 Bulk-Anlage ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(para, area);
}

fn render_admin_backup(frame: &mut Frame<'_>, _app: &App, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Backup & Wiederherstellung",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Drücke 's' um ein Backup zu erstellen."),
        Line::from("Drücke 'l' um ein Backup zu laden."),
        Line::from(""),
        Line::from("Das Backup wird als SQLite-Datei gespeichert"),
        Line::from("und kann bei Bedarf wiederhergestellt werden."),
    ];

    let para = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 💾 Backup ")
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(para, area);
}

// ========== Popup ==========

fn render_popup(frame: &mut Frame<'_>, popup: &PopupType) {
    let area = centered_rect(60, 30, frame.size());
    frame.render_widget(Clear, area);

    let (title, message, color) = match popup {
        PopupType::Confirmation { message, .. } => ("❓ Bestätigung", message.as_str(), Color::Yellow),
        PopupType::Info { message } => ("ℹ️ Info", message.as_str(), Color::Cyan),
        PopupType::Error { message } => ("❌ Fehler", message.as_str(), Color::Red),
    };

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(
            HELP_TEXT_POPUP,
            Style::default().fg(Color::Gray),
        )),
    ];

    let popup_widget = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .title(format!(" {} ", title))
                .border_type(BorderType::Double),
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    frame.render_widget(popup_widget, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// ========== Status Line ==========

fn render_status_line(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let help_text = match app.input_mode {
        InputMode::Normal => HELP_TEXT_NORMAL,
        InputMode::Searching => HELP_TEXT_SEARCH,
        InputMode::Wizard | InputMode::FormInput => HELP_TEXT_WIZARD,
        InputMode::Popup => HELP_TEXT_POPUP,
    };

    let content = if let Some(msg) = &app.status_message {
        Span::styled(
            msg,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(help_text, Style::default().fg(Color::DarkGray))
    };

    let status_line = Paragraph::new(Line::from(vec![content]));
    frame.render_widget(status_line, area);
}
