//! Schließfach-Manager - Terminal-based Locker Management System
//!
//! This is the main binary entry point for the Schließfach-Manager application.
//! For library usage, see the crate documentation.

use schliessfach_manager::{
    app::App,
    db::{self, Database},
    ui::{
        self,
        state::{AppScreen, InputMode, RentalManagementTab, ManagementTab},
    },
};

use std::{
    io::stdout,
    time::{Duration, Instant},
};

use color_eyre::eyre::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

/// Tick rate for the main event loop (250ms).
const TICK_RATE: Duration = Duration::from_millis(250);

/// Main entry point for the Schließfach-Manager application.
fn main() -> Result<()> {
    color_eyre::install()?;

    let db = Database::open_default()?;
    db::seed::seed_test_data(&db.conn)?;

    let app = App::new(db)?;
    start_terminal(app)
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
        terminal.draw(|frame| render_ui(frame, app))?;

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
            app.clear_expired_notifications();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn render_ui(frame: &mut ratatui::prelude::Frame, app: &App) {
    
    use ui::screens::{render_dashboard, render_finance, render_management, render_rental_management};
    use ui::widgets::render_notification;

    let area = frame.size();

    // Main content
    match &app.screen {
        AppScreen::Dashboard => {
            render_dashboard(frame, area, &app.dashboard_stats);
        }
        AppScreen::RentalManagement(_) => {
            render_rental_management(
                frame,
                area,
                &app.rental_state,
                &app.lockers,
                &app.active_rentals,
            );
        }
        AppScreen::Finance(_) => {
            render_finance(
                frame,
                area,
                &app.finance_state,
                &app.payment_summary,
                &app.debtors,
            );
        }
        AppScreen::Management(_) => {
            render_management(
                frame,
                area,
                &app.management_state,
                &app.lockers,
                &app.locations,
            );
        }
    }

    // Render notification overlay if present
    if let Some(ref notification) = app.notification {
        render_notification(frame, area, notification);
    }

    // Render confirmation dialog if present
    if let Some(ref dialog) = app.confirm_dialog {
        ui::widgets::render_confirmation_dialog(frame, area, dialog);
    }
}

fn handle_key(app: &mut App, key: KeyEvent) -> Result<bool> {
    // Global shortcuts
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Ok(true);
    }

    // Handle confirmation dialog
    if app.confirm_dialog.is_some() {
        return handle_dialog_key(app, key);
    }

    // Handle based on input mode
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Editing => handle_editing_mode(app, key),
    }
}

fn handle_dialog_key(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
            if let Some(ref mut dialog) = app.confirm_dialog {
                dialog.toggle_selection();
            }
        }
        KeyCode::Enter => {
            if let Some(ref dialog) = app.confirm_dialog {
                if dialog.is_confirmed() {
                    // Handle confirmed action based on context
                    // This would be expanded based on the specific dialog
                }
            }
            app.clear_confirm_dialog();
        }
        KeyCode::Esc => {
            app.clear_confirm_dialog();
        }
        _ => {}
    }
    Ok(false)
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        // Global quit
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            if matches!(app.screen, AppScreen::Dashboard) {
                return Ok(true);
            }
            app.switch_screen(AppScreen::Dashboard);
        }

        // Screen navigation
        KeyCode::Tab => {
            app.next_tab();
        }
        KeyCode::BackTab => {
            app.prev_screen();
        }

        // List navigation
        KeyCode::Up | KeyCode::Char('k') => {
            app.navigate_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.navigate_down();
        }

        // Dashboard shortcuts
        KeyCode::Char('1') if matches!(app.screen, AppScreen::Dashboard) => {
            app.switch_screen(AppScreen::RentalManagement(RentalManagementTab::Search));
        }
        KeyCode::Char('2') if matches!(app.screen, AppScreen::Dashboard) => {
            app.switch_screen(AppScreen::RentalManagement(RentalManagementTab::List));
        }
        KeyCode::Char('3') if matches!(app.screen, AppScreen::Dashboard) => {
            app.switch_screen(AppScreen::RentalManagement(RentalManagementTab::Extend));
        }
        KeyCode::Char('4') if matches!(app.screen, AppScreen::Dashboard) => {
            app.switch_screen(AppScreen::RentalManagement(RentalManagementTab::Return));
        }

        // Search
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Editing;
        }

        // Actions based on screen
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if matches!(
                app.screen,
                AppScreen::Management(ManagementTab::Lockers)
            ) {
                if let Some(locker) = app.selected_locker() {
                    if locker.is_damaged {
                        app.mark_locker_repaired(locker.id)?;
                    }
                }
            }
        }

        KeyCode::Char('d') | KeyCode::Char('D') => {
            // Mark as damaged or delete (context dependent)
            match &app.screen {
                AppScreen::RentalManagement(RentalManagementTab::Damage) => {
                    if let Some(locker) = app.selected_locker() {
                        app.mark_locker_damaged(locker.id)?;
                    }
                }
                _ => {}
            }
        }

        // Refresh data
        KeyCode::F(5) => {
            app.reload_data()?;
            app.show_success("Daten aktualisiert");
        }

        // Clear notification on Enter
        KeyCode::Enter => {
            if app.notification.is_some() {
                app.notification = None;
            }
        }

        KeyCode::Esc => {
            if app.notification.is_some() {
                app.notification = None;
            } else {
                app.switch_screen(AppScreen::Dashboard);
            }
        }

        _ => {}
    }

    Ok(false)
}

fn handle_editing_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.clear_search();
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
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
