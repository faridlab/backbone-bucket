//! Access Logger Service
//!
//! Provides file access logging capabilities for audit trails.
//! Logs downloads, views, shares, and other file access events.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entity::AccessAction;

/// A file access log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccessLogEntry {
    pub id: Uuid,
    pub file_id: Uuid,
    pub user_id: Option<Uuid>,
    pub share_id: Option<Uuid>,
    pub action: AccessAction,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub accessed_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

impl FileAccessLogEntry {
    /// Create a new log entry
    pub fn new(
        file_id: Uuid,
        action: AccessAction,
        user_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            user_id,
            share_id: None,
            action,
            ip_address: None,
            user_agent: None,
            referer: None,
            accessed_at: Utc::now(),
            metadata: serde_json::json!({}),
        }
    }

    /// Set the share ID
    pub fn with_share(mut self, share_id: Uuid) -> Self {
        self.share_id = Some(share_id);
        self
    }

    /// Set the IP address
    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip_address = Some(ip);
        self
    }

    /// Set the user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Set the referer
    pub fn with_referer(mut self, referer: String) -> Self {
        self.referer = Some(referer);
        self
    }

    /// Set additional metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

/// Access Logger Service
///
/// Creates access log entries for file operations.
/// These entries can be persisted using the repository layer.
#[derive(Debug, Clone, Default)]
pub struct AccessLoggerService;

impl AccessLoggerService {
    /// Create a new access logger service
    pub fn new() -> Self {
        Self
    }

    /// Log a file download
    pub fn log_download(
        &self,
        file_id: Uuid,
        user_id: Option<Uuid>,
        share_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> FileAccessLogEntry {
        let mut entry = FileAccessLogEntry::new(file_id, AccessAction::Download, user_id);

        if let Some(sid) = share_id {
            entry = entry.with_share(sid);
        }
        if let Some(ip) = ip_address {
            entry = entry.with_ip(ip);
        }
        if let Some(ua) = user_agent {
            entry = entry.with_user_agent(ua);
        }

        entry
    }

    /// Log a file view (preview)
    pub fn log_view(
        &self,
        file_id: Uuid,
        user_id: Option<Uuid>,
        share_id: Option<Uuid>,
    ) -> FileAccessLogEntry {
        let mut entry = FileAccessLogEntry::new(file_id, AccessAction::View, user_id);

        if let Some(sid) = share_id {
            entry = entry.with_share(sid);
        }

        entry
    }

    /// Log a file upload
    pub fn log_upload(
        &self,
        file_id: Uuid,
        user_id: Uuid,
        file_size: i64,
        mime_type: &str,
    ) -> FileAccessLogEntry {
        FileAccessLogEntry::new(file_id, AccessAction::Upload, Some(user_id))
            .with_metadata(serde_json::json!({
                "file_size": file_size,
                "mime_type": mime_type,
            }))
    }

    /// Log a file deletion
    pub fn log_delete(
        &self,
        file_id: Uuid,
        user_id: Uuid,
        soft_delete: bool,
    ) -> FileAccessLogEntry {
        FileAccessLogEntry::new(file_id, AccessAction::Delete, Some(user_id))
            .with_metadata(serde_json::json!({
                "soft_delete": soft_delete,
            }))
    }

    /// Log a file restore from trash
    pub fn log_restore(
        &self,
        file_id: Uuid,
        user_id: Uuid,
    ) -> FileAccessLogEntry {
        FileAccessLogEntry::new(file_id, AccessAction::Restore, Some(user_id))
    }

    /// Log a file share creation
    pub fn log_share(
        &self,
        file_id: Uuid,
        user_id: Uuid,
        share_id: Uuid,
        share_type: &str,
    ) -> FileAccessLogEntry {
        FileAccessLogEntry::new(file_id, AccessAction::Share, Some(user_id))
            .with_share(share_id)
            .with_metadata(serde_json::json!({
                "share_type": share_type,
            }))
    }

    /// Log a share access (someone used a share link)
    pub fn log_share_access(
        &self,
        file_id: Uuid,
        share_id: Uuid,
        user_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> FileAccessLogEntry {
        let mut entry = FileAccessLogEntry::new(file_id, AccessAction::ShareAccess, user_id)
            .with_share(share_id);

        if let Some(ip) = ip_address {
            entry = entry.with_ip(ip);
        }
        if let Some(ua) = user_agent {
            entry = entry.with_user_agent(ua);
        }

        entry
    }

    /// Log a file move
    pub fn log_move(
        &self,
        file_id: Uuid,
        user_id: Uuid,
        from_path: &str,
        to_path: &str,
    ) -> FileAccessLogEntry {
        FileAccessLogEntry::new(file_id, AccessAction::Move, Some(user_id))
            .with_metadata(serde_json::json!({
                "from_path": from_path,
                "to_path": to_path,
            }))
    }

    /// Log a file copy
    pub fn log_copy(
        &self,
        original_file_id: Uuid,
        new_file_id: Uuid,
        user_id: Uuid,
    ) -> FileAccessLogEntry {
        FileAccessLogEntry::new(original_file_id, AccessAction::Copy, Some(user_id))
            .with_metadata(serde_json::json!({
                "new_file_id": new_file_id.to_string(),
            }))
    }

    /// Log a metadata update
    pub fn log_metadata_update(
        &self,
        file_id: Uuid,
        user_id: Uuid,
        changes: serde_json::Value,
    ) -> FileAccessLogEntry {
        FileAccessLogEntry::new(file_id, AccessAction::MetadataUpdate, Some(user_id))
            .with_metadata(serde_json::json!({
                "changes": changes,
            }))
    }
}

/// Builder for creating access log entries with all details
#[derive(Debug, Clone)]
pub struct AccessLogBuilder {
    file_id: Uuid,
    action: AccessAction,
    user_id: Option<Uuid>,
    share_id: Option<Uuid>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
    metadata: serde_json::Value,
}

impl AccessLogBuilder {
    /// Create a new builder
    pub fn new(file_id: Uuid, action: AccessAction) -> Self {
        Self {
            file_id,
            action,
            user_id: None,
            share_id: None,
            ip_address: None,
            user_agent: None,
            referer: None,
            metadata: serde_json::json!({}),
        }
    }

    /// Set the user ID
    pub fn user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set the share ID
    pub fn share(mut self, share_id: Uuid) -> Self {
        self.share_id = Some(share_id);
        self
    }

    /// Set the IP address
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    /// Set the user agent
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Set the referer
    pub fn referer(mut self, referer: impl Into<String>) -> Self {
        self.referer = Some(referer.into());
        self
    }

    /// Set metadata
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    /// Build the log entry
    pub fn build(self) -> FileAccessLogEntry {
        FileAccessLogEntry {
            id: Uuid::new_v4(),
            file_id: self.file_id,
            user_id: self.user_id,
            share_id: self.share_id,
            action: self.action,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            referer: self.referer,
            accessed_at: Utc::now(),
            metadata: self.metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_download() {
        let logger = AccessLoggerService::new();
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let entry = logger.log_download(
            file_id,
            Some(user_id),
            None,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        assert_eq!(entry.file_id, file_id);
        assert_eq!(entry.user_id, Some(user_id));
        assert_eq!(entry.action, AccessAction::Download);
        assert_eq!(entry.ip_address, Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_log_upload() {
        let logger = AccessLoggerService::new();
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let entry = logger.log_upload(file_id, user_id, 1024, "text/plain");

        assert_eq!(entry.file_id, file_id);
        assert_eq!(entry.action, AccessAction::Upload);
        assert_eq!(entry.metadata["file_size"], 1024);
        assert_eq!(entry.metadata["mime_type"], "text/plain");
    }

    #[test]
    fn test_builder() {
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let entry = AccessLogBuilder::new(file_id, AccessAction::Download)
            .user(user_id)
            .ip("10.0.0.1")
            .user_agent("TestClient/1.0")
            .build();

        assert_eq!(entry.file_id, file_id);
        assert_eq!(entry.user_id, Some(user_id));
        assert_eq!(entry.ip_address, Some("10.0.0.1".to_string()));
    }
}
