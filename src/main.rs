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

use app::{App, InputMode};
use color_eyre::eyre::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use db::Database;
use model::Locker;
use ratatui::{Terminal, backend::CrosstermBackend};

const TICK_RATE: Duration = Duration::from_millis(250);
const DEFAULT_OCCUPANT: &str = "Unbekannt";

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut db = Database::open_default()?;
    db.seed_if_empty(&default_seed())?;

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
        terminal.draw(|frame| ui::render(frame, app))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if handle_key(app, key)? {
                        break;
                    }
                }
                Event::Resize(_, _) => {
                    // force UI redraw on next iteration
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

fn handle_key(app: &mut App, key: KeyEvent) -> Result<bool> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Ok(true);
    }

    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Searching => handle_search_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('/') => {
            app.set_input_mode(InputMode::Searching);
        }
        KeyCode::Char('j') | KeyCode::Down => app.next(),
        KeyCode::Char('k') | KeyCode::Up => app.previous(),
        KeyCode::Char('r') => capture(app.reload()),
        KeyCode::Enter => {
            let occupant = occupant_from_search(app);
            capture(app.assign_selected(occupant));
        }
        KeyCode::Backspace => capture(app.release_selected()),
        KeyCode::Char('m') => {
            let note = note_from_search(app);
            capture(app.toggle_maintenance(note));
        }
        _ => {}
    }

    Ok(false)
}

fn handle_search_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.clear_search();
            app.set_input_mode(InputMode::Normal);
        }
        KeyCode::Enter => {
            app.set_input_mode(InputMode::Normal);
        }
        KeyCode::Backspace => {
            app.pop_search_char();
        }
        KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.push_search_char(ch);
        }
        _ => {}
    }

    Ok(false)
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
