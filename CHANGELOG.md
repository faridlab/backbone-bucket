# Changelog

All notable changes to `backbone-bucket` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Adds the file-serving surface so consumer workspaces (e.g.
`bersihir-service`) can serve pretty-URL file downloads with auth
enforcement and real SigV4-signed S3/MinIO URLs. See
[`docs/serving.md`](docs/serving.md) for the reference. This release
also adds the multipart HTTP upload surface and a set of
consumer-ergonomics improvements (env-driven config, in-memory test
double, merged router) drawn from a pre-commit audit.

### Added — Multipart HTTP upload

- `BucketModule::upload_router::<A>(UploadConfig)` mounts five routes:
  `POST /uploads` (single-shot `multipart/form-data`),
  `POST /uploads/sessions` (initiate resumable),
  `POST /uploads/sessions/:id/parts/:n` (chunk),
  `POST /uploads/sessions/:id/complete`,
  `DELETE /uploads/sessions/:id` (abort).
- `UploadConfig`, `UploadContext`, `DEFAULT_UPLOAD_BODY_LIMIT` (256 MiB),
  `DEFAULT_CHUNK_BODY_LIMIT` (16 MiB) re-exported at crate root.
- `MultipartUploadService::check_capacity` and `record_completed_usage`
  wire the previously-unused `quota_repo` into the upload lifecycle.
- OpenAPI 3.0 operations under `/uploads/*` added to
  `docs/openapi/bucket-v2.yaml`.
- Hardened against bucket policy violations: every upload entry-point
  rejects when the bucket is `Readonly` / `Archived` (409), the body
  exceeds `max_file_size` (413), or the MIME type is not in
  `allowed_mime_types` (415).
- `#[tracing::instrument]` spans on every upload handler with body
  bytes skipped — uploads are visible in production logs without
  leaking content.
- New `BucketError` variants `InvalidInput` (400), `Conflict` (409),
  `PayloadTooLarge` (413), `UnsupportedMediaType` (415). The existing
  `IntoResponse` mapping routes each variant to the correct status.

### Added — Consumer ergonomics

- `BucketConfig::from_env()` reads the `BUCKET_STORAGE_BACKEND`,
  `BUCKET_STORAGE_ROOT`, `BUCKET_S3_*`, `BUCKET_SERVING_MODE`, … env
  vars. `BucketConfig::default()` produces a `LocalStorage` dev shape.
  Removes the biggest first-time wiring pain point.
- `BucketModule::router::<A>(RouterOptions)` returns CRUD + upload +
  serving merged into one Axum router with sensible defaults
  (`/cdn` as the serving prefix). Granular `crud_router()` /
  `upload_router()` / `serving_router()` remain for advanced
  composition.
- `InMemoryStorage` — full `ObjectStorage` impl backed by a `DashMap`,
  gated behind the new `test-utils` Cargo feature. Lets consumers test
  the upload / serving routers without touching the filesystem or S3.
- `serving_router` now requires the same `AuthExtractor + HasOwnerId`
  bound as `upload_router`. Consumers implement *one* identity-shape
  trait, not two.
- Crate-root re-exports grouped by tier (Essential / Storage / Auth /
  File ops / HTTP surfaces) with section-header comments — first-time
  readers can find the public API without grepping.
- `examples/serving/main.rs` now demonstrates both the merged
  `bucket.router()` flow and the granular three-call composition.

### Added — File-serving surface (carried over from earlier work)

- `ObjectStorage` trait (`storage::ObjectStorage`) — backend abstraction
  for `put` / `get` / `delete` / `head` / `presigned_get` /
  `presigned_put` / `public_url`.
- `LocalStorage` — filesystem backend with module-signed HMAC URLs.
  Suitable for development and single-node deployments.
- `S3Storage` — AWS S3 / MinIO backend via `aws-sdk-s3` with real
  SigV4 signing. Gated behind the default `s3` feature.
- `BucketConfig`, `StorageConfig`, `S3Config`, `ServingConfig`,
  `ServingMode` in `config::` — runtime config surface loaded through
  the existing Backbone config overlay chain.
- Pluggable authentication (`auth::AuthExtractor`) and authorization
  (`auth::AuthzPolicy`, `auth::AuthzDecision`, `DefaultOwnerOnlyPolicy`)
  traits. The module does not own identity or domain authz; consumers
  plug in their own extractor and policy.
- Mode-B serving handler (`presentation::http::serving`), exposed via
  `BucketModule::serving_router::<MyAuth>(policy)`. Responds in
  `Redirect`, `Stream`, or `SignedUrl` mode per `ServingConfig`.
- `FileService::upload` (auto-UUID key) and
  `FileService::upload_with_key` (caller-controlled key for
  human-readable paths like `public/product/image/slug.jpg`).
- `BucketError` — unified error enum with `From` into
  `backbone_core::ServiceError`.
- `BucketModule::crud_router()` alias, and new builder methods
  `.with_config(...)` and `.with_storage(...)`.
- `examples/serving/` — consumer-wiring demo that type-checks in CI.

### Changed

- `BucketModule` now carries optional `storage`, `bucket_config`, and
  `file_service` fields, plus a direct `stored_file_repository` handle
  needed by the serving handler's `find_by_storage_key` lookup.
- `Cargo.toml`: added `bytes`, `url`, and (feature-gated `s3`)
  `aws-config`, `aws-sdk-s3`, `aws-credential-types`. `s3` is in
  `default` features — MinIO-compatible SigV4 is the target deployment.
- Deprecated `CdnService` (HMAC-signed URLs not compatible with
  S3/MinIO clients). Will be removed in a later release.

### Security

- `BucketError::into_response` no longer leaks backend details in 500
  bodies. Internal-class errors (`S3`, `Io`, `Url`, `Config`, `Other`)
  now return a generic `"internal error"` body; the full error is
  captured via `tracing::error!` for operators.
- `validate_key` / `LocalStorage::resolve` check path segments for `..`
  instead of raw substring match, preventing legitimate filenames like
  `report..v2.pdf` from being rejected while still blocking traversal.

### Fixed

- S3 `get` / `head` now detect missing objects via the typed
  `GetObjectError::NoSuchKey` / `HeadObjectError::NotFound` variants
  instead of string-matching error messages.
- `S3Storage::head` safely handles negative `content_length` values via
  `u64::try_from` rather than a lossy `as u64` cast.
- Renamed the inline `humantime_serde` module to `duration_secs`; the
  previous name shadowed the published `humantime_serde` crate despite
  not accepting `"5m"` / `"1h"` strings.
- `FileService::upload_with_key` doc comment now matches behavior —
  public-prefixed keys log a debug event (not a hard rejection) when no
  public bucket is configured.

### Migration

Existing `/api/v1/bucket/*` CRUD endpoints are unchanged.

A single new migration ships in this release:

- `018_add_stored_files_storage_key_index.up.sql` — adds
  `UNIQUE INDEX idx_stored_files_storage_key ON stored_files
  (storage_key)`. Required by the mode-B serving handler, which looks
  files up by `storage_key` on every request. Without this index the
  lookup is a sequential scan and duplicate `storage_key` rows become
  possible.

- Replace any call-site of `cdn_service.get_or_generate_url(id, ...)`
  with `storage.presigned_get(&file.storage_key, ttl)?`.
- Wire `BucketModule::builder()` with `.with_config(...)` and
  `.with_storage(Arc::new(S3Storage::new(...)))` so that the new
  serving router is available.
- See `examples/serving/main.rs` for a copy-paste starting point.
