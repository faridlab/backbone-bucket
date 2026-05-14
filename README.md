# Bucket Module

**Domain:** File Storage & Content Management
**Type:** Backbone domain module (library crate)
**Architecture:** Domain-Driven Design — 4 layers (`domain` / `application` / `infrastructure` / `presentation`)
**Source of truth:** YAML schemas under [`schema/models/`](schema/models)

Most of this crate is generated from the schemas. Hand-written
additions live behind `// <<< CUSTOM` markers (for generated files) or
in sibling `*_custom.rs` files that the generator never touches.

---

## Table of contents

1. [What you get](#what-you-get)
2. [Module setup](#module-setup)
3. [Bucket usage — step by step](#bucket-usage--step-by-step)
   - [1. Create a bucket](#1-create-a-bucket)
   - [2. Read a bucket](#2-read-a-bucket)
   - [3. List & search buckets](#3-list--search-buckets)
   - [4. Update a bucket](#4-update-a-bucket)
   - [5. Soft-delete, restore, empty trash](#5-soft-delete-restore-empty-trash)
   - [6. State transitions](#6-state-transitions-lock--unlock--archive--)
   - [7. Bulk & upsert](#7-bulk--upsert)
   - [8. Counts](#8-counts)
4. [File upload (multipart HTTP)](#file-upload-multipart-http)
5. [File serving](#file-serving)
6. [Configuration](#configuration)
7. [Architecture](#architecture)
8. [Testing](#testing)
9. [Regeneration & custom code](#regeneration--custom-code)
10. [Documentation index](#documentation-index)

---

## What you get

| Surface | Description | Where it lives |
|---|---|---|
| **CRUD router** | 12+ standard endpoints per entity (list, create, get, update, patch, soft delete, bulk, upsert, trash, restore, empty, count) | `BucketModule::crud_router()` |
| **Upload router** | `multipart/form-data` HTTP entry — single-shot + resumable (initiate → parts → complete) | `BucketModule::upload_router::<A>()` |
| **Serving router** | Auth-aware delivery: 302 to presigned URL, byte stream, or JSON | `BucketModule::serving_router::<A>()` |
| **gRPC** (feature `grpc`) | Mirror of the CRUD surface | `presentation::grpc` |

**13 generated entities:** Bucket, StoredFile, FileVersion, FileShare,
FileLock, FileComment, ContentHash, UploadSession, ConversionJob,
ProcessingJob, Thumbnail, UserQuota, AccessLog.

**7 custom services** (regen-safe): `LockingService`,
`DeduplicationService`, `MultipartUploadService`, `ConversionService`,
`CdnService`, `VideoThumbnailService`, `DocumentPreviewService`.

---

## Module setup

```rust
use std::sync::Arc;
use sqlx::PgPool;
use backbone_bucket::{
    BucketModule, BucketConfig, StorageConfig, ServingConfig, ServingMode,
    UploadConfig,
    storage::{ObjectStorage, S3Storage, LocalStorage},
    auth::{HasOwnerId, AuthzPolicy, DefaultOwnerOnlyPolicy},
};
use url::Url;
use std::time::Duration;

// 1. Database
let pool: PgPool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

// 2. Storage backend — LocalStorage for dev, S3Storage in prod
let storage: Arc<dyn ObjectStorage> = Arc::new(
    LocalStorage::new(
        "/var/data/bucket".into(),
        Url::parse("http://localhost:3000/cdn/")?,
        "BUCKET_SIGNING_SECRET",   // env var name
    )?
);

// 3. Bucket-module runtime config
let config = BucketConfig {
    enabled: true,
    storage: StorageConfig::Local {
        root: "/var/data/bucket".into(),
        base_url: Url::parse("http://localhost:3000/cdn/")?,
        signing_secret_env: "BUCKET_SIGNING_SECRET".into(),
    },
    serving: ServingConfig {
        default_mode: ServingMode::Redirect,
        public_prefix: "public/".into(),
        presigned_ttl: Duration::from_secs(300),
    },
};

// 4. Build the module
let bucket = BucketModule::builder()
    .with_database(pool)
    .with_storage(storage)
    .with_config(config)
    .build()?;

// 5a. Simple wiring — one merged router (CRUD + upload + serving).
//     `MyUser` must implement Axum's `FromRequestParts` (so it's an
//     `AuthExtractor`) AND `HasOwnerId` (so `owner_id` is derived from
//     auth, never trusted from the request body).
let policy = Arc::new(DefaultOwnerOnlyPolicy);
let opts = RouterOptions::new(policy);
let app = axum::Router::new()
    .nest("/api/v1/bucket", bucket.router::<MyUser>(opts)?);

// 5b. Advanced wiring — mount each surface independently when you need
//     different prefixes, middleware, or auth policies per router.
let app = axum::Router::new()
    .nest("/api/v1/bucket", bucket.crud_router())
    .nest("/api/v1/bucket", bucket.upload_router::<MyUser>(UploadConfig::default())?)
    .nest("/cdn",           bucket.serving_router::<MyUser>(policy)?);
```

### Loading config from the environment

For first-time wiring, prefer `BucketConfig::from_env()` over
hand-building the struct:

```rust
// Reads BUCKET_STORAGE_BACKEND, BUCKET_STORAGE_ROOT, BUCKET_BASE_URL, …
// (S3 path) BUCKET_S3_ENDPOINT, BUCKET_S3_REGION, BUCKET_S3_PRIVATE_BUCKET, …
// See `BucketConfig::from_env` rustdoc for the full list.
let config = BucketConfig::from_env()?;
```

`BucketConfig::default()` produces a `LocalStorage` dev shape rooted at
`/tmp/bucket` with redirect-mode serving — useful for tests and the
example project.

---

## Bucket usage — step by step

All examples use `BASE = http://localhost:3000/api/v1/bucket` (matches
the `nest` above). Swap in your own prefix. Auth is whatever your
`AuthExtractor` requires; the examples assume `Authorization: Bearer
<jwt>`.

### Bucket fields (cheat sheet)

| Field | Type | Notes |
|---|---|---|
| `name` | string (req, ≤ 255) | Display name |
| `slug` | string (req, unique, ≤ 255, lowercased) | URL-safe identifier — **how you usually look it up** |
| `description` | string? | Free text |
| `owner_id` | uuid (req) | FK → User |
| `bucket_type` | enum | `user` (default) / `shared` / `system` / `temp` |
| `status` | enum | `active` (default) / `readonly` / `locked` / `archived` / `deleted` |
| `storage_backend` | enum | `local` (default) / `s3` / … |
| `root_path` | string (req, ≤ 1024) | Path inside the backend |
| `max_file_size` | int64? | bytes; `null` = no limit |
| `allowed_mime_types` | string[] | empty = all |
| `auto_delete_after_days` | int? | for `temp` buckets |
| `enable_cdn` / `enable_versioning` / `enable_deduplication` | bool | toggles |

Full schema: [schema/models/bucket.model.yaml](schema/models/bucket.model.yaml).

### 1. Create a bucket

```bash
curl -X POST "$BASE/buckets" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Product Images",
    "slug": "product-images",
    "description": "Public marketing imagery",
    "owner_id": "00000000-0000-0000-0000-000000000001",
    "bucket_type": "shared",
    "storage_backend": "s3",
    "root_path": "product-images",
    "enable_cdn": true,
    "enable_versioning": true,
    "enable_deduplication": true,
    "metadata": {}
  }'
```

Response: `201 Created` + the persisted bucket (UUID `id`, audit metadata, defaults filled in).

### 2. Read a bucket

By `id`:
```bash
curl -H "Authorization: Bearer $TOKEN" "$BASE/buckets/<id>"
```

By `slug` (no dedicated endpoint — slug is unique, so filter the list):
```bash
curl -H "Authorization: Bearer $TOKEN" \
  "$BASE/buckets?search=product-images&page_size=1"
```

### 3. List & search buckets

The list endpoint accepts these query parameters (all optional):

| Param | Default | Meaning |
|---|---|---|
| `page` | 1 | 1-based page index |
| `page_size` | 20 | Items per page (≤ 100) |
| `sort_by` | `created_at` | Column name |
| `sort_direction` | `desc` | `asc` / `desc` |
| `search` | — | Substring match against text fields (`name`, `slug`, `description`) |
| `status` | — | Filter by status enum |
| `tags` | — | Comma-separated tag filter |
| `created_by` | — | Filter by creator user id |

```bash
# Page 2, 50 per page, alphabetical by name
curl -H "Authorization: Bearer $TOKEN" \
  "$BASE/buckets?page=2&page_size=50&sort_by=name&sort_direction=asc"

# All active 'shared' buckets owned by a user, name contains "image"
curl -H "Authorization: Bearer $TOKEN" \
  "$BASE/buckets?status=active&search=image&created_by=00000000-0000-0000-0000-000000000001"
```

Response shape:
```json
{
  "success": true,
  "data": [ /* bucket objects */ ],
  "pagination": { "page": 2, "page_size": 50, "total": 137, "total_pages": 3 }
}
```

### 4. Update a bucket

Full update (`PUT` — send the whole representation):
```bash
curl -X PUT "$BASE/buckets/<id>" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{ "name": "Product Images (v2)", "slug": "product-images", "...": "..." }'
```

Partial update (`PATCH` — send only the changed fields):
```bash
curl -X PATCH "$BASE/buckets/<id>" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{ "enable_cdn": false, "max_file_size": 104857600 }'
```

### 5. Soft-delete, restore, empty trash

```bash
# Soft delete — sets metadata.deleted_at; bucket disappears from list
curl -X DELETE -H "Authorization: Bearer $TOKEN" "$BASE/buckets/<id>"

# Browse the trash
curl -H "Authorization: Bearer $TOKEN" "$BASE/buckets/trash"

# Bring it back
curl -X POST -H "Authorization: Bearer $TOKEN" "$BASE/buckets/<id>/restore"

# Permanent delete one bucket from trash
curl -X DELETE -H "Authorization: Bearer $TOKEN" "$BASE/buckets/trash/<id>"

# Permanent delete EVERYTHING in trash
curl -X DELETE -H "Authorization: Bearer $TOKEN" "$BASE/buckets/empty"
```

### 6. State transitions (lock / unlock / archive / …)

The Bucket state machine adds explicit transitions on top of CRUD.
They live under `/buckets/:id/transitions/<action>`:

| Action | Effect |
|---|---|
| `lock` | `active` → `locked` (no writes) |
| `unlock` | `locked` → `active` |
| `archive` | `active` → `archived` (read-only, hidden from default list) |
| `restore` | `archived` → `active` |
| `delete` | Triggers state-machine-managed delete (audited) |

```bash
curl -X POST -H "Authorization: Bearer $TOKEN" \
  "$BASE/buckets/<id>/transitions/archive"
```

### 7. Bulk & upsert

```bash
# Bulk create (one round-trip, transactional)
curl -X POST "$BASE/buckets/bulk" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{ "items": [ { "name": "A", "slug": "a", ... }, { "name": "B", "slug": "b", ... } ] }'

# Upsert by slug (or whatever unique key the schema declares)
curl -X POST "$BASE/buckets/upsert" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{ "slug": "product-images", "name": "Product Images", ... }'
```

### 8. Counts

```bash
curl -H "Authorization: Bearer $TOKEN" "$BASE/buckets/count"          # active
curl -H "Authorization: Bearer $TOKEN" "$BASE/buckets/trash/count"    # in trash
```

> The same 12-endpoint shape applies to every other entity: just swap
> `buckets` for `stored-files`, `file-shares`, `file-locks`,
> `upload-sessions`, etc. See the OpenAPI spec
> ([docs/openapi/bucket-v2.yaml](docs/openapi/bucket-v2.yaml)) for the
> per-entity field lists.

---

## File upload (multipart HTTP)

Two flows, both mounted by `upload_router::<A>(UploadConfig)`. `A` must
implement `AuthExtractor + HasOwnerId` — the authenticated identity
supplies `owner_id` (never trusted from the request body).

### Single-shot (one request)

```bash
curl -X POST "$BASE/uploads" \
  -H "Authorization: Bearer $TOKEN" \
  -F "bucket_id=<bucket-uuid>" \
  -F "path=images/2026/hero.png" \
  -F "file=@./hero.png;type=image/png"
```

Body limit defaults to 256 MiB (override via `UploadConfig::single_shot_limit`).

### Resumable (initiate → parts → complete)

```bash
# 1. Initiate
SESSION=$(curl -s -X POST "$BASE/uploads/sessions" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{
    "bucket_id": "<bucket-uuid>",
    "path": "videos/raw/large.mp4",
    "filename": "large.mp4",
    "mime_type": "video/mp4",
    "file_size": 1073741824,
    "chunk_size": 5242880
  }' | jq -r '.session_id')

# 2. Upload parts (1-based, in any order)
curl -X POST "$BASE/uploads/sessions/$SESSION/parts/1" \
  -H "Authorization: Bearer $TOKEN" \
  -F "chunk=@./part-1.bin;type=application/octet-stream"
# ... part 2, 3, …

# 3. Finalize — assembles, persists stored_files row, deletes staged parts
curl -X POST "$BASE/uploads/sessions/$SESSION/complete" \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" \
  -d '{}'

# Or abort
curl -X DELETE -H "Authorization: Bearer $TOKEN" \
  "$BASE/uploads/sessions/$SESSION"
```

Chunk limit defaults to 16 MiB; tune via `UploadConfig::chunk_limit`.

Full OpenAPI: see the `/uploads/*` operations in
[docs/openapi/bucket-v2.yaml](docs/openapi/bucket-v2.yaml).

---

## File serving

Mount `serving_router` to deliver bytes under pretty URLs:

```rust
let policy = Arc::new(DefaultOwnerOnlyPolicy);
let app = app.nest("/cdn", bucket.serving_router::<MyUser>(policy)?);
```

Three response strategies, configured via `ServingConfig::default_mode`:

| Mode | What the handler returns |
|---|---|
| `Redirect` (default) | `302` to a short-lived presigned URL |
| `Stream` | `200` with the bytes proxied through the service |
| `SignedUrl` | `200 {"url": "..."}` JSON |

Full reference (storage backends, key naming, public-prefix routing,
auth slots): [docs/serving.md](docs/serving.md).

---

## Configuration

### `application.yml`

```yaml
database:
  url: postgresql://root:password@localhost:5432/bucket
  max_connections: 20
  min_connections: 5

bucket:
  enabled: true
  storage:
    backend: local         # or s3
    root: /var/data/bucket
    base_url: http://localhost:3000/cdn/
    signing_secret_env: BUCKET_SIGNING_SECRET
  serving:
    default_mode: redirect # redirect | stream | signed_url
    public_prefix: "public/"
    presigned_ttl_secs: 300
```

### Environment

| Variable | Purpose |
|---|---|
| `DATABASE_URL` | Overrides the configured database URL |
| `BUCKET_SIGNING_SECRET` | HMAC secret for `LocalStorage` presigned URLs (name is configurable) |
| `AWS_*` | Picked up by `S3Storage` via `aws-config` |

Knobs and their defaults are documented in
[docs/CONFIGURATION_THRESHOLDS.md](docs/CONFIGURATION_THRESHOLDS.md).

---

## Architecture

```
bucket/
├── schema/                    SOURCE OF TRUTH
│   ├── models/*.model.yaml    entity DSL (generated → everything else)
│   ├── hooks/                 lifecycle hooks
│   ├── workflows/             saga / flow definitions
│   └── openapi/               generated index
│
├── src/
│   ├── domain/                entities, value objects, state machines, repos (traits)
│   ├── application/
│   │   ├── service/           {entity}_service.rs (type alias) + {entity}_service_custom.rs
│   │   ├── workflows/         multipart upload, file processing, share creation, …
│   │   └── triggers/ validator/ usecases/ events/
│   ├── infrastructure/
│   │   └── persistence/       newtype repos over GenericCrudRepository
│   ├── presentation/
│   │   └── http/
│   │       ├── {entity}_handler.rs   BackboneCrudHandler-driven CRUD
│   │       ├── serving.rs            custom — mode-B file delivery
│   │       └── upload.rs             custom — multipart HTTP upload
│   ├── storage/               ObjectStorage trait + Local/S3 backends (custom)
│   ├── auth/                  AuthExtractor / AuthzPolicy / HasOwnerId (custom)
│   ├── config/                BucketConfig (custom)
│   ├── bucket_module.rs       custom — crud_router / upload_router / serving_router
│   └── lib.rs                 generated re-exports + custom `// <<<` block
│
├── migrations/                NNN_*.up.sql / .down.sql
├── docs/                      human-maintained docs (this README + serving.md + bucket-spec.md …)
├── tests/                     integration + workflow + bench tests
└── examples/serving/          runnable wiring example
```

Why this shape: the [project CLAUDE.md](CLAUDE.md) has the
non-negotiable rules and naming conventions.

---

## Testing

```bash
# Unit tests (lib)
cargo test --lib

# Unit + handler-level tests for the upload module
cargo test --lib upload::

# HTTP integration suite — points reqwest at a running server
API_BASE_URL=http://localhost:3000 \
API_AUTH_TOKEN=$(jwt-mint) \
BUCKET_TEST_BUCKET_ID=00000000-0000-0000-0000-000000000001 \
  cargo test --test integration_tests
```

Integration tests skip (rather than fail) when `API_BASE_URL` is
unreachable, so they are safe to leave on in CI without a deployed
backend.

### In-process tests via `InMemoryStorage`

Consumers writing their own tests against the upload / serving routers
should avoid `LocalStorage` (filesystem) and S3 (credentials). Turn on
the `test-utils` feature instead — it ships an `InMemoryStorage` that
implements `ObjectStorage` against a `DashMap`:

```toml
# consumer Cargo.toml
[dev-dependencies]
backbone-bucket = { version = "*", features = ["test-utils"] }
```

```rust
use backbone_bucket::InMemoryStorage;
use std::sync::Arc;

let storage: Arc<dyn backbone_bucket::ObjectStorage> = Arc::new(InMemoryStorage::new());
// ... wire into BucketModule::builder().with_storage(storage)
```

`InMemoryStorage::len()`, `.contains_key(k)`, and `.clear()` are
available for assertions and per-test isolation. Presigned URLs are
synthetic (`memory://...`) and never call out to the network.

---

## Regeneration & custom code

Two rules keep custom code safe across regeneration:

1. **Inside generated files**, put hand-written code between
   `// <<< CUSTOM` and `// END CUSTOM` markers. Everything outside is
   overwritten by `metaphor schema generate` / `backbone schema generate`.
2. **For larger surfaces**, create a sibling file like
   `account_service_custom.rs` — files that don't match the generated
   filename pattern are never touched.

The custom-safe surfaces in this module today:

- [`src/bucket_module.rs`](src/bucket_module.rs) — `crud_router`, `upload_router`, `serving_router`
- [`src/presentation/http/upload.rs`](src/presentation/http/upload.rs) — multipart HTTP handler
- [`src/presentation/http/serving.rs`](src/presentation/http/serving.rs) — mode-B serving handler
- [`src/storage/`](src/storage/), [`src/auth/`](src/auth/), [`src/config/`](src/config/)
- `application/service/*_custom.rs` files
- `tests/integration/tests/upload_multipart_test.rs` (registered via a `// <<< CUSTOM` block in `tests/integration/tests/mod.rs`)

Regenerate everything:

```bash
metaphor schema validate
metaphor make entity <Name>           # add a new entity from schema
metaphor migration create <name>      # new migration
metaphor dev test                     # run the suite
metaphor lint check
```

---

## Documentation index

| File | What's inside |
|---|---|
| [`docs/README.md`](docs/README.md) | Doc map |
| [`docs/brd.md`](docs/brd.md) | Business requirements (917 lines) |
| [`docs/bucket-spec.md`](docs/bucket-spec.md) | Module specification |
| [`docs/bucket-plan.md`](docs/bucket-plan.md) | Implementation plan |
| [`docs/domain.md`](docs/domain.md) | Entity reference (generated from schemas) |
| [`docs/serving.md`](docs/serving.md) | File serving — storage + auth + URL shapes |
| [`docs/CONFIGURATION_THRESHOLDS.md`](docs/CONFIGURATION_THRESHOLDS.md) | Tunable defaults |
| [`docs/MIGRATION_V2.md`](docs/MIGRATION_V2.md) | V1 → V2 migration notes |
| [`docs/code-quality.md`](docs/code-quality.md) | Quality / lint conventions |
| [`docs/TRAIT_ABSTRACTION_ANALYSIS.md`](docs/TRAIT_ABSTRACTION_ANALYSIS.md) | Design rationale for the trait surface |
| [`docs/openapi/bucket-v2.yaml`](docs/openapi/bucket-v2.yaml) | Authoritative OpenAPI 3.0 spec (CRUD + uploads) |
| [`schema/models/`](schema/models) | Per-entity schema YAML (the actual source of truth) |
| [`CHANGELOG.md`](CHANGELOG.md) | Version history |
| [`CLAUDE.md`](CLAUDE.md) | Conventions / rules for AI-assisted edits |

---

## License

Part of the Backbone Framework.
