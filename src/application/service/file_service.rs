//! File upload service on top of [`ObjectStorage`].
//!
//! The plan calls for two upload surfaces:
//!
//! - [`FileService::upload`] — auto-generated UUID key (existing behavior).
//! - [`FileService::upload_with_key`] — caller-controlled key, used when
//!   the key is human-visible (e.g. mode-B public paths like
//!   `public/product/image/slug.jpg`).
//!
//! Public-prefix routing is embedded here: keys beginning with
//! `config.serving.public_prefix` route to the configured public bucket
//! inside the [`ObjectStorage`] impl (if any), otherwise private.

use std::sync::Arc;

use bytes::Bytes;
use chrono::Utc;
use uuid::Uuid;

use crate::config::BucketConfig;
use crate::domain::entity::{AuditMetadata, FileStatus, StoredFile};
use crate::error::{BucketError, BucketResult};
use crate::infrastructure::persistence::StoredFileRepository;
use crate::storage::ObjectStorage;

/// Caller-supplied metadata for a new file.
#[derive(Debug, Clone)]
pub struct FileMeta {
    pub bucket_id: Uuid,
    pub owner_id: Uuid,
    pub original_name: String,
    pub mime_type: String,
    pub path: String,
    /// Optional logical owner attachment (consumer entity/module/id).
    pub owner_module: Option<String>,
    pub owner_entity: Option<String>,
    pub owner_entity_id: Option<Uuid>,
}

/// File-operations service: writes bytes to [`ObjectStorage`] and the
/// matching row to the `stored_files` table.
pub struct FileService {
    storage: Arc<dyn ObjectStorage>,
    files: Arc<StoredFileRepository>,
    config: Arc<BucketConfig>,
}

impl FileService {
    pub fn new(
        storage: Arc<dyn ObjectStorage>,
        files: Arc<StoredFileRepository>,
        config: Arc<BucketConfig>,
    ) -> Self {
        Self { storage, files, config }
    }

    /// Upload with an auto-generated UUID storage key.
    ///
    /// The produced key has shape `{uuid}/{sanitized_name}` — stable
    /// across renames, avoids accidental collisions, but not human
    /// readable. Use [`Self::upload_with_key`] when the URL matters.
    pub async fn upload(
        &self,
        body: Bytes,
        meta: FileMeta,
    ) -> BucketResult<StoredFile> {
        let key = format!(
            "{}/{}",
            Uuid::new_v4(),
            sanitize_filename(&meta.original_name)
        );
        self.upload_with_key(&key, body, meta).await
    }

    /// Upload under an explicit key.
    ///
    /// Keys beginning with `public_prefix` route to the public bucket
    /// (handled inside the [`ObjectStorage`] impl). When the backend has
    /// no public bucket configured, the key still writes to the private
    /// bucket and a `debug` log event records the mismatch — dev
    /// environments without a public bucket still work, but operators
    /// can grep for the event before promoting a misconfiguration to
    /// production.
    pub async fn upload_with_key(
        &self,
        key: &str,
        body: Bytes,
        meta: FileMeta,
    ) -> BucketResult<StoredFile> {
        validate_key(key)?;
        self.check_public_routing(key)?;

        let size = body.len() as i64;
        self.storage.put(key, body, &meta.mime_type).await?;

        let now = Utc::now();
        let file = StoredFile {
            id: Uuid::new_v4(),
            bucket_id: meta.bucket_id,
            owner_id: meta.owner_id,
            path: meta.path,
            original_name: meta.original_name,
            size_bytes: size,
            mime_type: meta.mime_type,
            checksum: None,
            is_compressed: false,
            original_size: None,
            compression_algorithm: None,
            is_scanned: false,
            scan_result: None,
            threat_level: None,
            has_thumbnail: false,
            thumbnail_path: None,
            has_video_thumbnail: false,
            has_document_preview: false,
            processing_status: None,
            content_hash_id: None,
            cdn_url: None,
            cdn_url_expires_at: None,
            owner_module: meta.owner_module,
            owner_entity: meta.owner_entity,
            owner_entity_id: meta.owner_entity_id,
            field_name: None,
            sort_order: 0,
            storage_key: key.to_string(),
            version: 1,
            previous_version_id: None,
            download_count: 0,
            last_accessed_at: None,
            status: FileStatus::Active,
            metadata: AuditMetadata {
                created_at: Some(now),
                updated_at: Some(now),
                deleted_at: None,
                created_by: Some(meta.owner_id),
                updated_by: Some(meta.owner_id),
                deleted_by: None,
            },
        };

        self.files
            .create(&file)
            .await
            .map_err(|e| BucketError::Other(format!("persist stored_file: {e}")))?;

        Ok(file)
    }

    fn check_public_routing(&self, key: &str) -> BucketResult<()> {
        let prefix = &self.config.serving.public_prefix;
        if prefix.is_empty() || !key.starts_with(prefix.as_str()) {
            return Ok(());
        }
        // When the key is public-prefixed but no public URL exists, the
        // upload still succeeds (the backend silently falls back to
        // private). Surface a warning via the log, but don't fail —
        // otherwise dev envs without a public bucket would reject public
        // keys outright, which is overly rigid.
        if self.storage.public_url(key).is_none() {
            tracing::debug!(
                key,
                "public-prefixed key uploaded to backend without public bucket configured"
            );
        }
        Ok(())
    }
}

fn validate_key(key: &str) -> BucketResult<()> {
    if key.is_empty() {
        return Err(BucketError::Other("empty key".into()));
    }
    if key.starts_with('/') {
        return Err(BucketError::Other(format!("invalid key: {key}")));
    }
    // Reject `..` path segments (traversal) while still allowing `..` to
    // appear inside a filename like `report..v2.pdf`.
    if key.split('/').any(|seg| seg == "..") {
        return Err(BucketError::Other(format!("invalid key: {key}")));
    }
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c if c.is_ascii_control() => '_',
            c => c,
        })
        .collect();
    if s.is_empty() { "file".to_string() } else { s }
}
