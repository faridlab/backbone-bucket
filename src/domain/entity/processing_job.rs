use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ProcessingJobType;
use super::JobStatus;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{ProcessingJobStateMachine, ProcessingJobState, StateMachineError};

use thiserror::Error;

/// Domain error for joberror operations
#[derive(Debug, Clone, Error)]
pub enum JobError {
    #[error("{0}")]
    Message(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for JobError {
    fn from(msg: String) -> Self { Self::Message(msg) }
}

impl From<&str> for JobError {
    fn from(msg: &str) -> Self { Self::Message(msg.to_string()) }
}


/// Strongly-typed ID for ProcessingJob
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProcessingJobId(pub Uuid);

impl ProcessingJobId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ProcessingJobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ProcessingJobId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ProcessingJobId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ProcessingJobId> for Uuid {
    fn from(id: ProcessingJobId) -> Self { id.0 }
}

impl AsRef<Uuid> for ProcessingJobId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ProcessingJobId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProcessingJob {
    pub id: Uuid,
    pub file_id: Uuid,
    pub job_type: ProcessingJobType,
    pub(crate) status: JobStatus,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    pub retry_count: i32,
    pub max_retries: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl ProcessingJob {
    /// Create a builder for ProcessingJob
    pub fn builder() -> ProcessingJobBuilder {
        ProcessingJobBuilder::default()
    }

    /// Create a new ProcessingJob with required fields
    pub fn new(file_id: Uuid, job_type: ProcessingJobType, status: JobStatus, priority: i32, retry_count: i32, max_retries: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            job_type,
            status,
            priority,
            input_data: None,
            result_data: None,
            error_message: None,
            started_at: None,
            completed_at: None,
            retry_count,
            max_retries,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ProcessingJobId {
        ProcessingJobId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    /// Get the current status
    pub fn status(&self) -> &JobStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the input_data field (chainable)
    pub fn with_input_data(mut self, value: serde_json::Value) -> Self {
        self.input_data = Some(value);
        self
    }

    /// Set the result_data field (chainable)
    pub fn with_result_data(mut self, value: serde_json::Value) -> Self {
        self.result_data = Some(value);
        self
    }

    /// Set the error_message field (chainable)
    pub fn with_error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the started_at field (chainable)
    pub fn with_started_at(mut self, value: DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Set the completed_at field (chainable)
    pub fn with_completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: ProcessingJobState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<ProcessingJobState>()?;
        let mut sm = ProcessingJobStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<JobStatus>()
            .map_err(|e| StateMachineError::InvalidState(e.to_string()))?;
        Ok(())
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "file_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_id = v; }
                }
                "job_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.job_type = v; }
                }
                "priority" => {
                    if let Ok(v) = serde_json::from_value(value) { self.priority = v; }
                }
                "input_data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.input_data = v; }
                }
                "result_data" => {
                    if let Ok(v) = serde_json::from_value(value) { self.result_data = v; }
                }
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                "started_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.started_at = v; }
                }
                "completed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.completed_at = v; }
                }
                "retry_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.retry_count = v; }
                }
                "max_retries" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_retries = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

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
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for ProcessingJob {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "ProcessingJob"
    }
}

impl backbone_core::PersistentEntity for ProcessingJob {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for ProcessingJob {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("file_id".to_string(), "uuid".to_string());
        m.insert("job_type".to_string(), "processing_job_type".to_string());
        m.insert("status".to_string(), "job_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for ProcessingJob entity
///
/// Provides a fluent API for constructing ProcessingJob instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ProcessingJobBuilder {
    file_id: Option<Uuid>,
    job_type: Option<ProcessingJobType>,
    status: Option<JobStatus>,
    priority: Option<i32>,
    input_data: Option<serde_json::Value>,
    result_data: Option<serde_json::Value>,
    error_message: Option<String>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    retry_count: Option<i32>,
    max_retries: Option<i32>,
}

impl ProcessingJobBuilder {
    /// Set the file_id field (required)
    pub fn file_id(mut self, value: Uuid) -> Self {
        self.file_id = Some(value);
        self
    }

    /// Set the job_type field (required)
    pub fn job_type(mut self, value: ProcessingJobType) -> Self {
        self.job_type = Some(value);
        self
    }

    /// Set the status field (default: `JobStatus::default()`)
    pub fn status(mut self, value: JobStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the priority field (default: `0`)
    pub fn priority(mut self, value: i32) -> Self {
        self.priority = Some(value);
        self
    }

    /// Set the input_data field (optional)
    pub fn input_data(mut self, value: serde_json::Value) -> Self {
        self.input_data = Some(value);
        self
    }

    /// Set the result_data field (optional)
    pub fn result_data(mut self, value: serde_json::Value) -> Self {
        self.result_data = Some(value);
        self
    }

    /// Set the error_message field (optional)
    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the started_at field (optional)
    pub fn started_at(mut self, value: DateTime<Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Set the completed_at field (optional)
    pub fn completed_at(mut self, value: DateTime<Utc>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Set the retry_count field (default: `0`)
    pub fn retry_count(mut self, value: i32) -> Self {
        self.retry_count = Some(value);
        self
    }

    /// Set the max_retries field (default: `3`)
    pub fn max_retries(mut self, value: i32) -> Self {
        self.max_retries = Some(value);
        self
    }

    /// Build the ProcessingJob entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<ProcessingJob, String> {
        let file_id = self.file_id.ok_or_else(|| "file_id is required".to_string())?;
        let job_type = self.job_type.ok_or_else(|| "job_type is required".to_string())?;

        Ok(ProcessingJob {
            id: Uuid::new_v4(),
            file_id,
            job_type,
            status: self.status.unwrap_or(JobStatus::default()),
            priority: self.priority.unwrap_or(0),
            input_data: self.input_data,
            result_data: self.result_data,
            error_message: self.error_message,
            started_at: self.started_at,
            completed_at: self.completed_at,
            retry_count: self.retry_count.unwrap_or(0),
            max_retries: self.max_retries.unwrap_or(3),
            metadata: AuditMetadata::default(),
        })
    }
}