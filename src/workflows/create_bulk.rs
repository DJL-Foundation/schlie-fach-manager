/// Bulk creation workflow for creating multiple lockers at once
/// Per v2.1 spec section 9.4

use crate::db::Database;
use color_eyre::eyre::Result;

/// Workflow state for bulk locker creation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateBulkStep {
    SelectLocation,
    DefineRange,
    SetDefaults,
    Preview,
    Confirm,
    Complete,
}

/// Pattern for generating locker labels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelPattern {
    Numeric { start: i64, end: i64 },
    Alphanumeric { prefix: String, start: i64, end: i64 },
}

/// Workflow for bulk creating lockers
pub struct CreateBulkWorkflow {
    step: CreateBulkStep,
    location_id: Option<i64>,
    pattern: LabelPattern,
    default_size: String,
    preview_labels: Vec<String>,
}

impl CreateBulkWorkflow {
    pub fn new() -> Self {
        Self {
            step: CreateBulkStep::SelectLocation,
            location_id: None,
            pattern: LabelPattern::Numeric { start: 1, end: 10 },
            default_size: "Standard".to_string(),
            preview_labels: Vec::new(),
        }
    }

    pub fn current_step(&self) -> &CreateBulkStep {
        &self.step
    }

    pub fn next_step(&mut self) {
        self.step = match self.step {
            CreateBulkStep::SelectLocation => CreateBulkStep::DefineRange,
            CreateBulkStep::DefineRange => CreateBulkStep::SetDefaults,
            CreateBulkStep::SetDefaults => CreateBulkStep::Preview,
            CreateBulkStep::Preview => CreateBulkStep::Confirm,
            CreateBulkStep::Confirm => CreateBulkStep::Complete,
            CreateBulkStep::Complete => CreateBulkStep::Complete,
        };
    }

    pub fn previous_step(&mut self) {
        self.step = match self.step {
            CreateBulkStep::SelectLocation => CreateBulkStep::SelectLocation,
            CreateBulkStep::DefineRange => CreateBulkStep::SelectLocation,
            CreateBulkStep::SetDefaults => CreateBulkStep::DefineRange,
            CreateBulkStep::Preview => CreateBulkStep::SetDefaults,
            CreateBulkStep::Confirm => CreateBulkStep::Preview,
            CreateBulkStep::Complete => CreateBulkStep::Confirm,
        };
    }

    pub fn set_location(&mut self, location_id: i64) {
        self.location_id = Some(location_id);
    }

    pub fn set_pattern(&mut self, pattern: LabelPattern) {
        self.pattern = pattern;
        self.generate_preview();
    }

    pub fn set_default_size(&mut self, size: String) {
        self.default_size = size;
    }

    /// Generate preview of labels that will be created
    fn generate_preview(&mut self) {
        self.preview_labels = match &self.pattern {
            LabelPattern::Numeric { start, end } => {
                (*start..=*end).map(|n| format!("{}", n)).collect()
            }
            LabelPattern::Alphanumeric { prefix, start, end } => {
                (*start..=*end).map(|n| format!("{}-{:02}", prefix, n)).collect()
            }
        };
    }

    pub fn preview_labels(&self) -> &[String] {
        &self.preview_labels
    }

    /// Execute the bulk creation
    pub fn execute(&self, _db: &Database) -> Result<()> {
        // TODO: Implement actual bulk locker creation
        // This will:
        // 1. Create multiple locker records in database
        // 2. Set default properties (size, location, etc.)
        // 3. Log audit entry
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.step == CreateBulkStep::Complete
    }
}

impl Default for CreateBulkWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_progression() {
        let mut workflow = CreateBulkWorkflow::new();
        assert_eq!(workflow.current_step(), &CreateBulkStep::SelectLocation);

        workflow.next_step();
        assert_eq!(workflow.current_step(), &CreateBulkStep::DefineRange);
    }

    #[test]
    fn test_pattern_preview() {
        let mut workflow = CreateBulkWorkflow::new();
        workflow.set_pattern(LabelPattern::Alphanumeric {
            prefix: "A".to_string(),
            start: 1,
            end: 3,
        });

        let labels = workflow.preview_labels();
        assert_eq!(labels, &["A-01", "A-02", "A-03"]);
    }
}
