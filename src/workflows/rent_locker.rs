use crate::models::{Locker, TenantType};
use chrono::NaiveDate;

/// State machine states for the rent locker workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RentState {
    ChooseLocation,
    ChooseHeight,
    SelectLocker { available: Vec<i64> }, // locker IDs
    ConfirmLocker { locker_id: i64 },
    EnterUsername,
    ChooseTenantType,
    ManualEdit,
    ConfirmPayment,
    ConfirmReminder,
    Completed { rental_id: i64 },
    Cancelled,
}

/// Workflow for renting a new locker.
#[derive(Debug)]
pub struct RentLockerWorkflow {
    pub state: RentState,
    pub selected_location: Option<String>,
    pub selected_height_range: Option<(i32, i32)>,
    pub selected_locker_id: Option<i64>,
    pub username: String,
    pub tenant_type: Option<TenantType>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub deposit_paid: bool,
    pub reminded: bool,
}

impl RentLockerWorkflow {
    pub fn new() -> Self {
        let today = chrono::Utc::now().date_naive();
        Self {
            state: RentState::ChooseLocation,
            selected_location: None,
            selected_height_range: None,
            selected_locker_id: None,
            username: String::new(),
            tenant_type: None,
            start_date: today,
            end_date: today + chrono::Duration::days(365),
            deposit_paid: false,
            reminded: false,
        }
    }

    pub fn set_location(&mut self, location: String) {
        self.selected_location = Some(location);
        self.state = RentState::ChooseHeight;
    }

    pub fn set_height_range(&mut self, min: i32, max: i32) {
        self.selected_height_range = Some((min, max));
        // State will be updated when available lockers are loaded
    }

    pub fn set_available_lockers(&mut self, locker_ids: Vec<i64>) {
        self.state = RentState::SelectLocker { available: locker_ids };
    }

    pub fn select_locker(&mut self, locker_id: i64) {
        self.selected_locker_id = Some(locker_id);
        self.state = RentState::ConfirmLocker { locker_id };
    }

    pub fn confirm_locker(&mut self) {
        self.state = RentState::EnterUsername;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
        self.state = RentState::ChooseTenantType;
    }

    pub fn set_tenant_type(&mut self, tenant_type: TenantType) {
        self.tenant_type = Some(tenant_type);
        self.state = RentState::ManualEdit;
    }

    pub fn confirm_manual_edit(&mut self) {
        self.state = RentState::ConfirmPayment;
    }

    pub fn confirm_payment(&mut self, received: bool) {
        self.deposit_paid = received;
        self.state = RentState::ConfirmReminder;
    }

    pub fn confirm_reminder(&mut self, reminded: bool) {
        self.reminded = reminded;
        // State will be set to Completed when saved
    }

    pub fn complete(&mut self, rental_id: i64) {
        self.state = RentState::Completed { rental_id };
    }

    pub fn cancel(&mut self) {
        self.state = RentState::Cancelled;
    }

    pub fn go_back(&mut self) {
        self.state = match &self.state {
            RentState::ChooseHeight => RentState::ChooseLocation,
            RentState::SelectLocker { .. } => RentState::ChooseHeight,
            RentState::ConfirmLocker { .. } => {
                if let Some((min, max)) = self.selected_height_range {
                    // Would need to reload available lockers
                    RentState::ChooseHeight
                } else {
                    RentState::ChooseHeight
                }
            }
            RentState::EnterUsername => {
                if let Some(id) = self.selected_locker_id {
                    RentState::ConfirmLocker { locker_id: id }
                } else {
                    RentState::ChooseLocation
                }
            }
            RentState::ChooseTenantType => RentState::EnterUsername,
            RentState::ManualEdit => RentState::ChooseTenantType,
            RentState::ConfirmPayment => RentState::ManualEdit,
            RentState::ConfirmReminder => RentState::ConfirmPayment,
            _ => self.state.clone(),
        };
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, RentState::Completed { .. })
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.state, RentState::Cancelled)
    }
}

impl Default for RentLockerWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_state_progression() {
        let mut workflow = RentLockerWorkflow::new();
        assert_eq!(workflow.state, RentState::ChooseLocation);

        workflow.set_location("Hauptgebäude".to_string());
        assert_eq!(workflow.state, RentState::ChooseHeight);

        workflow.set_height_range(50, 150);
        workflow.set_available_lockers(vec![1, 2, 3]);
        assert!(matches!(workflow.state, RentState::SelectLocker { .. }));

        workflow.select_locker(1);
        assert!(matches!(workflow.state, RentState::ConfirmLocker { .. }));

        workflow.confirm_locker();
        assert_eq!(workflow.state, RentState::EnterUsername);
    }
}
