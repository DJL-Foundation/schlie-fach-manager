pub mod screens;
pub mod widgets;
pub mod theme;
pub mod state;

pub use theme::Theme;
pub use state::{UiState, InactivityState};
pub use widgets::{
    Header, Keybind, KeybindBar, KeybindScope,
    StatusBar, StatusLevel, StatusMessage,
    WizardRenderer, WizardMessage, MessageSender, MessageContent, WizardOption, WizardAction,
    OccupancyGraph, MessageBox,
};

// Temporary compatibility shim for old render function
// This will be replaced during integration phase
use crate::{
    app::{App, InputMode},
    model::LockerStatus,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use unicode_width::UnicodeWidthStr;

const SEARCH_PROMPT: &str = "Suche (/ zum Start, ENTER zum Bestätigen, ESC zum Abbrechen)";

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(frame.size());

    frame.render_widget(render_search_bar(app), areas[0]);

    if matches!(app.input_mode, InputMode::Searching) {
        let cursor_x = areas[0].x + 1 + search_prefix_width(app) + 1 + app.search_width() as u16;
        let cursor_y = areas[0].y + 1;
        frame.set_cursor(cursor_x, cursor_y);
    }

    frame.render_widget(render_locker_table(app), areas[1]);
    frame.render_widget(render_status_line(app), areas[2]);
}

fn render_search_bar(app: &App) -> Paragraph<'_> {
    let (mode_label, mode_style) = match app.input_mode {
        InputMode::Normal => (
            " NORMAL ",
            Style::default().fg(Color::Black).bg(Color::Gray),
        ),
        InputMode::Searching => (
            " SEARCH ",
            Style::default().fg(Color::Black).bg(Color::Yellow),
        ),
    };

    let prompt = Span::styled(SEARCH_PROMPT, Style::default().fg(Color::Gray));

    let query_style = if app.input_mode == InputMode::Searching {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::UNDERLINED)
    } else {
        Style::default().fg(Color::White)
    };

    Paragraph::new(Line::from(vec![
        Span::styled(mode_label, mode_style.add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        prompt,
        Span::styled(format!(" {}", app.search_query), query_style),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Filter")
            .title_alignment(ratatui::layout::Alignment::Left),
    )
}

fn render_locker_table<'a>(app: &'a App) -> Table<'a> {
    let header = Row::new(vec![
        header_cell("ID"),
        header_cell("Bezeichnung"),
        header_cell("Status"),
        header_cell("Belegt durch"),
        header_cell("Notiz"),
    ]);

    let selected_idx = app.selected_index();

    let rows: Vec<Row<'a>> = app
        .visible_lockers()
        .enumerate()
        .map(|(idx, locker)| {
            let mut row = Row::new(vec![
                Cell::from(locker.id.to_string()),
                Cell::from(locker.label.clone()),
                status_cell(locker.status),
                Cell::from(locker.occupant.clone().unwrap_or_else(|| "-".into())),
                Cell::from(locker.note.clone().unwrap_or_else(|| "-".into())),
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

    Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Percentage(30),
            Constraint::Length(14),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Schließfächer ({} Treffer)", app.visible_count())),
    )
    .column_spacing(2)
}

fn header_cell(text: &str) -> Cell<'_> {
    Cell::from(Span::styled(
        text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

fn status_cell(status: LockerStatus) -> Cell<'static> {
    let (label, color) = match status {
        LockerStatus::Available => ("frei", Color::Green),
        LockerStatus::Occupied => ("belegt", Color::Yellow),
        LockerStatus::Maintenance => ("wartung", Color::Red),
    };

    Cell::from(Span::styled(
        label,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}

fn search_prefix_width(app: &App) -> u16 {
    let mode_label = match app.input_mode {
        InputMode::Normal => " NORMAL ",
        InputMode::Searching => " SEARCH ",
    };
    let prefix = format!("{} {}", mode_label, SEARCH_PROMPT);
    UnicodeWidthStr::width(prefix.as_str()) as u16
}

fn render_status_line(app: &App) -> Paragraph<'_> {
    let content = app
        .status_message
        .as_deref()
        .map(|msg| {
            Span::styled(
                msg,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        })
        .unwrap_or_else(|| {
            Span::styled(
                "q: Beenden · Enter: belegen · Backspace: freigeben · m: Wartung umschalten · /: suchen",
                Style::default().fg(Color::DarkGray),
            )
        });

    Paragraph::new(Line::from(vec![content]))
}
