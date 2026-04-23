//! Bucket-module configuration surface.
//!
//! Consumed by [`crate::BucketModule::builder`] via `.with_config(...)`.
//! Loaded through the Backbone config overlay chain (YAML + env) — no new
//! config system is introduced here.
//!
//! # Secrets
//!
//! [`S3Config`] stores the *names* of environment variables that hold the
//! access/secret keys, never the secrets themselves. This keeps the config
//! file committable and matches how the upstream deployment plan wires
//! MinIO credentials via `.env`.

use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use url::Url;

/// Top-level bucket-module config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub storage: StorageConfig,
    #[serde(default)]
    pub serving: ServingConfig,
}

fn default_enabled() -> bool {
    true
}

/// Which backend to use.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum StorageConfig {
    Local {
        root: PathBuf,
        #[serde(default = "default_local_base_url")]
        base_url: Url,
        /// Name of the env var holding the signing secret.
        #[serde(default = "default_local_secret_env")]
        signing_secret_env: String,
    },
    S3(S3Config),
}

fn default_local_base_url() -> Url {
    Url::parse("http://localhost:8080/cdn/").expect("valid default base url")
}

fn default_local_secret_env() -> String {
    "BUCKET_LOCAL_SIGNING_SECRET".to_string()
}

/// S3 / MinIO credentials and endpoint config.
///
/// Same shape is used for AWS and MinIO — MinIO is S3-wire-compatible. Flip
/// `force_path_style` for MinIO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    pub endpoint: Url,
    pub region: String,
    /// Env var name holding the access key (NOT the key itself).
    pub access_key_env: String,
    /// Env var name holding the secret key (NOT the key itself).
    pub secret_key_env: String,
    pub private_bucket: String,
    /// Optional companion bucket for `public/*` keys; `None` disables mode A.
    #[serde(default)]
    pub public_bucket: Option<String>,
    /// Public-facing hostname for [`public_url`] — e.g. `https://bucket.bersihir.app`.
    /// Defaults to `endpoint` when unset.
    #[serde(default)]
    pub public_endpoint: Option<Url>,
    /// Set to `true` for MinIO; AWS S3 accepts virtual-hosted-style by default.
    #[serde(default = "default_force_path_style")]
    pub force_path_style: bool,
}

fn default_force_path_style() -> bool {
    true
}

/// How the serving handler responds by default for mode B.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServingConfig {
    #[serde(default = "default_serving_mode")]
    pub default_mode: ServingMode,
    /// Keys beginning with this prefix route to `public_bucket` when configured.
    /// Empty string disables public-prefix routing (everything goes private).
    #[serde(default = "default_public_prefix")]
    pub public_prefix: String,
    /// TTL (in whole seconds) for presigned URLs issued by mode-B
    /// redirects and mode-C responses.
    #[serde(default = "default_presigned_ttl", with = "duration_secs")]
    pub presigned_ttl: Duration,
}

impl Default for ServingConfig {
    fn default() -> Self {
        Self {
            default_mode: default_serving_mode(),
            public_prefix: default_public_prefix(),
            presigned_ttl: default_presigned_ttl(),
        }
    }
}

fn default_serving_mode() -> ServingMode {
    ServingMode::Redirect
}

fn default_public_prefix() -> String {
    "public/".to_string()
}

fn default_presigned_ttl() -> Duration {
    Duration::from_secs(60 * 15)
}

/// Serving-handler response strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServingMode {
    /// 302 → presigned URL. Default — lowest service bandwidth.
    Redirect,
    /// Proxy the bytes through the service. Use for `LocalStorage` dev or
    /// CORS-restricted clients.
    Stream,
    /// Return `{"url": "..."}` JSON — for mode-C clients that want the
    /// presigned URL without following the redirect.
    SignedUrl,
}

// Serialize Duration as whole seconds. Intentionally NOT the published
// `humantime_serde` crate — this one does not parse "5m" / "1h" strings.
mod duration_secs {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u64(d.as_secs())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let secs = u64::deserialize(d)?;
        Ok(Duration::from_secs(secs))
    }
}
