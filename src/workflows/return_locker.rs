/// Return locker workflow for processing locker returns
/// Per v2.1 spec section 9.3

use crate::db::Database;
use color_eyre::eyre::Result;

/// Workflow state for returning a locker
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnStep {
    SelectRental,
    InspectCondition,
    CalculateRefund,
    Confirm,
    Complete,
}

/// Condition of the locker upon return
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockerCondition {
    Perfect,
    Good,
    Damaged,
}

/// Workflow for processing a locker return
pub struct ReturnLockerWorkflow {
    step: ReturnStep,
    rental_id: Option<i64>,
    condition: LockerCondition,
    notes: String,
    refund_amount: i64,
}

impl ReturnLockerWorkflow {
    pub fn new() -> Self {
        Self {
            step: ReturnStep::SelectRental,
            rental_id: None,
            condition: LockerCondition::Good,
            notes: String::new(),
            refund_amount: 0,
        }
    }

    pub fn current_step(&self) -> &ReturnStep {
        &self.step
    }

    pub fn next_step(&mut self) {
        self.step = match self.step {
            ReturnStep::SelectRental => ReturnStep::InspectCondition,
            ReturnStep::InspectCondition => ReturnStep::CalculateRefund,
            ReturnStep::CalculateRefund => ReturnStep::Confirm,
            ReturnStep::Confirm => ReturnStep::Complete,
            ReturnStep::Complete => ReturnStep::Complete,
        };
    }

    pub fn previous_step(&mut self) {
        self.step = match self.step {
            ReturnStep::SelectRental => ReturnStep::SelectRental,
            ReturnStep::InspectCondition => ReturnStep::SelectRental,
            ReturnStep::CalculateRefund => ReturnStep::InspectCondition,
            ReturnStep::Confirm => ReturnStep::CalculateRefund,
            ReturnStep::Complete => ReturnStep::Confirm,
        };
    }

    pub fn set_rental(&mut self, rental_id: i64) {
        self.rental_id = Some(rental_id);
    }

    pub fn set_condition(&mut self, condition: LockerCondition) {
        self.condition = condition;
    }

    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }

    pub fn set_refund_amount(&mut self, amount: i64) {
        self.refund_amount = amount;
    }

    /// Execute the locker return
    pub fn execute(&self, _db: &Database) -> Result<()> {
        // TODO: Implement actual locker return
        // This will:
        // 1. Update rental end_date to now
        // 2. Set locker status back to Available (or Maintenance if damaged)
        // 3. Process refund if applicable
        // 4. Log audit entry
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.step == ReturnStep::Complete
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
    fn test_workflow_progression() {
        let mut workflow = ReturnLockerWorkflow::new();
        assert_eq!(workflow.current_step(), &ReturnStep::SelectRental);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &ReturnStep::InspectCondition);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &ReturnStep::CalculateRefund);
    }

    #[test]
    fn test_condition_setting() {
        let mut workflow = ReturnLockerWorkflow::new();
        workflow.set_condition(LockerCondition::Damaged);
        assert_eq!(workflow.condition, LockerCondition::Damaged);
    }
}
