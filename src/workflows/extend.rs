use crate::config::AppSettings;
use crate::db::connection::Database;
use crate::{
    db,
    ui::widgets::wizard::{
        MessageContent, MessageSender, WizardAction, WizardMessage, WizardOption, WizardRenderer,
    },
    workflows::common::{WorkflowAction, WorkflowResult},
};
use crossterm::event::KeyCode;

/// Workflow for extending a rental.
pub struct ExtendWorkflow {
    renderer: WizardRenderer,
    options: Vec<WizardOption>,
}

impl ExtendWorkflow {
    pub fn new(db: &Database, _settings: &AppSettings) -> Self {
        let rentals = db::rentals::list_rentals(db.connection()).unwrap_or_default();
        let options = rentals
            .into_iter()
            .map(|rental| WizardOption {
                label: format!("{} ({})", rental.renter_name, rental.end_date),
                value: rental.id.to_string(),
                metadata: Some(format!("ID {}", rental.id)),
            })
            .collect::<Vec<_>>();

        let message = WizardMessage {
            sender: MessageSender::System,
            content: MessageContent::Question {
                text: "Welchen Vertrag verlängern?".to_string(),
                options: if options.is_empty() {
                    vec![WizardOption {
                        label: "Keine Verträge verfügbar".to_string(),
                        value: "0".to_string(),
                        metadata: None,
                    }]
                } else {
                    options.clone()
                },
            },
            timestamp: None,
        };

        Self {
            renderer: WizardRenderer {
                messages: vec![message],
                current_question_index: 0,
                selected_option_index: 0,
            },
            options,
        }
    }

    pub fn renderer(&self) -> &WizardRenderer {
        &self.renderer
    }

    /// Handles keyboard input for the workflow.
    pub fn handle_key(&mut self, key: KeyCode) -> WorkflowAction {
        match self.renderer.handle_key(key) {
            WizardAction::Confirm(index) => {
                if let Some(option) = self.options.get(index) {
                    if let Ok(rental_id) = option.value.parse::<i64>() {
                        return WorkflowAction::Completed(WorkflowResult::Extend { rental_id });
                    }
                }
                WorkflowAction::None
            }
            WizardAction::Cancel => WorkflowAction::Cancelled,
            WizardAction::None => WorkflowAction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{self, connection::Database};
    use chrono::NaiveDate;

    #[test]
    fn extend_workflow_returns_selected_rental() {
        let db = Database::open_in_memory().expect("db");
        let locker = crate::model::Locker::new(1, "A-01", "Hauptgebäude", "Klein");
        db::lockers::upsert_locker(db.connection(), &locker).expect("locker");
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let rental = crate::model::Rental::new(1, locker.id, "Max", start, end);
        db::rentals::upsert_rental(db.connection(), &rental).expect("rental");

        let mut workflow = ExtendWorkflow::new(&db, &crate::config::AppSettings::default());
        let result = workflow.handle_key(KeyCode::Enter);
        assert!(matches!(result, WorkflowAction::Completed(_)));
    }
}
