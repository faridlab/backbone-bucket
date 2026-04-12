use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::CommentStatus;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{FileCommentStateMachine, FileCommentState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

use thiserror::Error;

/// Domain error for commenterror operations
#[derive(Debug, Clone, Error)]
pub enum CommentError {
    #[error("{0}")]
    Message(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for CommentError {
    fn from(msg: String) -> Self { Self::Message(msg) }
}

impl From<&str> for CommentError {
    fn from(msg: &str) -> Self { Self::Message(msg.to_string()) }
}


/// Strongly-typed ID for FileComment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FileCommentId(pub Uuid);

impl FileCommentId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for FileCommentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for FileCommentId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for FileCommentId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<FileCommentId> for Uuid {
    fn from(id: FileCommentId) -> Self { id.0 }
}

impl AsRef<Uuid> for FileCommentId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for FileCommentId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileComment {
    pub id: Uuid,
    pub file_id: Uuid,
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_region: Option<serde_json::Value>,
    pub mentions: Vec<Uuid>,
    pub resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<DateTime<Utc>>,
    pub(crate) status: CommentStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl FileComment {
    /// Create a builder for FileComment
    pub fn builder() -> FileCommentBuilder {
        FileCommentBuilder::default()
    }

    /// Create a new FileComment with required fields
    pub fn new(file_id: Uuid, user_id: Uuid, content: String, mentions: Vec<Uuid>, resolved: bool, status: CommentStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            user_id,
            parent_id: None,
            content,
            annotation_region: None,
            mentions,
            resolved,
            resolved_by: None,
            resolved_at: None,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> FileCommentId {
        FileCommentId(self.id)
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
    pub fn status(&self) -> &CommentStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the parent_id field (chainable)
    pub fn with_parent_id(mut self, value: Uuid) -> Self {
        self.parent_id = Some(value);
        self
    }

    /// Set the annotation_region field (chainable)
    pub fn with_annotation_region(mut self, value: serde_json::Value) -> Self {
        self.annotation_region = Some(value);
        self
    }

    /// Set the resolved_by field (chainable)
    pub fn with_resolved_by(mut self, value: Uuid) -> Self {
        self.resolved_by = Some(value);
        self
    }

    /// Set the resolved_at field (chainable)
    pub fn with_resolved_at(mut self, value: DateTime<Utc>) -> Self {
        self.resolved_at = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: FileCommentState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<FileCommentState>()?;
        let mut sm = FileCommentStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<CommentStatus>()
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
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
                }
                "parent_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.parent_id = v; }
                }
                "content" => {
                    if let Ok(v) = serde_json::from_value(value) { self.content = v; }
                }
                "annotation_region" => {
                    if let Ok(v) = serde_json::from_value(value) { self.annotation_region = v; }
                }
                "mentions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mentions = v; }
                }
                "resolved" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resolved = v; }
                }
                "resolved_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resolved_by = v; }
                }
                "resolved_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.resolved_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if this is a reply to another comment
    pub fn is_reply(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Check if comment has region annotations
    pub fn has_annotations(&self) -> bool {
        self.annotation_region
            .as_ref()
            .map(|v| !v.is_null() && v != &serde_json::Value::Object(serde_json::Map::new()))
            .unwrap_or(false)
    }

    /// Get reply comments (aggregate pattern - replies loaded separately via repository)
    pub fn get_replies(&self) -> Vec<FileComment> {
        Vec::new()
    }

    /// Mark comment as resolved
    pub fn resolve(&mut self, by_user_id: Uuid) -> Result<(), CommentError> {
        if self.resolved {
            return Err(CommentError::Conflict("Comment is already resolved".to_string()));
        }
        self.resolved = true;
        self.resolved_by = Some(by_user_id);
        self.resolved_at = Some(Utc::now());
        self.metadata.touch();
        Ok(())
    }

    /// Mark comment as unresolved
    pub fn unresolve(&mut self) -> Result<(), CommentError> {
        if !self.resolved {
            return Err(CommentError::Conflict("Comment is not resolved".to_string()));
        }
        self.resolved = false;
        self.resolved_by = None;
        self.resolved_at = None;
        self.metadata.touch();
        Ok(())
    }

    /// Count mentioned users
    pub fn mention_count(&self) -> i32 {
        self.mentions.len() as i32
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.content.len() > 10000 {
            errors.push("content must not exceed 10000 characters");
        }
        if self.parent_id == Some(self.id) {
            errors.push("parent_id must not reference self");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for FileComment {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "FileComment"
    }
}

impl backbone_core::PersistentEntity for FileComment {
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

impl backbone_orm::EntityRepoMeta for FileComment {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("file_id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("parent_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "comment_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["content"]
    }
}

/// Builder for FileComment entity
///
/// Provides a fluent API for constructing FileComment instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct FileCommentBuilder {
    file_id: Option<Uuid>,
    user_id: Option<Uuid>,
    parent_id: Option<Uuid>,
    content: Option<String>,
    annotation_region: Option<serde_json::Value>,
    mentions: Option<Vec<Uuid>>,
    resolved: Option<bool>,
    resolved_by: Option<Uuid>,
    resolved_at: Option<DateTime<Utc>>,
    status: Option<CommentStatus>,
}

impl FileCommentBuilder {
    /// Set the file_id field (required)
    pub fn file_id(mut self, value: Uuid) -> Self {
        self.file_id = Some(value);
        self
    }

    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the parent_id field (optional)
    pub fn parent_id(mut self, value: Uuid) -> Self {
        self.parent_id = Some(value);
        self
    }

    /// Set the content field (required)
    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    /// Set the annotation_region field (optional)
    pub fn annotation_region(mut self, value: serde_json::Value) -> Self {
        self.annotation_region = Some(value);
        self
    }

    /// Set the mentions field (required)
    pub fn mentions(mut self, value: Vec<Uuid>) -> Self {
        self.mentions = Some(value);
        self
    }

    /// Set the resolved field (default: `false`)
    pub fn resolved(mut self, value: bool) -> Self {
        self.resolved = Some(value);
        self
    }

    /// Set the resolved_by field (optional)
    pub fn resolved_by(mut self, value: Uuid) -> Self {
        self.resolved_by = Some(value);
        self
    }

    /// Set the resolved_at field (optional)
    pub fn resolved_at(mut self, value: DateTime<Utc>) -> Self {
        self.resolved_at = Some(value);
        self
    }

    /// Set the status field (default: `CommentStatus::default()`)
    pub fn status(mut self, value: CommentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the FileComment entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<FileComment, String> {
        let file_id = self.file_id.ok_or_else(|| "file_id is required".to_string())?;
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let content = self.content.ok_or_else(|| "content is required".to_string())?;
        let mentions = self.mentions.ok_or_else(|| "mentions is required".to_string())?;

        Ok(FileComment {
            id: Uuid::new_v4(),
            file_id,
            user_id,
            parent_id: self.parent_id,
            content,
            annotation_region: self.annotation_region,
            mentions,
            resolved: self.resolved.unwrap_or(false),
            resolved_by: self.resolved_by,
            resolved_at: self.resolved_at,
            status: self.status.unwrap_or(CommentStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}