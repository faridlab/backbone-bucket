use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ShareType;
use super::SharePermission;
use super::ShareStatus;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{FileShareStateMachine, FileShareState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for FileShare
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FileShareId(pub Uuid);

impl FileShareId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for FileShareId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for FileShareId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for FileShareId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<FileShareId> for Uuid {
    fn from(id: FileShareId) -> Self { id.0 }
}

impl AsRef<Uuid> for FileShareId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for FileShareId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileShare {
    pub id: Uuid,
    pub file_id: Uuid,
    pub owner_id: Uuid,
    pub token: String,
    pub share_type: ShareType,
    pub permission: SharePermission,
    pub shared_with: Vec<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_downloads: Option<i32>,
    pub download_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    pub(crate) share_status: ShareStatus,
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl FileShare {
    /// Create a builder for FileShare
    pub fn builder() -> FileShareBuilder {
        FileShareBuilder::default()
    }

    /// Create a new FileShare with required fields
    pub fn new(file_id: Uuid, owner_id: Uuid, token: String, share_type: ShareType, permission: SharePermission, shared_with: Vec<Uuid>, download_count: i32, share_status: ShareStatus, is_active: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            owner_id,
            token,
            share_type,
            permission,
            shared_with,
            password_hash: None,
            max_downloads: None,
            download_count,
            expires_at: None,
            share_status,
            is_active,
            revoked_at: None,
            revoked_by: None,
            message: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> FileShareId {
        FileShareId(self.id)
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

    /// Set the password_hash field (chainable)
    pub fn with_password_hash(mut self, value: String) -> Self {
        self.password_hash = Some(value);
        self
    }

    /// Set the max_downloads field (chainable)
    pub fn with_max_downloads(mut self, value: i32) -> Self {
        self.max_downloads = Some(value);
        self
    }

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the revoked_at field (chainable)
    pub fn with_revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (chainable)
    pub fn with_revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the message field (chainable)
    pub fn with_message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the share_status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.share_status` directly.
    pub fn transition_to(&mut self, new_state: FileShareState) -> Result<(), StateMachineError> {
        let current = self.share_status.to_string().parse::<FileShareState>()?;
        let mut sm = FileShareStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.share_status = new_state.to_string().parse::<ShareStatus>()
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
                "owner_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.owner_id = v; }
                }
                "token" => {
                    if let Ok(v) = serde_json::from_value(value) { self.token = v; }
                }
                "share_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.share_type = v; }
                }
                "permission" => {
                    if let Ok(v) = serde_json::from_value(value) { self.permission = v; }
                }
                "shared_with" => {
                    if let Ok(v) = serde_json::from_value(value) { self.shared_with = v; }
                }
                "password_hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.password_hash = v; }
                }
                "max_downloads" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_downloads = v; }
                }
                "download_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.download_count = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "is_active" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_active = v; }
                }
                "revoked_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_at = v; }
                }
                "revoked_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.revoked_by = v; }
                }
                "message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.message = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if share is active, not expired, and has downloads remaining
    pub fn is_valid(&self) -> bool {
        self.share_status == ShareStatus::Active
            && !self.is_expired()
            && self.has_downloads_remaining()
    }

    /// Check if user can access via this share
    pub fn can_access(&self, user_id: Option<Uuid>, password: Option<&str>) -> bool {
        if !self.is_valid() {
            return false;
        }

        match self.share_type {
            ShareType::Link => true,
            ShareType::User => {
                match user_id {
                    Some(uid) => self.shared_with.contains(&uid),
                    None => false,
                }
            }
            ShareType::Password => {
                match (password, &self.password_hash) {
                    (Some(pw), Some(hash)) => {
                        bcrypt::verify(pw, hash).unwrap_or(false)
                    }
                    _ => false,
                }
            }
        }
    }

    /// Increment download count, returns false if limit reached
    pub fn record_download(&mut self) -> bool {
        if !self.has_downloads_remaining() {
            return false;
        }
        self.download_count += 1;
        true
    }

    /// Revoke the share
    pub fn revoke(&mut self, by_user_id: Uuid) {
        self.share_status = ShareStatus::Revoked;
        self.is_active = false;
        self.revoked_at = Some(Utc::now());
        self.revoked_by = Some(by_user_id);
    }

    /// Check if share has expired
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires) => expires <= Utc::now(),
            None => false,
        }
    }

    /// Check if download limit not reached
    pub fn has_downloads_remaining(&self) -> bool {
        match self.max_downloads {
            Some(max) => self.download_count < max,
            None => true,
        }
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        // Invariant: token must not be empty
        if self.token.is_empty() {
            errors.push("token must not be empty");
        }

        // Invariant: token must not contain ".."
        if self.token.contains("..") {
            errors.push("token must not contain '..'");
        }

        // Invariant: download_count must be non-negative
        if self.download_count < 0 {
            errors.push("download_count must be non-negative");
        }

        // Invariant: download_count <= max_downloads (if set)
        if let Some(max) = self.max_downloads {
            if self.download_count > max {
                errors.push("download_count must not exceed max_downloads");
            }
        }

        // Invariant: password shares must have a password_hash
        if self.share_type == ShareType::Password && self.password_hash.is_none() {
            errors.push("password share must have a password_hash");
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for FileShare {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "FileShare"
    }
}

impl backbone_core::PersistentEntity for FileShare {
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

impl backbone_orm::EntityRepoMeta for FileShare {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("file_id".to_string(), "uuid".to_string());
        m.insert("owner_id".to_string(), "uuid".to_string());
        m.insert("share_type".to_string(), "share_type".to_string());
        m.insert("permission".to_string(), "share_permission".to_string());
        m.insert("share_status".to_string(), "share_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["token"]
    }
}

/// Builder for FileShare entity
///
/// Provides a fluent API for constructing FileShare instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct FileShareBuilder {
    file_id: Option<Uuid>,
    owner_id: Option<Uuid>,
    token: Option<String>,
    share_type: Option<ShareType>,
    permission: Option<SharePermission>,
    shared_with: Option<Vec<Uuid>>,
    password_hash: Option<String>,
    max_downloads: Option<i32>,
    download_count: Option<i32>,
    expires_at: Option<DateTime<Utc>>,
    share_status: Option<ShareStatus>,
    is_active: Option<bool>,
    revoked_at: Option<DateTime<Utc>>,
    revoked_by: Option<Uuid>,
    message: Option<String>,
}

impl FileShareBuilder {
    /// Set the file_id field (required)
    pub fn file_id(mut self, value: Uuid) -> Self {
        self.file_id = Some(value);
        self
    }

    /// Set the owner_id field (required)
    pub fn owner_id(mut self, value: Uuid) -> Self {
        self.owner_id = Some(value);
        self
    }

    /// Set the token field (required)
    pub fn token(mut self, value: String) -> Self {
        self.token = Some(value);
        self
    }

    /// Set the share_type field (default: `ShareType::default()`)
    pub fn share_type(mut self, value: ShareType) -> Self {
        self.share_type = Some(value);
        self
    }

    /// Set the permission field (default: `SharePermission::default()`)
    pub fn permission(mut self, value: SharePermission) -> Self {
        self.permission = Some(value);
        self
    }

    /// Set the shared_with field (required)
    pub fn shared_with(mut self, value: Vec<Uuid>) -> Self {
        self.shared_with = Some(value);
        self
    }

    /// Set the password_hash field (optional)
    pub fn password_hash(mut self, value: String) -> Self {
        self.password_hash = Some(value);
        self
    }

    /// Set the max_downloads field (optional)
    pub fn max_downloads(mut self, value: i32) -> Self {
        self.max_downloads = Some(value);
        self
    }

    /// Set the download_count field (default: `0`)
    pub fn download_count(mut self, value: i32) -> Self {
        self.download_count = Some(value);
        self
    }

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the share_status field (default: `ShareStatus::default()`)
    pub fn share_status(mut self, value: ShareStatus) -> Self {
        self.share_status = Some(value);
        self
    }

    /// Set the is_active field (default: `true`)
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Set the revoked_at field (optional)
    pub fn revoked_at(mut self, value: DateTime<Utc>) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Set the revoked_by field (optional)
    pub fn revoked_by(mut self, value: Uuid) -> Self {
        self.revoked_by = Some(value);
        self
    }

    /// Set the message field (optional)
    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    /// Build the FileShare entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<FileShare, String> {
        let file_id = self.file_id.ok_or_else(|| "file_id is required".to_string())?;
        let owner_id = self.owner_id.ok_or_else(|| "owner_id is required".to_string())?;
        let token = self.token.ok_or_else(|| "token is required".to_string())?;
        let shared_with = self.shared_with.ok_or_else(|| "shared_with is required".to_string())?;

        Ok(FileShare {
            id: Uuid::new_v4(),
            file_id,
            owner_id,
            token,
            share_type: self.share_type.unwrap_or(ShareType::default()),
            permission: self.permission.unwrap_or(SharePermission::default()),
            shared_with,
            password_hash: self.password_hash,
            max_downloads: self.max_downloads,
            download_count: self.download_count.unwrap_or(0),
            expires_at: self.expires_at,
            share_status: self.share_status.unwrap_or(ShareStatus::default()),
            is_active: self.is_active.unwrap_or(true),
            revoked_at: self.revoked_at,
            revoked_by: self.revoked_by,
            message: self.message,
            metadata: AuditMetadata::default(),
        })
    }
}