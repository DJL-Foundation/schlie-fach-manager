/// Damage workflow for reporting and managing damaged lockers
/// Per v2.1 spec section 9.5

use crate::db::Database;
use color_eyre::eyre::Result;

/// Workflow state for damage reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageStep {
    SelectLocker,
    DescribeDamage,
    SetPriority,
    AssignRepair,
    Confirm,
    Complete,
}

/// Priority level for damage repair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamagePriority {
    Low,
    Medium,
    High,
    Critical,
}

impl DamagePriority {
    pub fn name(&self) -> &str {
        match self {
            DamagePriority::Low => "Niedrig",
            DamagePriority::Medium => "Mittel",
            DamagePriority::High => "Hoch",
            DamagePriority::Critical => "Kritisch",
        }
    }
}

/// Workflow for reporting damaged lockers
pub struct DamageWorkflow {
    step: DamageStep,
    locker_id: Option<i64>,
    description: String,
    priority: DamagePriority,
    estimated_repair_date: Option<String>,
}

impl DamageWorkflow {
    pub fn new() -> Self {
        Self {
            step: DamageStep::SelectLocker,
            locker_id: None,
            description: String::new(),
            priority: DamagePriority::Medium,
            estimated_repair_date: None,
        }
    }

    pub fn current_step(&self) -> &DamageStep {
        &self.step
    }

    pub fn next_step(&mut self) {
        self.step = match self.step {
            DamageStep::SelectLocker => DamageStep::DescribeDamage,
            DamageStep::DescribeDamage => DamageStep::SetPriority,
            DamageStep::SetPriority => DamageStep::AssignRepair,
            DamageStep::AssignRepair => DamageStep::Confirm,
            DamageStep::Confirm => DamageStep::Complete,
            DamageStep::Complete => DamageStep::Complete,
        };
    }

    pub fn previous_step(&mut self) {
        self.step = match self.step {
            DamageStep::SelectLocker => DamageStep::SelectLocker,
            DamageStep::DescribeDamage => DamageStep::SelectLocker,
            DamageStep::SetPriority => DamageStep::DescribeDamage,
            DamageStep::AssignRepair => DamageStep::SetPriority,
            DamageStep::Confirm => DamageStep::AssignRepair,
            DamageStep::Complete => DamageStep::Confirm,
        };
    }

    pub fn set_locker(&mut self, locker_id: i64) {
        self.locker_id = Some(locker_id);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_priority(&mut self, priority: DamagePriority) {
        self.priority = priority;
    }

    pub fn set_estimated_repair_date(&mut self, date: Option<String>) {
        self.estimated_repair_date = date;
    }

    /// Execute the damage report
    pub fn execute(&self, _db: &Database) -> Result<()> {
        // TODO: Implement actual damage reporting
        // This will:
        // 1. Update locker status to Maintenance
        // 2. Set maintenance note with damage description
        // 3. Create damage record in database
        // 4. Log audit entry
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.step == DamageStep::Complete
    }
}

impl Default for DamageWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_progression() {
        let mut workflow = DamageWorkflow::new();
        assert_eq!(workflow.current_step(), &DamageStep::SelectLocker);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &DamageStep::DescribeDamage);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &DamageStep::SetPriority);
    }

    #[test]
    fn test_priority_names() {
        assert_eq!(DamagePriority::Low.name(), "Niedrig");
        assert_eq!(DamagePriority::High.name(), "Hoch");
        assert_eq!(DamagePriority::Critical.name(), "Kritisch");
    }

    #[test]
    fn test_damage_description() {
        let mut workflow = DamageWorkflow::new();
        workflow.set_description("Tür klemmt".to_string());
        assert_eq!(workflow.description, "Tür klemmt");
    }
}
