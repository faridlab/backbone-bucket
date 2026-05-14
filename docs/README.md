# Bucket Module — Documentation

This directory holds the human-maintained documentation for the
`backbone-bucket` bounded context. The root [`../README.md`](../README.md)
is the entry point — start there for setup, usage, and the API
walkthrough. Files here go deeper on specific concerns.

## Map

| File | Audience | What's inside |
|---|---|---|
| [brd.md](./brd.md) | Product / domain | Business requirements: who uses the module, why, what success looks like |
| [bucket-spec.md](./bucket-spec.md) | Engineers (design) | Module specification — capabilities, invariants, non-goals |
| [bucket-plan.md](./bucket-plan.md) | Engineers (build) | Implementation plan and roadmap |
| [domain.md](./domain.md) | Engineers (reference) | Entity / value-object / repository reference (regenerated from `schema/models/`) |
| [serving.md](./serving.md) | Engineers (integrate) | File serving: storage backends, URL shapes, pluggable auth, response modes (A/B/C) |
| [CONFIGURATION_THRESHOLDS.md](./CONFIGURATION_THRESHOLDS.md) | Operators | Tunable defaults — chunk sizes, presigned TTL, body limits, quotas |
| [MIGRATION_V2.md](./MIGRATION_V2.md) | Existing consumers | V1 → V2 upgrade notes |
| [code-quality.md](./code-quality.md) | Contributors | Lint / formatting / review conventions |
| [TRAIT_ABSTRACTION_ANALYSIS.md](./TRAIT_ABSTRACTION_ANALYSIS.md) | Engineers (design) | Why the public trait surface is shaped this way |
| [openapi/bucket-v2.yaml](./openapi/bucket-v2.yaml) | API consumers | **Authoritative** OpenAPI 3.0 spec — CRUD per entity, plus the custom `/uploads/*` multipart endpoints |
| [openapi/](./openapi) | — | OpenAPI source files (hand-maintained `bucket-v2.yaml`; `index.openapi.yaml` is generator output) |

## Quick links by task

**"How do I create / read / update / delete a bucket?"**
→ [../README.md#bucket-usage--step-by-step](../README.md#bucket-usage--step-by-step)

**"How do I upload a file?"**
→ [../README.md#file-upload-multipart-http](../README.md#file-upload-multipart-http)
or the OpenAPI operations under `/uploads` in
[openapi/bucket-v2.yaml](./openapi/bucket-v2.yaml).

**"How do I serve files?"**
→ [serving.md](./serving.md) (full surface)
or [../README.md#file-serving](../README.md#file-serving) (TL;DR).

**"What does entity X look like?"**
→ [domain.md](./domain.md) for the rendered reference, or
[`../schema/models/<entity>.model.yaml`](../schema/models) for the
authoritative source.

**"What are the upload size limits / chunk sizes / TTLs?"**
→ [CONFIGURATION_THRESHOLDS.md](./CONFIGURATION_THRESHOLDS.md).

**"How do I extend the module without losing my changes when regen runs?"**
→ [../README.md#regeneration--custom-code](../README.md#regeneration--custom-code)
and [../CLAUDE.md](../CLAUDE.md).

## What's generated vs. hand-written

| Status | Files | Regenerate with |
|---|---|---|
| **Hand-written** (this directory's `.md` files except `domain.md`, plus `openapi/bucket-v2.yaml`) | edited directly; never overwritten | — |
| **Generated** | `domain.md`, `openapi/index.openapi.yaml`, in-source code in `src/domain`, `src/application/service/*_service.rs` (non-custom), `src/presentation/http/*_handler.rs`, etc. | `backbone schema generate bucket --target <all\|docs\|openapi>` |

The OpenAPI **source of truth** is `openapi/bucket-v2.yaml` —
hand-maintained because it documents the custom upload and serving
endpoints in addition to the generated CRUD surface.
`openapi/index.openapi.yaml` is the generator's scaffold and is
overwritten on each regeneration.

## Module overview

**Bounded context:** `bucket` — file storage and content management.

**Entities (13):** Bucket, StoredFile, FileVersion, FileShare,
FileLock, FileComment, ContentHash, UploadSession, ConversionJob,
ProcessingJob, Thumbnail, UserQuota, AccessLog.

**Custom services (7):** `LockingService`, `DeduplicationService`,
`MultipartUploadService`, `ConversionService`, `CdnService`,
`VideoThumbnailService`, `DocumentPreviewService`.

**HTTP surfaces (three routers, mounted independently):**

1. `BucketModule::crud_router()` — 12+ generated CRUD endpoints per entity.
2. `BucketModule::upload_router::<A>(UploadConfig)` — multipart upload (single-shot + resumable).
3. `BucketModule::serving_router::<A>(policy)` — auth-aware file delivery.

For the full list of generated endpoints per entity, see the table at
[../README.md#what-you-get](../README.md#what-you-get).
