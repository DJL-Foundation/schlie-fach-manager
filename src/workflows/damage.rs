use crate::db::connection::Database;
use crate::{
    db,
    ui::widgets::wizard::{
        MessageContent, MessageSender, WizardAction, WizardMessage, WizardOption, WizardRenderer,
    },
    workflows::common::{WorkflowAction, WorkflowResult},
};
use crossterm::event::KeyCode;

/// Workflow for reporting or repairing locker damage.
pub struct DamageWorkflow {
    renderer: WizardRenderer,
    lockers: Vec<WizardOption>,
    selected_locker: Option<i64>,
    step: u8,
}

impl DamageWorkflow {
    pub fn new(db: &Database) -> Self {
        let lockers = db::lockers::list_lockers(db.connection())
            .unwrap_or_default()
            .into_iter()
            .map(|locker| WizardOption {
                label: format!("{} ({})", locker.number, locker.location),
                value: locker.id.to_string(),
                metadata: Some(if locker.is_damaged { "defekt" } else { "ok" }.to_string()),
            })
            .collect::<Vec<_>>();

        let message = WizardMessage {
            sender: MessageSender::System,
            content: MessageContent::Question {
                text: "Welches Schließfach ist betroffen?".to_string(),
                options: if lockers.is_empty() {
                    vec![WizardOption {
                        label: "Keine Schließfächer verfügbar".to_string(),
                        value: "0".to_string(),
                        metadata: None,
                    }]
                } else {
                    lockers.clone()
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
            lockers,
            selected_locker: None,
            step: 0,
        }
    }

    pub fn renderer(&self) -> &WizardRenderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut WizardRenderer {
        &mut self.renderer
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
            if let Some(option) = self.lockers.get(index) {
                if let Ok(locker_id) = option.value.parse::<i64>() {
                    self.selected_locker = Some(locker_id);
                }
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
                        text: "Aktion auswählen".to_string(),
                        options: vec![
                            WizardOption {
                                label: "Defekt melden".to_string(),
                                value: "damage".to_string(),
                                metadata: None,
                            },
                            WizardOption {
                                label: "Repariert".to_string(),
                                value: "repair".to_string(),
                                metadata: None,
                            },
                        ],
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
            if let Some(locker_id) = self.selected_locker {
                let damaged = match index {
                    0 => true,
                    _ => false,
                };
                return WorkflowAction::Completed(WorkflowResult::Damage { locker_id, damaged });
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
    fn damage_workflow_marks_selection() {
        let db = Database::open_in_memory().expect("db");
        let locker = crate::model::Locker::new(1, "A-01", "Hauptgebäude", "Klein");
        db::lockers::upsert_locker(db.connection(), &locker).expect("locker");

        let mut workflow = DamageWorkflow::new(&db);
        assert!(matches!(
            workflow.handle_key(KeyCode::Enter),
            WorkflowAction::None
        ));
        let result = workflow.handle_key(KeyCode::Enter);
        assert!(matches!(result, WorkflowAction::Completed(_)));
    }
}
