use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::QuotaStatus;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{UserQuotaStateMachine, UserQuotaState, StateMachineError};

use thiserror::Error;

/// Domain error for quotaexceeded operations
#[derive(Debug, Clone, Error)]
pub enum QuotaExceeded {
    #[error("{0}")]
    Message(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for QuotaExceeded {
    fn from(msg: String) -> Self { Self::Message(msg) }
}

impl From<&str> for QuotaExceeded {
    fn from(msg: &str) -> Self { Self::Message(msg.to_string()) }
}


/// Strongly-typed ID for UserQuota
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserQuotaId(pub Uuid);

impl UserQuotaId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for UserQuotaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserQuotaId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for UserQuotaId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<UserQuotaId> for Uuid {
    fn from(id: UserQuotaId) -> Self { id.0 }
}

impl AsRef<Uuid> for UserQuotaId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for UserQuotaId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserQuota {
    pub id: Uuid,
    pub user_id: Uuid,
    pub limit_bytes: i64,
    pub used_bytes: i64,
    pub file_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_count: Option<i32>,
    pub tier: String,
    pub(crate) quota_status: QuotaStatus,
    pub warning_threshold_percent: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_warning_sent_at: Option<DateTime<Utc>>,
    pub peak_usage_bytes: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_usage_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl UserQuota {
    /// Create a builder for UserQuota
    pub fn builder() -> UserQuotaBuilder {
        UserQuotaBuilder::default()
    }

    /// Create a new UserQuota with required fields
    pub fn new(user_id: Uuid, limit_bytes: i64, used_bytes: i64, file_count: i32, tier: String, quota_status: QuotaStatus, warning_threshold_percent: i32, peak_usage_bytes: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            limit_bytes,
            used_bytes,
            file_count,
            max_file_size: None,
            max_file_count: None,
            tier,
            quota_status,
            warning_threshold_percent,
            last_warning_sent_at: None,
            peak_usage_bytes,
            peak_usage_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> UserQuotaId {
        UserQuotaId(self.id)
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


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the max_file_size field (chainable)
    pub fn with_max_file_size(mut self, value: i64) -> Self {
        self.max_file_size = Some(value);
        self
    }

    /// Set the max_file_count field (chainable)
    pub fn with_max_file_count(mut self, value: i32) -> Self {
        self.max_file_count = Some(value);
        self
    }

    /// Set the last_warning_sent_at field (chainable)
    pub fn with_last_warning_sent_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_warning_sent_at = Some(value);
        self
    }

    /// Set the peak_usage_at field (chainable)
    pub fn with_peak_usage_at(mut self, value: DateTime<Utc>) -> Self {
        self.peak_usage_at = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the quota_status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.quota_status` directly.
    pub fn transition_to(&mut self, new_state: UserQuotaState) -> Result<(), StateMachineError> {
        let current = self.quota_status.to_string().parse::<UserQuotaState>()?;
        let mut sm = UserQuotaStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.quota_status = new_state.to_string().parse::<QuotaStatus>()
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
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
                }
                "limit_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.limit_bytes = v; }
                }
                "used_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.used_bytes = v; }
                }
                "file_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_count = v; }
                }
                "max_file_size" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_file_size = v; }
                }
                "max_file_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_file_count = v; }
                }
                "tier" => {
                    if let Ok(v) = serde_json::from_value(value) { self.tier = v; }
                }
                "warning_threshold_percent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.warning_threshold_percent = v; }
                }
                "last_warning_sent_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_warning_sent_at = v; }
                }
                "peak_usage_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.peak_usage_bytes = v; }
                }
                "peak_usage_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.peak_usage_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if user has space for additional bytes
    pub fn has_space_for(&self, bytes: i64) -> bool {
        self.used_bytes + bytes <= self.limit_bytes
    }

    /// Get current usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.limit_bytes <= 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.limit_bytes as f64) * 100.0
    }

    /// Check if usage exceeds warning threshold
    pub fn is_over_warning_threshold(&self) -> bool {
        self.usage_percent() >= self.warning_threshold_percent as f64
    }

    /// Add bytes to usage, returns error if exceeded
    pub fn add_usage(&mut self, bytes: i64) -> Result<(), QuotaExceeded> {
        if self.used_bytes + bytes > self.limit_bytes {
            return Err(QuotaExceeded::ValidationFailed(
                format!(
                    "Adding {} bytes would exceed quota limit ({}/{} bytes)",
                    bytes, self.used_bytes + bytes, self.limit_bytes
                ),
            ));
        }
        self.used_bytes += bytes;
        self.metadata.updated_at = Some(Utc::now());
        Ok(())
    }

    /// Subtract bytes from usage
    pub fn subtract_usage(&mut self, bytes: i64) {
        self.used_bytes = (self.used_bytes - bytes).max(0);
        self.metadata.updated_at = Some(Utc::now());
    }

    /// Update peak usage if current exceeds
    pub fn update_peak(&mut self) {
        if self.used_bytes > self.peak_usage_bytes {
            self.peak_usage_bytes = self.used_bytes;
            self.peak_usage_at = Some(Utc::now());
        }
    }

    /// Get remaining available bytes
    pub fn remaining_bytes(&self) -> i64 {
        (self.limit_bytes - self.used_bytes).max(0)
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        // Invariant 1: used_bytes <= limit_bytes
        if self.used_bytes > self.limit_bytes {
            errors.push("used_bytes must not exceed limit_bytes");
        }
        // Invariant 2: used_bytes >= 0
        if self.used_bytes < 0 {
            errors.push("used_bytes must be non-negative");
        }
        // Invariant 3: file_count >= 0
        if self.file_count < 0 {
            errors.push("file_count must be non-negative");
        }
        // Invariant 4: warning_threshold_percent in 0..=100
        if self.warning_threshold_percent < 0 || self.warning_threshold_percent > 100 {
            errors.push("warning_threshold_percent must be between 0 and 100");
        }
        // Invariant 5: limit_bytes must be positive
        if self.limit_bytes <= 0 {
            errors.push("limit_bytes must be positive");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for UserQuota {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "UserQuota"
    }
}

impl backbone_core::PersistentEntity for UserQuota {
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

impl backbone_orm::EntityRepoMeta for UserQuota {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("quota_status".to_string(), "quota_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["tier"]
    }
}

/// Builder for UserQuota entity
///
/// Provides a fluent API for constructing UserQuota instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct UserQuotaBuilder {
    user_id: Option<Uuid>,
    limit_bytes: Option<i64>,
    used_bytes: Option<i64>,
    file_count: Option<i32>,
    max_file_size: Option<i64>,
    max_file_count: Option<i32>,
    tier: Option<String>,
    quota_status: Option<QuotaStatus>,
    warning_threshold_percent: Option<i32>,
    last_warning_sent_at: Option<DateTime<Utc>>,
    peak_usage_bytes: Option<i64>,
    peak_usage_at: Option<DateTime<Utc>>,
}

impl UserQuotaBuilder {
    /// Set the user_id field (required)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the limit_bytes field (required)
    pub fn limit_bytes(mut self, value: i64) -> Self {
        self.limit_bytes = Some(value);
        self
    }

    /// Set the used_bytes field (default: `0`)
    pub fn used_bytes(mut self, value: i64) -> Self {
        self.used_bytes = Some(value);
        self
    }

    /// Set the file_count field (default: `0`)
    pub fn file_count(mut self, value: i32) -> Self {
        self.file_count = Some(value);
        self
    }

    /// Set the max_file_size field (optional)
    pub fn max_file_size(mut self, value: i64) -> Self {
        self.max_file_size = Some(value);
        self
    }

    /// Set the max_file_count field (optional)
    pub fn max_file_count(mut self, value: i32) -> Self {
        self.max_file_count = Some(value);
        self
    }

    /// Set the tier field (default: `Default::default()`)
    pub fn tier(mut self, value: String) -> Self {
        self.tier = Some(value);
        self
    }

    /// Set the quota_status field (default: `QuotaStatus::default()`)
    pub fn quota_status(mut self, value: QuotaStatus) -> Self {
        self.quota_status = Some(value);
        self
    }

    /// Set the warning_threshold_percent field (default: `80`)
    pub fn warning_threshold_percent(mut self, value: i32) -> Self {
        self.warning_threshold_percent = Some(value);
        self
    }

    /// Set the last_warning_sent_at field (optional)
    pub fn last_warning_sent_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_warning_sent_at = Some(value);
        self
    }

    /// Set the peak_usage_bytes field (default: `0`)
    pub fn peak_usage_bytes(mut self, value: i64) -> Self {
        self.peak_usage_bytes = Some(value);
        self
    }

    /// Set the peak_usage_at field (optional)
    pub fn peak_usage_at(mut self, value: DateTime<Utc>) -> Self {
        self.peak_usage_at = Some(value);
        self
    }

    /// Build the UserQuota entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<UserQuota, String> {
        let user_id = self.user_id.ok_or_else(|| "user_id is required".to_string())?;
        let limit_bytes = self.limit_bytes.ok_or_else(|| "limit_bytes is required".to_string())?;

        Ok(UserQuota {
            id: Uuid::new_v4(),
            user_id,
            limit_bytes,
            used_bytes: self.used_bytes.unwrap_or(0),
            file_count: self.file_count.unwrap_or(0),
            max_file_size: self.max_file_size,
            max_file_count: self.max_file_count,
            tier: self.tier.unwrap_or(Default::default()),
            quota_status: self.quota_status.unwrap_or(QuotaStatus::default()),
            warning_threshold_percent: self.warning_threshold_percent.unwrap_or(80),
            last_warning_sent_at: self.last_warning_sent_at,
            peak_usage_bytes: self.peak_usage_bytes.unwrap_or(0),
            peak_usage_at: self.peak_usage_at,
            metadata: AuditMetadata::default(),
        })
    }
}