use crate::models::Locker;
use chrono::Utc;

/// Height mode for bulk creation.
#[derive(Debug, Clone)]
pub enum HeightMode {
    Auto,       // Distribute evenly 0-300cm
    Fixed(i32), // Fixed height for all
}

/// Workflow for bulk creating lockers.
#[derive(Debug)]
pub struct BulkCreateWorkflow {
    pub location: String,
    pub prefix: String,
    pub start_num: i32,
    pub end_num: i32,
    pub height_mode: HeightMode,
}

impl BulkCreateWorkflow {
    pub fn new() -> Self {
        Self {
            location: String::new(),
            prefix: String::new(),
            start_num: 1,
            end_num: 50,
            height_mode: HeightMode::Auto,
        }
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = location.into();
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn with_range(mut self, start: i32, end: i32) -> Self {
        self.start_num = start;
        self.end_num = end;
        self
    }

    pub fn with_height_mode(mut self, mode: HeightMode) -> Self {
        self.height_mode = mode;
        self
    }

    /// Returns the count of lockers that will be created.
    pub fn count(&self) -> usize {
        (self.end_num - self.start_num + 1).max(0) as usize
    }

    /// Generates the lockers based on the configuration.
    pub fn generate_lockers(&self) -> Vec<Locker> {
        let count = self.count();
        if count == 0 {
            return Vec::new();
        }

        (self.start_num..=self.end_num)
            .enumerate()
            .map(|(i, num)| {
                let height = match &self.height_mode {
                    HeightMode::Auto => {
                        // Distribute evenly from 0 to 300cm
                        ((i as f64 / count as f64) * 300.0) as i32
                    }
                    HeightMode::Fixed(h) => *h,
                };

                let mut locker = Locker::new(
                    format!("{}{:03}", self.prefix, num),
                    self.location.clone(),
                    height,
                );
                locker.created_at = Utc::now();
                locker
            })
            .collect()
    }

    /// Returns a preview of the lockers that will be created.
    pub fn preview(&self, limit: usize) -> Vec<(String, i32)> {
        let lockers = self.generate_lockers();
        lockers
            .into_iter()
            .take(limit)
            .map(|l| (l.label, l.height))
            .collect()
    }
}

impl Default for BulkCreateWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lockers() {
        let workflow = BulkCreateWorkflow::new()
            .with_location("Test")
            .with_prefix("A-")
            .with_range(1, 10);

        let lockers = workflow.generate_lockers();
        assert_eq!(lockers.len(), 10);
        assert_eq!(lockers[0].label, "A-001");
        assert_eq!(lockers[9].label, "A-010");
    }

    #[test]
    fn test_auto_height_distribution() {
        let workflow = BulkCreateWorkflow::new()
            .with_location("Test")
            .with_prefix("A-")
            .with_range(1, 10)
            .with_height_mode(HeightMode::Auto);

        let lockers = workflow.generate_lockers();
        // First locker should be near 0, last should be near 270 (90% of 300)
        assert!(lockers[0].height <= 30);
        assert!(lockers[9].height >= 240);
    }

    #[test]
    fn test_fixed_height() {
        let workflow = BulkCreateWorkflow::new()
            .with_location("Test")
            .with_prefix("A-")
            .with_range(1, 5)
            .with_height_mode(HeightMode::Fixed(100));

        let lockers = workflow.generate_lockers();
        assert!(lockers.iter().all(|l| l.height == 100));
    }

    #[test]
    fn test_preview() {
        let workflow = BulkCreateWorkflow::new()
            .with_location("Test")
            .with_prefix("A-")
            .with_range(1, 100);

        let preview = workflow.preview(5);
        assert_eq!(preview.len(), 5);
    }
}
