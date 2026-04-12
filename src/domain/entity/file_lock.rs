use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::LockStatus;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{FileLockStateMachine, FileLockState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

use thiserror::Error;

/// Domain error for lockerror operations
#[derive(Debug, Clone, Error)]
pub enum LockError {
    #[error("{0}")]
    Message(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for LockError {
    fn from(msg: String) -> Self { Self::Message(msg) }
}

impl From<&str> for LockError {
    fn from(msg: &str) -> Self { Self::Message(msg.to_string()) }
}


/// Strongly-typed ID for FileLock
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FileLockId(pub Uuid);

impl FileLockId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for FileLockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for FileLockId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for FileLockId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<FileLockId> for Uuid {
    fn from(id: FileLockId) -> Self { id.0 }
}

impl AsRef<Uuid> for FileLockId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for FileLockId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileLock {
    pub id: Uuid,
    pub file_id: Uuid,
    pub user_id: Uuid,
    pub locked_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<DateTime<Utc>>,
    pub(crate) status: LockStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl FileLock {
    /// Create a builder for FileLock
    pub fn builder() -> FileLockBuilder {
        FileLockBuilder::default()
    }

    /// Create a new FileLock with required fields
    pub fn new(file_id: Uuid, user_id: Uuid, locked_at: DateTime<Utc>, expires_at: DateTime<Utc>, status: LockStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            user_id,
            locked_at,
            expires_at,
            refreshed_at: None,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> FileLockId {
        FileLockId(self.id)
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
    pub fn status(&self) -> &LockStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the refreshed_at field (chainable)
    pub fn with_refreshed_at(mut self, value: DateTime<Utc>) -> Self {
        self.refreshed_at = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: FileLockState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<FileLockState>()?;
        let mut sm = FileLockStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<LockStatus>()
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
                "locked_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.locked_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "refreshed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.refreshed_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if lock is still valid (active status and not yet expired)
    pub fn is_valid(&self) -> bool {
        self.status == LockStatus::Active && !self.is_expired()
    }

    /// Check if lock has expired (expires_at is at or before the current time)
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    /// Check if lock belongs to the given user
    pub fn is_owned_by(&self, user_id: Uuid) -> bool {
        self.user_id == user_id
    }

    /// Check if lock can be refreshed (must be currently valid)
    pub fn can_refresh(&self) -> bool {
        self.is_valid()
    }

    /// Refresh lock expiry time by extending it by the given duration
    pub fn refresh(&mut self, duration: Duration) -> Result<(), LockError> {
        if !self.can_refresh() {
            return Err(LockError::ValidationFailed(
                "Cannot refresh lock: lock is not active or has expired".to_string(),
            ));
        }

        let now = Utc::now();
        self.expires_at = now + duration;
        self.refreshed_at = Some(now);
        self.metadata.touch();

        Ok(())
    }

    /// Get remaining time until expiry. Returns zero duration if already expired.
    pub fn time_remaining(&self) -> Duration {
        let remaining = self.expires_at - Utc::now();
        if remaining < Duration::zero() {
            Duration::zero()
        } else {
            remaining
        }
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        // Invariant 1: expires_at must be after locked_at
        if self.expires_at <= self.locked_at {
            errors.push("expires_at must be after locked_at");
        }

        // Invariant 2: if status is Active, it must not be expired
        if self.status == LockStatus::Active && self.is_expired() {
            errors.push("active lock must not be expired (expires_at must be in the future)");
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for FileLock {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "FileLock"
    }
}

impl backbone_core::PersistentEntity for FileLock {
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

impl backbone_orm::EntityRepoMeta for FileLock {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("file_id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "lock_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for FileLock entity
///
/// Provides a fluent API for constructing FileLock instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct FileLockBuilder {
    file_id: Option<Uuid>,
    user_id: Option<Uuid>,
    locked_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    refreshed_at: Option<DateTime<Utc>>,
    status: Option<LockStatus>,
}

impl FileLockBuilder {
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

    /// Set the locked_at field (default: `Utc::now()`)
    pub fn locked_at(mut self, value: DateTime<Utc>) -> Self {
        self.locked_at = Some(value);
        self
    }

    /// Set the expires_at field (required)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the refreshed_at field (optional)
    pub fn refreshed_at(mut self, value: DateTime<Utc>) -> Self {
        self.refreshed_at = Some(value);
        self
    }

    /// Set the status field (default: `LockStatus::default()`)
    pub fn status(mut self, value: LockStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the FileLock entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<FileLock, String> {
        let file_id = self.file_id.ok_or_else(|| "file_id is required".to_string())?;
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let expires_at = self.expires_at.ok_or_else(|| "expires_at is required".to_string())?;

        Ok(FileLock {
            id: Uuid::new_v4(),
            file_id,
            user_id,
            locked_at: self.locked_at.unwrap_or(Utc::now()),
            expires_at,
            refreshed_at: self.refreshed_at,
            status: self.status.unwrap_or(LockStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}