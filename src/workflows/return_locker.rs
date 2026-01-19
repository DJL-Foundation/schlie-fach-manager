/// Return action based on debt status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnAction {
    AllowedWithDeposit,           // No debt, return deposit
    AllowedWithoutDeposit,        // Exactly 10€ debt, keep deposit
    RequiresPayment(i32),         // More than 10€ debt, requires payment
}

/// State machine states for the return locker workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnState {
    AskKnowsNumber,
    EnterNumber,
    EnterUsername,
    ConfirmRental { rental_id: i64 },
    CheckDebts { debt_cents: i32 },
    ConfirmDebtPayment { debt_cents: i32 },
    ConfirmDepositReturn,
    Completed,
    Cancelled,
}

/// Workflow for returning a locker.
#[derive(Debug)]
pub struct ReturnLockerWorkflow {
    pub state: ReturnState,
    pub knows_number: Option<bool>,
    pub locker_label: String,
    pub username: String,
    pub rental_id: Option<i64>,
    pub debt_cents: i32,
    pub debt_paid: bool,
    pub deposit_returned: bool,
}

impl ReturnLockerWorkflow {
    pub fn new() -> Self {
        Self {
            state: ReturnState::AskKnowsNumber,
            knows_number: None,
            locker_label: String::new(),
            username: String::new(),
            rental_id: None,
            debt_cents: 0,
            debt_paid: false,
            deposit_returned: false,
        }
    }

    pub fn set_knows_number(&mut self, knows: bool) {
        self.knows_number = Some(knows);
        self.state = if knows {
            ReturnState::EnterNumber
        } else {
            ReturnState::EnterUsername
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

    pub fn confirm_rental(&mut self, rental_id: i64, debt_cents: i32) {
        self.rental_id = Some(rental_id);
        self.debt_cents = debt_cents;
        self.state = ReturnState::CheckDebts { debt_cents };
    }

    pub fn process_debt(&mut self) -> ReturnAction {
        let action = calculate_return_action(self.debt_cents);
        
        match &action {
            ReturnAction::AllowedWithDeposit => {
                self.state = ReturnState::ConfirmDepositReturn;
            }
            ReturnAction::AllowedWithoutDeposit => {
                // Skip deposit return, go directly to completion
                self.deposit_returned = false;
                self.state = ReturnState::Completed;
            }
            ReturnAction::RequiresPayment(debt) => {
                self.state = ReturnState::ConfirmDebtPayment { debt_cents: *debt };
            }
        }
        
        action
    }

    pub fn confirm_debt_payment(&mut self, paid: bool) {
        if paid {
            self.debt_paid = true;
            // After paying, they get their deposit back
            self.state = ReturnState::ConfirmDepositReturn;
        } else {
            // Abort the return process
            self.state = ReturnState::Cancelled;
        }
    }

    pub fn confirm_deposit_return(&mut self, returned: bool) {
        self.deposit_returned = returned;
        self.state = ReturnState::Completed;
    }

    pub fn cancel(&mut self) {
        self.state = ReturnState::Cancelled;
    }

    pub fn go_back(&mut self) {
        self.state = match &self.state {
            ReturnState::EnterNumber | ReturnState::EnterUsername => ReturnState::AskKnowsNumber,
            ReturnState::ConfirmRental { .. } => {
                if self.knows_number == Some(true) {
                    ReturnState::EnterNumber
                } else {
                    ReturnState::EnterUsername
                }
            }
            ReturnState::CheckDebts { .. } => {
                if let Some(id) = self.rental_id {
                    ReturnState::ConfirmRental { rental_id: id }
                } else {
                    ReturnState::AskKnowsNumber
                }
            }
            ReturnState::ConfirmDebtPayment { .. } => ReturnState::CheckDebts {
                debt_cents: self.debt_cents,
            },
            ReturnState::ConfirmDepositReturn => {
                if self.debt_cents > 0 {
                    ReturnState::ConfirmDebtPayment {
                        debt_cents: self.debt_cents,
                    }
                } else {
                    ReturnState::CheckDebts {
                        debt_cents: self.debt_cents,
                    }
                }
            }
            _ => self.state.clone(),
        };
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, ReturnState::Completed)
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.state, ReturnState::Cancelled)
    }
}

/// Calculates the return action based on debt amount.
pub fn calculate_return_action(debt_cents: i32) -> ReturnAction {
    match debt_cents {
        0 => ReturnAction::AllowedWithDeposit,
        1000 => ReturnAction::AllowedWithoutDeposit, // Exactly 10€: keep deposit
        _ => ReturnAction::RequiresPayment(debt_cents),
    }
}

impl Default for ReturnLockerWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_action_no_debt() {
        assert_eq!(calculate_return_action(0), ReturnAction::AllowedWithDeposit);
    }

    #[test]
    fn test_return_action_exactly_10_euros() {
        assert_eq!(
            calculate_return_action(1000),
            ReturnAction::AllowedWithoutDeposit
        );
    }

    #[test]
    fn test_return_action_with_debt() {
        assert_eq!(
            calculate_return_action(2000),
            ReturnAction::RequiresPayment(2000)
        );
    }

    #[test]
    fn test_workflow_no_debt() {
        let mut workflow = ReturnLockerWorkflow::new();
        workflow.set_knows_number(true);
        workflow.set_locker_label("A-001".to_string());
        workflow.confirm_rental(1, 0);

        let action = workflow.process_debt();
        assert_eq!(action, ReturnAction::AllowedWithDeposit);
        assert_eq!(workflow.state, ReturnState::ConfirmDepositReturn);
    }
}
