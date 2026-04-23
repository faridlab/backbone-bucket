//! Filesystem-backed [`ObjectStorage`] implementation.
//!
//! Intended for development, tests, and single-node deployments. Objects
//! are stored as files under `root`; presigned URLs are module-signed
//! HMAC-SHA256 tokens that the serving handler validates in-process.
//!
//! # Presigned URL format
//!
//! ```text
//! {base_url}/{key}?expires={unix_ts}&sig={hex}
//! ```
//!
//! `sig = hex(hmac_sha256(secret, format!("{method}\n{key}\n{expires}")))`
//!
//! where `method` is `"GET"` or `"PUT"`. This is NOT compatible with S3
//! clients — it only works when the serving handler in this module fronts
//! the storage, which is exactly what mode-B does in dev.

use std::path::{Path, PathBuf};
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::error::{BucketError, BucketResult};
use crate::storage::{ObjectMeta, ObjectStorage};

type HmacSha256 = Hmac<Sha256>;

/// Filesystem-backed object storage.
#[derive(Debug, Clone)]
pub struct LocalStorage {
    root: PathBuf,
    /// Base URL used to build presigned URLs. Typically the URL at which
    /// the serving handler is mounted (e.g. `http://localhost:8080/cdn`).
    base_url: Url,
    /// Shared secret used to sign module-issued URLs.
    signing_secret: Vec<u8>,
}

impl LocalStorage {
    /// Build a new local-filesystem backend.
    pub fn new(
        root: impl Into<PathBuf>,
        base_url: Url,
        signing_secret: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            root: root.into(),
            base_url,
            signing_secret: signing_secret.into(),
        }
    }

    /// Resolve `key` to a filesystem path, rejecting traversal attempts.
    fn resolve(&self, key: &str) -> BucketResult<PathBuf> {
        if key.starts_with('/') || key.split('/').any(|seg| seg == "..") {
            return Err(BucketError::Other(format!("invalid key: {key}")));
        }
        Ok(self.root.join(key))
    }

    /// Verify a token produced by [`sign`](Self::sign_token).
    ///
    /// Returns `Ok(())` if `sig` matches and `expires` is in the future.
    pub fn verify_token(
        &self,
        method: &str,
        key: &str,
        expires: i64,
        sig: &str,
    ) -> BucketResult<()> {
        if expires < Utc::now().timestamp() {
            return Err(BucketError::InvalidSignature);
        }
        let expected = self.sign_token(method, key, expires);
        // constant-time compare
        if expected.len() != sig.len()
            || expected
                .as_bytes()
                .iter()
                .zip(sig.as_bytes())
                .fold(0u8, |acc, (a, b)| acc | (a ^ b))
                != 0
        {
            return Err(BucketError::InvalidSignature);
        }
        Ok(())
    }

    fn sign_token(&self, method: &str, key: &str, expires: i64) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.signing_secret)
            .expect("HMAC accepts any key length");
        mac.update(method.as_bytes());
        mac.update(b"\n");
        mac.update(key.as_bytes());
        mac.update(b"\n");
        mac.update(expires.to_string().as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    fn build_signed_url(
        &self,
        method: &str,
        key: &str,
        ttl: Duration,
    ) -> BucketResult<Url> {
        let expires = (Utc::now() + chrono::Duration::from_std(ttl).unwrap_or(chrono::Duration::hours(1)))
            .timestamp();
        let sig = self.sign_token(method, key, expires);
        let mut url = self
            .base_url
            .join(key)
            .map_err(|e| BucketError::Url(e.to_string()))?;
        url.query_pairs_mut()
            .append_pair("expires", &expires.to_string())
            .append_pair("sig", &sig);
        Ok(url)
    }
}

#[async_trait]
impl ObjectStorage for LocalStorage {
    async fn put(&self, key: &str, body: Bytes, _content_type: &str) -> BucketResult<()> {
        let path = self.resolve(key)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let mut file = fs::File::create(&path).await?;
        file.write_all(&body).await?;
        file.flush().await?;
        Ok(())
    }

    async fn get(&self, key: &str) -> BucketResult<Bytes> {
        let path = self.resolve(key)?;
        match fs::read(&path).await {
            Ok(bytes) => Ok(Bytes::from(bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(BucketError::NotFound),
            Err(e) => Err(e.into()),
        }
    }

    async fn delete(&self, key: &str) -> BucketResult<()> {
        let path = self.resolve(key)?;
        match fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn head(&self, key: &str) -> BucketResult<ObjectMeta> {
        let path = self.resolve(key)?;
        let meta = match fs::metadata(&path).await {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(BucketError::NotFound),
            Err(e) => return Err(e.into()),
        };
        let last_modified: Option<DateTime<Utc>> = meta
            .modified()
            .ok()
            .map(|t| DateTime::<Utc>::from(t));
        Ok(ObjectMeta {
            key: key.to_string(),
            size: meta.len(),
            content_type: None,
            etag: None,
            last_modified,
        })
    }

    async fn presigned_get(&self, key: &str, ttl: Duration) -> BucketResult<Url> {
        self.build_signed_url("GET", key, ttl)
    }

    async fn presigned_put(
        &self,
        key: &str,
        ttl: Duration,
        _content_type: &str,
    ) -> BucketResult<Url> {
        self.build_signed_url("PUT", key, ttl)
    }

    fn public_url(&self, _key: &str) -> Option<Url> {
        None
    }
}

/// Path helper used by tests and the serving handler when LocalStorage is active.
#[doc(hidden)]
pub fn local_object_path(root: &Path, key: &str) -> PathBuf {
    root.join(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make() -> (tempfile::TempDir, LocalStorage) {
        let dir = tempfile::tempdir().unwrap();
        let base = Url::parse("http://localhost:8080/cdn/").unwrap();
        let storage = LocalStorage::new(dir.path(), base, b"test-secret".to_vec());
        (dir, storage)
    }

    #[tokio::test]
    async fn roundtrip() {
        let (_dir, storage) = make();
        storage
            .put("foo/bar.txt", Bytes::from_static(b"hi"), "text/plain")
            .await
            .unwrap();
        let got = storage.get("foo/bar.txt").await.unwrap();
        assert_eq!(&got[..], b"hi");

        let meta = storage.head("foo/bar.txt").await.unwrap();
        assert_eq!(meta.size, 2);

        storage.delete("foo/bar.txt").await.unwrap();
        assert!(matches!(
            storage.get("foo/bar.txt").await,
            Err(BucketError::NotFound)
        ));
    }

    #[tokio::test]
    async fn signed_url_roundtrip() {
        let (_dir, storage) = make();
        let url = storage
            .presigned_get("a/b.txt", Duration::from_secs(60))
            .await
            .unwrap();
        let query: std::collections::HashMap<_, _> =
            url.query_pairs().into_owned().collect();
        let expires: i64 = query["expires"].parse().unwrap();
        storage
            .verify_token("GET", "a/b.txt", expires, &query["sig"])
            .unwrap();
    }

    #[test]
    fn signed_url_rejects_expired() {
        let (_dir, storage) = make();
        let past = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
        let sig = storage.sign_token("GET", "a/b.txt", past);
        assert!(matches!(
            storage.verify_token("GET", "a/b.txt", past, &sig),
            Err(BucketError::InvalidSignature)
        ));
    }

    #[test]
    fn rejects_path_traversal() {
        let (_dir, storage) = make();
        assert!(storage.resolve("../secrets").is_err());
        assert!(storage.resolve("foo/../bar").is_err());
        assert!(storage.resolve("/etc/passwd").is_err());
    }

    #[test]
    fn allows_dotdot_inside_filename() {
        let (_dir, storage) = make();
        // `..` inside a segment is a legitimate filename, not traversal.
        assert!(storage.resolve("reports/report..v2.pdf").is_ok());
    }
}
