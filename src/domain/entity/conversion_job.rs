use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ConversionStatus;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{ConversionJobStateMachine, ConversionJobState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

use thiserror::Error;

/// Domain error for conversionerror operations
#[derive(Debug, Clone, Error)]
pub enum ConversionError {
    #[error("{0}")]
    Message(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for ConversionError {
    fn from(msg: String) -> Self { Self::Message(msg) }
}

impl From<&str> for ConversionError {
    fn from(msg: &str) -> Self { Self::Message(msg.to_string()) }
}


/// Strongly-typed ID for ConversionJob
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConversionJobId(pub Uuid);

impl ConversionJobId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ConversionJobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ConversionJobId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ConversionJobId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ConversionJobId> for Uuid {
    fn from(id: ConversionJobId) -> Self { id.0 }
}

impl AsRef<Uuid> for ConversionJobId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ConversionJobId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversionJob {
    pub id: Uuid,
    pub source_file_id: Uuid,
    pub target_format: String,
    pub(crate) status: ConversionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_file_id: Option<Uuid>,
    pub progress: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl ConversionJob {
    /// Create a builder for ConversionJob
    pub fn builder() -> ConversionJobBuilder {
        ConversionJobBuilder::default()
    }

    /// Create a new ConversionJob with required fields
    pub fn new(source_file_id: Uuid, target_format: String, status: ConversionStatus, progress: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_file_id,
            target_format,
            status,
            conversion_options: None,
            result_file_id: None,
            progress,
            error_message: None,
            started_at: None,
            completed_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ConversionJobId {
        ConversionJobId(self.id)
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
    pub fn status(&self) -> &ConversionStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the conversion_options field (chainable)
    pub fn with_conversion_options(mut self, value: serde_json::Value) -> Self {
        self.conversion_options = Some(value);
        self
    }

    /// Set the result_file_id field (chainable)
    pub fn with_result_file_id(mut self, value: Uuid) -> Self {
        self.result_file_id = Some(value);
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
    pub fn transition_to(&mut self, new_state: ConversionJobState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<ConversionJobState>()?;
        let mut sm = ConversionJobStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<ConversionStatus>()
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
                "source_file_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.source_file_id = v; }
                }
                "target_format" => {
                    if let Ok(v) = serde_json::from_value(value) { self.target_format = v; }
                }
                "conversion_options" => {
                    if let Ok(v) = serde_json::from_value(value) { self.conversion_options = v; }
                }
                "result_file_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.result_file_id = v; }
                }
                "progress" => {
                    if let Ok(v) = serde_json::from_value(value) { self.progress = v; }
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
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

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
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for ConversionJob {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "ConversionJob"
    }
}

impl backbone_core::PersistentEntity for ConversionJob {
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

impl backbone_orm::EntityRepoMeta for ConversionJob {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("source_file_id".to_string(), "uuid".to_string());
        m.insert("result_file_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "conversion_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["target_format"]
    }
}

/// Builder for ConversionJob entity
///
/// Provides a fluent API for constructing ConversionJob instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ConversionJobBuilder {
    source_file_id: Option<Uuid>,
    target_format: Option<String>,
    status: Option<ConversionStatus>,
    conversion_options: Option<serde_json::Value>,
    result_file_id: Option<Uuid>,
    progress: Option<i32>,
    error_message: Option<String>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
}

impl ConversionJobBuilder {
    /// Set the source_file_id field (required)
    pub fn source_file_id(mut self, value: Uuid) -> Self {
        self.source_file_id = Some(value);
        self
    }

    /// Set the target_format field (required)
    pub fn target_format(mut self, value: String) -> Self {
        self.target_format = Some(value);
        self
    }

    /// Set the status field (default: `ConversionStatus::default()`)
    pub fn status(mut self, value: ConversionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the conversion_options field (optional)
    pub fn conversion_options(mut self, value: serde_json::Value) -> Self {
        self.conversion_options = Some(value);
        self
    }

    /// Set the result_file_id field (optional)
    pub fn result_file_id(mut self, value: Uuid) -> Self {
        self.result_file_id = Some(value);
        self
    }

    /// Set the progress field (default: `0`)
    pub fn progress(mut self, value: i32) -> Self {
        self.progress = Some(value);
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

    /// Build the ConversionJob entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<ConversionJob, String> {
        let source_file_id = self.source_file_id.ok_or_else(|| "source_file_id is required".to_string())?;
        let target_format = self.target_format.ok_or_else(|| "target_format is required".to_string())?;

        Ok(ConversionJob {
            id: Uuid::new_v4(),
            source_file_id,
            target_format,
            status: self.status.unwrap_or(ConversionStatus::default()),
            conversion_options: self.conversion_options,
            result_file_id: self.result_file_id,
            progress: self.progress.unwrap_or(0),
            error_message: self.error_message,
            started_at: self.started_at,
            completed_at: self.completed_at,
            metadata: AuditMetadata::default(),
        })
    }
}