# Changelog

All notable changes to `backbone-bucket` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Adds the file-serving surface so consumer workspaces (e.g.
`bersihir-service`) can serve pretty-URL file downloads with auth
enforcement and real SigV4-signed S3/MinIO URLs. See
[`docs/serving.md`](docs/serving.md) for the reference.

### Added

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
