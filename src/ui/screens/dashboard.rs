use crate::models::DashboardStats;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    style::Stylize,
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph, Wrap},
};

/// Renders the dashboard screen.
pub fn render_dashboard(frame: &mut Frame, area: Rect, stats: &DashboardStats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),  // Title
            Constraint::Min(10),    // Main content
            Constraint::Length(2),  // Footer
        ])
        .split(area);

    // Title
    let title = Paragraph::new(Line::from(vec![
        Span::styled("Schließfach-Manager v2.0", Theme::title()),
        Span::raw(" - "),
        Span::styled("Dashboard", Theme::dim()),
    ]));
    frame.render_widget(title, chunks[0]);

    // Main content - split into columns
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Left column
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // Belegung
            Constraint::Min(6),     // Aktionen erforderlich
        ])
        .split(main_chunks[0]);

    render_occupancy_box(frame, left_chunks[0], stats);
    render_actions_box(frame, left_chunks[1], stats);

    // Right column
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Standorte
            Constraint::Length(6),  // Finanzen
            Constraint::Min(5),     // Graph
        ])
        .split(main_chunks[1]);

    render_locations_box(frame, right_chunks[0], stats);
    render_finance_box(frame, right_chunks[1], stats);
    render_graph_box(frame, right_chunks[2], stats);

    // Footer with shortcuts
    render_footer(frame, chunks[2]);
}

fn render_occupancy_box(frame: &mut Frame, area: Rect, stats: &DashboardStats) {
    let block = Block::default()
        .title(" Belegung ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let free = stats.free_lockers();
    let occupancy_pct = stats.occupancy_percentage();
    let damage_pct = stats.damage_percentage();

    let lines = vec![
        Line::from(vec![
            Span::raw("Gesamt: "),
            Span::styled(stats.total_lockers.to_string(), Theme::normal().bold()),
        ]),
        Line::from(vec![
            Span::raw("Belegt: "),
            Span::styled(
                format!("{} ({:.0}%)", stats.occupied_lockers, occupancy_pct),
                Theme::status_occupied(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Frei:   "),
            Span::styled(
                format!("{} ({:.0}%)", free, 100.0 - occupancy_pct),
                Theme::status_free(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Defekt: "),
            Span::styled(
                format!("{} ({:.0}%)", stats.damaged_lockers, damage_pct),
                Theme::status_damaged(),
            ),
        ]),
        Line::from(vec![
            Span::raw("  davon belegt: "),
            Span::styled(stats.damaged_and_occupied.to_string(), Theme::dim()),
        ]),
        Line::from(""),
        Line::from(render_progress_bar(occupancy_pct as u8, 20)),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_locations_box(frame: &mut Frame, area: Rect, stats: &DashboardStats) {
    let block = Block::default()
        .title(" Standorte ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    for loc in &stats.locations {
        let pct = loc.occupancy_percentage() as u8;
        lines.push(Line::from(vec![
            Span::styled(format!("{:15}", loc.location), Theme::normal()),
            Span::raw(" "),
            Span::styled(format!("{:3}/{:3}", loc.occupied, loc.total), Theme::dim()),
            Span::raw(" "),
            Span::raw(render_progress_bar(pct, 10)),
        ]));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_finance_box(frame: &mut Frame, area: Rect, stats: &DashboardStats) {
    let block = Block::default()
        .title(" Finanzen ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let revenue_euros = stats.total_revenue_cents as f64 / 100.0;
    let outstanding_euros = stats.outstanding_payments_cents as f64 / 100.0;

    let lines = vec![
        Line::from(vec![
            Span::raw("Einnahmen (gesamt):  "),
            Span::styled(format!("{:>10.2} €", revenue_euros), Theme::success()),
        ]),
        Line::from(vec![
            Span::raw("Ausstehend:          "),
            Span::styled(format!("{:>10.2} €", outstanding_euros), Theme::warning()),
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_actions_box(frame: &mut Frame, area: Rect, stats: &DashboardStats) {
    let block = Block::default()
        .title(" Aktionen erforderlich ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    if !stats.overdue_rentals.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("• ", Theme::error()),
            Span::styled(
                format!("{} Verlängerungen überfällig", stats.overdue_rentals.len()),
                Theme::error(),
            ),
        ]));
    }

    if stats.damaged_lockers > 0 {
        lines.push(Line::from(vec![
            Span::styled("• ", Theme::warning()),
            Span::styled(
                format!("{} Schließfächer defekt", stats.damaged_lockers),
                Theme::warning(),
            ),
        ]));
    }

    if !stats.expiring_soon.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("• ", Theme::info_style()),
            Span::styled(
                format!("{} Verträge laufen in 30 Tagen aus", stats.expiring_soon.len()),
                Theme::normal(),
            ),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "Keine Aktionen erforderlich",
            Theme::dim(),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_graph_box(frame: &mut Frame, area: Rect, _stats: &DashboardStats) {
    let block = Block::default()
        .title(" Trend (12 Monate) ")
        .borders(Borders::ALL)
        .border_style(Theme::dim());

    frame.render_widget(block, area);

    // Placeholder for actual graph - would need historical data
    // For now, just show a simple bar representation
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("[1]", Theme::primary_style()),
        Span::raw(" Verleihen | "),
        Span::styled("[2]", Theme::primary_style()),
        Span::raw(" Liste | "),
        Span::styled("[3]", Theme::primary_style()),
        Span::raw(" Verlängern | "),
        Span::styled("[4]", Theme::primary_style()),
        Span::raw(" Zurückgeben | "),
        Span::styled("[Tab]", Theme::primary_style()),
        Span::raw(" Menüs | "),
        Span::styled("[Q]", Theme::primary_style()),
        Span::raw(" Beenden"),
    ]));
    frame.render_widget(footer, area);
}

fn render_progress_bar(percentage: u8, width: usize) -> String {
    let filled = (percentage as usize * width / 100).min(width);
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
