mod app;
mod config;
mod db;
mod model;
mod screensaver;
mod ui;

use std::{
    io::stdout,
    time::{Duration, Instant},
};

use app::{App, AppScreen, RentalTab, ManagementTab};
use color_eyre::eyre::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use db::Database;
use model::Locker;
use ratatui::{Terminal, backend::CrosstermBackend, Frame};
use ui::{InactivityState, screens::dashboard::DashboardScreen};

const TICK_RATE: Duration = Duration::from_millis(16); // ~60 FPS for smooth animations
const DEFAULT_OCCUPANT: &str = "Unbekannt";

fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialize database (migrations run automatically on open)
    let mut db = Database::open_default()?;
    db.seed_if_empty(&default_seed())?;

    // Create app with v2.1 architecture
    let app = App::new(db)?;
    start_terminal(app)
}

fn default_seed() -> Vec<Locker> {
    vec![
        Locker::new(1, "A-01"),
        Locker::new(2, "A-02"),
        Locker::new(3, "B-01"),
        Locker::new(4, "B-02"),
        Locker::new(5, "C-01"),
    ]
}

fn start_terminal(mut app: App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let mut last_tick = Instant::now();

    loop {
        // Check inactivity state and handle screensaver activation/countdown
        match app.check_inactivity() {
            InactivityState::Active => {
                // Normal operation - no countdown message
            }
            InactivityState::Countdown(seconds) => {
                // Show countdown in status bar
                app.status_bar.set_message(
                    format!("⏱ Screensaver in {} Sekunden...", seconds),
                    ui::StatusLevel::Info,
                );
            }
            InactivityState::ScreensaverActive => {
                // Activate screensaver if not already active
                if !app.is_screensaver_active() {
                    app.activate_screensaver();
                }
            }
        }

        // Update screensaver animation if active
        if app.is_screensaver_active() {
            if let Some(screensaver) = app.screensaver_screen_mut() {
                screensaver.update();
            }
        }

        // Render the UI
        terminal.draw(|frame| render(frame, app))?;

        // Poll for events with timeout
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    // Update activity timestamp on any key press
                    app.update_activity();
                    
                    // Handle the key
                    if handle_key(app, key)? {
                        break; // Quit the app
                    }
                }
                Event::Resize(_, _) => {
                    // Force UI redraw on next iteration
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

/// Main render function that dispatches to the correct screen
/// Based on v2.1 spec section 5.6
fn render(frame: &mut Frame, app: &App) {
    // If screensaver is active, render fullscreen (no widgets)
    if app.is_screensaver_active() {
        if let Some(screensaver) = app.screensaver_screen() {
            screensaver.render(frame);
        }
        return;
    }

    // Otherwise, render with standard layout (header, content, keybind bar, status bar)
    let area = frame.area();
    
    // Create layout for the screen
    use ratatui::layout::{Constraint, Direction, Layout};
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // Header (with optional window switcher)
            Constraint::Min(10),    // Main content area
            Constraint::Length(3),  // Keybind bar
            Constraint::Length(1),  // Status bar
        ])
        .split(area);
    
    // Render header
    app.header.render(main_chunks[0], frame.buffer_mut(), &app.theme);
    
    // Render screen-specific content
    render_screen_content(main_chunks[1], frame, app);
    
    // Render keybind bar
    app.keybind_bar.render(main_chunks[2], frame.buffer_mut(), &app.theme);
    
    // Render status bar
    app.status_bar.render(main_chunks[3], frame.buffer_mut(), &app.theme);
}

/// Render the main content area based on the current screen
fn render_screen_content(area: ratatui::layout::Rect, frame: &mut Frame, app: &App) {
    match app.screen() {
        AppScreen::Dashboard => {
            // Create a dashboard screen instance and load data
            let mut dashboard = DashboardScreen::new();
            if let Ok(()) = dashboard.load_data(app.db().connection()) {
                dashboard.render(area, frame.buffer_mut(), &app.theme);
            } else {
                // Fallback to legacy render if dashboard data loading fails
                render_legacy_screen(area, frame, app);
            }
        }
        AppScreen::Screensaver => {
            // Should not reach here (screensaver is handled above)
            // But just in case, render nothing
        }
        _ => {
            // For other screens (RentalManagement, Finances, Management), 
            // render placeholder for now (will be implemented in future)
            render_placeholder_screen(area, frame, app);
        }
    }
}

/// Render a placeholder screen for unimplemented screens
fn render_placeholder_screen(area: ratatui::layout::Rect, frame: &mut Frame, app: &App) {
    use ratatui::{
        style::{Color, Style, Modifier},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph},
    };
    
    let screen_name = match app.screen() {
        AppScreen::RentalManagement(tab) => format!("Verleih-Management: {}", tab.name()),
        AppScreen::Finances => "Finanzen".to_string(),
        AppScreen::Management(tab) => format!("Verwaltung: {}", tab.name()),
        AppScreen::Dashboard => "Dashboard".to_string(),
        AppScreen::Screensaver => "Screensaver".to_string(),
    };
    
    let content = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("[ {} ]", screen_name),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Diese Ansicht wird in einer zukünftigen Version implementiert.",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Drücke ESC 3x, um zum Dashboard zurückzukehren.",
            Style::default().fg(Color::Yellow),
        )),
    ];
    
    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    
    frame.render_widget(paragraph, area);
}

/// Legacy render function for backward compatibility
/// This renders the old locker table view
fn render_legacy_screen(area: ratatui::layout::Rect, frame: &mut Frame, app: &App) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    };
    use unicode_width::UnicodeWidthStr;
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(area);
    
    // Render search bar
    let (mode_label, mode_style) = match app.input_mode {
        app::InputMode::Normal => (
            " NORMAL ",
            Style::default().fg(Color::Black).bg(Color::Gray),
        ),
        app::InputMode::Searching => (
            " SEARCH ",
            Style::default().fg(Color::Black).bg(Color::Yellow),
        ),
    };
    
    let prompt = Span::styled(
        "Suche (/ zum Start, ENTER zum Bestätigen, ESC zum Abbrechen)",
        Style::default().fg(Color::Gray),
    );
    
    let query_style = if app.input_mode == app::InputMode::Searching {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::UNDERLINED)
    } else {
        Style::default().fg(Color::White)
    };
    
    let search_bar = Paragraph::new(Line::from(vec![
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
    );
    
    frame.render_widget(search_bar, chunks[0]);
    
    // Set cursor if in search mode
    if matches!(app.input_mode, app::InputMode::Searching) {
        let mode_text = " NORMAL ";
        let prompt_text = "Suche (/ zum Start, ENTER zum Bestätigen, ESC zum Abbrechen)";
        let prefix_width = UnicodeWidthStr::width(mode_text) + 1 + UnicodeWidthStr::width(prompt_text) + 1;
        let cursor_x = chunks[0].x + 1 + prefix_width as u16 + app.search_width() as u16;
        let cursor_y = chunks[0].y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    }
    
    // Render locker table
    let header = Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Bezeichnung",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Status",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Belegt durch",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Notiz",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
    ]);
    
    let selected_idx = app.selected_index();
    
    let rows: Vec<Row> = app
        .visible_lockers()
        .enumerate()
        .map(|(idx, locker)| {
            let (status_label, status_color) = match locker.status {
                model::LockerStatus::Available => ("frei", Color::Green),
                model::LockerStatus::Occupied => ("belegt", Color::Yellow),
                model::LockerStatus::Maintenance => ("wartung", Color::Red),
            };
            
            let mut row = Row::new(vec![
                Cell::from(locker.id.to_string()),
                Cell::from(locker.label.clone()),
                Cell::from(Span::styled(
                    status_label,
                    Style::default().fg(status_color).add_modifier(Modifier::BOLD),
                )),
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
    
    let table = Table::new(
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
    .column_spacing(2);
    
    frame.render_widget(table, chunks[1]);
}

fn handle_key(app: &mut App, key: KeyEvent) -> Result<bool> {
    // Handle Ctrl+C as quit
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Ok(true);
    }
    
    // Handle global keybinds including Tab/Shift+Tab navigation
    match key.code {
        KeyCode::Tab => {
            // Tab: navigate forward (either in window switcher or screen-specific)
            if app.header.is_window_switcher_active() {
                app.header.select_next_window();
                return Ok(false);
            }
            // Screen-specific tab handling will be in app.handle_key
        }
        KeyCode::BackTab => {
            // Shift+Tab: navigate backward
            if app.header.is_window_switcher_active() {
                app.header.select_previous_window();
                return Ok(false);
            }
            // Screen-specific backtab handling will be in app.handle_key
        }
        _ => {}
    }
    
    // Delegate to app's handle_key which handles:
    // - Screensaver exit on any key
    // - Window switcher keys (^, Enter, Esc when active)
    // - Triple-escape to Dashboard
    // - Global keybinds (Shift+Q to quit)
    // - Screen-specific key handling
    app.handle_key(key.code)
}

fn capture(action: Result<()>) {
    if let Err(err) = action {
        eprintln!("Aktion fehlgeschlagen: {err:?}");
    }
}

fn occupant_from_search(app: &App) -> String {
    let trimmed = app.search_query.trim();
    if trimmed.is_empty() {
        DEFAULT_OCCUPANT.to_string()
    } else {
        trimmed.to_string()
    }
}

fn note_from_search(app: &App) -> Option<String> {
    let trimmed = app.search_query.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
