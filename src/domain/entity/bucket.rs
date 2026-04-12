use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::BucketType;
use super::BucketStatus;
use super::StorageBackend;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{BucketStateMachine, BucketState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for Bucket
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BucketId(pub Uuid);

impl BucketId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for BucketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for BucketId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for BucketId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<BucketId> for Uuid {
    fn from(id: BucketId) -> Self { id.0 }
}

impl AsRef<Uuid> for BucketId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for BucketId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bucket {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub bucket_type: BucketType,
    pub(crate) status: BucketStatus,
    pub storage_backend: StorageBackend,
    pub root_path: String,
    pub file_count: i32,
    pub total_size_bytes: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,
    pub allowed_mime_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_after_days: Option<i32>,
    pub enable_cdn: bool,
    pub enable_versioning: bool,
    pub enable_deduplication: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Bucket {
    /// Create a builder for Bucket
    pub fn builder() -> BucketBuilder {
        BucketBuilder::default()
    }

    /// Create a new Bucket with required fields
    pub fn new(name: String, slug: String, owner_id: Uuid, bucket_type: BucketType, status: BucketStatus, storage_backend: StorageBackend, root_path: String, file_count: i32, total_size_bytes: i64, allowed_mime_types: Vec<String>, enable_cdn: bool, enable_versioning: bool, enable_deduplication: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            description: None,
            owner_id,
            bucket_type,
            status,
            storage_backend,
            root_path,
            file_count,
            total_size_bytes,
            max_file_size: None,
            allowed_mime_types,
            auto_delete_after_days: None,
            enable_cdn,
            enable_versioning,
            enable_deduplication,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> BucketId {
        BucketId(self.id)
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
    pub fn status(&self) -> &BucketStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the max_file_size field (chainable)
    pub fn with_max_file_size(mut self, value: i64) -> Self {
        self.max_file_size = Some(value);
        self
    }

    /// Set the auto_delete_after_days field (chainable)
    pub fn with_auto_delete_after_days(mut self, value: i32) -> Self {
        self.auto_delete_after_days = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: BucketState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<BucketState>()?;
        let mut sm = BucketStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<BucketStatus>()
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
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "slug" => {
                    if let Ok(v) = serde_json::from_value(value) { self.slug = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "owner_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.owner_id = v; }
                }
                "bucket_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bucket_type = v; }
                }
                "storage_backend" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_backend = v; }
                }
                "root_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.root_path = v; }
                }
                "file_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_count = v; }
                }
                "total_size_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.total_size_bytes = v; }
                }
                "max_file_size" => {
                    if let Ok(v) = serde_json::from_value(value) { self.max_file_size = v; }
                }
                "allowed_mime_types" => {
                    if let Ok(v) = serde_json::from_value(value) { self.allowed_mime_types = v; }
                }
                "auto_delete_after_days" => {
                    if let Ok(v) = serde_json::from_value(value) { self.auto_delete_after_days = v; }
                }
                "enable_cdn" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enable_cdn = v; }
                }
                "enable_versioning" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enable_versioning = v; }
                }
                "enable_deduplication" => {
                    if let Ok(v) = serde_json::from_value(value) { self.enable_deduplication = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if file can be uploaded to this bucket
    pub fn can_upload(&self, file_size: i64, mime_type: &str) -> bool {
        if self.status != BucketStatus::Active {
            return false;
        }
        if let Some(max_size) = self.max_file_size {
            if file_size > max_size {
                return false;
            }
        }
        if !self.allowed_mime_types.is_empty() && !self.allowed_mime_types.contains(&mime_type.to_string()) {
            return false;
        }
        true
    }

    /// Update file count and total size
    pub fn update_stats(&mut self, size_delta: i64, count_delta: i32) {
        self.file_count = (self.file_count + count_delta).max(0);
        self.total_size_bytes = (self.total_size_bytes + size_delta).max(0);
    }

    /// Check if bucket is accessible (not deleted/archived)
    pub fn is_accessible(&self) -> bool {
        !self.is_deleted() && self.status != BucketStatus::Archived
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.file_count < 0 {
            errors.push("file_count must be >= 0");
        }
        if self.total_size_bytes < 0 {
            errors.push("total_size_bytes must be >= 0");
        }
        if self.name.trim().is_empty() {
            errors.push("name must not be empty");
        }
        if self.slug.trim().is_empty() {
            errors.push("slug must not be empty");
        }
        if self.root_path.contains("..") {
            errors.push("root_path must not contain path traversal");
        }
        if let Some(max_file_size) = self.max_file_size {
            if max_file_size <= 0 {
                errors.push("max_file_size must be positive when set");
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Bucket {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Bucket"
    }
}

impl backbone_core::PersistentEntity for Bucket {
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

impl backbone_orm::EntityRepoMeta for Bucket {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("owner_id".to_string(), "uuid".to_string());
        m.insert("bucket_type".to_string(), "bucket_type".to_string());
        m.insert("status".to_string(), "bucket_status".to_string());
        m.insert("storage_backend".to_string(), "storage_backend".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name", "slug", "root_path"]
    }
}

/// Builder for Bucket entity
///
/// Provides a fluent API for constructing Bucket instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct BucketBuilder {
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
    owner_id: Option<Uuid>,
    bucket_type: Option<BucketType>,
    status: Option<BucketStatus>,
    storage_backend: Option<StorageBackend>,
    root_path: Option<String>,
    file_count: Option<i32>,
    total_size_bytes: Option<i64>,
    max_file_size: Option<i64>,
    allowed_mime_types: Option<Vec<String>>,
    auto_delete_after_days: Option<i32>,
    enable_cdn: Option<bool>,
    enable_versioning: Option<bool>,
    enable_deduplication: Option<bool>,
}

impl BucketBuilder {
    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the slug field (required)
    pub fn slug(mut self, value: String) -> Self {
        self.slug = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the owner_id field (required)
    pub fn owner_id(mut self, value: Uuid) -> Self {
        self.owner_id = Some(value);
        self
    }

    /// Set the bucket_type field (default: `BucketType::default()`)
    pub fn bucket_type(mut self, value: BucketType) -> Self {
        self.bucket_type = Some(value);
        self
    }

    /// Set the status field (default: `BucketStatus::default()`)
    pub fn status(mut self, value: BucketStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the storage_backend field (default: `StorageBackend::default()`)
    pub fn storage_backend(mut self, value: StorageBackend) -> Self {
        self.storage_backend = Some(value);
        self
    }

    /// Set the root_path field (required)
    pub fn root_path(mut self, value: String) -> Self {
        self.root_path = Some(value);
        self
    }

    /// Set the file_count field (default: `0`)
    pub fn file_count(mut self, value: i32) -> Self {
        self.file_count = Some(value);
        self
    }

    /// Set the total_size_bytes field (default: `0`)
    pub fn total_size_bytes(mut self, value: i64) -> Self {
        self.total_size_bytes = Some(value);
        self
    }

    /// Set the max_file_size field (optional)
    pub fn max_file_size(mut self, value: i64) -> Self {
        self.max_file_size = Some(value);
        self
    }

    /// Set the allowed_mime_types field (required)
    pub fn allowed_mime_types(mut self, value: Vec<String>) -> Self {
        self.allowed_mime_types = Some(value);
        self
    }

    /// Set the auto_delete_after_days field (optional)
    pub fn auto_delete_after_days(mut self, value: i32) -> Self {
        self.auto_delete_after_days = Some(value);
        self
    }

    /// Set the enable_cdn field (default: `false`)
    pub fn enable_cdn(mut self, value: bool) -> Self {
        self.enable_cdn = Some(value);
        self
    }

    /// Set the enable_versioning field (default: `true`)
    pub fn enable_versioning(mut self, value: bool) -> Self {
        self.enable_versioning = Some(value);
        self
    }

    /// Set the enable_deduplication field (default: `true`)
    pub fn enable_deduplication(mut self, value: bool) -> Self {
        self.enable_deduplication = Some(value);
        self
    }

    /// Build the Bucket entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Bucket, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let slug = self.slug.ok_or_else(|| "slug is required".to_string())?;
        let owner_id = self.owner_id.ok_or_else(|| "owner_id is required".to_string())?;
        let root_path = self.root_path.ok_or_else(|| "root_path is required".to_string())?;
        let allowed_mime_types = self.allowed_mime_types.ok_or_else(|| "allowed_mime_types is required".to_string())?;

        Ok(Bucket {
            id: Uuid::new_v4(),
            name,
            slug,
            description: self.description,
            owner_id,
            bucket_type: self.bucket_type.unwrap_or(BucketType::default()),
            status: self.status.unwrap_or(BucketStatus::default()),
            storage_backend: self.storage_backend.unwrap_or(StorageBackend::default()),
            root_path,
            file_count: self.file_count.unwrap_or(0),
            total_size_bytes: self.total_size_bytes.unwrap_or(0),
            max_file_size: self.max_file_size,
            allowed_mime_types,
            auto_delete_after_days: self.auto_delete_after_days,
            enable_cdn: self.enable_cdn.unwrap_or(false),
            enable_versioning: self.enable_versioning.unwrap_or(true),
            enable_deduplication: self.enable_deduplication.unwrap_or(true),
            metadata: AuditMetadata::default(),
        })
    }
}