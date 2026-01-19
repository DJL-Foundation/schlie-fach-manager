mod app;
mod config;
mod db;
mod model;
mod ui;

use std::{
    io::stdout,
    time::{Duration, Instant},
};

use app::{App, AppTab, InputMode, ManagementSubTab, AdminSubTab, PopupType};
use color_eyre::eyre::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use db::Database;
use model::LockerHeight;
use ratatui::{Terminal, backend::CrosstermBackend};

const TICK_RATE: Duration = Duration::from_millis(250);

fn main() -> Result<()> {
    color_eyre::install()?;

    let db = Database::open_default()?;
    
    // Seed default locations if empty
    db.seed_if_empty(&["Hauptgebäude", "Sporthalle", "Neubau"])?;

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
    // Global quit with Ctrl+C
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Ok(true);
    }

    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Searching => handle_search_mode(app, key),
        InputMode::Wizard => handle_wizard_mode(app, key),
        InputMode::FormInput => handle_form_input_mode(app, key),
        InputMode::Popup => handle_popup_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        
        // Tab navigation
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.prev_tab(),
        KeyCode::Char('1') => app.set_tab(AppTab::Dashboard),
        KeyCode::Char('2') => app.set_tab(AppTab::Management),
        KeyCode::Char('3') => app.set_tab(AppTab::Finance),
        KeyCode::Char('4') => app.set_tab(AppTab::Admin),
        
        // Sub-tab navigation
        KeyCode::Right | KeyCode::Char('l') => app.next_subtab(),
        KeyCode::Left | KeyCode::Char('h') => {
            // Go to previous subtab (simplified)
            app.next_subtab();
            app.next_subtab();
            app.next_subtab();
        }
        
        // List navigation
        KeyCode::Down | KeyCode::Char('j') => {
            match app.current_tab {
                AppTab::Admin if app.admin_subtab == AdminSubTab::Locations => {
                    if app.location_selected < app.locations.len().saturating_sub(1) {
                        app.location_selected += 1;
                    }
                }
                _ => app.next(),
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            match app.current_tab {
                AppTab::Admin if app.admin_subtab == AdminSubTab::Locations => {
                    app.location_selected = app.location_selected.saturating_sub(1);
                }
                _ => app.previous(),
            }
        }
        
        // Search
        KeyCode::Char('/') => app.set_input_mode(InputMode::Searching),
        
        // Mode toggle
        KeyCode::Char('c') => {
            app.use_chat_mode = true;
            app.set_status("Chat-Modus aktiviert");
        }
        KeyCode::Char('m') if app.current_tab != AppTab::Management => {
            app.use_chat_mode = false;
            app.set_status("Formular-Modus aktiviert");
        }
        
        // Actions based on current tab/subtab
        _ => handle_tab_specific_action(app, key)?,
    }

    Ok(false)
}

fn handle_tab_specific_action(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.current_tab {
        AppTab::Management => handle_management_action(app, key)?,
        AppTab::Admin => handle_admin_action(app, key)?,
        AppTab::Finance => handle_finance_action(app, key)?,
        AppTab::Dashboard => {
            // Dashboard is read-only, just reload on 'r'
            if key.code == KeyCode::Char('r') {
                app.reload()?;
                app.set_status("Daten aktualisiert");
            }
        }
    }
    Ok(())
}

fn handle_management_action(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.management_subtab {
        ManagementSubTab::Search => {
            if key.code == KeyCode::Char('n') || key.code == KeyCode::Enter {
                app.start_rental_wizard();
            }
        }
        ManagementSubTab::List => {
            match key.code {
                KeyCode::Char('d') => {
                    // Mark as damaged
                    if let Some(locker) = app.selected_locker().cloned() {
                        app.show_popup(PopupType::Confirmation {
                            message: format!("Schließfach {} als defekt markieren?", locker.display_number),
                            on_confirm: app::PopupAction::None,
                        });
                    }
                }
                KeyCode::Char('f') => {
                    // Repair locker
                    app.repair_locker()?;
                }
                KeyCode::Char('r') => {
                    app.reload()?;
                    app.set_status("Liste aktualisiert");
                }
                _ => {}
            }
        }
        ManagementSubTab::Extend => {
            match key.code {
                KeyCode::Char('e') | KeyCode::Enter => {
                    app.start_extend_wizard();
                }
                KeyCode::Char('r') => {
                    app.start_return_wizard();
                }
                _ => {}
            }
        }
        ManagementSubTab::Damage => {
            match key.code {
                KeyCode::Char('d') => {
                    app.mark_locker_damaged(Some("Defekt gemeldet".to_string()))?;
                }
                KeyCode::Char('f') => {
                    app.repair_locker()?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn handle_admin_action(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.admin_subtab {
        AdminSubTab::Locations => {
            match key.code {
                KeyCode::Char('a') => {
                    // Start adding location (switch to form input)
                    app.form_input.clear();
                    app.set_input_mode(InputMode::FormInput);
                    app.set_status("Standortname eingeben und Enter drücken");
                }
                KeyCode::Char('d') => {
                    // Delete selected location
                    if app.location_selected < app.locations.len() {
                        let loc = &app.locations[app.location_selected];
                        app.show_popup(PopupType::Confirmation {
                            message: format!("Standort '{}' wirklich löschen?", loc.name),
                            on_confirm: app::PopupAction::ConfirmDelete,
                        });
                    }
                }
                _ => {}
            }
        }
        AdminSubTab::Lockers => {
            match key.code {
                KeyCode::Char('d') => {
                    if let Some(locker) = app.selected_locker().cloned() {
                        app.show_popup(PopupType::Confirmation {
                            message: format!("Schließfach {} wirklich löschen?", locker.display_number),
                            on_confirm: app::PopupAction::ConfirmDelete,
                        });
                    }
                }
                _ => {}
            }
        }
        AdminSubTab::Bulk => {
            if key.code == KeyCode::Char('b') || key.code == KeyCode::Enter {
                // For now, create some sample lockers
                if let Some(loc) = app.locations.first() {
                    app.bulk_create_lockers(loc.id, "TEST-", 1, 5, LockerHeight::Middle)?;
                }
            }
        }
        AdminSubTab::Backup => {
            // Backup functionality placeholder
            if key.code == KeyCode::Char('s') {
                app.set_status("Backup-Funktion noch nicht implementiert");
            }
        }
    }
    Ok(())
}

fn handle_finance_action(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.code == KeyCode::Char('x') {
        // Export functionality
        app.set_status("Export-Funktion: Drücke 'j' für JSON, 'c' für CSV");
    } else if key.code == KeyCode::Char('r') {
        app.reload()?;
        app.set_status("Daten aktualisiert");
    }
    Ok(())
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

fn handle_wizard_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.cancel_wizard();
            app.set_status("Vorgang abgebrochen");
        }
        KeyCode::Enter => {
            // Advance the appropriate wizard
            if app.rental_wizard.is_some() {
                app.advance_rental_wizard()?;
            } else if app.extend_wizard.is_some() {
                app.advance_extend_wizard()?;
            } else if app.return_wizard.is_some() {
                app.advance_return_wizard()?;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Navigate in selection lists
            if let Some(wizard) = &app.rental_wizard {
                match wizard.state {
                    app::RentalWizardState::AskLocation => {
                        if app.location_selected < app.locations.len().saturating_sub(1) {
                            app.location_selected += 1;
                        }
                    }
                    app::RentalWizardState::AskHeight => {
                        let heights = LockerHeight::all();
                        if app.selected_index < heights.len().saturating_sub(1) {
                            app.selected_index += 1;
                        }
                    }
                    app::RentalWizardState::AskTenantType => {
                        let types = model::TenantType::all();
                        if app.selected_index < types.len().saturating_sub(1) {
                            app.selected_index += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Navigate in selection lists
            if let Some(wizard) = &app.rental_wizard {
                match wizard.state {
                    app::RentalWizardState::AskLocation => {
                        app.location_selected = app.location_selected.saturating_sub(1);
                    }
                    app::RentalWizardState::AskHeight | app::RentalWizardState::AskTenantType => {
                        app.selected_index = app.selected_index.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
        KeyCode::Backspace => {
            app.pop_form_char();
        }
        KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.push_form_char(ch);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_form_input_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.clear_form_input();
            app.set_input_mode(InputMode::Normal);
            app.clear_status();
        }
        KeyCode::Enter => {
            // Process the form input based on context
            let input = app.form_input.clone();
            if !input.trim().is_empty() {
                // If in admin locations, add location
                if app.current_tab == AppTab::Admin && app.admin_subtab == AdminSubTab::Locations {
                    app.add_location(&input)?;
                }
            }
            app.clear_form_input();
            app.set_input_mode(InputMode::Normal);
        }
        KeyCode::Backspace => {
            app.pop_form_char();
        }
        KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.push_form_char(ch);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_popup_mode(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('n') => {
            app.close_popup();
        }
        KeyCode::Enter | KeyCode::Char('j') | KeyCode::Char('y') => {
            app.confirm_popup()?;
        }
        _ => {}
    }
    Ok(false)
}
