use crate::models::{DebtorInfo, PaymentSummary};
use crate::ui::state::FinanceTab;
use crate::ui::theme::Theme;
use crate::ui::widgets::{render_table_with_detail, ColumnDef};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Tabs},
};

/// State for finance screens.
#[derive(Debug, Default)]
pub struct FinanceState {
    pub selected_tab: FinanceTab,
    pub debtors_selected: usize,
    pub time_period: TimePeriod,
}

impl Default for FinanceTab {
    fn default() -> Self {
        FinanceTab::Overview
    }
}

/// Time period filter for finance overview.
#[derive(Debug, Clone, Copy, Default)]
pub enum TimePeriod {
    #[default]
    Lifetime,
    OneYear,
    SixMonths,
    ThreeMonths,
    OneMonth,
}

impl TimePeriod {
    pub fn label(&self) -> &'static str {
        match self {
            TimePeriod::Lifetime => "Lebenszeit",
            TimePeriod::OneYear => "1 Jahr",
            TimePeriod::SixMonths => "6 Monate",
            TimePeriod::ThreeMonths => "3 Monate",
            TimePeriod::OneMonth => "1 Monat",
        }
    }
}

impl FinanceState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_tab(&mut self) {
        let idx = self.selected_tab.index();
        let next = FinanceTab::from_index((idx + 1) % FinanceTab::all().len());
        self.selected_tab = next.unwrap_or(FinanceTab::Overview);
    }

    pub fn prev_tab(&mut self) {
        let idx = self.selected_tab.index();
        let prev = if idx == 0 {
            FinanceTab::all().len() - 1
        } else {
            idx - 1
        };
        self.selected_tab = FinanceTab::from_index(prev).unwrap_or(FinanceTab::Overview);
    }
}

/// Renders the finance screen.
pub fn render_finance(
    frame: &mut Frame,
    area: Rect,
    state: &FinanceState,
    summary: &PaymentSummary,
    debtors: &[DebtorInfo],
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
        FinanceTab::Overview => {
            render_overview_tab(frame, chunks[1], state, summary, debtors);
        }
        FinanceTab::Debtors => {
            render_debtors_tab(frame, chunks[1], state, debtors);
        }
    }

    // Render footer
    render_footer(frame, chunks[2], state.selected_tab);
}

fn render_tabs(frame: &mut Frame, area: Rect, selected: FinanceTab) {
    let titles: Vec<Line> = FinanceTab::all()
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
                .title(" Finanzen ")
                .borders(Borders::ALL)
                .border_style(Theme::title()),
        )
        .select(selected.index())
        .divider("|");

    frame.render_widget(tabs, area);
}

fn render_overview_tab(
    frame: &mut Frame,
    area: Rect,
    state: &FinanceState,
    summary: &PaymentSummary,
    debtors: &[DebtorInfo],
) {
    let block = Block::default()
        .title(" Finanzübersicht ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // Period selector
            Constraint::Length(9), // Einnahmen box
            Constraint::Length(7), // Ausstehend box
            Constraint::Min(3),    // Spacer
        ])
        .split(inner);

    // Period selector
    let period_line = Line::from(vec![
        Span::raw("Zeitraum: "),
        Span::styled(
            format!("[{}]", state.time_period.label()),
            Theme::button_selected(),
        ),
    ]);
    frame.render_widget(Paragraph::new(period_line), chunks[0]);

    // Einnahmen box
    let deposits_euros = summary.deposits_cents as f64 / 100.0;
    let extensions_euros = summary.extensions_cents as f64 / 100.0;
    let returns_euros = summary.deposit_returns_cents as f64 / 100.0;
    let total_euros = summary.total_cents as f64 / 100.0;
    let _net_euros = summary.net_cents as f64 / 100.0;

    let einnahmen_block = Block::default()
        .title(format!(
            " Einnahmen (Zeitraum: {}) ",
            state.time_period.label()
        ))
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let einnahmen_inner = einnahmen_block.inner(chunks[1]);
    frame.render_widget(einnahmen_block, chunks[1]);

    let einnahmen_lines = vec![
        Line::from(vec![
            Span::raw("Pfand-Einnahmen:       "),
            Span::styled(
                format!(
                    "{:>10.2} € ({} × 10€)",
                    deposits_euros, summary.deposit_count
                ),
                Theme::success(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Verlängerungen:        "),
            Span::styled(
                format!(
                    "{:>10.2} € ({} × 10€)",
                    extensions_euros, summary.extension_count
                ),
                Theme::success(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Pfand-Rückgaben:       "),
            Span::styled(
                format!(
                    "{:>10.2} € ({} × 10€)",
                    returns_euros, summary.deposit_return_count
                ),
                Theme::warning(),
            ),
        ]),
        Line::from(Span::raw("─".repeat(50))),
        Line::from(vec![
            Span::styled("Gesamt-Einnahmen:      ", Theme::normal()),
            Span::styled(format!("{:>10.2} €", total_euros), Theme::success()),
        ]),
    ];

    frame.render_widget(Paragraph::new(einnahmen_lines), einnahmen_inner);

    // Ausstehend box
    let ausstehend_block = Block::default()
        .title(" Ausstehende Zahlungen ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let ausstehend_inner = ausstehend_block.inner(chunks[2]);
    frame.render_widget(ausstehend_block, chunks[2]);

    let student_debt: i32 = debtors
        .iter()
        .filter(|d| d.tenant_type == crate::models::TenantType::Schüler)
        .map(|d| d.total_debt_cents)
        .sum();
    let teacher_debt: i32 = debtors
        .iter()
        .filter(|d| d.tenant_type == crate::models::TenantType::Lehrer)
        .map(|d| d.total_debt_cents)
        .sum();
    let total_debt = student_debt + teacher_debt;

    let student_count = debtors
        .iter()
        .filter(|d| d.tenant_type == crate::models::TenantType::Schüler)
        .count();
    let teacher_count = debtors
        .iter()
        .filter(|d| d.tenant_type == crate::models::TenantType::Lehrer)
        .count();

    let ausstehend_lines = vec![
        Line::from(vec![
            Span::raw("Überfällige Verlängerungen: "),
            Span::styled(
                format!(
                    "{:>10.2} € ({} Verleih)",
                    total_debt as f64 / 100.0,
                    debtors.len()
                ),
                Theme::warning(),
            ),
        ]),
        Line::from(vec![
            Span::raw("  davon Schüler:            "),
            Span::styled(
                format!(
                    "{:>10.2} € ({} Verleih)",
                    student_debt as f64 / 100.0,
                    student_count
                ),
                Theme::dim(),
            ),
        ]),
        Line::from(vec![
            Span::raw("  davon Lehrer:             "),
            Span::styled(
                format!(
                    "{:>10.2} € ({} Verleih)",
                    teacher_debt as f64 / 100.0,
                    teacher_count
                ),
                Theme::dim(),
            ),
        ]),
    ];

    frame.render_widget(Paragraph::new(ausstehend_lines), ausstehend_inner);
}

fn render_debtors_tab(frame: &mut Frame, area: Rect, state: &FinanceState, debtors: &[DebtorInfo]) {
    let columns = vec![
        ColumnDef::new("E-Mail", Constraint::Percentage(40)),
        ColumnDef::new("Typ", Constraint::Length(10)),
        ColumnDef::new("Schulden", Constraint::Length(12)),
        ColumnDef::new("Seit Tagen", Constraint::Length(12)),
    ];

    let row_mapper = |debtor: &DebtorInfo, _idx: usize| -> Row<'static> {
        Row::new(vec![
            Cell::from(debtor.email.clone()),
            Cell::from(debtor.tenant_type.as_str().to_string()),
            Cell::from(Span::styled(debtor.format_debt(), Theme::warning())),
            Cell::from(debtor.days_overdue.to_string()),
        ])
    };

    let detail_renderer = |debtor: Option<&DebtorInfo>| -> Text<'static> {
        if let Some(d) = debtor {
            let lines = vec![
                Line::from(vec![
                    Span::styled("Benutzername: ".to_string(), Theme::dim()),
                    Span::styled(d.username.clone(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("E-Mail: ".to_string(), Theme::dim()),
                    Span::styled(d.email.clone(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Typ: ".to_string(), Theme::dim()),
                    Span::styled(d.tenant_type.as_str().to_string(), Theme::normal()),
                ]),
                Line::from(vec![
                    Span::styled("Schulden: ".to_string(), Theme::dim()),
                    Span::styled(d.format_debt(), Theme::warning()),
                ]),
                Line::from(vec![
                    Span::styled("Überfällig seit: ".to_string(), Theme::dim()),
                    Span::styled(format!("{} Tagen", d.days_overdue), Theme::warning()),
                ]),
            ];
            Text::from(lines)
        } else {
            Text::from(Line::from(Span::styled(
                "Kein Schuldner ausgewählt".to_string(),
                Theme::dim(),
            )))
        }
    };

    // Calculate total
    let total_debt: i32 = debtors.iter().map(|d| d.total_debt_cents).sum();

    render_table_with_detail(
        frame,
        area,
        "Schuldentabelle",
        &columns,
        debtors,
        Some(state.debtors_selected),
        row_mapper,
        detail_renderer,
        &format!(
            "Gesamt: {} Schuldner, {:.2} € ausstehend",
            debtors.len(),
            total_debt as f64 / 100.0
        ),
    );
}

fn render_footer(frame: &mut Frame, area: Rect, tab: FinanceTab) {
    let help_text = match tab {
        FinanceTab::Overview => "[E] Exportieren | [Tab] Schuldentabelle",
        FinanceTab::Debtors => "[E] Exportieren | [/] Suchen | [↑↓] Navigation | [Tab] Übersicht",
    };

    let footer = Paragraph::new(Line::from(Span::styled(help_text, Theme::dim())));
    frame.render_widget(footer, area);
}
