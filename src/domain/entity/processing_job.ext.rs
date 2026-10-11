//! The extension of `processing_job.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl ProcessingJob {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if job can be retried (failed and retries remaining)
    pub fn can_retry(&self) -> bool {
        self.status == JobStatus::Failed && self.retry_count < self.max_retries
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
        self.metadata.touch();
    }

    /// Mark job as started
    pub fn mark_started(&mut self) -> Result<(), JobError> {
        self.status = JobStatus::Running;
        self.started_at = Some(Utc::now());
        self.metadata.touch();
        Ok(())
    }

    /// Mark job as completed with result
    pub fn mark_completed(&mut self, result: serde_json::Value) -> Result<(), JobError> {
        self.status = JobStatus::Completed;
        self.result_data = Some(result);
        self.completed_at = Some(Utc::now());
        self.metadata.touch();
        Ok(())
    }

    /// Mark job as failed with error
    pub fn mark_failed(&mut self, error: String) -> Result<(), JobError> {
        self.status = JobStatus::Failed;
        self.error_message = Some(error);
        self.metadata.touch();
        Ok(())
    }

    /// Cancel the job
    pub fn cancel(&mut self) -> Result<(), JobError> {
        self.status = JobStatus::Cancelled;
        self.metadata.touch();
        Ok(())
    }

    /// Get job execution duration (completed_at - started_at if both present)
    pub fn duration(&self) -> Option<Duration> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some(end - start),
            _ => None,
        }
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.retry_count > self.max_retries {
            errors.push("retry_count must not exceed max_retries");
        }
        if self.status == JobStatus::Running && self.started_at.is_none() {
            errors.push("running job must have started_at");
        }
        if self.status == JobStatus::Completed && self.completed_at.is_none() {
            errors.push("completed job must have completed_at");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
