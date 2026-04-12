use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::StorageBackend;
use super::AuditMetadata;

use super::*;

use thiserror::Error;

/// Domain error for deduperror operations
#[derive(Debug, Clone, Error)]
pub enum DedupError {
    #[error("{0}")]
    Message(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<String> for DedupError {
    fn from(msg: String) -> Self { Self::Message(msg) }
}

impl From<&str> for DedupError {
    fn from(msg: &str) -> Self { Self::Message(msg.to_string()) }
}


/// Strongly-typed ID for ContentHash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContentHashId(pub Uuid);

impl ContentHashId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ContentHashId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ContentHashId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ContentHashId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ContentHashId> for Uuid {
    fn from(id: ContentHashId) -> Self { id.0 }
}

impl AsRef<Uuid> for ContentHashId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ContentHashId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContentHash {
    pub id: Uuid,
    pub hash: String,
    pub size_bytes: i64,
    pub storage_key: String,
    pub storage_backend: StorageBackend,
    pub reference_count: i32,
    pub first_uploaded_at: DateTime<Utc>,
    pub last_referenced_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl ContentHash {
    /// Create a builder for ContentHash
    pub fn builder() -> ContentHashBuilder {
        ContentHashBuilder::default()
    }

    /// Create a new ContentHash with required fields
    pub fn new(hash: String, size_bytes: i64, storage_key: String, storage_backend: StorageBackend, reference_count: i32, first_uploaded_at: DateTime<Utc>, last_referenced_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            hash,
            size_bytes,
            storage_key,
            storage_backend,
            reference_count,
            first_uploaded_at,
            last_referenced_at,
            fingerprint: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ContentHashId {
        ContentHashId(self.id)
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

    /// Set the fingerprint field (chainable)
    pub fn with_fingerprint(mut self, value: String) -> Self {
        self.fingerprint = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "hash" => {
                    if let Ok(v) = serde_json::from_value(value) { self.hash = v; }
                }
                "size_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.size_bytes = v; }
                }
                "storage_key" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_key = v; }
                }
                "storage_backend" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_backend = v; }
                }
                "reference_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.reference_count = v; }
                }
                "first_uploaded_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.first_uploaded_at = v; }
                }
                "last_referenced_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_referenced_at = v; }
                }
                "fingerprint" => {
                    if let Ok(v) = serde_json::from_value(value) { self.fingerprint = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Increment reference count
    pub fn increment_reference(&mut self) -> Result<(), DedupError> {
        self.reference_count += 1;
        self.last_referenced_at = Utc::now();
        self.metadata.touch();
        Ok(())
    }

    /// Decrement reference count (min 0), return true if now 0 (eligible for deletion)
    pub fn decrement_reference(&mut self) -> Result<bool, DedupError> {
        self.reference_count = (self.reference_count - 1).max(0);
        self.metadata.touch();
        Ok(self.reference_count == 0)
    }

    /// Check if content can be deleted (ref_count == 0)
    pub fn can_delete(&self) -> bool {
        self.reference_count == 0
    }

    /// Check if content has no references
    pub fn is_unused(&self) -> bool {
        self.reference_count == 0
    }

    /// Calculate storage saved by deduplication: size_bytes * (reference_count - 1)
    pub fn storage_saved(&self) -> i64 {
        self.size_bytes * (self.reference_count - 1).max(0) as i64
    }

    /// Get age in days since first upload
    pub fn age_days(&self) -> i32 {
        (Utc::now() - self.first_uploaded_at).num_days() as i32
    }

    /// Days since last reference
    pub fn days_since_last_reference(&self) -> i32 {
        (Utc::now() - self.last_referenced_at).num_days() as i32
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.reference_count < 0 {
            errors.push("reference_count must be >= 0");
        }
        if self.hash.is_empty() || self.hash.contains("..") {
            errors.push("hash must not be empty and must not contain '..'");
        }
        if self.size_bytes <= 0 {
            errors.push("size_bytes must be > 0");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for ContentHash {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "ContentHash"
    }
}

impl backbone_core::PersistentEntity for ContentHash {
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

impl backbone_orm::EntityRepoMeta for ContentHash {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("storage_backend".to_string(), "storage_backend".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["hash", "storage_key"]
    }
}

/// Builder for ContentHash entity
///
/// Provides a fluent API for constructing ContentHash instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ContentHashBuilder {
    hash: Option<String>,
    size_bytes: Option<i64>,
    storage_key: Option<String>,
    storage_backend: Option<StorageBackend>,
    reference_count: Option<i32>,
    first_uploaded_at: Option<DateTime<Utc>>,
    last_referenced_at: Option<DateTime<Utc>>,
    fingerprint: Option<String>,
}

impl ContentHashBuilder {
    /// Set the hash field (required)
    pub fn hash(mut self, value: String) -> Self {
        self.hash = Some(value);
        self
    }

    /// Set the size_bytes field (required)
    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Set the storage_key field (required)
    pub fn storage_key(mut self, value: String) -> Self {
        self.storage_key = Some(value);
        self
    }

    /// Set the storage_backend field (default: `StorageBackend::default()`)
    pub fn storage_backend(mut self, value: StorageBackend) -> Self {
        self.storage_backend = Some(value);
        self
    }

    /// Set the reference_count field (default: `1`)
    pub fn reference_count(mut self, value: i32) -> Self {
        self.reference_count = Some(value);
        self
    }

    /// Set the first_uploaded_at field (default: `Utc::now()`)
    pub fn first_uploaded_at(mut self, value: DateTime<Utc>) -> Self {
        self.first_uploaded_at = Some(value);
        self
    }

    /// Set the last_referenced_at field (default: `Utc::now()`)
    pub fn last_referenced_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_referenced_at = Some(value);
        self
    }

    /// Set the fingerprint field (optional)
    pub fn fingerprint(mut self, value: String) -> Self {
        self.fingerprint = Some(value);
        self
    }

    /// Build the ContentHash entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<ContentHash, String> {
        let hash = self.hash.ok_or_else(|| "hash is required".to_string())?;
        let size_bytes = self.size_bytes.ok_or_else(|| "size_bytes is required".to_string())?;
        let storage_key = self.storage_key.ok_or_else(|| "storage_key is required".to_string())?;

        Ok(ContentHash {
            id: Uuid::new_v4(),
            hash,
            size_bytes,
            storage_key,
            storage_backend: self.storage_backend.unwrap_or(StorageBackend::default()),
            reference_count: self.reference_count.unwrap_or(1),
            first_uploaded_at: self.first_uploaded_at.unwrap_or(Utc::now()),
            last_referenced_at: self.last_referenced_at.unwrap_or(Utc::now()),
            fingerprint: self.fingerprint,
            metadata: AuditMetadata::default(),
        })
    }
}