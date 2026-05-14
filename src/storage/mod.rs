//! Object storage abstraction.
//!
//! The [`ObjectStorage`] trait is the single boundary between the bucket
//! module and whichever backend is serving bytes — local filesystem,
//! MinIO, Amazon S3, or any S3-compatible service.
//!
//! # Backends
//!
//! - [`LocalStorage`] — filesystem; emits module-signed HMAC URLs for
//!   presigned access. Suitable for development and single-node deployments.
//! - [`S3Storage`] — AWS S3 / MinIO via `aws-sdk-s3`; emits real SigV4
//!   presigned URLs. Requires the `s3` feature.
//!
//! # Design
//!
//! - Trait object safe: consumers wire an `Arc<dyn ObjectStorage>` into
//!   [`crate::BucketModule`] at build time.
//! - All trait methods except `public_url` are async. `aws-sdk-s3`'s
//!   presigner requires a Tokio runtime context for its internal timer
//!   infrastructure, so `presigned_get` / `presigned_put` are async too.
//! - `public_url` returns `None` when the configured backend has no public
//!   bucket or the key does not map to the public prefix.

use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use url::Url;

use crate::error::BucketResult;

pub mod local;
pub use local::LocalStorage;

#[cfg(feature = "s3")]
pub mod s3;
#[cfg(feature = "s3")]
pub use s3::S3Storage;

#[cfg(feature = "test-utils")]
pub mod memory;
#[cfg(feature = "test-utils")]
pub use memory::InMemoryStorage;

/// Metadata returned from `head` without fetching the body.
#[derive(Debug, Clone)]
pub struct ObjectMeta {
    pub key: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
}

/// Object storage backend contract.
///
/// Beta scope is buffered reads via `get`; streaming `Body::from_stream`
/// variants are planned post-beta (see `docs/serving.md`).
#[async_trait]
pub trait ObjectStorage: Send + Sync {
    /// Upload an object under `key` with the given content type.
    async fn put(&self, key: &str, body: Bytes, content_type: &str) -> BucketResult<()>;

    /// Download the full object body (buffered).
    async fn get(&self, key: &str) -> BucketResult<Bytes>;

    /// Remove the object. Idempotent: absent keys return `Ok(())`.
    async fn delete(&self, key: &str) -> BucketResult<()>;

    /// Stat the object without fetching the body.
    async fn head(&self, key: &str) -> BucketResult<ObjectMeta>;

    /// Short-lived signed URL for direct GET.
    ///
    /// Used by mode-B redirect serving and by mode-C direct clients.
    async fn presigned_get(&self, key: &str, ttl: Duration) -> BucketResult<Url>;

    /// Short-lived signed URL for direct PUT (browser direct-upload flow).
    async fn presigned_put(
        &self,
        key: &str,
        ttl: Duration,
        content_type: &str,
    ) -> BucketResult<Url>;

    /// Public URL for keys that map to the configured public bucket / prefix,
    /// or `None` when the backend has no public side or the key doesn't qualify.
    ///
    /// Consumed by mode-A fast-path callers that want to bypass the service hop.
    fn public_url(&self, key: &str) -> Option<Url>;
}
