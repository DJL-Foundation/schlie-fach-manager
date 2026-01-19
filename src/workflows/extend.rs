/// Extend workflow for extending existing rentals
/// Per v2.1 spec section 9.2

use crate::db::Database;
use color_eyre::eyre::Result;

/// Workflow state for extending a rental
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendStep {
    SelectRental,
    SelectNewDuration,
    EnterPayment,
    Confirm,
    Complete,
}

/// Workflow for extending an existing rental
pub struct ExtendWorkflow {
    step: ExtendStep,
    rental_id: Option<i64>,
    additional_days: i64,
    payment_amount: i64,
}

impl ExtendWorkflow {
    pub fn new() -> Self {
        Self {
            step: ExtendStep::SelectRental,
            rental_id: None,
            additional_days: 30,
            payment_amount: 0,
        }
    }

    pub fn current_step(&self) -> &ExtendStep {
        &self.step
    }

    pub fn next_step(&mut self) {
        self.step = match self.step {
            ExtendStep::SelectRental => ExtendStep::SelectNewDuration,
            ExtendStep::SelectNewDuration => ExtendStep::EnterPayment,
            ExtendStep::EnterPayment => ExtendStep::Confirm,
            ExtendStep::Confirm => ExtendStep::Complete,
            ExtendStep::Complete => ExtendStep::Complete,
        };
    }

    pub fn previous_step(&mut self) {
        self.step = match self.step {
            ExtendStep::SelectRental => ExtendStep::SelectRental,
            ExtendStep::SelectNewDuration => ExtendStep::SelectRental,
            ExtendStep::EnterPayment => ExtendStep::SelectNewDuration,
            ExtendStep::Confirm => ExtendStep::EnterPayment,
            ExtendStep::Complete => ExtendStep::Confirm,
        };
    }

    pub fn set_rental(&mut self, rental_id: i64) {
        self.rental_id = Some(rental_id);
    }

    pub fn set_additional_days(&mut self, days: i64) {
        self.additional_days = days;
    }

    pub fn set_payment_amount(&mut self, amount: i64) {
        self.payment_amount = amount;
    }

    /// Execute the rental extension
    pub fn execute(&self, _db: &Database) -> Result<()> {
        // TODO: Implement actual rental extension
        // This will:
        // 1. Update rental end_date in database
        // 2. Create payment record
        // 3. Log audit entry
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.step == ExtendStep::Complete
    }
}

impl Default for ExtendWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_progression() {
        let mut workflow = ExtendWorkflow::new();
        assert_eq!(workflow.current_step(), &ExtendStep::SelectRental);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &ExtendStep::SelectNewDuration);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &ExtendStep::EnterPayment);
    }
}
