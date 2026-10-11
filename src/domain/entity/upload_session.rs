use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::UploadStatus;
use super::StorageBackend;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{UploadSessionStateMachine, UploadSessionState, StateMachineError};

use thiserror::Error;

/// Domain error for uploaderror operations
#[derive(Debug, Clone, Error)]
pub enum UploadError {
    #[error("{0}")]
    Message(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for UploadError {
    fn from(msg: String) -> Self { Self::Message(msg) }
}

impl From<&str> for UploadError {
    fn from(msg: &str) -> Self { Self::Message(msg.to_string()) }
}


/// Strongly-typed ID for UploadSession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UploadSessionId(pub Uuid);

impl UploadSessionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for UploadSessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UploadSessionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for UploadSessionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<UploadSessionId> for Uuid {
    fn from(id: UploadSessionId) -> Self { id.0 }
}

impl AsRef<Uuid> for UploadSessionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for UploadSessionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UploadSession {
    pub id: Uuid,
    pub bucket_id: Uuid,
    pub user_id: Uuid,
    pub path: String,
    pub filename: String,
    pub mime_type: Option<String>,
    pub file_size: i64,
    pub chunk_size: i32,
    pub total_chunks: i32,
    pub uploaded_chunks: i32,
    pub(crate) status: UploadStatus,
    pub storage_backend: StorageBackend,
    pub completed_parts: Vec<i32>,
    pub part_etags: Option<serde_json::Value>,
    pub expires_at: DateTime<Utc>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl UploadSession {
    /// Create a builder for UploadSession
    pub fn builder() -> UploadSessionBuilder {
        <UploadSessionBuilder as Default>::default()
    }

    /// Create a new UploadSession with required fields
    pub fn new(bucket_id: Uuid, user_id: Uuid, path: String, filename: String, file_size: i64, chunk_size: i32, total_chunks: i32, uploaded_chunks: i32, status: UploadStatus, storage_backend: StorageBackend, completed_parts: Vec<i32>, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            bucket_id,
            user_id,
            path,
            filename,
            mime_type: None,
            file_size,
            chunk_size,
            total_chunks,
            uploaded_chunks,
            status,
            storage_backend,
            completed_parts,
            part_etags: None,
            expires_at,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> UploadSessionId {
        UploadSessionId(self.id)
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
    pub fn status(&self) -> &UploadStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the mime_type field (chainable)
    pub fn with_mime_type(mut self, value: String) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Set the part_etags field (chainable)
    pub fn with_part_etags(mut self, value: serde_json::Value) -> Self {
        self.part_etags = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: UploadSessionState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<UploadSessionState>()?;
        let mut sm = UploadSessionStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<UploadStatus>()
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
                "bucket_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bucket_id = v; }
                }
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
                }
                "path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.path = v; }
                }
                "filename" => {
                    if let Ok(v) = serde_json::from_value(value) { self.filename = v; }
                }
                "mime_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mime_type = v; }
                }
                "file_size" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_size = v; }
                }
                "chunk_size" => {
                    if let Ok(v) = serde_json::from_value(value) { self.chunk_size = v; }
                }
                "total_chunks" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total_chunks = v; }
                }
                "uploaded_chunks" => {
                    if let Ok(v) = serde_json::from_value(value) { self.uploaded_chunks = v; }
                }
                "storage_backend" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_backend = v; }
                }
                "completed_parts" => {
                    if let Ok(v) = serde_json::from_value(value) { self.completed_parts = v; }
                }
                "part_etags" => {
                    if let Ok(v) = serde_json::from_value(value) { self.part_etags = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

}

impl super::Entity for UploadSession {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "UploadSession"
    }
}

impl backbone_core::PersistentEntity for UploadSession {
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

impl backbone_orm::EntityRepoMeta for UploadSession {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("bucket_id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "upload_status".to_string());
        m.insert("storage_backend".to_string(), "storage_backend".to_string());
        m.insert("expires_at".to_string(), "timestamptz".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["path", "filename"]
    }
}

/// Builder for UploadSession entity
///
/// Provides a fluent API for constructing UploadSession instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct UploadSessionBuilder {
    bucket_id: Option<Uuid>,
    user_id: Option<Uuid>,
    path: Option<String>,
    filename: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
    chunk_size: Option<i32>,
    total_chunks: Option<i32>,
    uploaded_chunks: Option<i32>,
    status: Option<UploadStatus>,
    storage_backend: Option<StorageBackend>,
    completed_parts: Option<Vec<i32>>,
    part_etags: Option<serde_json::Value>,
    expires_at: Option<DateTime<Utc>>,
}

impl UploadSessionBuilder {
    /// Set the bucket_id field (required)
    pub fn bucket_id(mut self, value: Uuid) -> Self {
        self.bucket_id = Some(value);
        self
    }

    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the path field (required)
    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    /// Set the filename field (required)
    pub fn filename(mut self, value: String) -> Self {
        self.filename = Some(value);
        self
    }

    /// Set the mime_type field (optional)
    pub fn mime_type(mut self, value: String) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Set the file_size field (required)
    pub fn file_size(mut self, value: i64) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Set the chunk_size field (required)
    pub fn chunk_size(mut self, value: i32) -> Self {
        self.chunk_size = Some(value);
        self
    }

    /// Set the total_chunks field (required)
    pub fn total_chunks(mut self, value: i32) -> Self {
        self.total_chunks = Some(value);
        self
    }

    /// Set the uploaded_chunks field (default: `0`)
    pub fn uploaded_chunks(mut self, value: i32) -> Self {
        self.uploaded_chunks = Some(value);
        self
    }

    /// Set the status field (default: `UploadStatus::default()`)
    pub fn status(mut self, value: UploadStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the storage_backend field (default: `StorageBackend::default()`)
    pub fn storage_backend(mut self, value: StorageBackend) -> Self {
        self.storage_backend = Some(value);
        self
    }

    /// Set the completed_parts field (required)
    pub fn completed_parts(mut self, value: Vec<i32>) -> Self {
        self.completed_parts = Some(value);
        self
    }

    /// Set the part_etags field (optional)
    pub fn part_etags(mut self, value: serde_json::Value) -> Self {
        self.part_etags = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Build the UploadSession entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<UploadSession, String> {
        let bucket_id = self.bucket_id.ok_or_else(|| "bucket_id is required".to_string())?;
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let path = self.path.ok_or_else(|| "path is required".to_string())?;
        let filename = self.filename.ok_or_else(|| "filename is required".to_string())?;
        let file_size = self.file_size.ok_or_else(|| "file_size is required".to_string())?;
        let chunk_size = self.chunk_size.ok_or_else(|| "chunk_size is required".to_string())?;
        let total_chunks = self.total_chunks.ok_or_else(|| "total_chunks is required".to_string())?;
        let completed_parts = self.completed_parts.ok_or_else(|| "completed_parts is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(UploadSession {
            id: Uuid::new_v4(),
            bucket_id,
            user_id,
            path,
            filename,
            mime_type: self.mime_type,
            file_size,
            chunk_size,
            total_chunks,
            uploaded_chunks: self.uploaded_chunks.unwrap_or(0),
            status: self.status.unwrap_or_default(),
            storage_backend: self.storage_backend.unwrap_or_default(),
            completed_parts,
            part_etags: self.part_etags,
            expires_at,
            metadata: AuditMetadata::default(),
        })
    }
}

// This file's extension (hand-written; ADR-0031).
#[path = "upload_session.ext.rs"]
mod upload_session_ext;
pub use upload_session_ext::*;
