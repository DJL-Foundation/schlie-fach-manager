mod app;
mod config;
mod db;
mod export;
mod import;
mod model;
mod screensaver;
mod ui;
mod workflows;

use anyhow::{Result, anyhow};
use app::{App, AppAction, InactivityState};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use db::Database;
use model::Locker;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

const TICK_RATE: Duration = Duration::from_millis(250);

/// Application entry point.
fn main() -> Result<()> {
    color_eyre::install().map_err(|err| anyhow!(err))?;

    let db = Database::open_default()?;
    seed_if_empty(&db)?;

    let mut app = App::new(db)?;
    start_terminal(&mut app)
}

/// Seeds the database with default lockers if needed.
fn seed_if_empty(db: &Database) -> Result<()> {
    if db::lockers::count_lockers(db.connection())? == 0 {
        let seed = vec![
            Locker::new(1, "A-01", "Hauptgebäude", "Klein"),
            Locker::new(2, "A-02", "Hauptgebäude", "Klein"),
            Locker::new(3, "B-01", "Nebengebäude", "Mittel"),
            Locker::new(4, "B-02", "Nebengebäude", "Mittel"),
            Locker::new(5, "C-01", "Keller", "Groß"),
        ];
        for locker in seed {
            db::lockers::upsert_locker(db.connection(), &locker)?;
        }
    }
    Ok(())
}

/// Starts the terminal UI runtime.
fn start_terminal(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let result = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show)?;
    terminal.show_cursor()?;

    result
}

/// Runs the main event loop and rendering logic.
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let mut last_tick = Instant::now();

    loop {
        match app.check_inactivity() {
            InactivityState::Active => {}
            InactivityState::Countdown(seconds) => {
                app.status_bar_mut().show_screensaver_countdown(seconds);
            }
            InactivityState::ScreensaverActive => {
                app.activate_screensaver();
            }
        }

        terminal.draw(|frame| ui::render(frame, app))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_millis(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.update_activity();
                    if app.screensaver_active() {
                        app.exit_screensaver();
                    } else if matches!(app.handle_key(key)?, AppAction::Quit) {
                        break;
                    }
                }
            }
        }

        if app.screensaver_active() {
            app.screensaver_screen_mut().update();
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }
    }

    Ok(())
}
