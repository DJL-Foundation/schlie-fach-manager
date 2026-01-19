use crate::{
    db::Database,
    model::{
        Debtor, Lease, Location, LocationStatistics, Locker, LockerHeight, LockerStatistics,
        LockerStatus, TenantType, is_valid_payment, is_valid_username,
    },
};
use chrono::{Duration, Utc};
use color_eyre::eyre::{Result, eyre};
use unicode_width::UnicodeWidthStr;

/// Main application tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTab {
    Dashboard,
    Management,
    Finance,
    Admin,
}

impl AppTab {
    pub fn all() -> &'static [AppTab] {
        &[AppTab::Dashboard, AppTab::Management, AppTab::Finance, AppTab::Admin]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppTab::Dashboard => "Dashboard",
            AppTab::Management => "Schließfächer",
            AppTab::Finance => "Finanzen",
            AppTab::Admin => "Admin",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            AppTab::Dashboard => 0,
            AppTab::Management => 1,
            AppTab::Finance => 2,
            AppTab::Admin => 3,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => AppTab::Dashboard,
            1 => AppTab::Management,
            2 => AppTab::Finance,
            3 => AppTab::Admin,
            _ => AppTab::Dashboard,
        }
    }
}

/// Sub-tabs for Management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagementSubTab {
    Search,      // Suchen & Verleihen
    List,        // Schließfächer Liste
    Extend,      // Verlängern & Rückgabe
    Damage,      // Defekt Meldung
}

impl ManagementSubTab {
    pub fn all() -> &'static [ManagementSubTab] {
        &[ManagementSubTab::Search, ManagementSubTab::List, ManagementSubTab::Extend, ManagementSubTab::Damage]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ManagementSubTab::Search => "Verleihen",
            ManagementSubTab::List => "Liste",
            ManagementSubTab::Extend => "Verlängern",
            ManagementSubTab::Damage => "Defekt",
        }
    }
}

/// Sub-tabs for Admin
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminSubTab {
    Locations,
    Lockers,
    Bulk,
    Backup,
}

impl AdminSubTab {
    pub fn all() -> &'static [AdminSubTab] {
        &[AdminSubTab::Locations, AdminSubTab::Lockers, AdminSubTab::Bulk, AdminSubTab::Backup]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AdminSubTab::Locations => "Standorte",
            AdminSubTab::Lockers => "Schließfächer",
            AdminSubTab::Bulk => "Bulk-Anlage",
            AdminSubTab::Backup => "Backup",
        }
    }
}

/// Input mode for the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Searching,
    Wizard,
    FormInput,
    Popup,
}

/// Wizard states for the rental process
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RentalWizardState {
    AskLocation,
    AskHeight,
    ConfirmLocker,
    AskUsername,
    AskTenantType,
    ConfirmSave,
    PreSaveCheck,      // "Hast du 20€ erhalten?"
    PostSaveReminder,  // "Hast du den Schüler erinnert?"
    Complete,
}

/// Wizard states for extending a lease
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendWizardState {
    AskIdentifier,      // Number or username
    ConfirmLease,       // Confirm the found lease
    AskPayment,         // How much was paid?
    ConfirmExtend,
    Complete,
}

/// Wizard states for returning a locker
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnWizardState {
    AskIdentifier,
    ConfirmLease,
    CheckDebt,          // Check for overdue payments
    ConfirmDepositReturn,
    Complete,
}

/// Chat message for wizard dialogs
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub from_bot: bool,
    pub text: String,
}

/// Popup types
#[derive(Debug, Clone)]
pub enum PopupType {
    Confirmation { message: String, on_confirm: PopupAction },
    Info { message: String },
    Error { message: String },
}

#[derive(Debug, Clone)]
pub enum PopupAction {
    ConfirmRental,
    ConfirmExtend,
    ConfirmReturn,
    ConfirmDelete,
    None,
}

/// Application state container
pub struct App {
    pub db: Database,

    // Navigation
    pub current_tab: AppTab,
    pub management_subtab: ManagementSubTab,
    pub admin_subtab: AdminSubTab,

    // Input handling
    pub input_mode: InputMode,
    pub search_query: String,
    pub form_input: String,

    // Data caches
    pub lockers: Vec<Locker>,
    pub locations: Vec<Location>,
    pub active_leases: Vec<Lease>,
    pub debtors: Vec<Debtor>,
    pub statistics: LockerStatistics,
    pub location_statistics: Vec<LocationStatistics>,

    // List navigation
    pub filtered_indices: Vec<usize>,
    pub selected_index: usize,
    pub location_selected: usize,

    // Wizard state
    pub rental_wizard: Option<RentalWizardData>,
    pub extend_wizard: Option<ExtendWizardData>,
    pub return_wizard: Option<ReturnWizardData>,
    pub chat_messages: Vec<ChatMessage>,

    // Popup
    pub popup: Option<PopupType>,

    // Status
    pub status_message: Option<String>,

    // Form mode vs chat mode
    pub use_chat_mode: bool,
}

/// Data for rental wizard
#[derive(Debug, Clone)]
pub struct RentalWizardData {
    pub state: RentalWizardState,
    pub selected_location_id: Option<i64>,
    pub selected_height: Option<LockerHeight>,
    pub suggested_locker: Option<Locker>,
    pub username: String,
    pub tenant_type: Option<TenantType>,
}

impl Default for RentalWizardData {
    fn default() -> Self {
        Self {
            state: RentalWizardState::AskLocation,
            selected_location_id: None,
            selected_height: None,
            suggested_locker: None,
            username: String::new(),
            tenant_type: None,
        }
    }
}

/// Data for extend wizard
#[derive(Debug, Clone)]
pub struct ExtendWizardData {
    pub state: ExtendWizardState,
    pub identifier: String,
    pub found_lease: Option<Lease>,
    pub payment_amount: i64,
}

impl Default for ExtendWizardData {
    fn default() -> Self {
        Self {
            state: ExtendWizardState::AskIdentifier,
            identifier: String::new(),
            found_lease: None,
            payment_amount: 10,
        }
    }
}

/// Data for return wizard
#[derive(Debug, Clone)]
pub struct ReturnWizardData {
    pub state: ReturnWizardState,
    pub identifier: String,
    pub found_lease: Option<Lease>,
    pub debt_amount: i64,
    pub debt_paid: bool,
}

impl Default for ReturnWizardData {
    fn default() -> Self {
        Self {
            state: ReturnWizardState::AskIdentifier,
            identifier: String::new(),
            found_lease: None,
            debt_amount: 0,
            debt_paid: false,
        }
    }
}

impl App {
    /// Creates a new application instance
    pub fn new(db: Database) -> Result<Self> {
        let mut app = Self {
            db,
            current_tab: AppTab::Dashboard,
            management_subtab: ManagementSubTab::List,
            admin_subtab: AdminSubTab::Locations,
            input_mode: InputMode::Normal,
            search_query: String::new(),
            form_input: String::new(),
            lockers: Vec::new(),
            locations: Vec::new(),
            active_leases: Vec::new(),
            debtors: Vec::new(),
            statistics: LockerStatistics::default(),
            location_statistics: Vec::new(),
            filtered_indices: Vec::new(),
            selected_index: 0,
            location_selected: 0,
            rental_wizard: None,
            extend_wizard: None,
            return_wizard: None,
            chat_messages: Vec::new(),
            popup: None,
            status_message: None,
            use_chat_mode: true,
        };
        app.reload()?;
        Ok(app)
    }

    /// Reloads all data from the database
    pub fn reload(&mut self) -> Result<()> {
        self.lockers = self.db.list_lockers()?;
        self.locations = self.db.list_locations()?;
        self.active_leases = self.db.list_active_leases()?;
        self.debtors = self.db.list_overdue_leases(Utc::now())?;
        self.statistics = self.db.get_locker_statistics()?;
        self.location_statistics = self.db.get_location_statistics()?;
        self.apply_filter();
        Ok(())
    }

    // ========== Navigation ==========

    pub fn next_tab(&mut self) {
        let next = (self.current_tab.index() + 1) % AppTab::all().len();
        self.current_tab = AppTab::from_index(next);
        self.reset_selection();
    }

    pub fn prev_tab(&mut self) {
        let tabs = AppTab::all().len();
        let prev = (self.current_tab.index() + tabs - 1) % tabs;
        self.current_tab = AppTab::from_index(prev);
        self.reset_selection();
    }

    pub fn set_tab(&mut self, tab: AppTab) {
        self.current_tab = tab;
        self.reset_selection();
    }

    pub fn next_subtab(&mut self) {
        match self.current_tab {
            AppTab::Management => {
                let subtabs = ManagementSubTab::all();
                let current = subtabs.iter().position(|s| *s == self.management_subtab).unwrap_or(0);
                self.management_subtab = subtabs[(current + 1) % subtabs.len()];
            }
            AppTab::Admin => {
                let subtabs = AdminSubTab::all();
                let current = subtabs.iter().position(|s| *s == self.admin_subtab).unwrap_or(0);
                self.admin_subtab = subtabs[(current + 1) % subtabs.len()];
            }
            _ => {}
        }
        self.reset_selection();
    }

    fn reset_selection(&mut self) {
        self.selected_index = 0;
        self.apply_filter();
        self.cancel_wizard();
    }

    // ========== List Navigation ==========

    pub fn visible_count(&self) -> usize {
        self.filtered_indices.len()
    }

    pub fn selected_idx(&self) -> Option<usize> {
        if self.filtered_indices.is_empty() {
            None
        } else {
            Some(self.selected_index)
        }
    }

    pub fn next(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.filtered_indices.len();
    }

    pub fn previous(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = self.filtered_indices.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn selected_locker(&self) -> Option<&Locker> {
        self.filtered_indices
            .get(self.selected_index)
            .and_then(|&idx| self.lockers.get(idx))
    }

    pub fn visible_lockers(&self) -> impl Iterator<Item = &Locker> + '_ {
        self.filtered_indices
            .iter()
            .filter_map(|&idx| self.lockers.get(idx))
    }

    // ========== Search & Filter ==========

    pub fn search_width(&self) -> usize {
        UnicodeWidthStr::width(self.search_query.as_str())
    }

    pub fn push_search_char(&mut self, ch: char) {
        self.search_query.push(ch);
        self.apply_filter();
    }

    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
        self.apply_filter();
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.apply_filter();
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    fn apply_filter(&mut self) {
        if self.search_query.trim().is_empty() {
            self.filtered_indices = (0..self.lockers.len()).collect();
            self.selected_index = 0;
            return;
        }

        let needle = self.search_query.to_lowercase();
        self.filtered_indices = self
            .lockers
            .iter()
            .enumerate()
            .filter(|(_, locker)| locker.matches_query(&needle))
            .map(|(idx, _)| idx)
            .collect();

        self.selected_index = 0;
    }

    // ========== Status Messages ==========

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    // ========== Popup Management ==========

    pub fn show_popup(&mut self, popup: PopupType) {
        self.popup = Some(popup);
        self.input_mode = InputMode::Popup;
    }

    pub fn close_popup(&mut self) {
        self.popup = None;
        if self.rental_wizard.is_some() || self.extend_wizard.is_some() || self.return_wizard.is_some() {
            self.input_mode = InputMode::Wizard;
        } else {
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn confirm_popup(&mut self) -> Result<()> {
        if let Some(popup) = self.popup.take() {
            match popup {
                PopupType::Confirmation { on_confirm, .. } => {
                    match on_confirm {
                        PopupAction::ConfirmRental => self.complete_rental()?,
                        PopupAction::ConfirmExtend => self.complete_extend()?,
                        PopupAction::ConfirmReturn => self.complete_return()?,
                        PopupAction::ConfirmDelete => self.delete_selected()?,
                        PopupAction::None => {}
                    }
                }
                _ => {}
            }
        }
        self.close_popup();
        Ok(())
    }

    // ========== Rental Wizard ==========

    pub fn start_rental_wizard(&mut self) {
        self.rental_wizard = Some(RentalWizardData::default());
        self.chat_messages.clear();
        self.chat_messages.push(ChatMessage {
            from_bot: true,
            text: "Willkommen! Welcher Standort wird bevorzugt?".to_string(),
        });
        self.input_mode = InputMode::Wizard;
        self.form_input.clear();
    }

    pub fn advance_rental_wizard(&mut self) -> Result<()> {
        // First, get the current state without mutable borrow
        let current_state = match &self.rental_wizard {
            Some(w) => w.state.clone(),
            None => return Ok(()),
        };

        match current_state {
            RentalWizardState::AskLocation => {
                if self.location_selected < self.locations.len() {
                    let loc_id = self.locations[self.location_selected].id;
                    let loc_name = self.locations[self.location_selected].name.clone();
                    
                    if let Some(wizard) = &mut self.rental_wizard {
                        wizard.selected_location_id = Some(loc_id);
                        wizard.state = RentalWizardState::AskHeight;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: false,
                        text: loc_name,
                    });
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Welche Höhe? (Oben/Mitte/Unten)".to_string(),
                    });
                    self.selected_index = 1; // Default to Middle
                }
            }
            RentalWizardState::AskHeight => {
                let heights = LockerHeight::all();
                if self.selected_index < heights.len() {
                    let height = heights[self.selected_index];
                    let location_id = self.rental_wizard.as_ref().and_then(|w| w.selected_location_id);
                    
                    if let Some(wizard) = &mut self.rental_wizard {
                        wizard.selected_height = Some(height);
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: false,
                        text: height.display_name().to_string(),
                    });

                    // Find a free locker
                    let free_lockers = self.db.list_free_lockers(location_id, Some(height))?;

                    if let Some(locker) = free_lockers.first() {
                        let msg = format!(
                            "Vorschlag: Fach {} ({}). Ist das okay? (j/n)",
                            locker.display_number,
                            locker.location_name.as_deref().unwrap_or("?")
                        );
                        if let Some(wizard) = &mut self.rental_wizard {
                            wizard.suggested_locker = Some(locker.clone());
                            wizard.state = RentalWizardState::ConfirmLocker;
                        }
                        self.chat_messages.push(ChatMessage {
                            from_bot: true,
                            text: msg,
                        });
                    } else {
                        self.chat_messages.push(ChatMessage {
                            from_bot: true,
                            text: "Keine freien Schließfächer mit diesen Kriterien gefunden.".to_string(),
                        });
                        if let Some(wizard) = &mut self.rental_wizard {
                            wizard.state = RentalWizardState::AskLocation;
                        }
                        self.chat_messages.push(ChatMessage {
                            from_bot: true,
                            text: "Bitte wähle einen anderen Standort oder Höhe.".to_string(),
                        });
                    }
                }
            }
            RentalWizardState::ConfirmLocker => {
                let input = self.form_input.trim().to_lowercase();
                let form_input_copy = self.form_input.clone();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: form_input_copy,
                });
                if input == "j" || input == "ja" || input == "y" || input == "yes" {
                    if let Some(wizard) = &mut self.rental_wizard {
                        wizard.state = RentalWizardState::AskUsername;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Wie lautet der IServ Benutzername? (ohne @athenetz.de)".to_string(),
                    });
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "💡 Denk an den IServ Namen für Mahnungen!".to_string(),
                    });
                } else {
                    if let Some(wizard) = &mut self.rental_wizard {
                        wizard.state = RentalWizardState::AskLocation;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Okay, lass uns neu suchen. Welcher Standort?".to_string(),
                    });
                }
                self.form_input.clear();
            }
            RentalWizardState::AskUsername => {
                let username = self.form_input.trim().to_string();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: username.clone(),
                });

                if !is_valid_username(&username) {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "⚠️ Ungültiger Benutzername! Bitte erneut eingeben:".to_string(),
                    });
                } else {
                    let email_msg = format!("✓ Email wird: {}@athenetz.de", username);
                    if let Some(wizard) = &mut self.rental_wizard {
                        wizard.username = username;
                        wizard.state = RentalWizardState::AskTenantType;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: email_msg,
                    });
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Ist der Mieter Lehrer oder Schüler?".to_string(),
                    });
                    self.selected_index = 0;
                }
                self.form_input.clear();
            }
            RentalWizardState::AskTenantType => {
                let types = TenantType::all();
                if self.selected_index < types.len() {
                    let tenant_type = types[self.selected_index];
                    if let Some(wizard) = &mut self.rental_wizard {
                        wizard.tenant_type = Some(tenant_type);
                        wizard.state = RentalWizardState::ConfirmSave;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: false,
                        text: tenant_type.display_name().to_string(),
                    });
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Alles klar! Speichern? (j/n)".to_string(),
                    });
                }
            }
            RentalWizardState::ConfirmSave => {
                let input = self.form_input.trim().to_lowercase();
                let form_input_copy = self.form_input.clone();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: form_input_copy,
                });
                if input == "j" || input == "ja" || input == "y" || input == "yes" {
                    if let Some(wizard) = &mut self.rental_wizard {
                        wizard.state = RentalWizardState::PreSaveCheck;
                    }
                    self.show_popup(PopupType::Confirmation {
                        message: "💰 Hast du 20€ erhalten? (10€ Pfand + 10€ Miete)".to_string(),
                        on_confirm: PopupAction::ConfirmRental,
                    });
                } else {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Vorgang abgebrochen.".to_string(),
                    });
                    self.cancel_wizard();
                }
                self.form_input.clear();
            }
            RentalWizardState::PreSaveCheck => {
                // Handled by popup
            }
            RentalWizardState::PostSaveReminder => {
                self.close_popup();
                self.cancel_wizard();
                self.reload()?;
            }
            RentalWizardState::Complete => {
                self.cancel_wizard();
                self.reload()?;
            }
        }

        Ok(())
    }

    fn complete_rental(&mut self) -> Result<()> {
        let wizard = match &self.rental_wizard {
            Some(w) => w.clone(),
            None => return Err(eyre!("No rental wizard active")),
        };

        let locker = wizard.suggested_locker.ok_or_else(|| eyre!("No locker selected"))?;
        let tenant_type = wizard.tenant_type.ok_or_else(|| eyre!("No tenant type selected"))?;

        let now = Utc::now();
        let end = now + Duration::days(365);

        let mut lease = Lease::new(
            0,
            locker.id,
            &wizard.username,
            tenant_type,
            now,
            end,
        );
        lease.deposit_paid = true;

        self.db.create_lease(&lease)?;

        if let Some(w) = &mut self.rental_wizard {
            w.state = RentalWizardState::PostSaveReminder;
        }

        self.show_popup(PopupType::Info {
            message: "✅ Schließfach erfolgreich vermietet!\n\n📝 Hast du den Mieter daran erinnert, dass jährlich verlängert werden muss?".to_string(),
        });

        self.set_status(&format!(
            "Schließfach {} an {} vermietet",
            locker.display_number, wizard.username
        ));

        Ok(())
    }

    // ========== Extend Wizard ==========

    pub fn start_extend_wizard(&mut self) {
        self.extend_wizard = Some(ExtendWizardData::default());
        self.chat_messages.clear();
        self.chat_messages.push(ChatMessage {
            from_bot: true,
            text: "Verlängerung: Bitte Schließfachnummer oder Benutzername eingeben:".to_string(),
        });
        self.input_mode = InputMode::Wizard;
        self.form_input.clear();
    }

    pub fn advance_extend_wizard(&mut self) -> Result<()> {
        let current_state = match &self.extend_wizard {
            Some(w) => w.state.clone(),
            None => return Ok(()),
        };

        match current_state {
            ExtendWizardState::AskIdentifier => {
                let identifier = self.form_input.trim().to_string();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: identifier.clone(),
                });

                // Try to find by locker number or username
                let lease = self.find_lease_by_identifier(&identifier)?;

                if let Some(lease) = lease {
                    let msg = format!(
                        "Gefunden: Fach {} von {}. Richtig? (j/n)",
                        lease.locker_display_number.as_deref().unwrap_or("?"),
                        lease.tenant_username
                    );
                    if let Some(wizard) = &mut self.extend_wizard {
                        wizard.found_lease = Some(lease);
                        wizard.state = ExtendWizardState::ConfirmLease;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: msg,
                    });
                } else {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "⚠️ Kein aktiver Mietvertrag gefunden. Bitte erneut eingeben:".to_string(),
                    });
                }
                self.form_input.clear();
            }
            ExtendWizardState::ConfirmLease => {
                let input = self.form_input.trim().to_lowercase();
                let form_input_copy = self.form_input.clone();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: form_input_copy,
                });
                if input == "j" || input == "ja" || input == "y" || input == "yes" {
                    if let Some(wizard) = &mut self.extend_wizard {
                        wizard.state = ExtendWizardState::AskPayment;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Wie viel Geld wurde übergeben? (Muss durch 10 teilbar sein, Standard: 10€)".to_string(),
                    });
                } else {
                    if let Some(wizard) = &mut self.extend_wizard {
                        wizard.state = ExtendWizardState::AskIdentifier;
                        wizard.found_lease = None;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Bitte erneut Schließfachnummer oder Benutzername eingeben:".to_string(),
                    });
                }
                self.form_input.clear();
            }
            ExtendWizardState::AskPayment => {
                let input = self.form_input.trim().to_string();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: input.clone(),
                });

                let amount: i64 = if input.is_empty() {
                    10
                } else {
                    input.parse().unwrap_or(0)
                };

                if !is_valid_payment(amount) {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "⚠️ Betrag muss durch 10 teilbar sein! Erneut eingeben:".to_string(),
                    });
                } else {
                    let years = amount / 10;
                    let msg = format!("{}€ = {} Jahr(e) Verlängerung. Bestätigen? (j/n)", amount, years);
                    if let Some(wizard) = &mut self.extend_wizard {
                        wizard.payment_amount = amount;
                        wizard.state = ExtendWizardState::ConfirmExtend;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: msg,
                    });
                }
                self.form_input.clear();
            }
            ExtendWizardState::ConfirmExtend => {
                let input = self.form_input.trim().to_lowercase();
                let form_input_copy = self.form_input.clone();
                let payment_amount = self.extend_wizard.as_ref().map(|w| w.payment_amount).unwrap_or(10);
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: form_input_copy,
                });
                if input == "j" || input == "ja" || input == "y" || input == "yes" {
                    self.show_popup(PopupType::Confirmation {
                        message: format!("Verlängerung um {}€ bestätigen?", payment_amount),
                        on_confirm: PopupAction::ConfirmExtend,
                    });
                } else {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Vorgang abgebrochen.".to_string(),
                    });
                    self.cancel_wizard();
                }
                self.form_input.clear();
            }
            ExtendWizardState::Complete => {
                self.cancel_wizard();
                self.reload()?;
            }
        }

        Ok(())
    }

    fn complete_extend(&mut self) -> Result<()> {
        let wizard = match &self.extend_wizard {
            Some(w) => w.clone(),
            None => return Err(eyre!("No extend wizard active")),
        };

        let mut lease = wizard.found_lease.ok_or_else(|| eyre!("No lease found"))?;
        lease.extend(wizard.payment_amount);

        self.db.extend_lease(lease.id, lease.end_date)?;

        self.set_status(&format!(
            "Mietvertrag für {} um {} Jahr(e) verlängert",
            lease.tenant_username,
            wizard.payment_amount / 10
        ));

        if let Some(w) = &mut self.extend_wizard {
            w.state = ExtendWizardState::Complete;
        }

        self.chat_messages.push(ChatMessage {
            from_bot: true,
            text: format!("✅ Verlängert bis {}", lease.end_date.format("%d.%m.%Y")),
        });

        self.cancel_wizard();
        self.reload()?;

        Ok(())
    }

    // ========== Return Wizard ==========

    pub fn start_return_wizard(&mut self) {
        self.return_wizard = Some(ReturnWizardData::default());
        self.chat_messages.clear();
        self.chat_messages.push(ChatMessage {
            from_bot: true,
            text: "Rückgabe: Bitte Schließfachnummer oder Benutzername eingeben:".to_string(),
        });
        self.input_mode = InputMode::Wizard;
        self.form_input.clear();
    }

    pub fn advance_return_wizard(&mut self) -> Result<()> {
        let current_state = match &self.return_wizard {
            Some(w) => w.state.clone(),
            None => return Ok(()),
        };

        match current_state {
            ReturnWizardState::AskIdentifier => {
                let identifier = self.form_input.trim().to_string();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: identifier.clone(),
                });

                let lease = self.find_lease_by_identifier(&identifier)?;

                if let Some(lease) = lease {
                    let debt = lease.calculate_debt(Utc::now());
                    let msg = format!(
                        "Gefunden: Fach {} von {}. Richtig? (j/n)",
                        lease.locker_display_number.as_deref().unwrap_or("?"),
                        lease.tenant_username
                    );
                    if let Some(wizard) = &mut self.return_wizard {
                        wizard.found_lease = Some(lease);
                        wizard.debt_amount = debt;
                        wizard.state = ReturnWizardState::ConfirmLease;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: msg,
                    });
                } else {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "⚠️ Kein aktiver Mietvertrag gefunden. Bitte erneut eingeben:".to_string(),
                    });
                }
                self.form_input.clear();
            }
            ReturnWizardState::ConfirmLease => {
                let input = self.form_input.trim().to_lowercase();
                let form_input_copy = self.form_input.clone();
                let debt_amount = self.return_wizard.as_ref().map(|w| w.debt_amount).unwrap_or(0);
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: form_input_copy,
                });
                if input == "j" || input == "ja" || input == "y" || input == "yes" {
                    if debt_amount > 0 {
                        if let Some(wizard) = &mut self.return_wizard {
                            wizard.state = ReturnWizardState::CheckDebt;
                        }
                        self.chat_messages.push(ChatMessage {
                            from_bot: true,
                            text: format!("⚠️ Benutzer schuldet noch {}€. Wurde bezahlt? (j/n)", debt_amount),
                        });
                    } else {
                        if let Some(wizard) = &mut self.return_wizard {
                            wizard.state = ReturnWizardState::ConfirmDepositReturn;
                        }
                        self.chat_messages.push(ChatMessage {
                            from_bot: true,
                            text: "Pfand (10€) zurückgegeben? (j/n)".to_string(),
                        });
                    }
                } else {
                    if let Some(wizard) = &mut self.return_wizard {
                        wizard.state = ReturnWizardState::AskIdentifier;
                        wizard.found_lease = None;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Bitte erneut Schließfachnummer oder Benutzername eingeben:".to_string(),
                    });
                }
                self.form_input.clear();
            }
            ReturnWizardState::CheckDebt => {
                let input = self.form_input.trim().to_lowercase();
                let form_input_copy = self.form_input.clone();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: form_input_copy,
                });
                if input == "j" || input == "ja" || input == "y" || input == "yes" {
                    if let Some(wizard) = &mut self.return_wizard {
                        wizard.debt_paid = true;
                        wizard.state = ReturnWizardState::ConfirmDepositReturn;
                    }
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "Pfand (10€) zurückgegeben? (j/n)".to_string(),
                    });
                } else {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "❌ Rückgabe abgebrochen - Schulden nicht bezahlt.".to_string(),
                    });
                    self.cancel_wizard();
                }
                self.form_input.clear();
            }
            ReturnWizardState::ConfirmDepositReturn => {
                let input = self.form_input.trim().to_lowercase();
                let form_input_copy = self.form_input.clone();
                self.chat_messages.push(ChatMessage {
                    from_bot: false,
                    text: form_input_copy,
                });
                if input == "j" || input == "ja" || input == "y" || input == "yes" {
                    self.show_popup(PopupType::Confirmation {
                        message: "Rückgabe abschließen?".to_string(),
                        on_confirm: PopupAction::ConfirmReturn,
                    });
                } else {
                    self.chat_messages.push(ChatMessage {
                        from_bot: true,
                        text: "⚠️ Bitte Pfand zurückgeben, dann erneut bestätigen.".to_string(),
                    });
                }
                self.form_input.clear();
            }
            ReturnWizardState::Complete => {
                self.cancel_wizard();
                self.reload()?;
            }
        }

        Ok(())
    }

    fn complete_return(&mut self) -> Result<()> {
        let wizard = match &self.return_wizard {
            Some(w) => w.clone(),
            None => return Err(eyre!("No return wizard active")),
        };

        let lease = wizard.found_lease.ok_or_else(|| eyre!("No lease found"))?;

        self.db.end_lease(lease.id, lease.locker_id)?;

        self.set_status(&format!(
            "Schließfach {} von {} zurückgegeben",
            lease.locker_display_number.as_deref().unwrap_or("?"),
            lease.tenant_username
        ));

        if let Some(w) = &mut self.return_wizard {
            w.state = ReturnWizardState::Complete;
        }

        self.chat_messages.push(ChatMessage {
            from_bot: true,
            text: "✅ Rückgabe erfolgreich abgeschlossen!".to_string(),
        });

        self.cancel_wizard();
        self.reload()?;

        Ok(())
    }

    fn find_lease_by_identifier(&self, identifier: &str) -> Result<Option<Lease>> {
        // First try to find by locker display number
        for locker in &self.lockers {
            if locker.display_number.to_lowercase() == identifier.to_lowercase() {
                return self.db.get_active_lease(locker.id);
            }
        }

        // Then try by username
        self.db.get_lease_by_username(identifier)
    }

    pub fn cancel_wizard(&mut self) {
        self.rental_wizard = None;
        self.extend_wizard = None;
        self.return_wizard = None;
        self.chat_messages.clear();
        self.form_input.clear();
        self.input_mode = InputMode::Normal;
    }

    // ========== Form Input ==========

    pub fn push_form_char(&mut self, ch: char) {
        self.form_input.push(ch);
    }

    pub fn pop_form_char(&mut self) {
        self.form_input.pop();
    }

    pub fn clear_form_input(&mut self) {
        self.form_input.clear();
    }

    pub fn form_input_width(&self) -> usize {
        UnicodeWidthStr::width(self.form_input.as_str())
    }

    // ========== Admin Operations ==========

    pub fn add_location(&mut self, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(eyre!("Name darf nicht leer sein"));
        }
        self.db.insert_location(name.trim())?;
        self.reload()?;
        self.set_status(&format!("Standort '{}' hinzugefügt", name.trim()));
        Ok(())
    }

    pub fn delete_selected(&mut self) -> Result<()> {
        match self.current_tab {
            AppTab::Admin => {
                match self.admin_subtab {
                    AdminSubTab::Locations => {
                        if let Some(&idx) = self.filtered_indices.get(self.selected_index) {
                            if let Some(loc) = self.locations.get(idx) {
                                self.db.delete_location(loc.id)?;
                                self.reload()?;
                                self.set_status("Standort gelöscht");
                            }
                        }
                    }
                    AdminSubTab::Lockers => {
                        if let Some(locker) = self.selected_locker() {
                            self.db.delete_locker(locker.id)?;
                            self.reload()?;
                            self.set_status("Schließfach gelöscht");
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn bulk_create_lockers(
        &mut self,
        location_id: i64,
        prefix: &str,
        start: i64,
        end: i64,
        height: LockerHeight,
    ) -> Result<()> {
        let count = self.db.bulk_create_lockers(location_id, prefix, start, end, height)?;
        self.reload()?;
        self.set_status(&format!("{} Schließfächer erstellt", count));
        Ok(())
    }

    pub fn mark_locker_damaged(&mut self, note: Option<String>) -> Result<()> {
        if let Some(locker) = self.selected_locker().cloned() {
            let mut updated = locker.clone();
            updated.is_damaged = true;
            updated.note = note;

            // If not occupied, set to maintenance
            if updated.status == LockerStatus::Free {
                updated.status = LockerStatus::Maintenance;
            }

            self.db.update_locker(&updated)?;
            self.reload()?;
            self.set_status(&format!("Schließfach {} als defekt markiert", locker.display_number));
        }
        Ok(())
    }

    pub fn repair_locker(&mut self) -> Result<()> {
        if let Some(locker) = self.selected_locker().cloned() {
            let mut updated = locker.clone();
            updated.is_damaged = false;

            // If in maintenance and no active lease, set to free
            if updated.status == LockerStatus::Maintenance {
                let lease = self.db.get_active_lease(updated.id)?;
                if lease.is_none() {
                    updated.status = LockerStatus::Free;
                }
            }

            self.db.update_locker(&updated)?;
            self.reload()?;
            self.set_status(&format!("Schließfach {} repariert", locker.display_number));
        }
        Ok(())
    }

    // ========== Mode Toggle ==========

    pub fn toggle_chat_mode(&mut self) {
        self.use_chat_mode = !self.use_chat_mode;
        let mode = if self.use_chat_mode { "Chat" } else { "Formular" };
        self.set_status(&format!("Modus: {}", mode));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_app() -> App {
        let db = Database::open_in_memory().expect("in-memory db");
        db.insert_location("Hauptgebäude").expect("insert location");
        db.insert_location("Sporthalle").expect("insert location");

        // Create some lockers
        let locations = db.list_locations().expect("list locations");
        for i in 1..=5 {
            let locker = Locker::new(0, format!("A-{:02}", i), locations[0].id, LockerHeight::Middle);
            db.insert_locker(&locker).expect("insert locker");
        }

        App::new(db).expect("app init")
    }

    #[test]
    fn tab_navigation() {
        let mut app = setup_app();

        assert_eq!(app.current_tab, AppTab::Dashboard);
        app.next_tab();
        assert_eq!(app.current_tab, AppTab::Management);
        app.next_tab();
        assert_eq!(app.current_tab, AppTab::Finance);
        app.next_tab();
        assert_eq!(app.current_tab, AppTab::Admin);
        app.next_tab();
        assert_eq!(app.current_tab, AppTab::Dashboard);

        app.prev_tab();
        assert_eq!(app.current_tab, AppTab::Admin);
    }

    #[test]
    fn filtering_lockers() {
        let mut app = setup_app();

        assert_eq!(app.visible_count(), 5);

        app.push_search_char('A');
        app.push_search_char('-');
        app.push_search_char('0');
        app.push_search_char('1');
        assert_eq!(app.visible_count(), 1);

        app.clear_search();
        assert_eq!(app.visible_count(), 5);
    }

    #[test]
    fn list_navigation() {
        let mut app = setup_app();

        assert_eq!(app.selected_idx(), Some(0));
        app.next();
        assert_eq!(app.selected_idx(), Some(1));
        app.previous();
        assert_eq!(app.selected_idx(), Some(0));
        app.previous();
        assert_eq!(app.selected_idx(), Some(4)); // Wrap around
    }
}
