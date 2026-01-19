use std::time::{Duration, Instant};

/// Main app screens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    Dashboard,
    RentalManagement(RentalManagementTab),
    Finance(FinanceTab),
    Management(ManagementTab),
}

impl Default for AppScreen {
    fn default() -> Self {
        AppScreen::Dashboard
    }
}

/// Tabs within Rental Management.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RentalManagementTab {
    Search,  // Verleihen
    List,    // Liste
    Extend,  // Verlängern
    Return,  // Zurückgeben
    Damage,  // Defekt melden
}

impl RentalManagementTab {
    pub fn all() -> &'static [RentalManagementTab] {
        &[
            RentalManagementTab::Search,
            RentalManagementTab::List,
            RentalManagementTab::Extend,
            RentalManagementTab::Return,
            RentalManagementTab::Damage,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            RentalManagementTab::Search => "Verleihen",
            RentalManagementTab::List => "Liste",
            RentalManagementTab::Extend => "Verlängern",
            RentalManagementTab::Return => "Zurückgeben",
            RentalManagementTab::Damage => "Defekt melden",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            RentalManagementTab::Search => 0,
            RentalManagementTab::List => 1,
            RentalManagementTab::Extend => 2,
            RentalManagementTab::Return => 3,
            RentalManagementTab::Damage => 4,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// Tabs within Finance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinanceTab {
    Overview, // Übersicht
    Debtors,  // Schuldentabelle
}

impl FinanceTab {
    pub fn all() -> &'static [FinanceTab] {
        &[FinanceTab::Overview, FinanceTab::Debtors]
    }

    pub fn label(&self) -> &'static str {
        match self {
            FinanceTab::Overview => "Übersicht",
            FinanceTab::Debtors => "Schuldentabelle",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            FinanceTab::Overview => 0,
            FinanceTab::Debtors => 1,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// Tabs within Management.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagementTab {
    Lockers,   // Schließfächer
    Locations, // Standorte
    Backup,    // Backup
}

impl ManagementTab {
    pub fn all() -> &'static [ManagementTab] {
        &[
            ManagementTab::Lockers,
            ManagementTab::Locations,
            ManagementTab::Backup,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            ManagementTab::Lockers => "Schließfächer",
            ManagementTab::Locations => "Standorte",
            ManagementTab::Backup => "Backup",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            ManagementTab::Lockers => 0,
            ManagementTab::Locations => 1,
            ManagementTab::Backup => 2,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// Input mode for text input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Notification level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// A notification message.
#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub created_at: Instant,
    pub duration: Duration,
}

impl Notification {
    pub fn new(message: impl Into<String>, level: NotificationLevel) -> Self {
        Self {
            message: message.into(),
            level,
            created_at: Instant::now(),
            duration: Duration::from_secs(5),
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message, NotificationLevel::Info)
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message, NotificationLevel::Success)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message, NotificationLevel::Warning)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message, NotificationLevel::Error)
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }
}

/// Dialog state for confirmation dialogs.
#[derive(Debug, Clone)]
pub struct ConfirmDialog {
    pub title: String,
    pub message: String,
    pub subtitle: Option<String>,
    pub confirm_label: String,
    pub cancel_label: String,
    pub selected: usize, // 0 = confirm, 1 = cancel
}

impl ConfirmDialog {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            subtitle: None,
            confirm_label: "Ja".to_string(),
            cancel_label: "Nein".to_string(),
            selected: 0,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_labels(mut self, confirm: impl Into<String>, cancel: impl Into<String>) -> Self {
        self.confirm_label = confirm.into();
        self.cancel_label = cancel.into();
        self
    }

    pub fn toggle_selection(&mut self) {
        self.selected = if self.selected == 0 { 1 } else { 0 };
    }

    pub fn is_confirmed(&self) -> bool {
        self.selected == 0
    }
}

/// Combined UI state.
#[derive(Debug, Clone, Default)]
pub struct UiState {
    pub input_mode: InputMode,
    pub search_query: String,
    pub selected_index: usize,
    pub scroll_offset: usize,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

impl UiState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
    }

    pub fn push_search_char(&mut self, ch: char) {
        self.search_query.push(ch);
    }

    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
    }
}
