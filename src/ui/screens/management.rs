use crate::{
    app::ManagementTab,
    db,
    ui::{state::SettingsEditor, theme::Theme, widgets::table::TableWidget},
};
use anyhow::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Tabs, Widget},
};

/// Render the management screen.
pub fn render(
    area: Rect,
    buf: &mut Buffer,
    tab: &ManagementTab,
    settings_editor: Option<&SettingsEditor>,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    render_tabs(chunks[0], buf, tab, theme);

    match tab {
        ManagementTab::Lockers => render_lockers(chunks[1], buf, db, theme)?,
        ManagementTab::Locations => render_locations(chunks[1], buf, db, theme)?,
        ManagementTab::Settings => {
            if let Some(editor) = settings_editor {
                editor.render(chunks[1], buf);
            }
        }
        ManagementTab::AuditLog => render_audit_log(chunks[1], buf, db, theme)?,
    }

    Ok(())
}

/// Render the management tabs.
fn render_tabs(area: Rect, buf: &mut Buffer, tab: &ManagementTab, theme: &Theme) {
    let titles = ManagementTab::all()
        .iter()
        .map(|tab| Span::styled(tab.label(), Style::default()))
        .collect::<Vec<_>>();
    let index = ManagementTab::all()
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
        .block(Block::default().borders(Borders::ALL).title("Verwaltung"));
    tabs.render(area, buf);
}

/// Render the lockers table.
fn render_lockers(
    area: Rect,
    buf: &mut Buffer,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let lockers = db::lockers::list_lockers(db.connection())?;
    let rows = lockers
        .iter()
        .map(|locker| {
            vec![
                locker.id.to_string(),
                locker.number.clone(),
                locker.location.clone(),
                locker.size.clone(),
                if locker.is_damaged { "defekt" } else { "ok" }.to_string(),
            ]
        })
        .collect();

    let table = TableWidget {
        title: "Schließfächer",
        headers: vec!["ID", "Nummer", "Standort", "Größe", "Status"],
        rows,
        widths: vec![
            Constraint::Length(6),
            Constraint::Length(12),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Length(8),
        ],
    };
    table.render(area, buf, theme);
    Ok(())
}

/// Render the locations table.
fn render_locations(
    area: Rect,
    buf: &mut Buffer,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let locations = db::locations::list_locations(db.connection())?;
    let rows = locations
        .iter()
        .map(|location| vec![location.id.to_string(), location.name.clone()])
        .collect();

    let table = TableWidget {
        title: "Standorte",
        headers: vec!["ID", "Name"],
        rows,
        widths: vec![Constraint::Length(6), Constraint::Percentage(80)],
    };
    table.render(area, buf, theme);
    Ok(())
}

/// Render the audit log table.
fn render_audit_log(
    area: Rect,
    buf: &mut Buffer,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let entries = db::audit::list_audit_entries(db.connection())?;
    let rows = entries
        .iter()
        .map(|entry| {
            vec![
                entry.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                entry.action.clone(),
                entry.entity_type.clone(),
                entry
                    .entity_id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "-".to_string()),
            ]
        })
        .collect();

    let table = TableWidget {
        title: "Audit Log",
        headers: vec!["Zeit", "Aktion", "Typ", "ID"],
        rows,
        widths: vec![
            Constraint::Length(18),
            Constraint::Length(10),
            Constraint::Percentage(40),
            Constraint::Length(6),
        ],
    };
    table.render(area, buf, theme);
    Ok(())
}
