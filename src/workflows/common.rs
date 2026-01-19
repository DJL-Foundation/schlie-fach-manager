/// Workflow action emitted after handling input.
#[derive(Debug, Clone)]
pub enum WorkflowAction {
    None,
    Completed(WorkflowResult),
    Cancelled,
}

/// Result produced by a completed workflow.
#[derive(Debug, Clone)]
pub enum WorkflowResult {
    Rent { location: String, size: String },
    Extend { rental_id: i64 },
    Return { rental_id: i64 },
    Damage { locker_id: i64, damaged: bool },
}
