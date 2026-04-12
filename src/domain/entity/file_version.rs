use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::VersionType;
use super::AuditMetadata;

/// Strongly-typed ID for FileVersion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FileVersionId(pub Uuid);

impl FileVersionId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for FileVersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for FileVersionId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for FileVersionId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<FileVersionId> for Uuid {
    fn from(id: FileVersionId) -> Self { id.0 }
}

impl AsRef<Uuid> for FileVersionId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for FileVersionId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileVersion {
    pub id: Uuid,
    pub file_id: Uuid,
    pub version_number: i32,
    pub version_type: VersionType,
    pub storage_key: String,
    pub storage_backend: String,
    pub name: String,
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_sha256: Option<String>,
    pub created_by_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_summary: Option<String>,
    pub is_current: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restored_from_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    pub size_bytes: i64,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl FileVersion {
    /// Create a builder for FileVersion
    pub fn builder() -> FileVersionBuilder {
        FileVersionBuilder::default()
    }

    /// Create a new FileVersion with required fields
    pub fn new(file_id: Uuid, version_number: i32, version_type: VersionType, storage_key: String, storage_backend: String, name: String, mime_type: String, created_by_id: Uuid, is_current: bool, size_bytes: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            version_number,
            version_type,
            storage_key,
            storage_backend,
            name,
            mime_type,
            checksum_md5: None,
            checksum_sha256: None,
            created_by_id,
            change_summary: None,
            is_current,
            restored_from_id: None,
            expires_at: None,
            is_deleted: Default::default(),
            deleted_at: None,
            size_bytes,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> FileVersionId {
        FileVersionId(self.id)
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

    /// Set the checksum_md5 field (chainable)
    pub fn with_checksum_md5(mut self, value: String) -> Self {
        self.checksum_md5 = Some(value);
        self
    }

    /// Set the checksum_sha256 field (chainable)
    pub fn with_checksum_sha256(mut self, value: String) -> Self {
        self.checksum_sha256 = Some(value);
        self
    }

    /// Set the change_summary field (chainable)
    pub fn with_change_summary(mut self, value: String) -> Self {
        self.change_summary = Some(value);
        self
    }

    /// Set the restored_from_id field (chainable)
    pub fn with_restored_from_id(mut self, value: Uuid) -> Self {
        self.restored_from_id = Some(value);
        self
    }

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
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
                "version_number" => {
                    if let Ok(v) = serde_json::from_value(value) { self.version_number = v; }
                }
                "version_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.version_type = v; }
                }
                "storage_key" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_key = v; }
                }
                "storage_backend" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_backend = v; }
                }
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "mime_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mime_type = v; }
                }
                "checksum_md5" => {
                    if let Ok(v) = serde_json::from_value(value) { self.checksum_md5 = v; }
                }
                "checksum_sha256" => {
                    if let Ok(v) = serde_json::from_value(value) { self.checksum_sha256 = v; }
                }
                "created_by_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.created_by_id = v; }
                }
                "change_summary" => {
                    if let Ok(v) = serde_json::from_value(value) { self.change_summary = v; }
                }
                "is_current" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_current = v; }
                }
                "restored_from_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.restored_from_id = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "size_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.size_bytes = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for FileVersion {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "FileVersion"
    }
}

impl backbone_core::PersistentEntity for FileVersion {
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

impl backbone_orm::EntityRepoMeta for FileVersion {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("file_id".to_string(), "uuid".to_string());
        m.insert("created_by_id".to_string(), "uuid".to_string());
        m.insert("restored_from_id".to_string(), "uuid".to_string());
        m.insert("version_type".to_string(), "version_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["storage_key", "storage_backend", "name", "mime_type"]
    }
}

/// Builder for FileVersion entity
///
/// Provides a fluent API for constructing FileVersion instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct FileVersionBuilder {
    file_id: Option<Uuid>,
    version_number: Option<i32>,
    version_type: Option<VersionType>,
    storage_key: Option<String>,
    storage_backend: Option<String>,
    name: Option<String>,
    mime_type: Option<String>,
    checksum_md5: Option<String>,
    checksum_sha256: Option<String>,
    created_by_id: Option<Uuid>,
    change_summary: Option<String>,
    is_current: Option<bool>,
    restored_from_id: Option<Uuid>,
    expires_at: Option<DateTime<Utc>>,
    size_bytes: Option<i64>,
}

impl FileVersionBuilder {
    /// Set the file_id field (required)
    pub fn file_id(mut self, value: Uuid) -> Self {
        self.file_id = Some(value);
        self
    }

    /// Set the version_number field (required)
    pub fn version_number(mut self, value: i32) -> Self {
        self.version_number = Some(value);
        self
    }

    /// Set the version_type field (required)
    pub fn version_type(mut self, value: VersionType) -> Self {
        self.version_type = Some(value);
        self
    }

    /// Set the storage_key field (required)
    pub fn storage_key(mut self, value: String) -> Self {
        self.storage_key = Some(value);
        self
    }

    /// Set the storage_backend field (default: `"local".to_string()`)
    pub fn storage_backend(mut self, value: String) -> Self {
        self.storage_backend = Some(value);
        self
    }

    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the mime_type field (required)
    pub fn mime_type(mut self, value: String) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Set the checksum_md5 field (optional)
    pub fn checksum_md5(mut self, value: String) -> Self {
        self.checksum_md5 = Some(value);
        self
    }

    /// Set the checksum_sha256 field (optional)
    pub fn checksum_sha256(mut self, value: String) -> Self {
        self.checksum_sha256 = Some(value);
        self
    }

    /// Set the created_by_id field (required)
    pub fn created_by_id(mut self, value: Uuid) -> Self {
        self.created_by_id = Some(value);
        self
    }

    /// Set the change_summary field (optional)
    pub fn change_summary(mut self, value: String) -> Self {
        self.change_summary = Some(value);
        self
    }

    /// Set the is_current field (default: `false`)
    pub fn is_current(mut self, value: bool) -> Self {
        self.is_current = Some(value);
        self
    }

    /// Set the restored_from_id field (optional)
    pub fn restored_from_id(mut self, value: Uuid) -> Self {
        self.restored_from_id = Some(value);
        self
    }

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the size_bytes field (required)
    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Build the FileVersion entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<FileVersion, String> {
        let file_id = self.file_id.ok_or_else(|| "file_id is required".to_string())?;
        let version_number = self.version_number.ok_or_else(|| "version_number is required".to_string())?;
        let version_type = self.version_type.ok_or_else(|| "version_type is required".to_string())?;
        let storage_key = self.storage_key.ok_or_else(|| "storage_key is required".to_string())?;
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let mime_type = self.mime_type.ok_or_else(|| "mime_type is required".to_string())?;
        let created_by_id = self.created_by_id.ok_or_else(|| "created_by_id is required".to_string())?;
        let size_bytes = self.size_bytes.ok_or_else(|| "size_bytes is required".to_string())?;

        Ok(FileVersion {
            id: Uuid::new_v4(),
            file_id,
            version_number,
            version_type,
            storage_key,
            storage_backend: self.storage_backend.unwrap_or("local".to_string()),
            name,
            mime_type,
            checksum_md5: self.checksum_md5,
            checksum_sha256: self.checksum_sha256,
            created_by_id,
            change_summary: self.change_summary,
            is_current: self.is_current.unwrap_or(false),
            restored_from_id: self.restored_from_id,
            expires_at: self.expires_at,
            is_deleted: Default::default(),
            deleted_at: None,
            size_bytes,
            metadata: AuditMetadata::default(),
        })
    }
}
