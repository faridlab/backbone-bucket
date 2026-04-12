use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AccessAction;
use super::AuditMetadata;

/// Strongly-typed ID for AccessLog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccessLogId(pub Uuid);

impl AccessLogId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for AccessLogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AccessLogId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AccessLogId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<AccessLogId> for Uuid {
    fn from(id: AccessLogId) -> Self { id.0 }
}

impl AsRef<Uuid> for AccessLogId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for AccessLogId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessLog {
    pub id: Uuid,
    pub file_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<Uuid>,
    pub action: AccessAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<Uuid>,
    pub is_owner: bool,
    pub is_shared: bool,
    pub is_public: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i32>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    pub accessed_at: DateTime<Utc>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl AccessLog {
    /// Create a builder for AccessLog
    pub fn builder() -> AccessLogBuilder {
        AccessLogBuilder::default()
    }

    /// Create a new AccessLog with required fields
    pub fn new(file_id: Uuid, action: AccessAction, is_owner: bool, is_shared: bool, is_public: bool, success: bool, accessed_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            bucket_id: None,
            action,
            user_id: None,
            share_id: None,
            is_owner,
            is_shared,
            is_public,
            ip_address: None,
            user_agent: None,
            referer: None,
            country_code: None,
            city: None,
            bytes_transferred: None,
            duration_ms: None,
            success,
            error_message: None,
            accessed_at,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AccessLogId {
        AccessLogId(self.id)
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

    /// Set the bucket_id field (chainable)
    pub fn with_bucket_id(mut self, value: Uuid) -> Self {
        self.bucket_id = Some(value);
        self
    }

    /// Set the user_id field (chainable)
    pub fn with_user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the share_id field (chainable)
    pub fn with_share_id(mut self, value: Uuid) -> Self {
        self.share_id = Some(value);
        self
    }

    /// Set the ip_address field (chainable)
    pub fn with_ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the user_agent field (chainable)
    pub fn with_user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the referer field (chainable)
    pub fn with_referer(mut self, value: String) -> Self {
        self.referer = Some(value);
        self
    }

    /// Set the country_code field (chainable)
    pub fn with_country_code(mut self, value: String) -> Self {
        self.country_code = Some(value);
        self
    }

    /// Set the city field (chainable)
    pub fn with_city(mut self, value: String) -> Self {
        self.city = Some(value);
        self
    }

    /// Set the bytes_transferred field (chainable)
    pub fn with_bytes_transferred(mut self, value: i64) -> Self {
        self.bytes_transferred = Some(value);
        self
    }

    /// Set the duration_ms field (chainable)
    pub fn with_duration_ms(mut self, value: i32) -> Self {
        self.duration_ms = Some(value);
        self
    }

    /// Set the error_message field (chainable)
    pub fn with_error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
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
                "bucket_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bucket_id = v; }
                }
                "action" => {
                    if let Ok(v) = serde_json::from_value(value) { self.action = v; }
                }
                "user_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_id = v; }
                }
                "share_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.share_id = v; }
                }
                "is_owner" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_owner = v; }
                }
                "is_shared" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_shared = v; }
                }
                "is_public" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_public = v; }
                }
                "ip_address" => {
                    if let Ok(v) = serde_json::from_value(value) { self.ip_address = v; }
                }
                "user_agent" => {
                    if let Ok(v) = serde_json::from_value(value) { self.user_agent = v; }
                }
                "referer" => {
                    if let Ok(v) = serde_json::from_value(value) { self.referer = v; }
                }
                "country_code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.country_code = v; }
                }
                "city" => {
                    if let Ok(v) = serde_json::from_value(value) { self.city = v; }
                }
                "bytes_transferred" => {
                    if let Ok(v) = serde_json::from_value(value) { self.bytes_transferred = v; }
                }
                "duration_ms" => {
                    if let Ok(v) = serde_json::from_value(value) { self.duration_ms = v; }
                }
                "success" => {
                    if let Ok(v) = serde_json::from_value(value) { self.success = v; }
                }
                "error_message" => {
                    if let Ok(v) = serde_json::from_value(value) { self.error_message = v; }
                }
                "accessed_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.accessed_at = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for AccessLog {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "AccessLog"
    }
}

impl backbone_core::PersistentEntity for AccessLog {
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

impl backbone_orm::EntityRepoMeta for AccessLog {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("file_id".to_string(), "uuid".to_string());
        m.insert("bucket_id".to_string(), "uuid".to_string());
        m.insert("user_id".to_string(), "uuid".to_string());
        m.insert("share_id".to_string(), "uuid".to_string());
        m.insert("action".to_string(), "access_action".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for AccessLog entity
///
/// Provides a fluent API for constructing AccessLog instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AccessLogBuilder {
    file_id: Option<Uuid>,
    bucket_id: Option<Uuid>,
    action: Option<AccessAction>,
    user_id: Option<Uuid>,
    share_id: Option<Uuid>,
    is_owner: Option<bool>,
    is_shared: Option<bool>,
    is_public: Option<bool>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
    country_code: Option<String>,
    city: Option<String>,
    bytes_transferred: Option<i64>,
    duration_ms: Option<i32>,
    success: Option<bool>,
    error_message: Option<String>,
    accessed_at: Option<DateTime<Utc>>,
}

impl AccessLogBuilder {
    /// Set the file_id field (required)
    pub fn file_id(mut self, value: Uuid) -> Self {
        self.file_id = Some(value);
        self
    }

    /// Set the bucket_id field (optional)
    pub fn bucket_id(mut self, value: Uuid) -> Self {
        self.bucket_id = Some(value);
        self
    }

    /// Set the action field (required)
    pub fn action(mut self, value: AccessAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Set the user_id field (optional)
    pub fn user_id(mut self, value: Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Set the share_id field (optional)
    pub fn share_id(mut self, value: Uuid) -> Self {
        self.share_id = Some(value);
        self
    }

    /// Set the is_owner field (default: `false`)
    pub fn is_owner(mut self, value: bool) -> Self {
        self.is_owner = Some(value);
        self
    }

    /// Set the is_shared field (default: `false`)
    pub fn is_shared(mut self, value: bool) -> Self {
        self.is_shared = Some(value);
        self
    }

    /// Set the is_public field (default: `false`)
    pub fn is_public(mut self, value: bool) -> Self {
        self.is_public = Some(value);
        self
    }

    /// Set the ip_address field (optional)
    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    /// Set the user_agent field (optional)
    pub fn user_agent(mut self, value: String) -> Self {
        self.user_agent = Some(value);
        self
    }

    /// Set the referer field (optional)
    pub fn referer(mut self, value: String) -> Self {
        self.referer = Some(value);
        self
    }

    /// Set the country_code field (optional)
    pub fn country_code(mut self, value: String) -> Self {
        self.country_code = Some(value);
        self
    }

    /// Set the city field (optional)
    pub fn city(mut self, value: String) -> Self {
        self.city = Some(value);
        self
    }

    /// Set the bytes_transferred field (optional)
    pub fn bytes_transferred(mut self, value: i64) -> Self {
        self.bytes_transferred = Some(value);
        self
    }

    /// Set the duration_ms field (optional)
    pub fn duration_ms(mut self, value: i32) -> Self {
        self.duration_ms = Some(value);
        self
    }

    /// Set the success field (default: `true`)
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Set the error_message field (optional)
    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    /// Set the accessed_at field (default: `Utc::now()`)
    pub fn accessed_at(mut self, value: DateTime<Utc>) -> Self {
        self.accessed_at = Some(value);
        self
    }

    /// Build the AccessLog entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<AccessLog, String> {
        let file_id = self.file_id.ok_or_else(|| "file_id is required".to_string())?;
        let action = self.action.ok_or_else(|| "action is required".to_string())?;

        Ok(AccessLog {
            id: Uuid::new_v4(),
            file_id,
            bucket_id: self.bucket_id,
            action,
            user_id: self.user_id,
            share_id: self.share_id,
            is_owner: self.is_owner.unwrap_or(false),
            is_shared: self.is_shared.unwrap_or(false),
            is_public: self.is_public.unwrap_or(false),
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            referer: self.referer,
            country_code: self.country_code,
            city: self.city,
            bytes_transferred: self.bytes_transferred,
            duration_ms: self.duration_ms,
            success: self.success.unwrap_or(true),
            error_message: self.error_message,
            accessed_at: self.accessed_at.unwrap_or(Utc::now()),
            metadata: AuditMetadata::default(),
        })
    }
}
