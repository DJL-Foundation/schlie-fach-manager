use crate::{
    app::{ActiveWorkflow, RentalTab},
    db,
    ui::{theme::Theme, widgets::table::TableWidget},
};
use anyhow::Result;
use chrono::Utc;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Tabs, Widget},
};

/// Render the rental management screen.
pub fn render(
    area: Rect,
    buf: &mut Buffer,
    tab: &RentalTab,
    workflow: Option<&ActiveWorkflow>,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    render_tabs(chunks[0], buf, tab, theme);

    if let Some(active_workflow) = workflow {
        active_workflow.renderer().render(chunks[1], buf, theme);
        return Ok(());
    }

    if matches!(tab, RentalTab::List) {
        render_list(chunks[1], buf, db, theme)?;
    } else {
        let placeholder = Block::default().borders(Borders::ALL).title("Workflow");
        placeholder.render(chunks[1], buf);
    }

    Ok(())
}

/// Render the rental management tabs.
fn render_tabs(area: Rect, buf: &mut Buffer, tab: &RentalTab, theme: &Theme) {
    let titles = RentalTab::all()
        .iter()
        .map(|tab| Span::styled(tab.label(), Style::default()))
        .collect::<Vec<_>>();
    let index = RentalTab::all()
        .iter()
        .position(|candidate| candidate == tab)
        .unwrap_or(0);

    let tabs = Tabs::new(titles)
        .select(index)
        .highlight_style(
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Verleih-Management"),
        );
    tabs.render(area, buf);
}

/// Render the rental list table.
fn render_list(
    area: Rect,
    buf: &mut Buffer,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let rentals = db::rentals::list_rentals(db.connection())?;
    let lockers = db::lockers::list_lockers(db.connection())?;
    let today = Utc::now().date_naive();

    let rows = rentals
        .iter()
        .map(|rental| {
            let locker_label = lockers
                .iter()
                .find(|locker| locker.id == rental.locker_id)
                .map(|locker| locker.number.as_str())
                .unwrap_or("-");
            let status = if rental.is_active_on(today) {
                "aktiv"
            } else {
                "abgelaufen"
            };
            vec![
                rental.id.to_string(),
                locker_label.to_string(),
                rental.renter_name.clone(),
                format!("{} → {}", rental.start_date, rental.end_date),
                status.to_string(),
            ]
        })
        .collect::<Vec<_>>();

    let table = TableWidget {
        title: "Verleihliste",
        headers: vec!["ID", "Schließfach", "Verleiher", "Zeitraum", "Status"],
        rows,
        widths: vec![
            Constraint::Length(6),
            Constraint::Length(12),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Length(10),
        ],
    };
    table.render(area, buf, theme);
    Ok(())
}
