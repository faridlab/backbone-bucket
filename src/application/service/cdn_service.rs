//! CDN Service
//!
//! Hand-written — NOT generated. This file is safe from regeneration.
//!
//! Manages CDN URL generation and caching for stored files.
//! Provides signed URL generation with configurable expiry.
//!
//! The whole surface is `#[deprecated]` (see [`CdnService`]); allow
//! directives below are narrowed to the `impl` that references the
//! deprecated type to avoid self-referential warnings while still
//! flagging any unrelated deprecated usage.

use std::sync::Arc;

use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

use super::error::{ServiceError, ServiceResult};
use crate::domain::entity::StoredFile;
use crate::infrastructure::persistence::StoredFileRepository;
use crate::infrastructure::persistence::BucketRepository;

/// Default CDN URL expiry: 1 hour
const DEFAULT_CDN_EXPIRY_HOURS: i64 = 1;

/// Environment variable for CDN signing secret
const CDN_SECRET_ENV: &str = "CDN_SIGNING_SECRET";

/// Default CDN secret (development only)
const DEFAULT_CDN_SECRET: &str = "bucket-cdn-dev-secret-change-in-production";

/// Service for managing CDN URLs for stored files.
///
/// # Deprecated
///
/// Signs URLs with a raw module-local HMAC scheme, which is not
/// compatible with S3/MinIO clients and cannot be validated by a
/// reverse proxy. Use [`crate::storage::ObjectStorage::presigned_get`]
/// instead — it emits real SigV4 URLs for S3/MinIO and signed
/// module-local URLs for `LocalStorage`.
///
/// Scheduled for removal in a later release (see `docs/serving.md`).
#[deprecated(
    note = "HMAC-signed CDN URLs are not S3-compatible. Use ObjectStorage::presigned_get for real SigV4 URLs."
)]
pub struct CdnService {
    file_repo: Arc<StoredFileRepository>,
    bucket_repo: Arc<BucketRepository>,
}

#[allow(deprecated)]
impl CdnService {
    pub fn new(
        file_repo: Arc<StoredFileRepository>,
        bucket_repo: Arc<BucketRepository>,
    ) -> Self {
        Self { file_repo, bucket_repo }
    }

    /// Get or generate a CDN URL for a file.
    ///
    /// If the file already has a valid (non-expired) CDN URL, returns it.
    /// Otherwise, generates a new URL and caches it on the file record.
    pub async fn get_or_generate_url(
        &self,
        file_id: Uuid,
        expiry_hours: Option<i64>,
    ) -> ServiceResult<String> {
        let file = self.file_repo
            .find_by_id(&file_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        // Check if bucket has CDN enabled
        let bucket = self.bucket_repo
            .find_by_id(&file.bucket_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        if !bucket.enable_cdn {
            return Err(ServiceError::Validation(format!("CDN is not enabled for bucket {}", bucket.id)));
        }

        // Return existing valid URL
        if let (Some(ref url), Some(ref expires)) = (&file.cdn_url, &file.cdn_url_expires_at) {
            if *expires > Utc::now() {
                return Ok(url.clone());
            }
        }

        // Generate new CDN URL
        let hours = expiry_hours.unwrap_or(DEFAULT_CDN_EXPIRY_HOURS);
        let expires_at = Utc::now() + Duration::hours(hours);

        // Build the CDN URL from file path and storage key
        // In production, this would call the actual CDN provider API
        let cdn_url = self.generate_signed_url(&file, expires_at);

        // TODO: file_repo.update_cdn_url — implement custom repository method
        let _ = (&cdn_url, expires_at);

        Ok(cdn_url)
    }

    /// Invalidate the cached CDN URL for a file.
    pub async fn invalidate(&self, _file_id: Uuid) -> ServiceResult<()> {
        // TODO: file_repo.update_cdn_url — implement custom repository method
        Ok(())
    }

    /// Invalidate all CDN URLs for files in a bucket.
    pub async fn invalidate_bucket(&self, _bucket_id: Uuid) -> ServiceResult<u64> {
        // TODO: file_repo.invalidate_cdn_urls_by_bucket — implement custom repository method
        Ok(0)
    }

    // ---- internal ----

    /// Generate an HMAC-SHA256 signed CDN URL for a file.
    ///
    /// The signature covers the path and expiry timestamp to prevent
    /// URL tampering or expiry manipulation.
    fn generate_signed_url(
        &self,
        file: &StoredFile,
        expires_at: chrono::DateTime<Utc>,
    ) -> String {
        let timestamp = expires_at.timestamp();
        let path = format!("/cdn/files/{}/{}", file.bucket_id, file.id);
        let signature = Self::sign_url(&path, timestamp);
        format!("{}?expires={}&sig={}", path, timestamp, signature)
    }

    /// Compute HMAC-SHA256 signature over path and expiry.
    fn sign_url(path: &str, expires_timestamp: i64) -> String {
        let secret = std::env::var(CDN_SECRET_ENV)
            .unwrap_or_else(|_| DEFAULT_CDN_SECRET.to_string());
        let message = format!("{}:{}", path, expires_timestamp);

        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
}
