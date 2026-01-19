//! Schließfach-Manager - Terminal-based Locker Management System
//!
//! This is the main binary entry point for the Schließfach-Manager application.
//! For library usage, see the crate documentation.

use schliessfach_manager::{
    app::{App, InactivityState},
    db::{self, Database},
    ui::{
        self,
        state::{AppScreen, InputMode, ManagementTab, RentalManagementTab},
        widgets::{Header, KeybindBar},
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
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

/// Tick rate for the main event loop (250ms).
const TICK_RATE: Duration = Duration::from_millis(250);

/// Number of main window screens (Dashboard, Verleih-Management, Finanzen, Verwaltung).
const NUM_WINDOWS: usize = 4;

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
                    // Any keypress resets activity and exits screensaver
                    app.update_activity();

                    // If screensaver was active, just exit it (don't process key further)
                    if app.screen == AppScreen::Screensaver {
                        continue;
                    }

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
            app.status_bar.clear_expired_messages();
            app.status_bar.check_escape_timeout();

            // Update screensaver animation if active
            if let Some(ref mut screen) = app.screensaver_screen {
                screen.update();
            }

            // Check inactivity state
            match app.check_inactivity() {
                InactivityState::Active => {}
                InactivityState::Countdown(secs) => {
                    app.status_bar.show_screensaver_countdown(secs);
                }
                InactivityState::ScreensaverActive => {
                    if !app.screensaver_active {
                        app.activate_screensaver();
                    }
                }
            }

            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn render_ui(frame: &mut ratatui::prelude::Frame, app: &App) {
    use ui::screens::{
        render_dashboard, render_finance, render_management, render_rental_management,
        render_screensaver,
    };
    use ui::widgets::render_notification;

    let area = frame.size();

    // Screensaver takes over the entire screen
    if app.screen == AppScreen::Screensaver {
        if let Some(ref screen) = app.screensaver_screen {
            render_screensaver(frame, area, screen);
        }
        return;
    }

    // Global layout: Header (2) | Main content (dynamic) | Keybind bar (2) | Status bar (1)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(2), // Keybind bar
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Header
    let current_screen_name = match &app.screen {
        AppScreen::Dashboard => "Dashboard",
        AppScreen::RentalManagement(_) => "Verleih-Management",
        AppScreen::Finance(_) => "Finanzen",
        AppScreen::Management(_) => "Verwaltung",
        AppScreen::Screensaver => "Screensaver",
    };

    let header = Header::new("2.1.0")
        .current_screen(current_screen_name)
        .window_switcher_active(app.window_switcher_active)
        .selected_window_index(app.selected_window_index);
    frame.render_widget(header, chunks[0]);

    // Main content
    match &app.screen {
        AppScreen::Dashboard => {
            render_dashboard(frame, chunks[1], &app.dashboard_stats);
        }
        AppScreen::RentalManagement(_) => {
            render_rental_management(
                frame,
                chunks[1],
                &app.rental_state,
                &app.lockers,
                &app.active_rentals,
            );
        }
        AppScreen::Finance(_) => {
            render_finance(
                frame,
                chunks[1],
                &app.finance_state,
                &app.payment_summary,
                &app.debtors,
            );
        }
        AppScreen::Management(_) => {
            render_management(
                frame,
                chunks[1],
                &app.management_state,
                &app.lockers,
                &app.locations,
            );
        }
        AppScreen::Screensaver => {
            // Handled above
        }
    }

    // Keybind bar
    let keybind_bar = if app.window_switcher_active {
        KeybindBar::new()
            .context_binds(KeybindBar::window_switcher_context())
            .context_message("Wähle Fenster aus")
    } else {
        match &app.screen {
            AppScreen::Dashboard => {
                KeybindBar::new().context_binds(KeybindBar::dashboard_context())
            }
            _ => KeybindBar::new().context_binds(KeybindBar::list_context()),
        }
    };
    frame.render_widget(keybind_bar, chunks[2]);

    // Status bar
    let status_bar = app.status_bar.to_widget();
    frame.render_widget(status_bar, chunks[3]);

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
    // Handle window switcher mode
    if app.window_switcher_active {
        return handle_window_switcher(app, key);
    }

    match key.code {
        // Global quit with Shift+Q
        KeyCode::Char('Q') => {
            return Ok(true);
        }
        KeyCode::Char('q') => {
            if matches!(app.screen, AppScreen::Dashboard) {
                return Ok(true);
            }
            app.switch_screen(AppScreen::Dashboard);
        }

        // Window switcher activation with ^
        KeyCode::Char('^') | KeyCode::Char('6') if key.modifiers.contains(KeyModifiers::SHIFT) => {
            app.window_switcher_active = true;
            app.selected_window_index = app.current_screen_index();
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
        KeyCode::Char('5') if matches!(app.screen, AppScreen::Dashboard) => {
            app.switch_screen(AppScreen::RentalManagement(RentalManagementTab::Damage));
        }

        // Search
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Editing;
        }

        // Actions based on screen
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if matches!(app.screen, AppScreen::Management(ManagementTab::Lockers)) {
                if let Some(locker) = app.selected_locker() {
                    if locker.is_damaged {
                        app.mark_locker_repaired(locker.id)?;
                    }
                }
            }
        }

        KeyCode::Char('d') | KeyCode::Char('D') => {
            // Mark as damaged or delete (context dependent)
            if let AppScreen::RentalManagement(RentalManagementTab::Damage) = &app.screen {
                if let Some(locker) = app.selected_locker() {
                    app.mark_locker_damaged(locker.id)?;
                }
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

        // Handle Escape with 3x to Dashboard feature
        KeyCode::Esc => {
            if app.notification.is_some() {
                app.notification = None;
                app.status_bar.reset_escape_count();
            } else if !matches!(app.screen, AppScreen::Dashboard) {
                // Increment escape counter
                app.status_bar.increment_escape_count();

                // Check if we should return to dashboard
                if app.status_bar.should_return_to_dashboard() {
                    app.switch_screen(AppScreen::Dashboard);
                    app.status_bar.reset_escape_count();
                    app.status_bar.success("Zurück zum Dashboard");
                }
            }
        }

        _ => {}
    }

    Ok(false)
}

fn handle_window_switcher(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        // Navigate to next window
        KeyCode::Tab => {
            app.selected_window_index = (app.selected_window_index + 1) % NUM_WINDOWS;
        }
        // Navigate to previous window
        KeyCode::BackTab => {
            app.selected_window_index = if app.selected_window_index == 0 {
                NUM_WINDOWS - 1
            } else {
                app.selected_window_index - 1
            };
        }
        // Confirm selection
        KeyCode::Enter => {
            app.switch_to_screen_by_index(app.selected_window_index);
            app.window_switcher_active = false;
        }
        // Cancel
        KeyCode::Esc => {
            app.window_switcher_active = false;
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
