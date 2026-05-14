//! In-memory [`ObjectStorage`] test double.
//!
//! Backs a `DashMap<String, (Bytes, String)>` — bytes + content-type.
//! Implements the full `ObjectStorage` trait so consumer tests can mount
//! the upload / serving routers without touching the filesystem or
//! standing up MinIO.
//!
//! Available only when the crate is built with the `test-utils` Cargo
//! feature:
//!
//! ```toml
//! [dev-dependencies]
//! backbone-bucket = { version = "...", features = ["test-utils"] }
//! ```
//!
//! Presigned URLs return a synthetic `memory://` scheme — never call
//! out to the network. Consumers asserting on URL format should treat
//! them as opaque.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use dashmap::DashMap;
use url::Url;

use crate::error::{BucketError, BucketResult};

use super::{ObjectMeta, ObjectStorage};

/// In-memory object storage. Cheap to clone (Arc-shared inner map).
#[derive(Clone, Default)]
pub struct InMemoryStorage {
    inner: Arc<DashMap<String, Entry>>,
}

#[derive(Clone)]
struct Entry {
    body: Bytes,
    content_type: String,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of objects currently stored — useful for assertions.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Drop every object. Tests calling `.clear()` between cases must
    /// share the same instance via `Arc::clone`.
    pub fn clear(&self) {
        self.inner.clear()
    }

    /// True iff `key` exists. Skips the async `head` call.
    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }
}

#[async_trait]
impl ObjectStorage for InMemoryStorage {
    async fn put(&self, key: &str, body: Bytes, content_type: &str) -> BucketResult<()> {
        self.inner.insert(
            key.to_string(),
            Entry {
                body,
                content_type: content_type.to_string(),
            },
        );
        Ok(())
    }

    async fn get(&self, key: &str) -> BucketResult<Bytes> {
        self.inner
            .get(key)
            .map(|e| e.body.clone())
            .ok_or(BucketError::NotFound)
    }

    async fn delete(&self, key: &str) -> BucketResult<()> {
        self.inner.remove(key);
        Ok(())
    }

    async fn head(&self, key: &str) -> BucketResult<ObjectMeta> {
        self.inner
            .get(key)
            .map(|e| ObjectMeta {
                key: key.to_string(),
                size: e.body.len() as u64,
                content_type: Some(e.content_type.clone()),
                etag: None,
                last_modified: None,
            })
            .ok_or(BucketError::NotFound)
    }

    async fn presigned_get(&self, key: &str, _ttl: Duration) -> BucketResult<Url> {
        Url::parse(&format!("memory://get/{key}"))
            .map_err(|e| BucketError::Url(e.to_string()))
    }

    async fn presigned_put(
        &self,
        key: &str,
        _ttl: Duration,
        _content_type: &str,
    ) -> BucketResult<Url> {
        Url::parse(&format!("memory://put/{key}"))
            .map_err(|e| BucketError::Url(e.to_string()))
    }

    fn public_url(&self, _key: &str) -> Option<Url> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn put_get_round_trip() {
        let s = InMemoryStorage::new();
        s.put("foo/bar.txt", Bytes::from_static(b"hello"), "text/plain")
            .await
            .unwrap();
        assert_eq!(s.len(), 1);
        assert!(s.contains_key("foo/bar.txt"));
        let body = s.get("foo/bar.txt").await.unwrap();
        assert_eq!(&body[..], b"hello");
        let meta = s.head("foo/bar.txt").await.unwrap();
        assert_eq!(meta.size, 5);
        assert_eq!(meta.content_type.as_deref(), Some("text/plain"));
    }

    #[tokio::test]
    async fn missing_key_yields_not_found() {
        let s = InMemoryStorage::new();
        let err = s.get("absent").await.unwrap_err();
        assert!(matches!(err, BucketError::NotFound));
        let err = s.head("absent").await.unwrap_err();
        assert!(matches!(err, BucketError::NotFound));
    }

    #[tokio::test]
    async fn delete_is_idempotent() {
        let s = InMemoryStorage::new();
        s.delete("never-existed").await.unwrap();
        s.put("k", Bytes::from_static(b"x"), "application/octet-stream")
            .await
            .unwrap();
        s.delete("k").await.unwrap();
        s.delete("k").await.unwrap();
        assert!(s.is_empty());
    }

    #[tokio::test]
    async fn presigned_urls_are_synthetic() {
        let s = InMemoryStorage::new();
        let u = s.presigned_get("k", Duration::from_secs(60)).await.unwrap();
        assert_eq!(u.scheme(), "memory");
    }
}
