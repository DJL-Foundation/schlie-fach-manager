use crate::{config::AppSettings, model::BillingPeriod};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Editable settings fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsField {
    Deposit,
    YearlyFee,
    BillingPeriod,
    Currency,
    ScreensaverTimeout,
    ExportFormat,
    ExportDirectory,
}

/// Local editor state for the settings screen.
#[derive(Debug, Clone)]
pub struct SettingsEditor {
    fields: Vec<SettingsField>,
    selected: usize,
    pub draft: AppSettings,
    pub dirty: bool,
}

impl SettingsEditor {
    /// Creates a new settings editor based on the provided settings.
    pub fn new(settings: &AppSettings) -> Self {
        Self {
            fields: vec![
                SettingsField::Deposit,
                SettingsField::YearlyFee,
                SettingsField::BillingPeriod,
                SettingsField::Currency,
                SettingsField::ScreensaverTimeout,
                SettingsField::ExportFormat,
                SettingsField::ExportDirectory,
            ],
            selected: 0,
            draft: settings.clone(),
            dirty: false,
        }
    }

    /// Handles keyboard input and returns an action if necessary.
    pub fn handle_key(&mut self, key: KeyCode) -> SettingsAction {
        match key {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected + 1 < self.fields.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Left => self.adjust_selected(-1),
            KeyCode::Right => self.adjust_selected(1),
            KeyCode::Enter => return SettingsAction::Save(self.draft.clone()),
            KeyCode::Esc => return SettingsAction::Cancel,
            _ => {}
        }
        SettingsAction::None
    }

    /// Renders the settings editor to the provided area.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();
        for (idx, field) in self.fields.iter().enumerate() {
            let line = self.format_field(*field);
            let style = if idx == self.selected {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            lines.push(line.style(style));
        }

        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Einstellungen"),
            )
            .render(area, buf);
    }

    /// Adjusts the selected field by the provided delta.
    fn adjust_selected(&mut self, direction: i32) {
        let delta = direction.signum() as i32;
        match self.fields[self.selected] {
            SettingsField::Deposit => {
                self.draft.deposit_cents =
                    (self.draft.deposit_cents + (delta as i64 * 100)).max(0);
                self.dirty = true;
            }
            SettingsField::YearlyFee => {
                self.draft.yearly_fee_cents =
                    (self.draft.yearly_fee_cents + (delta as i64 * 100)).max(0);
                self.dirty = true;
            }
            SettingsField::BillingPeriod => {
                self.draft.billing_period = match self.draft.billing_period {
                    BillingPeriod::Monthly => BillingPeriod::Yearly,
                    BillingPeriod::Yearly => BillingPeriod::Monthly,
                };
                self.dirty = true;
            }
            SettingsField::Currency => {
                let options = ["EUR", "USD", "CHF"];
                let current = options
                    .iter()
                    .position(|opt| *opt == self.draft.currency)
                    .unwrap_or(0) as i32;
                let next = (current + delta).rem_euclid(options.len() as i32) as usize;
                self.draft.currency = options[next].to_string();
                self.dirty = true;
            }
            SettingsField::ScreensaverTimeout => {
                let next =
                    (self.draft.screensaver_timeout_seconds as i64 + (delta as i64 * 5)).max(15);
                self.draft.screensaver_timeout_seconds = next as u64;
                self.dirty = true;
            }
            SettingsField::ExportFormat => {
                let options = ["toml", "json", "csv"];
                let current = options
                    .iter()
                    .position(|opt| *opt == self.draft.export_format)
                    .unwrap_or(0) as i32;
                let next = (current + delta).rem_euclid(options.len() as i32) as usize;
                self.draft.export_format = options[next].to_string();
                self.dirty = true;
            }
            SettingsField::ExportDirectory => {}
        }
    }

    /// Formats a single field line for rendering.
    fn format_field(&self, field: SettingsField) -> Line {
        match field {
            SettingsField::Deposit => Line::from(format!(
                "Pfandbetrag: {:.2} € ({} Cents)",
                self.draft.deposit_cents as f64 / 100.0,
                self.draft.deposit_cents
            )),
            SettingsField::YearlyFee => Line::from(format!(
                "Jahresgebühr: {:.2} € ({} Cents)",
                self.draft.yearly_fee_cents as f64 / 100.0,
                self.draft.yearly_fee_cents
            )),
            SettingsField::BillingPeriod => Line::from(format!(
                "Berechnungszeitraum: {}",
                self.draft.billing_period.as_str()
            )),
            SettingsField::Currency => Line::from(format!("Währung: {}", self.draft.currency)),
            SettingsField::ScreensaverTimeout => Line::from(format!(
                "Screensaver-Timeout: {} Sekunden",
                self.draft.screensaver_timeout_seconds
            )),
            SettingsField::ExportFormat => {
                Line::from(format!("Standard-Format: {}", self.draft.export_format))
            }
            SettingsField::ExportDirectory => Line::from(format!(
                "Export-Verzeichnis: {}",
                self.draft.export_directory.to_string_lossy()
            )),
        }
    }
}

/// Possible actions resulting from the settings editor.
#[derive(Debug, Clone)]
pub enum SettingsAction {
    None,
    Save(AppSettings),
    Cancel,
}
