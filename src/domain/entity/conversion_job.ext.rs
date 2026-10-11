//! The extension of `conversion_job.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl ConversionJob {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Update conversion progress (0-100)
    pub fn update_progress(&mut self, progress: i32) -> Result<(), ConversionError> {
        if !(0..=100).contains(&progress) {
            return Err(ConversionError::ValidationFailed(
                "Progress must be between 0 and 100".to_string(),
            ));
        }
        self.progress = progress;
        self.metadata.touch();
        Ok(())
    }

    /// Mark conversion complete
    pub fn complete(&mut self, result_file_id: Uuid) -> Result<(), ConversionError> {
        self.status = ConversionStatus::Completed;
        self.result_file_id = Some(result_file_id);
        self.progress = 100;
        self.completed_at = Some(Utc::now());
        self.metadata.touch();
        Ok(())
    }

    /// Mark conversion failed
    pub fn fail(&mut self, error: String) -> Result<(), ConversionError> {
        self.status = ConversionStatus::Failed;
        self.error_message = Some(error);
        self.completed_at = Some(Utc::now());
        self.metadata.touch();
        Ok(())
    }

    /// Check if conversion is complete
    pub fn is_complete(&self) -> bool {
        self.status == ConversionStatus::Completed
    }

    /// Get progress as percentage (clamped 0-100)
    pub fn get_progress_percentage(&self) -> i32 {
        self.progress.clamp(0, 100)
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.progress < 0 || self.progress > 100 {
            errors.push("progress must be between 0 and 100");
        }
        if self.status == ConversionStatus::Completed && self.result_file_id.is_none() {
            errors.push("completed conversion must have result_file_id");
        }
        if self.status == ConversionStatus::Failed && self.error_message.is_none() {
            errors.push("failed conversion must have error_message");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
