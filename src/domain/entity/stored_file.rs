use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ThreatLevel;
use super::ProcessingStatus;
use super::FileStatus;
use super::AuditMetadata;

use super::*;

use crate::domain::state_machine::{StoredFileStateMachine, StoredFileState, StateMachineError};
use backbone_core::state_machine::StateMachineBehavior;

/// Strongly-typed ID for StoredFile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StoredFileId(pub Uuid);

impl StoredFileId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for StoredFileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for StoredFileId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for StoredFileId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<StoredFileId> for Uuid {
    fn from(id: StoredFileId) -> Self { id.0 }
}

impl AsRef<Uuid> for StoredFileId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for StoredFileId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoredFile {
    pub id: Uuid,
    pub bucket_id: Uuid,
    pub owner_id: Uuid,
    pub path: String,
    pub original_name: String,
    pub size_bytes: i64,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    pub is_compressed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_algorithm: Option<String>,
    pub is_scanned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_level: Option<ThreatLevel>,
    pub has_thumbnail: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
    pub has_video_thumbnail: bool,
    pub has_document_preview: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<ProcessingStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_url_expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_module: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_entity_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    pub sort_order: i32,
    pub(crate) status: FileStatus,
    pub storage_key: String,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<Uuid>,
    pub download_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl StoredFile {
    /// Create a builder for StoredFile
    pub fn builder() -> StoredFileBuilder {
        StoredFileBuilder::default()
    }

    /// Create a new StoredFile with required fields
    pub fn new(bucket_id: Uuid, owner_id: Uuid, path: String, original_name: String, size_bytes: i64, mime_type: String, is_compressed: bool, is_scanned: bool, has_thumbnail: bool, has_video_thumbnail: bool, has_document_preview: bool, sort_order: i32, status: FileStatus, storage_key: String, version: i32, download_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            bucket_id,
            owner_id,
            path,
            original_name,
            size_bytes,
            mime_type,
            checksum: None,
            is_compressed,
            original_size: None,
            compression_algorithm: None,
            is_scanned,
            scan_result: None,
            threat_level: None,
            has_thumbnail,
            thumbnail_path: None,
            has_video_thumbnail,
            has_document_preview,
            processing_status: None,
            content_hash_id: None,
            cdn_url: None,
            cdn_url_expires_at: None,
            owner_module: None,
            owner_entity: None,
            owner_entity_id: None,
            field_name: None,
            sort_order,
            status,
            storage_key,
            version,
            previous_version_id: None,
            download_count,
            last_accessed_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> StoredFileId {
        StoredFileId(self.id)
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
    pub fn status(&self) -> &FileStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the checksum field (chainable)
    pub fn with_checksum(mut self, value: String) -> Self {
        self.checksum = Some(value);
        self
    }

    /// Set the original_size field (chainable)
    pub fn with_original_size(mut self, value: i64) -> Self {
        self.original_size = Some(value);
        self
    }

    /// Set the compression_algorithm field (chainable)
    pub fn with_compression_algorithm(mut self, value: String) -> Self {
        self.compression_algorithm = Some(value);
        self
    }

    /// Set the scan_result field (chainable)
    pub fn with_scan_result(mut self, value: serde_json::Value) -> Self {
        self.scan_result = Some(value);
        self
    }

    /// Set the threat_level field (chainable)
    pub fn with_threat_level(mut self, value: ThreatLevel) -> Self {
        self.threat_level = Some(value);
        self
    }

    /// Set the thumbnail_path field (chainable)
    pub fn with_thumbnail_path(mut self, value: String) -> Self {
        self.thumbnail_path = Some(value);
        self
    }

    /// Set the processing_status field (chainable)
    pub fn with_processing_status(mut self, value: ProcessingStatus) -> Self {
        self.processing_status = Some(value);
        self
    }

    /// Set the content_hash_id field (chainable)
    pub fn with_content_hash_id(mut self, value: Uuid) -> Self {
        self.content_hash_id = Some(value);
        self
    }

    /// Set the cdn_url field (chainable)
    pub fn with_cdn_url(mut self, value: String) -> Self {
        self.cdn_url = Some(value);
        self
    }

    /// Set the cdn_url_expires_at field (chainable)
    pub fn with_cdn_url_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.cdn_url_expires_at = Some(value);
        self
    }

    /// Set the owner_module field (chainable)
    pub fn with_owner_module(mut self, value: String) -> Self {
        self.owner_module = Some(value);
        self
    }

    /// Set the owner_entity field (chainable)
    pub fn with_owner_entity(mut self, value: String) -> Self {
        self.owner_entity = Some(value);
        self
    }

    /// Set the owner_entity_id field (chainable)
    pub fn with_owner_entity_id(mut self, value: Uuid) -> Self {
        self.owner_entity_id = Some(value);
        self
    }

    /// Set the field_name field (chainable)
    pub fn with_field_name(mut self, value: String) -> Self {
        self.field_name = Some(value);
        self
    }

    /// Set the previous_version_id field (chainable)
    pub fn with_previous_version_id(mut self, value: Uuid) -> Self {
        self.previous_version_id = Some(value);
        self
    }

    /// Set the last_accessed_at field (chainable)
    pub fn with_last_accessed_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_accessed_at = Some(value);
        self
    }

    // ==========================================================
    // State Machine
    // ==========================================================

    /// Transition to a new state via the status state machine.
    ///
    /// Returns `Err` if the transition is not permitted from the current state.
    /// Use this method instead of assigning `self.status` directly.
    pub fn transition_to(&mut self, new_state: StoredFileState) -> Result<(), StateMachineError> {
        let current = self.status.to_string().parse::<StoredFileState>()?;
        let mut sm = StoredFileStateMachine::from_state(current);
        sm.transition_to_state(new_state)?;
        self.status = new_state.to_string().parse::<FileStatus>()
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
                "owner_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.owner_id = v; }
                }
                "path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.path = v; }
                }
                "original_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.original_name = v; }
                }
                "size_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.size_bytes = v; }
                }
                "mime_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mime_type = v; }
                }
                "checksum" => {
                    if let Ok(v) = serde_json::from_value(value) { self.checksum = v; }
                }
                "is_compressed" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_compressed = v; }
                }
                "original_size" => {
                    if let Ok(v) = serde_json::from_value(value) { self.original_size = v; }
                }
                "compression_algorithm" => {
                    if let Ok(v) = serde_json::from_value(value) { self.compression_algorithm = v; }
                }
                "is_scanned" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_scanned = v; }
                }
                "scan_result" => {
                    if let Ok(v) = serde_json::from_value(value) { self.scan_result = v; }
                }
                "threat_level" => {
                    if let Ok(v) = serde_json::from_value(value) { self.threat_level = v; }
                }
                "has_thumbnail" => {
                    if let Ok(v) = serde_json::from_value(value) { self.has_thumbnail = v; }
                }
                "thumbnail_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.thumbnail_path = v; }
                }
                "has_video_thumbnail" => {
                    if let Ok(v) = serde_json::from_value(value) { self.has_video_thumbnail = v; }
                }
                "has_document_preview" => {
                    if let Ok(v) = serde_json::from_value(value) { self.has_document_preview = v; }
                }
                "processing_status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.processing_status = v; }
                }
                "content_hash_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.content_hash_id = v; }
                }
                "cdn_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cdn_url = v; }
                }
                "cdn_url_expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cdn_url_expires_at = v; }
                }
                "owner_module" => {
                    if let Ok(v) = serde_json::from_value(value) { self.owner_module = v; }
                }
                "owner_entity" => {
                    if let Ok(v) = serde_json::from_value(value) { self.owner_entity = v; }
                }
                "owner_entity_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.owner_entity_id = v; }
                }
                "field_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.field_name = v; }
                }
                "sort_order" => {
                    if let Ok(v) = serde_json::from_value(value) { self.sort_order = v; }
                }
                "storage_key" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_key = v; }
                }
                "version" => {
                    if let Ok(v) = serde_json::from_value(value) { self.version = v; }
                }
                "previous_version_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.previous_version_id = v; }
                }
                "download_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.download_count = v; }
                }
                "last_accessed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_accessed_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if file is accessible (active and not deleted)
    pub fn is_accessible(&self) -> bool {
        !self.is_deleted() && self.status == FileStatus::Active
    }

    /// Check if file passed security scan
    pub fn is_safe(&self) -> bool {
        self.is_scanned
            && match &self.threat_level {
                None => true,
                Some(ThreatLevel::Safe) => true,
                _ => false,
            }
    }

    /// Check if file needs processing (not yet scanned)
    pub fn needs_processing(&self) -> bool {
        !self.is_scanned
    }

    /// Increment download count and update last_accessed_at
    pub fn record_access(&mut self) {
        self.download_count += 1;
        self.last_accessed_at = Some(Utc::now());
    }

    /// Mark file as deleted (move to trash)
    pub fn soft_delete(&mut self) {
        self.status = FileStatus::Deleted;
        self.metadata.deleted_at = Some(Utc::now());
    }

    /// Restore file from trash
    pub fn restore(&mut self) {
        self.status = FileStatus::Active;
        self.metadata.deleted_at = None;
    }

    /// Mark file as quarantined due to threats
    pub fn quarantine(&mut self, threats: Vec<String>) {
        self.status = FileStatus::Quarantined;
        self.scan_result = Some(serde_json::json!(threats));
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        if self.size_bytes <= 0 {
            errors.push("size_bytes must be greater than 0");
        }

        if self.path.contains("..") {
            errors.push("path must not contain '..' (path traversal)");
        }

        if self.original_name.is_empty() {
            errors.push("original_name must not be empty");
        }

        if self.storage_key.is_empty() {
            errors.push("storage_key must not be empty");
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for StoredFile {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "StoredFile"
    }
}

impl backbone_core::PersistentEntity for StoredFile {
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

impl backbone_orm::EntityRepoMeta for StoredFile {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("bucket_id".to_string(), "uuid".to_string());
        m.insert("owner_id".to_string(), "uuid".to_string());
        m.insert("content_hash_id".to_string(), "uuid".to_string());
        m.insert("owner_entity_id".to_string(), "uuid".to_string());
        m.insert("previous_version_id".to_string(), "uuid".to_string());
        m.insert("threat_level".to_string(), "threat_level".to_string());
        m.insert("processing_status".to_string(), "processing_status".to_string());
        m.insert("status".to_string(), "file_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["path", "original_name", "mime_type", "storage_key"]
    }
}

/// Builder for StoredFile entity
///
/// Provides a fluent API for constructing StoredFile instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct StoredFileBuilder {
    bucket_id: Option<Uuid>,
    owner_id: Option<Uuid>,
    path: Option<String>,
    original_name: Option<String>,
    size_bytes: Option<i64>,
    mime_type: Option<String>,
    checksum: Option<String>,
    is_compressed: Option<bool>,
    original_size: Option<i64>,
    compression_algorithm: Option<String>,
    is_scanned: Option<bool>,
    scan_result: Option<serde_json::Value>,
    threat_level: Option<ThreatLevel>,
    has_thumbnail: Option<bool>,
    thumbnail_path: Option<String>,
    has_video_thumbnail: Option<bool>,
    has_document_preview: Option<bool>,
    processing_status: Option<ProcessingStatus>,
    content_hash_id: Option<Uuid>,
    cdn_url: Option<String>,
    cdn_url_expires_at: Option<DateTime<Utc>>,
    owner_module: Option<String>,
    owner_entity: Option<String>,
    owner_entity_id: Option<Uuid>,
    field_name: Option<String>,
    sort_order: Option<i32>,
    status: Option<FileStatus>,
    storage_key: Option<String>,
    version: Option<i32>,
    previous_version_id: Option<Uuid>,
    download_count: Option<i32>,
    last_accessed_at: Option<DateTime<Utc>>,
}

impl StoredFileBuilder {
    /// Set the bucket_id field (required)
    pub fn bucket_id(mut self, value: Uuid) -> Self {
        self.bucket_id = Some(value);
        self
    }

    /// Set the owner_id field (required)
    pub fn owner_id(mut self, value: Uuid) -> Self {
        self.owner_id = Some(value);
        self
    }

    /// Set the path field (required)
    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
        self
    }

    /// Set the original_name field (required)
    pub fn original_name(mut self, value: String) -> Self {
        self.original_name = Some(value);
        self
    }

    /// Set the size_bytes field (required)
    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Set the mime_type field (required)
    pub fn mime_type(mut self, value: String) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Set the checksum field (optional)
    pub fn checksum(mut self, value: String) -> Self {
        self.checksum = Some(value);
        self
    }

    /// Set the is_compressed field (default: `false`)
    pub fn is_compressed(mut self, value: bool) -> Self {
        self.is_compressed = Some(value);
        self
    }

    /// Set the original_size field (optional)
    pub fn original_size(mut self, value: i64) -> Self {
        self.original_size = Some(value);
        self
    }

    /// Set the compression_algorithm field (optional)
    pub fn compression_algorithm(mut self, value: String) -> Self {
        self.compression_algorithm = Some(value);
        self
    }

    /// Set the is_scanned field (default: `false`)
    pub fn is_scanned(mut self, value: bool) -> Self {
        self.is_scanned = Some(value);
        self
    }

    /// Set the scan_result field (optional)
    pub fn scan_result(mut self, value: serde_json::Value) -> Self {
        self.scan_result = Some(value);
        self
    }

    /// Set the threat_level field (optional)
    pub fn threat_level(mut self, value: ThreatLevel) -> Self {
        self.threat_level = Some(value);
        self
    }

    /// Set the has_thumbnail field (default: `false`)
    pub fn has_thumbnail(mut self, value: bool) -> Self {
        self.has_thumbnail = Some(value);
        self
    }

    /// Set the thumbnail_path field (optional)
    pub fn thumbnail_path(mut self, value: String) -> Self {
        self.thumbnail_path = Some(value);
        self
    }

    /// Set the has_video_thumbnail field (default: `false`)
    pub fn has_video_thumbnail(mut self, value: bool) -> Self {
        self.has_video_thumbnail = Some(value);
        self
    }

    /// Set the has_document_preview field (default: `false`)
    pub fn has_document_preview(mut self, value: bool) -> Self {
        self.has_document_preview = Some(value);
        self
    }

    /// Set the processing_status field (optional)
    pub fn processing_status(mut self, value: ProcessingStatus) -> Self {
        self.processing_status = Some(value);
        self
    }

    /// Set the content_hash_id field (optional)
    pub fn content_hash_id(mut self, value: Uuid) -> Self {
        self.content_hash_id = Some(value);
        self
    }

    /// Set the cdn_url field (optional)
    pub fn cdn_url(mut self, value: String) -> Self {
        self.cdn_url = Some(value);
        self
    }

    /// Set the cdn_url_expires_at field (optional)
    pub fn cdn_url_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.cdn_url_expires_at = Some(value);
        self
    }

    /// Set the owner_module field (optional)
    pub fn owner_module(mut self, value: String) -> Self {
        self.owner_module = Some(value);
        self
    }

    /// Set the owner_entity field (optional)
    pub fn owner_entity(mut self, value: String) -> Self {
        self.owner_entity = Some(value);
        self
    }

    /// Set the owner_entity_id field (optional)
    pub fn owner_entity_id(mut self, value: Uuid) -> Self {
        self.owner_entity_id = Some(value);
        self
    }

    /// Set the field_name field (optional)
    pub fn field_name(mut self, value: String) -> Self {
        self.field_name = Some(value);
        self
    }

    /// Set the sort_order field (default: `0`)
    pub fn sort_order(mut self, value: i32) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Set the status field (default: `FileStatus::default()`)
    pub fn status(mut self, value: FileStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the storage_key field (required)
    pub fn storage_key(mut self, value: String) -> Self {
        self.storage_key = Some(value);
        self
    }

    /// Set the version field (default: `1`)
    pub fn version(mut self, value: i32) -> Self {
        self.version = Some(value);
        self
    }

    /// Set the previous_version_id field (optional)
    pub fn previous_version_id(mut self, value: Uuid) -> Self {
        self.previous_version_id = Some(value);
        self
    }

    /// Set the download_count field (default: `0`)
    pub fn download_count(mut self, value: i32) -> Self {
        self.download_count = Some(value);
        self
    }

    /// Set the last_accessed_at field (optional)
    pub fn last_accessed_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_accessed_at = Some(value);
        self
    }

    /// Build the StoredFile entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<StoredFile, String> {
        let bucket_id = self.bucket_id.ok_or_else(|| "bucket_id is required".to_string())?;
        let owner_id = self.owner_id.ok_or_else(|| "owner_id is required".to_string())?;
        let path = self.path.ok_or_else(|| "path is required".to_string())?;
        let original_name = self.original_name.ok_or_else(|| "original_name is required".to_string())?;
        let size_bytes = self.size_bytes.ok_or_else(|| "size_bytes is required".to_string())?;
        let mime_type = self.mime_type.ok_or_else(|| "mime_type is required".to_string())?;
        let storage_key = self.storage_key.ok_or_else(|| "storage_key is required".to_string())?;

        Ok(StoredFile {
            id: Uuid::new_v4(),
            bucket_id,
            owner_id,
            path,
            original_name,
            size_bytes,
            mime_type,
            checksum: self.checksum,
            is_compressed: self.is_compressed.unwrap_or(false),
            original_size: self.original_size,
            compression_algorithm: self.compression_algorithm,
            is_scanned: self.is_scanned.unwrap_or(false),
            scan_result: self.scan_result,
            threat_level: self.threat_level,
            has_thumbnail: self.has_thumbnail.unwrap_or(false),
            thumbnail_path: self.thumbnail_path,
            has_video_thumbnail: self.has_video_thumbnail.unwrap_or(false),
            has_document_preview: self.has_document_preview.unwrap_or(false),
            processing_status: self.processing_status,
            content_hash_id: self.content_hash_id,
            cdn_url: self.cdn_url,
            cdn_url_expires_at: self.cdn_url_expires_at,
            owner_module: self.owner_module,
            owner_entity: self.owner_entity,
            owner_entity_id: self.owner_entity_id,
            field_name: self.field_name,
            sort_order: self.sort_order.unwrap_or(0),
            status: self.status.unwrap_or(FileStatus::default()),
            storage_key,
            version: self.version.unwrap_or(1),
            previous_version_id: self.previous_version_id,
            download_count: self.download_count.unwrap_or(0),
            last_accessed_at: self.last_accessed_at,
            metadata: AuditMetadata::default(),
        })
    }
}