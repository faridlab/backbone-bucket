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

/// Error returned by [`BucketConfig::from_env`].
#[derive(Debug, thiserror::Error)]
pub enum ConfigEnvError {
    #[error("environment variable `{0}` is required but not set")]
    Missing(&'static str),
    #[error("environment variable `{name}` has invalid value `{value}`: {source}")]
    Invalid {
        name: &'static str,
        value: String,
        #[source]
        source: anyhow::Error,
    },
}

impl Default for BucketConfig {
    /// Dev-friendly default: `LocalStorage` rooted at `/tmp/bucket`,
    /// redirect serving, signing secret read from `BUCKET_LOCAL_SIGNING_SECRET`.
    ///
    /// Suitable for examples and first-time wiring. Production deployments
    /// should construct `BucketConfig` explicitly or use [`Self::from_env`].
    fn default() -> Self {
        Self {
            enabled: true,
            storage: StorageConfig::Local {
                root: PathBuf::from("/tmp/bucket"),
                base_url: default_local_base_url(),
                signing_secret_env: default_local_secret_env(),
            },
            serving: ServingConfig::default(),
        }
    }
}

impl BucketConfig {
    /// Build a [`BucketConfig`] from environment variables.
    ///
    /// Backend is selected by `BUCKET_STORAGE_BACKEND` (`local` | `s3`,
    /// default `local`). Per-backend variables:
    ///
    /// **Local** (default)
    /// - `BUCKET_STORAGE_ROOT` — filesystem root (default `/tmp/bucket`)
    /// - `BUCKET_BASE_URL` — public-facing URL (default `http://localhost:8080/cdn/`)
    /// - `BUCKET_SIGNING_SECRET_ENV` — name of the env var holding the HMAC secret
    ///   (default `BUCKET_LOCAL_SIGNING_SECRET`; the *value* is read by the
    ///   storage backend at startup, not by this loader)
    ///
    /// **S3 / MinIO**
    /// - `BUCKET_S3_ENDPOINT` (required)
    /// - `BUCKET_S3_REGION` (required)
    /// - `BUCKET_S3_PRIVATE_BUCKET` (required)
    /// - `BUCKET_S3_PUBLIC_BUCKET` (optional)
    /// - `BUCKET_S3_PUBLIC_ENDPOINT` (optional)
    /// - `BUCKET_S3_ACCESS_KEY_ENV` (default `BUCKET_S3_ACCESS_KEY`)
    /// - `BUCKET_S3_SECRET_KEY_ENV` (default `BUCKET_S3_SECRET_KEY`)
    /// - `BUCKET_S3_FORCE_PATH_STYLE` (`true` | `false`, default `true` — MinIO-friendly)
    ///
    /// **Serving** (both backends)
    /// - `BUCKET_SERVING_MODE` (`redirect` | `stream` | `signed_url`, default `redirect`)
    /// - `BUCKET_PUBLIC_PREFIX` (default `public/`)
    /// - `BUCKET_PRESIGNED_TTL_SECS` (default `900` — 15 min)
    pub fn from_env() -> Result<Self, ConfigEnvError> {
        let backend = env_or("BUCKET_STORAGE_BACKEND", "local");
        let storage = match backend.to_ascii_lowercase().as_str() {
            "local" => StorageConfig::Local {
                root: PathBuf::from(env_or("BUCKET_STORAGE_ROOT", "/tmp/bucket")),
                base_url: env_url("BUCKET_BASE_URL", "http://localhost:8080/cdn/")?,
                signing_secret_env: env_or(
                    "BUCKET_SIGNING_SECRET_ENV",
                    "BUCKET_LOCAL_SIGNING_SECRET",
                ),
            },
            "s3" => StorageConfig::S3(S3Config {
                endpoint: env_url_required("BUCKET_S3_ENDPOINT")?,
                region: env_required("BUCKET_S3_REGION")?,
                access_key_env: env_or("BUCKET_S3_ACCESS_KEY_ENV", "BUCKET_S3_ACCESS_KEY"),
                secret_key_env: env_or("BUCKET_S3_SECRET_KEY_ENV", "BUCKET_S3_SECRET_KEY"),
                private_bucket: env_required("BUCKET_S3_PRIVATE_BUCKET")?,
                public_bucket: std::env::var("BUCKET_S3_PUBLIC_BUCKET").ok(),
                public_endpoint: match std::env::var("BUCKET_S3_PUBLIC_ENDPOINT") {
                    Ok(s) => Some(parse_url("BUCKET_S3_PUBLIC_ENDPOINT", &s)?),
                    Err(_) => None,
                },
                force_path_style: env_bool("BUCKET_S3_FORCE_PATH_STYLE", true)?,
            }),
            other => {
                return Err(ConfigEnvError::Invalid {
                    name: "BUCKET_STORAGE_BACKEND",
                    value: other.to_string(),
                    source: anyhow::anyhow!("expected `local` or `s3`"),
                })
            }
        };

        let serving = ServingConfig {
            default_mode: parse_serving_mode(&env_or("BUCKET_SERVING_MODE", "redirect"))?,
            public_prefix: env_or("BUCKET_PUBLIC_PREFIX", "public/"),
            presigned_ttl: Duration::from_secs(env_u64("BUCKET_PRESIGNED_TTL_SECS", 900)?),
        };

        Ok(Self {
            enabled: env_bool("BUCKET_ENABLED", true)?,
            storage,
            serving,
        })
    }
}

fn env_or(name: &'static str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn env_required(name: &'static str) -> Result<String, ConfigEnvError> {
    std::env::var(name).map_err(|_| ConfigEnvError::Missing(name))
}

fn env_url(name: &'static str, default: &str) -> Result<Url, ConfigEnvError> {
    let raw = env_or(name, default);
    parse_url(name, &raw)
}

fn env_url_required(name: &'static str) -> Result<Url, ConfigEnvError> {
    let raw = env_required(name)?;
    parse_url(name, &raw)
}

fn parse_url(name: &'static str, raw: &str) -> Result<Url, ConfigEnvError> {
    Url::parse(raw).map_err(|e| ConfigEnvError::Invalid {
        name,
        value: raw.to_string(),
        source: anyhow::Error::from(e),
    })
}

fn env_u64(name: &'static str, default: u64) -> Result<u64, ConfigEnvError> {
    match std::env::var(name) {
        Ok(s) => s.parse::<u64>().map_err(|e| ConfigEnvError::Invalid {
            name,
            value: s,
            source: anyhow::Error::from(e),
        }),
        Err(_) => Ok(default),
    }
}

fn env_bool(name: &'static str, default: bool) -> Result<bool, ConfigEnvError> {
    match std::env::var(name) {
        Ok(s) => match s.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => Err(ConfigEnvError::Invalid {
                name,
                value: s,
                source: anyhow::anyhow!("expected boolean (true/false/1/0/yes/no)"),
            }),
        },
        Err(_) => Ok(default),
    }
}

fn parse_serving_mode(s: &str) -> Result<ServingMode, ConfigEnvError> {
    match s.to_ascii_lowercase().as_str() {
        "redirect" => Ok(ServingMode::Redirect),
        "stream" => Ok(ServingMode::Stream),
        "signed_url" | "signedurl" => Ok(ServingMode::SignedUrl),
        other => Err(ConfigEnvError::Invalid {
            name: "BUCKET_SERVING_MODE",
            value: other.to_string(),
            source: anyhow::anyhow!("expected `redirect`, `stream`, or `signed_url`"),
        }),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// `from_env` tests share process-wide env. They run serially in a
    /// single test by clearing/setting only the vars they care about
    /// inside one function — splitting into multiple `#[test]`s would
    /// race because cargo test runs them in parallel.
    #[test]
    fn from_env_round_trip_and_defaults() {
        // Clear anything stale.
        for v in [
            "BUCKET_STORAGE_BACKEND",
            "BUCKET_STORAGE_ROOT",
            "BUCKET_BASE_URL",
            "BUCKET_SERVING_MODE",
            "BUCKET_PUBLIC_PREFIX",
            "BUCKET_PRESIGNED_TTL_SECS",
            "BUCKET_S3_ENDPOINT",
            "BUCKET_S3_REGION",
            "BUCKET_S3_PRIVATE_BUCKET",
            "BUCKET_S3_FORCE_PATH_STYLE",
        ] {
            std::env::remove_var(v);
        }

        // Default backend → Local with documented fallbacks.
        let cfg = BucketConfig::from_env().expect("defaults must work");
        assert!(cfg.enabled);
        match cfg.storage {
            StorageConfig::Local { ref root, .. } => {
                assert_eq!(root, std::path::Path::new("/tmp/bucket"))
            }
            _ => panic!("expected Local backend by default"),
        }
        assert_eq!(cfg.serving.default_mode, ServingMode::Redirect);
        assert_eq!(cfg.serving.public_prefix, "public/");
        assert_eq!(cfg.serving.presigned_ttl, Duration::from_secs(900));

        // S3 with all required vars.
        std::env::set_var("BUCKET_STORAGE_BACKEND", "s3");
        std::env::set_var("BUCKET_S3_ENDPOINT", "https://minio.example/");
        std::env::set_var("BUCKET_S3_REGION", "us-east-1");
        std::env::set_var("BUCKET_S3_PRIVATE_BUCKET", "private");
        std::env::set_var("BUCKET_S3_FORCE_PATH_STYLE", "false");
        std::env::set_var("BUCKET_SERVING_MODE", "signed_url");
        std::env::set_var("BUCKET_PRESIGNED_TTL_SECS", "60");

        let cfg = BucketConfig::from_env().expect("s3 config must parse");
        match cfg.storage {
            StorageConfig::S3(ref s3) => {
                assert_eq!(s3.endpoint.as_str(), "https://minio.example/");
                assert_eq!(s3.region, "us-east-1");
                assert_eq!(s3.private_bucket, "private");
                assert!(!s3.force_path_style);
            }
            _ => panic!("expected S3 backend"),
        }
        assert_eq!(cfg.serving.default_mode, ServingMode::SignedUrl);
        assert_eq!(cfg.serving.presigned_ttl, Duration::from_secs(60));

        // Required field missing → Missing error.
        std::env::remove_var("BUCKET_S3_PRIVATE_BUCKET");
        let err = BucketConfig::from_env().unwrap_err();
        assert!(matches!(err, ConfigEnvError::Missing("BUCKET_S3_PRIVATE_BUCKET")));

        // Invalid backend → Invalid error.
        std::env::set_var("BUCKET_STORAGE_BACKEND", "azure");
        let err = BucketConfig::from_env().unwrap_err();
        assert!(matches!(err, ConfigEnvError::Invalid { name: "BUCKET_STORAGE_BACKEND", .. }));

        // Cleanup so we don't leak into other tests.
        for v in [
            "BUCKET_STORAGE_BACKEND",
            "BUCKET_S3_ENDPOINT",
            "BUCKET_S3_REGION",
            "BUCKET_S3_PRIVATE_BUCKET",
            "BUCKET_S3_FORCE_PATH_STYLE",
            "BUCKET_SERVING_MODE",
            "BUCKET_PRESIGNED_TTL_SECS",
        ] {
            std::env::remove_var(v);
        }
    }

    #[test]
    fn default_impl_matches_local_dev_shape() {
        let cfg = BucketConfig::default();
        assert!(cfg.enabled);
        assert!(matches!(cfg.storage, StorageConfig::Local { .. }));
        assert_eq!(cfg.serving.default_mode, ServingMode::Redirect);
    }
}
