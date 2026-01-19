use crate::{
    config::AppSettings,
    db::{self, Database},
    export::{self, ExportFormat},
    import,
    model::{BillingPeriod, Rental},
    ui::{
        state::{SettingsAction, SettingsEditor},
        theme::Theme,
        widgets::{
            header::Header,
            keybind_bar::{Keybind, KeybindBar, KeybindScope},
            status_bar::{StatusBar, StatusLevel, StatusMessage},
            wizard::WizardRenderer,
        },
    },
    workflows::{
        common::{WorkflowAction, WorkflowResult},
        damage::DamageWorkflow,
        extend::ExtendWorkflow,
        rent::RentWorkflow,
        return_locker::ReturnWorkflow,
    },
};
use anyhow::{Result, anyhow};
use chrono::{Duration as ChronoDuration, Utc};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

/// Screens in the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    Dashboard,
    RentalManagement(RentalTab),
    Finances,
    Management(ManagementTab),
    Screensaver,
}

/// Tabs inside the rental management screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RentalTab {
    Search,
    List,
    Extend,
    Return,
    Damage,
}

impl RentalTab {
    /// Returns all rental tabs in order.
    pub fn all() -> Vec<RentalTab> {
        vec![
            RentalTab::Search,
            RentalTab::List,
            RentalTab::Extend,
            RentalTab::Return,
            RentalTab::Damage,
        ]
    }

    /// Returns the display label for the tab.
    pub fn label(&self) -> &'static str {
        match self {
            RentalTab::Search => "Search",
            RentalTab::List => "List",
            RentalTab::Extend => "Extend",
            RentalTab::Return => "Return",
            RentalTab::Damage => "Damage",
        }
    }
}

/// Tabs inside the management screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagementTab {
    Lockers,
    Locations,
    Settings,
    AuditLog,
}

impl ManagementTab {
    /// Returns all management tabs in order.
    pub fn all() -> Vec<ManagementTab> {
        vec![
            ManagementTab::Lockers,
            ManagementTab::Locations,
            ManagementTab::Settings,
            ManagementTab::AuditLog,
        ]
    }

    /// Returns the display label for the tab.
    pub fn label(&self) -> &'static str {
        match self {
            ManagementTab::Lockers => "Schließfächer",
            ManagementTab::Locations => "Standorte",
            ManagementTab::Settings => "Einstellungen",
            ManagementTab::AuditLog => "Audit Log",
        }
    }
}

/// Workflow wrapper used by the rental management screen.
pub enum ActiveWorkflow {
    Rent(RentWorkflow),
    Extend(ExtendWorkflow),
    Return(ReturnWorkflow),
    Damage(DamageWorkflow),
}

impl ActiveWorkflow {
    /// Returns the renderer for the active workflow.
    pub fn renderer(&self) -> &WizardRenderer {
        match self {
            ActiveWorkflow::Rent(flow) => flow.renderer(),
            ActiveWorkflow::Extend(flow) => flow.renderer(),
            ActiveWorkflow::Return(flow) => flow.renderer(),
            ActiveWorkflow::Damage(flow) => flow.renderer(),
        }
    }

    /// Handles key input for the active workflow.
    pub fn handle_key(&mut self, key: KeyCode) -> WorkflowAction {
        match self {
            ActiveWorkflow::Rent(flow) => flow.handle_key(key),
            ActiveWorkflow::Extend(flow) => flow.handle_key(key),
            ActiveWorkflow::Return(flow) => flow.handle_key(key),
            ActiveWorkflow::Damage(flow) => flow.handle_key(key),
        }
    }
}

/// Actions the app event loop can react to.
pub enum AppAction {
    Continue,
    Quit,
}

/// Screensaver inactivity state.
pub enum InactivityState {
    Active,
    Countdown(u64),
    ScreensaverActive,
}

/// Application state container.
pub struct App {
    db: Database,
    screen: AppScreen,
    status_bar: StatusBar,
    window_switcher_active: bool,
    window_list: Vec<String>,
    selected_window_index: usize,
    settings: AppSettings,
    settings_editor: SettingsEditor,
    active_workflow: Option<ActiveWorkflow>,
    last_rental_tab: RentalTab,
    last_management_tab: ManagementTab,
    last_activity: Instant,
    screensaver_timeout: Duration,
    countdown_duration: Duration,
    screensaver_active: bool,
    previous_status_message: Option<StatusMessage>,
    screensaver_screen: crate::ui::screens::screensaver::ScreensaverScreen,
    theme: Theme,
}

impl App {
    /// Creates a new application instance using the provided `Database`.
    pub fn new(db: Database) -> Result<Self> {
        let settings = Self::load_settings(&db)?;
        let settings_editor = SettingsEditor::new(&settings);
        let screensaver_timeout = Duration::from_secs(settings.screensaver_timeout_seconds);
        let theme = Theme::default_dark();
        let window_list = vec![
            "Dashboard".to_string(),
            "Verleih-Management".to_string(),
            "Finanzen".to_string(),
            "Verwaltung".to_string(),
        ];

        Ok(Self {
            db,
            screen: AppScreen::Dashboard,
            status_bar: StatusBar::default(),
            window_switcher_active: false,
            window_list,
            selected_window_index: 0,
            settings,
            settings_editor,
            active_workflow: None,
            last_rental_tab: RentalTab::List,
            last_management_tab: ManagementTab::Lockers,
            last_activity: Instant::now(),
            screensaver_timeout,
            countdown_duration: Duration::from_secs(15),
            screensaver_active: false,
            previous_status_message: None,
            screensaver_screen: crate::ui::screens::screensaver::ScreensaverScreen::new(),
            theme,
        })
    }

    /// Creates a test-friendly application using an in-memory database.
    /// Creates a test-friendly application using an in-memory database.
    pub fn new_test() -> Result<Self> {
        let db = Database::open_in_memory()?;
        Self::new(db)
    }

    /// Returns the database handle.
    pub fn db(&self) -> &Database {
        &self.db
    }

    /// Returns the current screen.
    pub fn screen(&self) -> &AppScreen {
        &self.screen
    }

    /// Returns the status bar state.
    pub fn status_bar(&self) -> &StatusBar {
        &self.status_bar
    }

    /// Returns mutable access to the status bar state.
    pub fn status_bar_mut(&mut self) -> &mut StatusBar {
        &mut self.status_bar
    }

    /// Builds the current keybind bar state.
    pub fn keybind_bar(&self) -> KeybindBar {
        self.build_keybind_bar()
    }

    /// Returns the loaded application settings.
    pub fn settings(&self) -> &AppSettings {
        &self.settings
    }

    /// Returns the settings editor when the settings screen is active.
    pub fn settings_editor(&self) -> Option<&SettingsEditor> {
        if matches!(self.screen, AppScreen::Management(ManagementTab::Settings)) {
            Some(&self.settings_editor)
        } else {
            None
        }
    }

    /// Returns the screensaver screen state.
    pub fn screensaver_screen(&self) -> &crate::ui::screens::screensaver::ScreensaverScreen {
        &self.screensaver_screen
    }

    /// Returns mutable access to the screensaver screen.
    pub fn screensaver_screen_mut(
        &mut self,
    ) -> &mut crate::ui::screens::screensaver::ScreensaverScreen {
        &mut self.screensaver_screen
    }

    /// Returns whether the screensaver is active.
    pub fn screensaver_active(&self) -> bool {
        self.screensaver_active
    }

    /// Returns the currently active workflow if present.
    pub fn active_workflow(&self) -> Option<&ActiveWorkflow> {
        self.active_workflow.as_ref()
    }

    /// Returns the current UI theme.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Builds the header widget state.
    pub fn header_widget(&self) -> Header {
        Header {
            version: "2.1.0".to_string(),
            current_screen: self.screen_label().to_string(),
            window_switcher_active: self.window_switcher_active,
            window_list: self.window_list.clone(),
            selected_window_index: self.selected_window_index,
        }
    }

    /// Handles keyboard input and returns an action for the main loop.
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<AppAction> {
        if key.code == KeyCode::Char('Q')
            || (key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::SHIFT))
        {
            return Ok(AppAction::Quit);
        }

        if key.code != KeyCode::Esc {
            self.status_bar.reset_escape_count();
        }

        if key.code == KeyCode::Char('^') {
            self.window_switcher_active = true;
            self.sync_window_index();
            return Ok(AppAction::Continue);
        }

        if self.window_switcher_active {
            self.handle_window_switcher_key(key);
            return Ok(AppAction::Continue);
        }

        match key.code {
            KeyCode::Esc => {
                self.status_bar.increment_escape_count();
                if self.status_bar.should_return_to_dashboard() {
                    self.status_bar.reset_escape_count();
                    self.switch_screen(AppScreen::Dashboard);
                    self.status_bar
                        .set_message("Zurück zum Dashboard".to_string(), StatusLevel::Info);
                    return Ok(AppAction::Continue);
                }
                self.handle_escape_normal();
            }
            _ => {
                self.handle_screen_key(key.code)?;
            }
        }

        Ok(AppAction::Continue)
    }

    /// Updates last activity timestamp and exits the screensaver if needed.
    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
        if self.screensaver_active {
            self.exit_screensaver();
        }
    }

    /// Returns the inactivity state (active, countdown, or screensaver).
    pub fn check_inactivity(&mut self) -> InactivityState {
        let inactive = self.last_activity.elapsed();
        if inactive >= self.screensaver_timeout + self.countdown_duration {
            InactivityState::ScreensaverActive
        } else if inactive >= self.screensaver_timeout {
            let remaining = self.countdown_duration - (inactive - self.screensaver_timeout);
            InactivityState::Countdown(remaining.as_secs())
        } else {
            InactivityState::Active
        }
    }

    /// Activates the screensaver and stores previous state.
    pub fn activate_screensaver(&mut self) {
        if !self.screensaver_active {
            self.previous_status_message = self.status_bar.message.clone();
            self.screensaver_active = true;
            self.screen = AppScreen::Screensaver;
        }
    }

    /// Deactivates the screensaver and returns to the dashboard.
    pub fn exit_screensaver(&mut self) {
        if self.screensaver_active {
            self.screensaver_active = false;
            self.screen = AppScreen::Dashboard;
            if let Some(prev_msg) = self.previous_status_message.take() {
                self.status_bar.message = Some(prev_msg);
            }
            self.last_activity = Instant::now();
        }
    }

    /// Handles key input while the window switcher is active.
    fn handle_window_switcher_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.selected_window_index =
                    (self.selected_window_index + 1) % self.window_list.len();
            }
            KeyCode::BackTab => {
                if self.selected_window_index == 0 {
                    self.selected_window_index = self.window_list.len() - 1;
                } else {
                    self.selected_window_index -= 1;
                }
            }
            KeyCode::Enter => {
                let screen = self.screen_for_window_index(self.selected_window_index);
                self.switch_screen(screen);
                self.window_switcher_active = false;
            }
            KeyCode::Esc => {
                self.window_switcher_active = false;
            }
            _ => {}
        }
    }

    /// Handles escape behavior outside of the triple-escape shortcut.
    fn handle_escape_normal(&mut self) {
        match self.screen {
            AppScreen::RentalManagement(_) => {
                self.active_workflow = None;
            }
            AppScreen::Management(ManagementTab::Settings) => {
                self.settings_editor = SettingsEditor::new(&self.settings);
            }
            _ => {}
        }
    }

    /// Routes key input to the active screen handler.
    fn handle_screen_key(&mut self, key: KeyCode) -> Result<()> {
        match self.screen {
            AppScreen::Dashboard => self.handle_dashboard_key(key),
            AppScreen::RentalManagement(tab) => self.handle_rental_key(key, tab),
            AppScreen::Finances => self.handle_finances_key(key),
            AppScreen::Management(tab) => self.handle_management_key(key, tab),
            AppScreen::Screensaver => Ok(()),
        }
    }

    /// Handles dashboard-specific keybinds.
    fn handle_dashboard_key(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('1') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Search))
            }
            KeyCode::Char('2') => self.switch_screen(AppScreen::RentalManagement(RentalTab::List)),
            KeyCode::Char('3') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Extend))
            }
            KeyCode::Char('4') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Return))
            }
            KeyCode::Char('5') => {
                self.switch_screen(AppScreen::RentalManagement(RentalTab::Damage))
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles key input for the rental management screen.
    fn handle_rental_key(&mut self, key: KeyCode, tab: RentalTab) -> Result<()> {
        if let Some(workflow) = &mut self.active_workflow {
            match workflow.handle_key(key) {
                WorkflowAction::Completed(result) => {
                    self.apply_workflow_result(result)?;
                    self.active_workflow = None;
                    self.switch_screen(AppScreen::RentalManagement(RentalTab::List));
                }
                WorkflowAction::Cancelled => {
                    self.active_workflow = None;
                    self.switch_screen(AppScreen::RentalManagement(RentalTab::List));
                }
                WorkflowAction::None => {}
            }
            return Ok(());
        }

        match key {
            KeyCode::Tab => {
                let next = next_rental_tab(tab, 1);
                self.switch_screen(AppScreen::RentalManagement(next));
            }
            KeyCode::BackTab => {
                let next = next_rental_tab(tab, -1);
                self.switch_screen(AppScreen::RentalManagement(next));
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles key input for the finances screen.
    fn handle_finances_key(&mut self, _key: KeyCode) -> Result<()> {
        match _key {
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let format = ExportFormat::from_str(&self.settings.export_format);
                let path = export::export_all(&self.db, &self.settings, format)?;
                db::audit::log_action(
                    self.db.connection(),
                    "export",
                    "database",
                    None,
                    &format!("{{\"path\":\"{}\"}}", path.to_string_lossy()),
                    "system",
                )?;
                self.status_bar.set_message(
                    format!("Export gespeichert: {}", path.to_string_lossy()),
                    StatusLevel::Success,
                );
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                let format = ExportFormat::from_str(&self.settings.export_format);
                let import_path = match format {
                    ExportFormat::Csv => self.settings.export_directory.join("import_csv"),
                    _ => self
                        .settings
                        .export_directory
                        .join(format!("import.{}", format.extension())),
                };
                if import_path.exists() {
                    import::import_all(&self.db, format, &import_path)?;
                    db::audit::log_action(
                        self.db.connection(),
                        "import",
                        "database",
                        None,
                        &format!("{{\"path\":\"{}\"}}", import_path.to_string_lossy()),
                        "system",
                    )?;
                    self.status_bar
                        .set_message("Import abgeschlossen".to_string(), StatusLevel::Success);
                } else {
                    self.status_bar.set_message(
                        format!("Import-Datei fehlt: {}", import_path.to_string_lossy()),
                        StatusLevel::Warning,
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles key input for the management screen.
    fn handle_management_key(&mut self, key: KeyCode, tab: ManagementTab) -> Result<()> {
        match tab {
            ManagementTab::Settings => {
                let action = self.settings_editor.handle_key(key);
                match action {
                    SettingsAction::Save(updated) => {
                        self.save_settings(updated)?;
                        self.status_bar.set_message(
                            "Einstellungen gespeichert".to_string(),
                            StatusLevel::Success,
                        );
                    }
                    SettingsAction::Cancel => {
                        self.settings_editor = SettingsEditor::new(&self.settings);
                    }
                    SettingsAction::None => {}
                }
            }
            _ => match key {
                KeyCode::Tab => {
                    let next = next_management_tab(tab, 1);
                    self.switch_screen(AppScreen::Management(next));
                }
                KeyCode::BackTab => {
                    let next = next_management_tab(tab, -1);
                    self.switch_screen(AppScreen::Management(next));
                }
                _ => {}
            },
        }
        Ok(())
    }

    /// Switches screens and updates cached tab state.
    fn switch_screen(&mut self, screen: AppScreen) {
        match screen {
            AppScreen::RentalManagement(tab) => {
                self.last_rental_tab = tab;
                self.active_workflow = self.workflow_for_tab(tab);
            }
            AppScreen::Management(tab) => {
                self.last_management_tab = tab;
            }
            _ => {}
        }
        self.screen = screen;
        self.sync_window_index();
    }

    /// Returns the workflow associated with a rental tab.
    fn workflow_for_tab(&self, tab: RentalTab) -> Option<ActiveWorkflow> {
        match tab {
            RentalTab::Search => Some(ActiveWorkflow::Rent(RentWorkflow::new(&self.db))),
            RentalTab::Extend => Some(ActiveWorkflow::Extend(ExtendWorkflow::new(
                &self.db,
                &self.settings,
            ))),
            RentalTab::Return => Some(ActiveWorkflow::Return(ReturnWorkflow::new(&self.db))),
            RentalTab::Damage => Some(ActiveWorkflow::Damage(DamageWorkflow::new(&self.db))),
            RentalTab::List => None,
        }
    }

    /// Returns the label used for the header and window switcher.
    fn screen_label(&self) -> &str {
        match self.screen {
            AppScreen::Dashboard => "Dashboard",
            AppScreen::RentalManagement(_) => "Verleih-Management",
            AppScreen::Finances => "Finanzen",
            AppScreen::Management(_) => "Verwaltung",
            AppScreen::Screensaver => "Screensaver",
        }
    }

    /// Synchronizes the window switcher index with the current screen.
    fn sync_window_index(&mut self) {
        let label = self.screen_label();
        if let Some(index) = self.window_list.iter().position(|entry| entry == label) {
            self.selected_window_index = index;
        }
    }

    /// Returns the screen for the given window switcher index.
    fn screen_for_window_index(&self, index: usize) -> AppScreen {
        match index {
            0 => AppScreen::Dashboard,
            1 => AppScreen::RentalManagement(self.last_rental_tab),
            2 => AppScreen::Finances,
            3 => AppScreen::Management(self.last_management_tab),
            _ => AppScreen::Dashboard,
        }
    }

    /// Applies workflow output to the database and logs audit entries.
    fn apply_workflow_result(&mut self, result: WorkflowResult) -> Result<()> {
        match result {
            WorkflowResult::Rent { location, size } => {
                let lockers = db::lockers::list_lockers(self.db.connection())?;
                let rentals = db::rentals::list_rentals(self.db.connection())?;
                let today = Utc::now().date_naive();
                let active_ids: Vec<i64> = rentals
                    .iter()
                    .filter(|rental| rental.is_active_on(today))
                    .map(|rental| rental.locker_id)
                    .collect();
                let locker = lockers
                    .into_iter()
                    .find(|locker| {
                        locker.location == location
                            && locker.size == size
                            && !locker.is_damaged
                            && !active_ids.contains(&locker.id)
                    })
                    .ok_or_else(|| anyhow!("kein verfügbares Schließfach gefunden"))?;

                let end_date = match self.settings.billing_period {
                    BillingPeriod::Monthly => today + ChronoDuration::days(30),
                    BillingPeriod::Yearly => today + ChronoDuration::days(365),
                };

                let rental = Rental::new(0, locker.id, "Unbekannt", today, end_date);
                let rental_id = db::rentals::create_rental(self.db.connection(), &rental)?;
                db::audit::log_action(
                    self.db.connection(),
                    "create",
                    "rental",
                    Some(rental_id),
                    &format!("{{\"locker_id\":{}}}", locker.id),
                    "system",
                )?;
                self.status_bar
                    .set_message("Verleih angelegt".to_string(), StatusLevel::Success);
            }
            WorkflowResult::Extend { rental_id } => {
                let mut rental = db::rentals::get_rental(self.db.connection(), rental_id)?
                    .ok_or_else(|| anyhow!("Verleihvorgang nicht gefunden"))?;
                let new_end_date = match self.settings.billing_period {
                    BillingPeriod::Monthly => rental.end_date + ChronoDuration::days(30),
                    BillingPeriod::Yearly => rental.end_date + ChronoDuration::days(365),
                };
                rental.extend_to(new_end_date);
                db::rentals::update_rental(self.db.connection(), &rental)?;
                db::audit::log_action(
                    self.db.connection(),
                    "update",
                    "rental",
                    Some(rental.id),
                    &format!("{{\"end_date\":\"{}\"}}", rental.end_date),
                    "system",
                )?;
                self.status_bar
                    .set_message("Verleih verlängert".to_string(), StatusLevel::Success);
            }
            WorkflowResult::Return { rental_id } => {
                let mut rental = db::rentals::get_rental(self.db.connection(), rental_id)?
                    .ok_or_else(|| anyhow!("Verleihvorgang nicht gefunden"))?;
                let today = Utc::now().date_naive();
                rental.close(today);
                db::rentals::update_rental(self.db.connection(), &rental)?;
                db::audit::log_action(
                    self.db.connection(),
                    "update",
                    "rental",
                    Some(rental.id),
                    &format!("{{\"return_date\":\"{}\"}}", today),
                    "system",
                )?;
                self.status_bar.set_message(
                    "Schließfach zurückgegeben".to_string(),
                    StatusLevel::Success,
                );
            }
            WorkflowResult::Damage { locker_id, damaged } => {
                let mut locker = db::lockers::get_locker(self.db.connection(), locker_id)?
                    .ok_or_else(|| anyhow!("Schließfach nicht gefunden"))?;
                if damaged {
                    locker.is_damaged = true;
                } else {
                    locker.is_damaged = false;
                }
                db::lockers::upsert_locker(self.db.connection(), &locker)?;
                db::audit::log_action(
                    self.db.connection(),
                    "update",
                    "locker",
                    Some(locker.id),
                    &format!("{{\"is_damaged\":{}}}", locker.is_damaged),
                    "system",
                )?;
                self.status_bar.set_message(
                    "Schließfachstatus aktualisiert".to_string(),
                    StatusLevel::Success,
                );
            }
        }
        Ok(())
    }

    /// Builds the keybind bar content for the current screen.
    fn build_keybind_bar(&self) -> KeybindBar {
        let global_binds = vec![
            Keybind {
                key: "Tab".to_string(),
                description: "Nächster Eintrag".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "Shift+Tab".to_string(),
                description: "Vorheriger Eintrag".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "^".to_string(),
                description: "Window Switcher".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "Shift+Q".to_string(),
                description: "Beenden".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "Esc".to_string(),
                description: "Abbrechen (3x = Dashboard)".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "Enter".to_string(),
                description: "Bestätigen".to_string(),
                scope: KeybindScope::Global,
            },
        ];

        if self.window_switcher_active {
            return KeybindBar {
                global_binds,
                context_binds: vec![],
                context_message: Some(
                    "--- Wähle Fenster aus ---  [Tab] Vor  [Shift+Tab] Zurück  [Enter] Öffnen  [Esc] Abbrechen"
                        .to_string(),
                ),
            };
        }

        let mut context_binds = Vec::new();
        let mut context_message = None;

        if self.active_workflow.is_some() {
            context_binds.push(Keybind {
                key: "↑↓".to_string(),
                description: "Auswahl".to_string(),
                scope: KeybindScope::Context,
            });
            context_binds.push(Keybind {
                key: "Enter".to_string(),
                description: "Bestätigen".to_string(),
                scope: KeybindScope::Context,
            });
            context_binds.push(Keybind {
                key: "Esc".to_string(),
                description: "Abbrechen".to_string(),
                scope: KeybindScope::Context,
            });
        } else {
            match self.screen {
                AppScreen::Dashboard => {
                    context_binds.extend(
                        [
                            ("1", "Suchen"),
                            ("2", "Liste"),
                            ("3", "Verlängern"),
                            ("4", "Rückgabe"),
                            ("5", "Defekt melden"),
                        ]
                        .iter()
                        .map(|(key, desc)| Keybind {
                            key: key.to_string(),
                            description: desc.to_string(),
                            scope: KeybindScope::Context,
                        }),
                    );
                }
                AppScreen::RentalManagement(_) => {
                    context_binds.push(Keybind {
                        key: "Tab".to_string(),
                        description: "Tab wechseln".to_string(),
                        scope: KeybindScope::Context,
                    });
                }
                AppScreen::Finances => {
                    context_binds.extend(
                        [
                            ("↑↓", "Navigation"),
                            ("Enter", "Details"),
                            ("E", "Export"),
                            ("I", "Import"),
                        ]
                        .iter()
                        .map(|(key, desc)| Keybind {
                            key: key.to_string(),
                            description: desc.to_string(),
                            scope: KeybindScope::Context,
                        }),
                    );
                }
                AppScreen::Management(_) => {
                    context_binds.push(Keybind {
                        key: "Tab".to_string(),
                        description: "Tab wechseln".to_string(),
                        scope: KeybindScope::Context,
                    });
                    if matches!(self.screen, AppScreen::Management(ManagementTab::Settings)) {
                        context_message = Some(
                            "Pfeiltasten ändern Werte · Enter speichert · Esc verwirft".to_string(),
                        );
                    }
                }
                AppScreen::Screensaver => {}
            }
        }

        KeybindBar {
            global_binds,
            context_binds,
            context_message,
        }
    }

    /// Loads application settings from the database.
    fn load_settings(db: &Database) -> Result<AppSettings> {
        let mut settings = AppSettings::default();
        let map = db::settings::load_settings_map(db.connection())?;
        for (key, value) in map {
            settings.apply_setting(&key, &value);
        }
        if settings.export_directory.as_os_str().is_empty() {
            settings.export_directory = crate::config::export_directory()?;
        }
        Ok(settings)
    }

    /// Persists settings updates and updates local state.
    fn save_settings(&mut self, updated: AppSettings) -> Result<()> {
        for (key, value, description) in updated.as_settings_entries() {
            db::settings::upsert_setting(self.db.connection(), &key, &value, &description)?;
        }
        self.settings = updated.clone();
        self.settings_editor = SettingsEditor::new(&updated);
        self.screensaver_timeout = Duration::from_secs(updated.screensaver_timeout_seconds);
        db::audit::log_action(
            self.db.connection(),
            "update",
            "settings",
            None,
            "{\"action\":\"save\"}",
            "system",
        )?;
        Ok(())
    }
}

/// Returns the next rental tab given a delta.
fn next_rental_tab(current: RentalTab, delta: i32) -> RentalTab {
    let tabs = RentalTab::all();
    let index = tabs.iter().position(|tab| *tab == current).unwrap_or(0) as i32;
    let next = (index + delta).rem_euclid(tabs.len() as i32) as usize;
    tabs[next]
}

/// Returns the next management tab given a delta.
fn next_management_tab(current: ManagementTab, delta: i32) -> ManagementTab {
    let tabs = ManagementTab::all();
    let index = tabs.iter().position(|tab| *tab == current).unwrap_or(0) as i32;
    let next = (index + delta).rem_euclid(tabs.len() as i32) as usize;
    tabs[next]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn dashboard_keys_open_correct_tabs() -> Result<()> {
        let mut app = App::new_test()?;
        app.handle_key(key(KeyCode::Char('1')))?;
        assert_eq!(app.screen, AppScreen::RentalManagement(RentalTab::Search));

        app.screen = AppScreen::Dashboard;
        app.handle_key(key(KeyCode::Char('2')))?;
        assert_eq!(app.screen, AppScreen::RentalManagement(RentalTab::List));

        app.screen = AppScreen::Dashboard;
        app.handle_key(key(KeyCode::Char('3')))?;
        assert_eq!(app.screen, AppScreen::RentalManagement(RentalTab::Extend));

        app.screen = AppScreen::Dashboard;
        app.handle_key(key(KeyCode::Char('4')))?;
        assert_eq!(app.screen, AppScreen::RentalManagement(RentalTab::Return));

        app.screen = AppScreen::Dashboard;
        app.handle_key(key(KeyCode::Char('5')))?;
        assert_eq!(app.screen, AppScreen::RentalManagement(RentalTab::Damage));

        Ok(())
    }

    #[test]
    fn triple_escape_returns_to_dashboard() -> Result<()> {
        let mut app = App::new_test()?;
        app.screen = AppScreen::RentalManagement(RentalTab::List);

        app.handle_key(key(KeyCode::Esc))?;
        assert_eq!(app.status_bar.escape_count, 1);
        app.handle_key(key(KeyCode::Esc))?;
        assert_eq!(app.status_bar.escape_count, 2);
        app.handle_key(key(KeyCode::Esc))?;
        assert_eq!(app.status_bar.escape_count, 0);
        assert_eq!(app.screen, AppScreen::Dashboard);
        Ok(())
    }

    #[test]
    fn escape_counter_resets_on_other_key() -> Result<()> {
        let mut app = App::new_test()?;
        app.handle_key(key(KeyCode::Esc))?;
        assert_eq!(app.status_bar.escape_count, 1);
        app.handle_key(key(KeyCode::Char('a')))?;
        assert_eq!(app.status_bar.escape_count, 0);
        Ok(())
    }

    #[test]
    fn screensaver_activation_and_exit() -> Result<()> {
        let mut app = App::new_test()?;
        app.screensaver_timeout = Duration::from_secs(0);
        app.countdown_duration = Duration::from_secs(0);
        app.last_activity = Instant::now() - Duration::from_secs(2);

        assert!(matches!(
            app.check_inactivity(),
            InactivityState::ScreensaverActive
        ));

        app.activate_screensaver();
        assert!(app.screensaver_active);
        assert_eq!(app.screen, AppScreen::Screensaver);

        app.exit_screensaver();
        assert!(!app.screensaver_active);
        assert_eq!(app.screen, AppScreen::Dashboard);
        Ok(())
    }
}
