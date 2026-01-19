use chrono::NaiveDate;

/// State machine states for the extend rental workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendState {
    AskKnowsNumber,
    EnterNumber,
    EnterUsername,
    ConfirmRental { rental_id: i64 },
    ChooseDuration,
    ConfirmPayment { amount_cents: i32 },
    ConfirmReminder,
    Completed,
    Cancelled,
}

/// Workflow for extending a rental.
#[derive(Debug)]
pub struct ExtendRentalWorkflow {
    pub state: ExtendState,
    pub knows_number: Option<bool>,
    pub locker_label: String,
    pub username: String,
    pub rental_id: Option<i64>,
    pub extension_years: i32,
    pub amount_cents: i32,
    pub new_end_date: Option<NaiveDate>,
    pub payment_received: bool,
    pub reminded: bool,
}

impl ExtendRentalWorkflow {
    pub fn new() -> Self {
        Self {
            state: ExtendState::AskKnowsNumber,
            knows_number: None,
            locker_label: String::new(),
            username: String::new(),
            rental_id: None,
            extension_years: 1,
            amount_cents: 1000, // 10€ default
            new_end_date: None,
            payment_received: false,
            reminded: false,
        }
    }

    pub fn set_knows_number(&mut self, knows: bool) {
        self.knows_number = Some(knows);
        self.state = if knows {
            ExtendState::EnterNumber
        } else {
            ExtendState::EnterUsername
        };
    }

    pub fn set_locker_label(&mut self, label: String) {
        self.locker_label = label;
        // Rental will be looked up and state set to ConfirmRental
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
        // Rental will be looked up and state set to ConfirmRental
    }

    pub fn confirm_rental(&mut self, rental_id: i64) {
        self.rental_id = Some(rental_id);
        self.state = ExtendState::ChooseDuration;
    }

    pub fn set_extension_years(&mut self, years: i32) {
        self.extension_years = years;
        self.amount_cents = years * 1000; // 10€ per year
        self.state = ExtendState::ConfirmPayment {
            amount_cents: self.amount_cents,
        };
    }

    pub fn confirm_payment(&mut self, received: bool) {
        self.payment_received = received;
        self.state = ExtendState::ConfirmReminder;
    }

    pub fn confirm_reminder(&mut self, reminded: bool) {
        self.reminded = reminded;
        self.state = ExtendState::Completed;
    }

    pub fn cancel(&mut self) {
        self.state = ExtendState::Cancelled;
    }

    pub fn go_back(&mut self) {
        self.state = match &self.state {
            ExtendState::EnterNumber | ExtendState::EnterUsername => ExtendState::AskKnowsNumber,
            ExtendState::ConfirmRental { .. } => {
                if self.knows_number == Some(true) {
                    ExtendState::EnterNumber
                } else {
                    ExtendState::EnterUsername
                }
            }
            ExtendState::ChooseDuration => {
                if let Some(id) = self.rental_id {
                    ExtendState::ConfirmRental { rental_id: id }
                } else {
                    ExtendState::AskKnowsNumber
                }
            }
            ExtendState::ConfirmPayment { .. } => ExtendState::ChooseDuration,
            ExtendState::ConfirmReminder => ExtendState::ConfirmPayment {
                amount_cents: self.amount_cents,
            },
            _ => self.state.clone(),
        };
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, ExtendState::Completed)
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.state, ExtendState::Cancelled)
    }
}

impl Default for ExtendRentalWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_workflow_with_number() {
        let mut workflow = ExtendRentalWorkflow::new();
        assert_eq!(workflow.state, ExtendState::AskKnowsNumber);

        workflow.set_knows_number(true);
        assert_eq!(workflow.state, ExtendState::EnterNumber);
    }

    #[test]
    fn test_extend_workflow_without_number() {
        let mut workflow = ExtendRentalWorkflow::new();

        workflow.set_knows_number(false);
        assert_eq!(workflow.state, ExtendState::EnterUsername);
    }
}
