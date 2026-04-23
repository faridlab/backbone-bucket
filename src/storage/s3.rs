//! S3 / MinIO backend using `aws-sdk-s3`.
//!
//! Works against any S3-compatible service (AWS, MinIO, Wasabi, …). For
//! MinIO, set `force_path_style = true` and point `endpoint` at the MinIO
//! URL. Credentials are sourced from the env var *names* supplied via
//! [`S3Config`] — the module never holds the secret at rest, only the
//! env-var key used to read it at request time.
//!
//! # SigV4
//!
//! Presigning is delegated to `aws-sdk-s3`'s `presigned_config` /
//! `presigned` builders, which implement the AWS SigV4 spec in full. This
//! replaces the ad-hoc HMAC scheme that previously shipped in
//! [`crate::CdnService`].

use std::time::Duration;

use async_trait::async_trait;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::SharedCredentialsProvider;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use url::Url;

use crate::config::{S3Config, ServingConfig};
use crate::error::{BucketError, BucketResult};
use crate::storage::{ObjectMeta, ObjectStorage};

/// AWS S3 / MinIO backend.
pub struct S3Storage {
    client: Client,
    cfg: S3Config,
    serving: ServingConfig,
}

impl S3Storage {
    /// Build an S3 client from the supplied config.
    ///
    /// Reads access/secret keys from the env-var names in `S3Config`.
    pub fn new(cfg: S3Config, serving: ServingConfig) -> BucketResult<Self> {
        let access_key = std::env::var(&cfg.access_key_env)
            .map_err(|_| BucketError::Config(format!("{} not set", cfg.access_key_env)))?;
        let secret_key = std::env::var(&cfg.secret_key_env)
            .map_err(|_| BucketError::Config(format!("{} not set", cfg.secret_key_env)))?;

        let creds = Credentials::new(access_key, secret_key, None, None, "backbone-bucket");
        let s3_conf = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(cfg.region.clone()))
            .endpoint_url(cfg.endpoint.as_str())
            .credentials_provider(SharedCredentialsProvider::new(creds))
            .force_path_style(cfg.force_path_style)
            .build();

        Ok(Self {
            client: Client::from_conf(s3_conf),
            cfg,
            serving,
        })
    }

    fn bucket_for(&self, key: &str) -> &str {
        if self.is_public_key(key) {
            self.cfg
                .public_bucket
                .as_deref()
                .unwrap_or(&self.cfg.private_bucket)
        } else {
            &self.cfg.private_bucket
        }
    }

    fn is_public_key(&self, key: &str) -> bool {
        !self.serving.public_prefix.is_empty()
            && key.starts_with(&self.serving.public_prefix)
            && self.cfg.public_bucket.is_some()
    }

    fn presigning_config(ttl: Duration) -> BucketResult<PresigningConfig> {
        PresigningConfig::expires_in(ttl)
            .map_err(|e| BucketError::S3(format!("presigning config: {e}")))
    }
}

#[async_trait]
impl ObjectStorage for S3Storage {
    async fn put(&self, key: &str, body: Bytes, content_type: &str) -> BucketResult<()> {
        self.client
            .put_object()
            .bucket(self.bucket_for(key))
            .key(key)
            .content_type(content_type)
            .body(ByteStream::from(body))
            .send()
            .await
            .map_err(|e| BucketError::S3(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, key: &str) -> BucketResult<Bytes> {
        let resp = self
            .client
            .get_object()
            .bucket(self.bucket_for(key))
            .key(key)
            .send()
            .await
            .map_err(|e| match e.into_service_error() {
                GetObjectError::NoSuchKey(_) => BucketError::NotFound,
                other => BucketError::S3(other.to_string()),
            })?;
        let data = resp
            .body
            .collect()
            .await
            .map_err(|e| BucketError::S3(e.to_string()))?;
        Ok(data.into_bytes())
    }

    async fn delete(&self, key: &str) -> BucketResult<()> {
        self.client
            .delete_object()
            .bucket(self.bucket_for(key))
            .key(key)
            .send()
            .await
            .map_err(|e| BucketError::S3(e.to_string()))?;
        Ok(())
    }

    async fn head(&self, key: &str) -> BucketResult<ObjectMeta> {
        let resp = self
            .client
            .head_object()
            .bucket(self.bucket_for(key))
            .key(key)
            .send()
            .await
            .map_err(|e| match e.into_service_error() {
                HeadObjectError::NotFound(_) => BucketError::NotFound,
                other => BucketError::S3(other.to_string()),
            })?;

        let size = resp
            .content_length()
            .and_then(|n| u64::try_from(n).ok())
            .unwrap_or(0);
        Ok(ObjectMeta {
            key: key.to_string(),
            size,
            content_type: resp.content_type().map(str::to_string),
            etag: resp.e_tag().map(str::to_string),
            last_modified: resp
                .last_modified()
                .and_then(|t| {
                    let secs = t.secs();
                    DateTime::<Utc>::from_timestamp(secs, t.subsec_nanos())
                }),
        })
    }

    async fn presigned_get(&self, key: &str, ttl: Duration) -> BucketResult<Url> {
        let presigning = Self::presigning_config(ttl)?;
        let presigned = self
            .client
            .get_object()
            .bucket(self.bucket_for(key))
            .key(key)
            .presigned(presigning)
            .await
            .map_err(|e| BucketError::S3(format!("presign get: {e}")))?;
        Url::parse(presigned.uri()).map_err(Into::into)
    }

    async fn presigned_put(
        &self,
        key: &str,
        ttl: Duration,
        content_type: &str,
    ) -> BucketResult<Url> {
        let presigning = Self::presigning_config(ttl)?;
        let presigned = self
            .client
            .put_object()
            .bucket(self.bucket_for(key))
            .key(key)
            .content_type(content_type)
            .presigned(presigning)
            .await
            .map_err(|e| BucketError::S3(format!("presign put: {e}")))?;
        Url::parse(presigned.uri()).map_err(Into::into)
    }

    fn public_url(&self, key: &str) -> Option<Url> {
        if !self.is_public_key(key) {
            return None;
        }
        let bucket = self.cfg.public_bucket.as_deref()?;
        let base = self
            .cfg
            .public_endpoint
            .clone()
            .unwrap_or_else(|| self.cfg.endpoint.clone());
        if self.cfg.force_path_style {
            base.join(&format!("{}/{}", bucket, key)).ok()
        } else {
            // virtual-hosted-style: bucket becomes a subdomain host.
            let mut u = base;
            if let Some(host) = u.host_str() {
                let new_host = format!("{bucket}.{host}");
                u.set_host(Some(&new_host)).ok()?;
            }
            u.join(key).ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> (S3Config, ServingConfig) {
        // SAFETY: test-only. No live S3 calls are made — we only invoke
        // the synchronous presigner, which reads these via env lookup.
        std::env::set_var("_TEST_AK", "AKIAIOSFODNN7EXAMPLE");
        std::env::set_var(
            "_TEST_SK",
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        );
        let cfg = S3Config {
            endpoint: Url::parse("http://minio:9000").unwrap(),
            region: "us-east-1".into(),
            access_key_env: "_TEST_AK".into(),
            secret_key_env: "_TEST_SK".into(),
            private_bucket: "private".into(),
            public_bucket: Some("public".into()),
            public_endpoint: Some(Url::parse("https://bucket.example.com").unwrap()),
            force_path_style: true,
        };
        let serving = ServingConfig {
            default_mode: crate::config::ServingMode::Redirect,
            public_prefix: "public/".into(),
            presigned_ttl: Duration::from_secs(60),
        };
        (cfg, serving)
    }

    #[tokio::test]
    async fn presign_url_contains_sigv4_query_params() {
        let (cfg, serving) = test_config();
        let s3 = S3Storage::new(cfg, serving).unwrap();

        let url = s3
            .presigned_get("foo/bar.txt", Duration::from_secs(60))
            .await
            .expect("presign should succeed with env creds set");

        let query: std::collections::HashMap<_, _> =
            url.query_pairs().into_owned().collect();

        // SigV4 canonical query parameters.
        assert!(query.contains_key("X-Amz-Algorithm"));
        assert_eq!(query["X-Amz-Algorithm"], "AWS4-HMAC-SHA256");
        assert!(query.contains_key("X-Amz-Credential"));
        assert!(query.contains_key("X-Amz-Date"));
        assert!(query.contains_key("X-Amz-Expires"));
        assert!(query.contains_key("X-Amz-Signature"));
        assert_eq!(query["X-Amz-Expires"], "60");
    }

    #[tokio::test]
    async fn public_url_routes_public_prefix_only() {
        let (cfg, serving) = test_config();
        let s3 = S3Storage::new(cfg, serving).unwrap();

        assert!(s3.public_url("public/foo.jpg").is_some());
        assert!(s3.public_url("private/foo.jpg").is_none());
        assert!(s3.public_url("foo.jpg").is_none());
    }
}
