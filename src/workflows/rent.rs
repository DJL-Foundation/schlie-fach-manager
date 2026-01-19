use crate::db::connection::Database;
use crate::workflows::common::{WorkflowAction, WorkflowResult};
use crate::{
    db,
    ui::widgets::wizard::{
        MessageContent, MessageSender, WizardAction, WizardMessage, WizardOption, WizardRenderer,
    },
};
use crossterm::event::KeyCode;

/// Workflow for creating a new rental.
pub struct RentWorkflow {
    renderer: WizardRenderer,
    locations: Vec<WizardOption>,
    sizes: Vec<WizardOption>,
    selected_location: Option<String>,
    step: u8,
}

impl RentWorkflow {
    pub fn new(db: &Database) -> Self {
        let mut locations = db::locations::list_locations(db.connection())
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|location| WizardOption {
                label: location.name.clone(),
                value: location.name,
                metadata: None,
            })
            .collect::<Vec<_>>();
        if locations.is_empty() {
            locations.push(WizardOption {
                label: "Hauptgebäude".to_string(),
                value: "Hauptgebäude".to_string(),
                metadata: None,
            });
        }

        let mut sizes = db::lockers::list_lockers(db.connection())
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|locker| locker.size)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|size| WizardOption {
                label: size.clone(),
                value: size,
                metadata: None,
            })
            .collect::<Vec<_>>();
        if sizes.is_empty() {
            sizes.push(WizardOption {
                label: "Standard".to_string(),
                value: "Standard".to_string(),
                metadata: None,
            });
        }

        let first_question = WizardMessage {
            sender: MessageSender::System,
            content: MessageContent::Question {
                text: "Welchen Standort bevorzugt der Verleiher?".to_string(),
                options: locations.clone(),
            },
            timestamp: None,
        };

        Self {
            renderer: WizardRenderer {
                messages: vec![first_question],
                current_question_index: 0,
                selected_option_index: 0,
            },
            locations,
            sizes,
            selected_location: None,
            step: 0,
        }
    }

    pub fn renderer(&self) -> &WizardRenderer {
        &self.renderer
    }

    /// Handles keyboard input for the workflow.
    pub fn handle_key(&mut self, key: KeyCode) -> WorkflowAction {
        match self.renderer.handle_key(key) {
            WizardAction::Confirm(index) => self.handle_confirm(index),
            WizardAction::Cancel => WorkflowAction::Cancelled,
            WizardAction::None => WorkflowAction::None,
        }
    }

    /// Handles selection confirmation for the current step.
    fn handle_confirm(&mut self, index: usize) -> WorkflowAction {
        if self.step == 0 {
            if let Some(option) = self.locations.get(index) {
                self.selected_location = Some(option.value.clone());
                self.renderer.messages.push(WizardMessage {
                    sender: MessageSender::User,
                    content: MessageContent::Answer {
                        text: option.label.clone(),
                    },
                    timestamp: None,
                });
                self.renderer.messages.push(WizardMessage {
                    sender: MessageSender::System,
                    content: MessageContent::Question {
                        text: "Welche Größe wird benötigt?".to_string(),
                        options: self.sizes.clone(),
                    },
                    timestamp: None,
                });
                self.renderer.current_question_index = self.renderer.messages.len() - 1;
                self.renderer.selected_option_index = 0;
                self.step = 1;
            }
            return WorkflowAction::None;
        }

        if self.step == 1 {
            if let Some(option) = self.sizes.get(index) {
                let location = self
                    .selected_location
                    .clone()
                    .unwrap_or_else(|| "Unbekannt".to_string());
                return WorkflowAction::Completed(WorkflowResult::Rent {
                    location,
                    size: option.value.clone(),
                });
            }
        }

        WorkflowAction::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{self, connection::Database};

    #[test]
    fn rent_workflow_completes_after_two_answers() {
        let db = Database::open_in_memory().expect("db");
        let locker = crate::model::Locker::new(1, "A-01", "Hauptgebäude", "Klein");
        db::lockers::upsert_locker(db.connection(), &locker).expect("locker");
        let location = crate::model::Location::new(1, "Hauptgebäude");
        db::locations::upsert_location(db.connection(), &location).expect("location");

        let mut workflow = RentWorkflow::new(&db);
        assert!(matches!(
            workflow.handle_key(KeyCode::Enter),
            WorkflowAction::None
        ));
        let result = workflow.handle_key(KeyCode::Enter);
        assert!(matches!(result, WorkflowAction::Completed(_)));
    }
}
