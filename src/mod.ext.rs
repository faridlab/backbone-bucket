// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

pub mod error;
pub mod storage;
pub mod auth;
pub mod config;
#[cfg(feature = "s3")]
pub mod provision;
// ─── Essential (every consumer wires these) ─────────────────────────────
pub use error::{BucketError, BucketResult};
pub use config::{BucketConfig, ConfigEnvError, S3Config, ServingConfig, ServingMode, StorageConfig};
// ─── Storage backends ───────────────────────────────────────────────────
// `ObjectStorage` is the boundary trait. `LocalStorage` ships always;
// `S3Storage` is gated on the `s3` feature; `InMemoryStorage` on `test-utils`.
pub use storage::{ObjectMeta, ObjectStorage, LocalStorage};
#[cfg(feature = "s3")]
pub use storage::S3Storage;
#[cfg(feature = "test-utils")]
pub use storage::InMemoryStorage;
// ─── Auth slots (consumer-implemented) ──────────────────────────────────
pub use auth::{
    ArcAuthzPolicy, AuthExtractor, AuthzDecision, AuthzPolicy, DefaultOwnerOnlyPolicy, HasOwnerId,
};
// ─── File operations (single-shot programmatic API) ─────────────────────
pub use application::service::{FileMeta, FileService};
// ─── HTTP surfaces (serving + multipart upload) ─────────────────────────
pub use presentation::http::{lookup_by_key as lookup_file_by_key, serving_router, ServingContext};
pub use presentation::http::{
    upload_router, UploadConfig, UploadContext, DEFAULT_CHUNK_BODY_LIMIT, DEFAULT_UPLOAD_BODY_LIMIT,
};
pub use application::service::LockingService;
pub use application::service::MultipartUploadService;
pub use application::service::ConversionService;
#[allow(deprecated)]
pub use application::service::CdnService;
pub use application::service::VideoThumbnailService;
pub use application::service::DocumentPreviewService;
mod bucket_module; // Phase 6: http_routes() — regeneration-safe BucketModule extensions
pub use bucket_module::RouterOptions;
