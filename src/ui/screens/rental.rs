use crate::models::{Locker, RentalWithLocker};
use crate::ui::state::{InputMode, RentalManagementTab};
use crate::ui::theme::Theme;
use crate::ui::widgets::{render_table_with_detail, ColumnDef};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Tabs},
};

/// State for rental management screens.
#[derive(Debug, Default)]
pub struct RentalState {
    pub selected_tab: RentalManagementTab,
    pub list_selected: usize,
    pub search_query: String,
    pub input_mode: InputMode,
    pub filtered_indices: Vec<usize>,
}

impl RentalState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_tab(&mut self) {
        let idx = self.selected_tab.index();
        let next = RentalManagementTab::from_index((idx + 1) % RentalManagementTab::all().len());
        self.selected_tab = next.unwrap_or(RentalManagementTab::List);
    }

    pub fn prev_tab(&mut self) {
        let idx = self.selected_tab.index();
        let prev = if idx == 0 {
            RentalManagementTab::all().len() - 1
        } else {
            idx - 1
        };
        self.selected_tab =
            RentalManagementTab::from_index(prev).unwrap_or(RentalManagementTab::List);
    }
}

/// Renders the rental management screen.
pub fn render_rental_management(
    frame: &mut Frame,
    area: Rect,
    state: &RentalState,
    lockers: &[Locker],
    active_rentals: &[RentalWithLocker],
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
        RentalManagementTab::Search => {
            render_search_tab(frame, chunks[1], state);
        }
        RentalManagementTab::List => {
            render_list_tab(frame, chunks[1], state, lockers, active_rentals);
        }
        RentalManagementTab::Extend => {
            render_extend_tab(frame, chunks[1], state);
        }
        RentalManagementTab::Return => {
            render_return_tab(frame, chunks[1], state);
        }
        RentalManagementTab::Damage => {
            render_damage_tab(frame, chunks[1], state, lockers);
        }
    }

    // Render footer
    render_footer(frame, chunks[2], state.selected_tab);
}

fn render_tabs(frame: &mut Frame, area: Rect, selected: RentalManagementTab) {
    let titles: Vec<Line> = RentalManagementTab::all()
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
                .title(" Verleih-Verwaltung ")
                .borders(Borders::ALL)
                .border_style(Theme::title()),
        )
        .select(selected.index())
        .divider("|");

    frame.render_widget(tabs, area);
}

fn render_search_tab(frame: &mut Frame, area: Rect, _state: &RentalState) {
    let block = Block::default()
        .title(" Neuer Verleih ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "> Bevorzugter Standort?",
            Theme::primary_style(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("[Hauptgebäude]", Theme::button_selected()),
            Span::raw(" "),
            Span::styled("[Turnhalle]", Theme::button()),
            Span::raw(" "),
            Span::styled("[Neubau]", Theme::button()),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Drücke [Enter] um fortzufahren, [ESC] um abzubrechen",
            Theme::dim(),
        )),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_list_tab(
    frame: &mut Frame,
    area: Rect,
    state: &RentalState,
    lockers: &[Locker],
    active_rentals: &[RentalWithLocker],
) {
    // Build combined view with both locker and rental data
    #[derive(Clone)]
    struct LockerView {
        locker: Locker,
        tenant_username: Option<String>,
        tenant_type: Option<String>,
        rental_start: Option<String>,
        rental_end: Option<String>,
        days_until_expiration: Option<i64>,
    }

    let locker_views: Vec<LockerView> = lockers
        .iter()
        .map(|locker| {
            let rental = active_rentals.iter().find(|r| r.locker.id == locker.id);
            if let Some(r) = rental {
                LockerView {
                    locker: locker.clone(),
                    tenant_username: Some(r.rental.tenant_username.clone()),
                    tenant_type: Some(r.rental.tenant_type.as_str().to_string()),
                    rental_start: Some(r.rental.rental_start_date.to_string()),
                    rental_end: Some(r.rental.rental_end_date.to_string()),
                    days_until_expiration: Some(r.rental.days_until_expiration()),
                }
            } else {
                LockerView {
                    locker: locker.clone(),
                    tenant_username: None,
                    tenant_type: None,
                    rental_start: None,
                    rental_end: None,
                    days_until_expiration: None,
                }
            }
        })
        .collect();

    let columns = vec![
        ColumnDef::new("ID", Constraint::Length(10)),
        ColumnDef::new("Standort", Constraint::Percentage(25)),
        ColumnDef::new("Höhe", Constraint::Length(8)),
        ColumnDef::new("Mieter", Constraint::Percentage(25)),
        ColumnDef::new("Status", Constraint::Length(12)),
    ];

    let row_mapper = |view: &LockerView, _idx: usize| -> Row<'static> {
        let (tenant, status, status_style) = if let Some(ref username) = view.tenant_username {
            let days = view.days_until_expiration.unwrap_or(0);
            let status_text = if days < 0 {
                format!("Überfällig ({}T)", days.abs())
            } else if days < 30 {
                format!("Läuft aus ({}T)", days)
            } else {
                "Belegt".to_string()
            };
            let style = if days < 0 {
                Theme::status_overdue()
            } else if days < 30 {
                Theme::status_expiring()
            } else {
                Theme::status_occupied()
            };
            (username.clone(), status_text, style)
        } else if view.locker.is_damaged {
            (
                "-".to_string(),
                "Defekt".to_string(),
                Theme::status_damaged(),
            )
        } else {
            ("-".to_string(), "Frei".to_string(), Theme::status_free())
        };

        Row::new(vec![
            Cell::from(view.locker.label.clone()),
            Cell::from(view.locker.location.clone()),
            Cell::from(format!("{} cm", view.locker.height)),
            Cell::from(tenant),
            Cell::from(Span::styled(status, status_style)),
        ])
    };

    let detail_renderer = |view: Option<&LockerView>| -> Text<'static> {
        if let Some(v) = view {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("Label: ".to_string(), Theme::dim()),
                    Span::styled(v.locker.label.clone(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Standort: ".to_string(), Theme::dim()),
                    Span::styled(v.locker.location.clone(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Höhe: ".to_string(), Theme::dim()),
                    Span::styled(format!("{} cm", v.locker.height), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Defekt: ".to_string(), Theme::dim()),
                    if v.locker.is_damaged {
                        Span::styled("Ja ⚠".to_string(), Theme::status_damaged())
                    } else {
                        Span::styled("Nein".to_string(), Theme::status_free())
                    },
                ]),
                Line::from("".to_string()),
            ];

            if let Some(ref username) = v.tenant_username {
                lines.push(Line::from(Span::styled(
                    "─ Aktueller Verleih ─".to_string(),
                    Theme::dim(),
                )));
                lines.push(Line::from(vec![
                    Span::styled("Mieter: ".to_string(), Theme::dim()),
                    Span::styled(username.clone(), Theme::normal()),
                ]));
                if let Some(ref tenant_type) = v.tenant_type {
                    lines.push(Line::from(vec![
                        Span::styled("Typ: ".to_string(), Theme::dim()),
                        Span::styled(tenant_type.clone(), Theme::normal()),
                    ]));
                }
                if let Some(ref start) = v.rental_start {
                    lines.push(Line::from(vec![
                        Span::styled("Beginn: ".to_string(), Theme::dim()),
                        Span::styled(start.clone(), Theme::normal()),
                    ]));
                }
                if let Some(ref end) = v.rental_end {
                    lines.push(Line::from(vec![
                        Span::styled("Ende: ".to_string(), Theme::dim()),
                        Span::styled(end.clone(), Theme::normal()),
                    ]));
                }

                let days = v.days_until_expiration.unwrap_or(0);
                let status_style = if days < 0 {
                    Theme::status_overdue()
                } else if days < 30 {
                    Theme::status_expiring()
                } else {
                    Theme::status_occupied()
                };
                let status_text = if days < 0 {
                    format!("Überfällig seit {} Tagen", days.abs())
                } else if days < 30 {
                    format!("Läuft aus in {} Tagen", days)
                } else {
                    format!("Noch {} Tage", days)
                };
                lines.push(Line::from(vec![
                    Span::styled("Status: ".to_string(), Theme::dim()),
                    Span::styled(status_text, status_style),
                ]));
            } else {
                lines.push(Line::from(Span::styled(
                    "Kein aktiver Verleih".to_string(),
                    Theme::dim(),
                )));
            }

            lines.push(Line::from("".to_string()));
            lines.push(Line::from(vec![
                Span::styled("[E]".to_string(), Theme::primary_style()),
                Span::raw(" Bearbeiten  ".to_string()),
                Span::styled("[N]".to_string(), Theme::primary_style()),
                Span::raw(" Neuer Verleih".to_string()),
            ]));

            Text::from(lines)
        } else {
            Text::from(Line::from(Span::styled(
                "Kein Schließfach ausgewählt".to_string(),
                Theme::dim(),
            )))
        }
    };

    render_table_with_detail(
        frame,
        area,
        "Schließfächer",
        &columns,
        &locker_views,
        Some(state.list_selected),
        row_mapper,
        detail_renderer,
        "",
    );
}

fn render_extend_tab(frame: &mut Frame, area: Rect, _state: &RentalState) {
    let block = Block::default()
        .title(" Verleih verlängern ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "> Erinnert sich der Mieter an die Schließfach-Nummer?",
            Theme::primary_style(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("[Ja]", Theme::button_selected()),
            Span::raw(" "),
            Span::styled("[Nein]", Theme::button()),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Drücke [Enter] um fortzufahren, [ESC] um abzubrechen",
            Theme::dim(),
        )),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_return_tab(frame: &mut Frame, area: Rect, _state: &RentalState) {
    let block = Block::default()
        .title(" Schließfach zurückgeben ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "> Erinnert sich der Mieter an die Schließfach-Nummer?",
            Theme::primary_style(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("[Ja]", Theme::button_selected()),
            Span::raw(" "),
            Span::styled("[Nein]", Theme::button()),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Drücke [Enter] um fortzufahren, [ESC] um abzubrechen",
            Theme::dim(),
        )),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_damage_tab(frame: &mut Frame, area: Rect, _state: &RentalState, _lockers: &[Locker]) {
    let block = Block::default()
        .title(" Defekt melden ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("Schließfach-Nummer: "),
            Span::styled("[________]", Theme::input()),
        ]),
        Line::from(""),
        Line::from(Span::styled("Notizen (optional):", Theme::dim())),
        Line::from(Span::styled(
            "[_______________________________]",
            Theme::input(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "⚠ Hinweis: Ein defektes Schließfach kann weiterhin verliehen sein!",
            Theme::warning(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Drücke [Enter] um als defekt zu markieren, [ESC] um abzubrechen",
            Theme::dim(),
        )),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_footer(frame: &mut Frame, area: Rect, tab: RentalManagementTab) {
    let help_text = match tab {
        RentalManagementTab::List => {
            "[/] Suchen | [↑↓] Navigation | [E] Bearbeiten | [N] Neuer Verleih | [Tab] Nächster Tab"
        }
        _ => "[Enter] Bestätigen | [ESC] Abbrechen | [Tab] Nächster Tab",
    };

    let footer = Paragraph::new(Line::from(Span::styled(help_text, Theme::dim())));
    frame.render_widget(footer, area);
}
