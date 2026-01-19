/// Rent workflow for creating new rentals
/// Per v2.1 spec section 9.1

use crate::db::Database;
use color_eyre::eyre::Result;

/// Workflow state for renting a locker
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RentStep {
    SelectLocker,
    EnterTenantInfo,
    SelectDuration,
    EnterPayment,
    Confirm,
    Complete,
}

/// Workflow for creating a new rental
pub struct RentWorkflow {
    step: RentStep,
    locker_id: Option<i64>,
    tenant_name: String,
    tenant_email: String,
    duration_days: i64,
    payment_amount: i64,
}

impl RentWorkflow {
    pub fn new() -> Self {
        Self {
            step: RentStep::SelectLocker,
            locker_id: None,
            tenant_name: String::new(),
            tenant_email: String::new(),
            duration_days: 30,
            payment_amount: 0,
        }
    }

    pub fn current_step(&self) -> &RentStep {
        &self.step
    }

    pub fn next_step(&mut self) {
        self.step = match self.step {
            RentStep::SelectLocker => RentStep::EnterTenantInfo,
            RentStep::EnterTenantInfo => RentStep::SelectDuration,
            RentStep::SelectDuration => RentStep::EnterPayment,
            RentStep::EnterPayment => RentStep::Confirm,
            RentStep::Confirm => RentStep::Complete,
            RentStep::Complete => RentStep::Complete,
        };
    }

    pub fn previous_step(&mut self) {
        self.step = match self.step {
            RentStep::SelectLocker => RentStep::SelectLocker,
            RentStep::EnterTenantInfo => RentStep::SelectLocker,
            RentStep::SelectDuration => RentStep::EnterTenantInfo,
            RentStep::EnterPayment => RentStep::SelectDuration,
            RentStep::Confirm => RentStep::EnterPayment,
            RentStep::Complete => RentStep::Confirm,
        };
    }

    pub fn set_locker(&mut self, locker_id: i64) {
        self.locker_id = Some(locker_id);
    }

    pub fn set_tenant_name(&mut self, name: String) {
        self.tenant_name = name;
    }

    pub fn set_tenant_email(&mut self, email: String) {
        self.tenant_email = email;
    }

    pub fn set_duration(&mut self, days: i64) {
        self.duration_days = days;
    }

    pub fn set_payment_amount(&mut self, amount: i64) {
        self.payment_amount = amount;
    }

    /// Execute the rental creation
    pub fn execute(&self, _db: &Database) -> Result<()> {
        // TODO: Implement actual rental creation
        // This will:
        // 1. Create rental record in database
        // 2. Update locker status to Occupied
        // 3. Create payment record
        // 4. Log audit entry
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.step == RentStep::Complete
    }
}

impl Default for RentWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_progression() {
        let mut workflow = RentWorkflow::new();
        assert_eq!(workflow.current_step(), &RentStep::SelectLocker);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &RentStep::EnterTenantInfo);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &RentStep::SelectDuration);

        workflow.previous_step();
        assert_eq!(workflow.current_step(), &RentStep::EnterTenantInfo);
    }

    #[test]
    fn test_data_setting() {
        let mut workflow = RentWorkflow::new();
        workflow.set_locker(42);
        workflow.set_tenant_name("Max Mustermann".to_string());
        workflow.set_duration(60);

        assert_eq!(workflow.locker_id, Some(42));
        assert_eq!(workflow.tenant_name, "Max Mustermann");
        assert_eq!(workflow.duration_days, 60);
    }
}
