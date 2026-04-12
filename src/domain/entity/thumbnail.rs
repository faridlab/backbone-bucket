use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ThumbnailSize;
use super::AuditMetadata;

/// Strongly-typed ID for Thumbnail
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ThumbnailId(pub Uuid);

impl ThumbnailId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for ThumbnailId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ThumbnailId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for ThumbnailId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<ThumbnailId> for Uuid {
    fn from(id: ThumbnailId) -> Self { id.0 }
}

impl AsRef<Uuid> for ThumbnailId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for ThumbnailId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Thumbnail {
    pub id: Uuid,
    pub file_id: Uuid,
    pub size: ThumbnailSize,
    pub width: i32,
    pub height: i32,
    pub storage_key: String,
    pub storage_backend: String,
    pub mime_type: String,
    pub format: String,
    pub quality: i32,
    pub size_bytes: i64,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_time_ms: Option<i32>,
    pub source_version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expires_at: Option<DateTime<Utc>>,
    pub is_stale: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Thumbnail {
    /// Create a builder for Thumbnail
    pub fn builder() -> ThumbnailBuilder {
        ThumbnailBuilder::default()
    }

    /// Create a new Thumbnail with required fields
    pub fn new(file_id: Uuid, size: ThumbnailSize, width: i32, height: i32, storage_key: String, storage_backend: String, mime_type: String, format: String, quality: i32, size_bytes: i64, generated_at: DateTime<Utc>, source_version: i32, is_stale: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            size,
            width,
            height,
            storage_key,
            storage_backend,
            mime_type,
            format,
            quality,
            size_bytes,
            generated_at,
            generation_time_ms: None,
            source_version,
            cdn_url: None,
            cache_expires_at: None,
            is_stale,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> ThumbnailId {
        ThumbnailId(self.id)
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

    /// Set the generation_time_ms field (chainable)
    pub fn with_generation_time_ms(mut self, value: i32) -> Self {
        self.generation_time_ms = Some(value);
        self
    }

    /// Set the cdn_url field (chainable)
    pub fn with_cdn_url(mut self, value: String) -> Self {
        self.cdn_url = Some(value);
        self
    }

    /// Set the cache_expires_at field (chainable)
    pub fn with_cache_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.cache_expires_at = Some(value);
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
                "size" => {
                    if let Ok(v) = serde_json::from_value(value) { self.size = v; }
                }
                "width" => {
                    if let Ok(v) = serde_json::from_value(value) { self.width = v; }
                }
                "height" => {
                    if let Ok(v) = serde_json::from_value(value) { self.height = v; }
                }
                "storage_key" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_key = v; }
                }
                "storage_backend" => {
                    if let Ok(v) = serde_json::from_value(value) { self.storage_backend = v; }
                }
                "mime_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.mime_type = v; }
                }
                "format" => {
                    if let Ok(v) = serde_json::from_value(value) { self.format = v; }
                }
                "quality" => {
                    if let Ok(v) = serde_json::from_value(value) { self.quality = v; }
                }
                "size_bytes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.size_bytes = v; }
                }
                "generated_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generated_at = v; }
                }
                "generation_time_ms" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generation_time_ms = v; }
                }
                "source_version" => {
                    if let Ok(v) = serde_json::from_value(value) { self.source_version = v; }
                }
                "cdn_url" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cdn_url = v; }
                }
                "cache_expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cache_expires_at = v; }
                }
                "is_stale" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_stale = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Thumbnail {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Thumbnail"
    }
}

impl backbone_core::PersistentEntity for Thumbnail {
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

impl backbone_orm::EntityRepoMeta for Thumbnail {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("file_id".to_string(), "uuid".to_string());
        m.insert("size".to_string(), "thumbnail_size".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["storage_key", "storage_backend", "mime_type", "format"]
    }
}

/// Builder for Thumbnail entity
///
/// Provides a fluent API for constructing Thumbnail instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct ThumbnailBuilder {
    file_id: Option<Uuid>,
    size: Option<ThumbnailSize>,
    width: Option<i32>,
    height: Option<i32>,
    storage_key: Option<String>,
    storage_backend: Option<String>,
    mime_type: Option<String>,
    format: Option<String>,
    quality: Option<i32>,
    size_bytes: Option<i64>,
    generated_at: Option<DateTime<Utc>>,
    generation_time_ms: Option<i32>,
    source_version: Option<i32>,
    cdn_url: Option<String>,
    cache_expires_at: Option<DateTime<Utc>>,
    is_stale: Option<bool>,
}

impl ThumbnailBuilder {
    /// Set the file_id field (required)
    pub fn file_id(mut self, value: Uuid) -> Self {
        self.file_id = Some(value);
        self
    }

    /// Set the size field (required)
    pub fn size(mut self, value: ThumbnailSize) -> Self {
        self.size = Some(value);
        self
    }

    /// Set the width field (required)
    pub fn width(mut self, value: i32) -> Self {
        self.width = Some(value);
        self
    }

    /// Set the height field (required)
    pub fn height(mut self, value: i32) -> Self {
        self.height = Some(value);
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

    /// Set the mime_type field (default: `"image/webp".to_string()`)
    pub fn mime_type(mut self, value: String) -> Self {
        self.mime_type = Some(value);
        self
    }

    /// Set the format field (default: `"webp".to_string()`)
    pub fn format(mut self, value: String) -> Self {
        self.format = Some(value);
        self
    }

    /// Set the quality field (default: `80`)
    pub fn quality(mut self, value: i32) -> Self {
        self.quality = Some(value);
        self
    }

    /// Set the size_bytes field (required)
    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Set the generated_at field (default: `Utc::now()`)
    pub fn generated_at(mut self, value: DateTime<Utc>) -> Self {
        self.generated_at = Some(value);
        self
    }

    /// Set the generation_time_ms field (optional)
    pub fn generation_time_ms(mut self, value: i32) -> Self {
        self.generation_time_ms = Some(value);
        self
    }

    /// Set the source_version field (default: `1`)
    pub fn source_version(mut self, value: i32) -> Self {
        self.source_version = Some(value);
        self
    }

    /// Set the cdn_url field (optional)
    pub fn cdn_url(mut self, value: String) -> Self {
        self.cdn_url = Some(value);
        self
    }

    /// Set the cache_expires_at field (optional)
    pub fn cache_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.cache_expires_at = Some(value);
        self
    }

    /// Set the is_stale field (default: `false`)
    pub fn is_stale(mut self, value: bool) -> Self {
        self.is_stale = Some(value);
        self
    }

    /// Build the Thumbnail entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Thumbnail, String> {
        let file_id = self.file_id.ok_or_else(|| "file_id is required".to_string())?;
        let size = self.size.ok_or_else(|| "size is required".to_string())?;
        let width = self.width.ok_or_else(|| "width is required".to_string())?;
        let height = self.height.ok_or_else(|| "height is required".to_string())?;
        let storage_key = self.storage_key.ok_or_else(|| "storage_key is required".to_string())?;
        let size_bytes = self.size_bytes.ok_or_else(|| "size_bytes is required".to_string())?;

        Ok(Thumbnail {
            id: Uuid::new_v4(),
            file_id,
            size,
            width,
            height,
            storage_key,
            storage_backend: self.storage_backend.unwrap_or("local".to_string()),
            mime_type: self.mime_type.unwrap_or("image/webp".to_string()),
            format: self.format.unwrap_or("webp".to_string()),
            quality: self.quality.unwrap_or(80),
            size_bytes,
            generated_at: self.generated_at.unwrap_or(Utc::now()),
            generation_time_ms: self.generation_time_ms,
            source_version: self.source_version.unwrap_or(1),
            cdn_url: self.cdn_url,
            cache_expires_at: self.cache_expires_at,
            is_stale: self.is_stale.unwrap_or(false),
            metadata: AuditMetadata::default(),
        })
    }
}
