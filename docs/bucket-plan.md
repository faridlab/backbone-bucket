# Bucket Module - Implementation Plan V2.0

> **Module**: `bucket`
> **Version**: 2.0
> **Last Updated**: 2026-01-01
> **Status**: Implementation Plan

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Current State Analysis](#2-current-state-analysis)
3. [Implementation Phases](#3-implementation-phases)
4. [Schema Updates](#4-schema-updates)
5. [Code Generation Tasks](#5-code-generation-tasks)
6. [Custom Logic Implementation](#6-custom-logic-implementation)
7. [Testing Strategy](#7-testing-strategy)
8. [Migration Strategy](#8-migration-strategy)
9. [Dependencies & Risks](#9-dependencies--risks)

---

## 1. Executive Summary

### 1.1 Project Overview

Bucket is a comprehensive file storage management system that provides:
- S3-like object storage operations
- Google Drive-like sharing and collaboration
- Enterprise security with virus scanning
- Smart media processing (images, videos, documents)
- Storage optimization through deduplication

### 1.2 V2.0 Enhancement Goals

| Area | Current State | Target State | Impact |
|------|---------------|--------------|--------|
| **Media Processing** | Images only | Images + Videos + Documents | 3x more content types |
| **Collaboration** | Basic sharing | Locking + Comments | Team collaboration ready |
| **Storage** | Simple upload | Multipart + Deduplication | 40% storage savings |
| **Performance** | Direct storage | CDN integration | 10x faster downloads |

### 1.3 Success Criteria

- [x] All new schema model files created and validated
- [x] Zero compilation errors after code generation
- [x] All existing tests pass + new tests added (196 tests total)
- [ ] API endpoints functional and documented
- [x] Performance benchmarks meet targets

---

## 2. Current State Analysis

### 2.1 Existing Entities (8)

| Entity | Status | Notes |
|--------|--------|-------|
| `StoredFile` | ✅ Complete | Core file entity with scanning/compression |
| `Bucket` | ✅ Complete | Container with backend support |
| `FileShare` | ✅ Complete | Sharing with permissions |
| `UserQuota` | ✅ Complete | Per-user storage limits |
| `FileVersion` | ✅ Complete | Version history |
| `Thumbnail` | ✅ Complete | Image thumbnails |
| `FileAccessLog` | ✅ Complete | Access audit trail |
| `AccessLog` | ✅ Complete | General access logging |

### 2.2 Existing Schema Files

```
libs/modules/bucket/schema/models/
├── index.model.yaml          ✅ Module config, shared types, services
├── stored_file.model.yaml    ✅ File + FileAccessLog entities
├── bucket.model.yaml         ✅ Bucket entity with enums
├── file_share.model.yaml     ✅ Share entity with enums
├── user_quota.model.yaml     ✅ Quota entity with projections
├── file_version.model.yaml   ✅ Version history entity
├── thumbnail.model.yaml      ✅ Thumbnail entity
└── access_log.model.yaml     ✅ Access logging entity
```

### 2.3 Gaps Identified

| Gap | Impact | Priority |
|-----|--------|----------|
| No file locking mechanism | Concurrent edit conflicts | High |
| No comment/annotation system | Limited collaboration | High |
| No multipart upload support | Large file uploads fail | High |
| No deduplication tracking | Wasted storage | Medium |
| No video/document processing | Limited media support | Medium |
| No CDN integration | Slow downloads | Medium |
| No format conversion | Manual format changes | Low |

---

## 3. Implementation Phases

### Phase 1: Schema Foundation (Week 1)

**Goal**: Create all new schema model files

| Task | Output | Status |
|------|--------|--------|
| Create `file_lock.model.yaml` | Lock entity definition | Done |
| Create `file_comment.model.yaml` | Comment entity definition | Done |
| Create `processing_job.model.yaml` | Processing job entity | Done |
| Create `conversion_job.model.yaml` | Conversion job entity | Done |
| Create `upload_session.model.yaml` | Multipart session entity | Done |
| Create `content_hash.model.yaml` | Deduplication entity | Done |
| Update `index.model.yaml` | Add new imports | Done |
| Validate all schemas | Zero validation errors | Done |

### Phase 2: Enhanced StoredFile (Week 1-2)

**Goal**: Update existing entities with new V2.0 fields

| Task | Output | Status |
|------|--------|--------|
| Add deduplication fields to StoredFile | content_hash_id, has_video_thumbnail, etc. | Done |
| Add CDN fields to StoredFile | cdn_url, cdn_url_expires_at | Done |
| Add processing status enum | ProcessingStatus enum | Done |
| Update Bucket with CDN toggle | enable_cdn field | Done |

### Phase 3: Code Generation (Week 2)

**Goal**: Regenerate all code from updated schemas

| Task | Command | Status |
|------|---------|--------|
| Validate schemas | `backbone schema validate bucket` | Done |
| Generate proto files | `backbone schema generate --target proto bucket` | Done |
| Generate Rust code | `backbone schema generate --target rust bucket` | Done |
| Generate migrations | `backbone schema generate --target sql bucket` | Done |
| Generate repositories | `backbone schema generate --target repository bucket` | Done |
| Generate handlers | `backbone schema generate --target handler bucket` | Done |
| Generate services | `backbone schema generate --target service bucket` | Done |
| Generate all targets | `backbone schema generate --target all bucket` | Done |
| Verify compilation | `cargo check -p backbone-bucket` | Done |

### Phase 4: Custom Logic Implementation (Week 2-3)

**Goal**: Implement business logic in custom sections

| Task | File | Status |
|------|------|--------|
| Multipart upload service | `src/application/service/multipart_upload_service.rs` | Done |
| Video thumbnail processor | `src/application/service/video_thumbnail_service.rs` | Done |
| Document preview generator | `src/application/service/document_preview_service.rs` | Done |
| Format conversion service | `src/application/service/conversion_service.rs` | Done |
| File locking service | `src/application/service/locking_service.rs` | Done |
| Deduplication service | `src/application/service/deduplication_service.rs` | Done |
| CDN service | `src/application/service/cdn_service.rs` | Done |
| Custom repository extensions | `src/infrastructure/persistence/*_custom.rs` | Done |
| Module integration (lib.rs) | `src/lib.rs` — custom services wired into BucketModule | Done |

### Phase 5: Testing & Documentation (Week 3-4)

**Goal**: Comprehensive testing and documentation

| Task | Output | Status |
|------|--------|--------|
| Unit tests for new entities | `tests/domain_tests.rs` | Done |
| Integration tests for workflows | `tests/workflow_tests.rs` | Done |
| API endpoint tests | `tests/integration_tests.rs` | Done (generated) |
| Performance benchmarks | `tests/processing_bench.rs` | Done |
| Update API documentation | `docs/openapi/bucket-v2.yaml` | Done |
| Migration guide | `docs/MIGRATION_V2.md` | Done |

---

## 4. Schema Updates

### 4.1 New Schema Files to Create

#### File: `file_lock.model.yaml`

```yaml
# FileLock Entity - File editing locks for collaboration
# Prevents concurrent edits with automatic timeout

models:
  - name: FileLock
    collection: file_locks
    extends: [Metadata]

    fields:
      id:
        type: uuid
        attributes: ["@id", "@default(uuid)"]

      file_id:
        type: uuid
        attributes: ["@required", "@foreign_key(StoredFile.id)", "@unique"]

      user_id:
        type: uuid
        attributes: ["@required"]
        description: "Lock owner"

      locked_at:
        type: datetime
        attributes: ["@default(now)"]

      expires_at:
        type: datetime
        attributes: ["@required"]
        description: "Automatic lock release time"

      refreshed_at:
        type: datetime?
        description: "Last lock refresh"

    indexes:
      - type: unique
        fields: [file_id]
        description: "One lock per file"

      - type: index
        fields: [expires_at]
        description: "For cleanup of expired locks"

entities:
  FileLock:
    model: FileLock
    description: "File editing lock with timeout"

    methods:
      - name: is_valid
        returns: bool
        description: "Check if lock is still valid (not expired)"

      - name: is_owned_by
        params:
          user_id: Uuid
        returns: bool
        description: "Check if lock belongs to user"

      - name: refresh
        mutates: true
        params:
          duration: Duration
        description: "Refresh lock expiry"

    invariants:
      - "expires_at > locked_at"
      - "file_id is unique"
```

#### File: `file_comment.model.yaml`

```yaml
# FileComment Entity - Comments and annotations on files
# Supports threaded replies and region annotations

models:
  - name: FileComment
    collection: file_comments
    extends: [Metadata]

    fields:
      id:
        type: uuid
        attributes: ["@id", "@default(uuid)"]

      file_id:
        type: uuid
        attributes: ["@required", "@foreign_key(StoredFile.id)"]

      user_id:
        type: uuid
        attributes: ["@required"]
        description: "Comment author"

      parent_id:
        type: uuid?
        attributes: ["@foreign_key(FileComment.id)"]
        description: "Parent comment for replies"

      content:
        type: string
        attributes: ["@required", "@max(10000)"]
        description: "Comment text"

      annotation_region:
        type: json?
        description: "Region annotation: {x, y, width, height} in percentages"

      mentions:
        type: "uuid[]"
        description: "Mentioned users for notifications"

      resolved:
        type: bool
        attributes: ["@default(false)"]

      resolved_by:
        type: uuid?
        description: "User who resolved comment"

      resolved_at:
        type: datetime?

    indexes:
      - type: index
        fields: [file_id]

      - type: index
        fields: [parent_id]
        description: "For reply lookups"

      - type: index
        fields: [user_id]

enums:
  - name: CommentStatus
    description: "Comment status"
    variants:
      - name: active
        description: "Active comment"
        default: true
      - name: resolved
        description: "Resolved by owner"
      - name: archived
        description: "Archived comment"

entities:
  FileComment:
    model: FileComment
    description: "File comment with threading and annotations"

    methods:
      - name: is_reply
        returns: bool
        description: "Check if this is a reply to another comment"

      - name: has_annotations
        returns: bool
        description: "Check if comment has region annotations"

      - name: resolve
        mutates: true
        params:
          by_user_id: Uuid
        description: "Mark comment as resolved"
```

#### File: `processing_job.model.yaml`

```yaml
# ProcessingJob Entity - Async file processing tasks
# Handles thumbnails, video processing, document previews

models:
  - name: ProcessingJob
    collection: processing_jobs
    extends: [Metadata]

    fields:
      id:
        type: uuid
        attributes: ["@id", "@default(uuid)"]

      file_id:
        type: uuid
        attributes: ["@required", "@foreign_key(StoredFile.id)"]

      job_type:
        type: ProcessingJobType
        attributes: ["@required"]

      status:
        type: JobStatus
        attributes: ["@default(pending)"]

      priority:
        type: int
        attributes: ["@default(0)"]
        description: "Higher values = higher priority"

      input_data:
        type: json?
        description: "Job parameters"

      result_data:
        type: json?
        description: "Job output data"

      error_message:
        type: string?

      started_at:
        type: datetime?

      completed_at:
        type: datetime?

      retry_count:
        type: int
        attributes: ["@default(0)", "@non_negative"]

      max_retries:
        type: int
        attributes: ["@default(3)"]

    indexes:
      - type: index
        fields: [file_id, status]

      - type: index
        fields: [status, priority]
        description: "For job queue queries"

      - type: index
        fields: [created_at]

enums:
  - name: ProcessingJobType
    description: "Type of processing job"
    variants:
      - name: thumbnail_generation
        description: "Generate image thumbnails"
        default: true
      - name: video_thumbnail
        description: "Extract video thumbnail frame"
      - name: document_preview
        description: "Generate document preview"
      - name: compression
        description: "Compress file"
      - name: virus_scan
        description: "Scan for threats"
      - name: deduplication_check
        description: "Check for duplicate content"

  - name: JobStatus
    description: "Processing job status"
    variants:
      - name: pending
        description: "Queued for processing"
        default: true
      - name: running
        description: "Currently processing"
      - name: completed
        description: "Finished successfully"
      - name: failed
        description: "Failed with error"
      - name: cancelled
        description: "Cancelled by user"

entities:
  ProcessingJob:
    model: ProcessingJob
    description: "Async processing job with retry logic"

    methods:
      - name: can_retry
        returns: bool
        description: "Check if job can be retried"

      - name: increment_retry
        mutates: true
        description: "Increment retry count"

      - name: mark_started
        mutates: true
        description: "Mark job as started"

      - name: mark_completed
        mutates: true
        params:
          result: json
        description: "Mark job as completed with result"

      - name: mark_failed
        mutates: true
        params:
          error: string
        description: "Mark job as failed"
```

#### File: `conversion_job.model.yaml`

```yaml
# ConversionJob Entity - Format conversion tasks
# Converts between image, video, and document formats

models:
  - name: ConversionJob
    collection: conversion_jobs
    extends: [Metadata]

    fields:
      id:
        type: uuid
        attributes: ["@id", "@default(uuid)"]

      source_file_id:
        type: uuid
        attributes: ["@required", "@foreign_key(StoredFile.id)"]

      target_format:
        type: string
        attributes: ["@required", "@max(50)"]
        description: "Target format (e.g., webp, pdf, mp4)"

      status:
        type: ConversionStatus
        attributes: ["@default(pending)"]

      conversion_options:
        type: json?
        description: "Conversion parameters (quality, resolution, etc.)"

      result_file_id:
        type: uuid?
        attributes: ["@foreign_key(StoredFile.id)"]
        description: "Converted file"

      progress:
        type: int
        attributes: ["@default(0)", "@range(0,100)"]

      error_message:
        type: string?

      started_at:
        type: datetime?

      completed_at:
        type: datetime?

    indexes:
      - type: index
        fields: [source_file_id, status]

      - type: index
        fields: [result_file_id]

      - type: index
        fields: [status, created_at]

enums:
  - name: ConversionStatus
    description: "Conversion job status"
    variants:
      - name: pending
        description: "Queued for conversion"
        default: true
      - name: processing
        description: "Converting"
      - name: completed
        description: "Conversion successful"
      - name: failed
        description: "Conversion failed"

entities:
  ConversionJob:
    model: ConversionJob
    description: "Format conversion job"

    methods:
      - name: update_progress
        mutates: true
        params:
          progress: int
        description: "Update conversion progress (0-100)"

      - name: complete
        mutates: true
        params:
          result_file_id: Uuid
        description: "Mark conversion complete"

      - name: fail
        mutates: true
        params:
          error: string
        description: "Mark conversion failed"
```

#### File: `upload_session.model.yaml`

```yaml
# UploadSession Entity - Multipart upload sessions
# Manages large file uploads with chunking

models:
  - name: UploadSession
    collection: upload_sessions
    extends: [Metadata]

    fields:
      id:
        type: uuid
        attributes: ["@id", "@default(uuid)"]

      bucket_id:
        type: uuid
        attributes: ["@required", "@foreign_key(Bucket.id)"]

      user_id:
        type: uuid
        attributes: ["@required"]

      path:
        type: string
        attributes: ["@required", "@max(1024)"]

      filename:
        type: string
        attributes: ["@required", "@max(255)"]

      mime_type:
        type: string?
        attributes: ["@max(127)"]

      file_size:
        type: int64
        attributes: ["@required", "@positive"]

      chunk_size:
        type: int
        attributes: ["@required", "@positive"]
        description: "Size of each chunk in bytes"

      total_chunks:
        type: int
        attributes: ["@required", "@positive"]

      uploaded_chunks:
        type: int
        attributes: ["@default(0)", "@non_negative"]

      status:
        type: UploadStatus
        attributes: ["@default(initiated)"]

      storage_backend:
        type: StorageBackend
        attributes: ["@default(local)"]

      # Chunk tracking (array of uploaded chunk numbers)
      completed_parts:
        type: "int[]"
        attributes: ["@distinct"]
        description: "List of completed part numbers"

      # ETags for verification (array of {part_number, etag})
      part_etags:
        type: json?
        description: "Part ETags for final assembly"

      expires_at:
        type: datetime
        attributes: ["@required"]
        description: "Session expiration time"

    indexes:
      - type: index
        fields: [user_id, status]

      - type: index
        fields: [expires_at]
        description: "For cleanup of expired sessions"

      - type: index
        fields: [bucket_id, status]

enums:
  - name: UploadStatus
    description: "Multipart upload status"
    variants:
      - name: initiated
        description: "Session created"
        default: true
      - name: uploading
        description: "Receiving chunks"
      - name: completing
        description: "Assembling final file"
      - name: completed
        description: "Upload complete"
      - name: expired
        description: "Session expired"
      - name: failed
        description: "Upload failed"

entities:
  UploadSession:
    model: UploadSession
    description: "Multipart upload session"

    methods:
      - name: is_expired
        returns: bool
        description: "Check if session has expired"

      - name: add_part
        mutates: true
        params:
          part_number: int
          etag: string
        description: "Add completed part"

      - name: is_complete
        returns: bool
        description: "Check if all parts uploaded"

      - name: calculate_progress
        returns: int
        description: "Get upload progress percentage"
```

#### File: `content_hash.model.yaml`

```yaml
# ContentHash Entity - Content deduplication tracking
# SHA-256 based deduplication with reference counting

models:
  - name: ContentHash
    collection: content_hashes
    extends: [Metadata]

    fields:
      id:
        type: uuid
        attributes: ["@id", "@default(uuid)"]

      hash:
        type: string
        attributes: ["@unique", "@required", "@length(64)", "@lowercase"]
        description: "SHA-256 hash (hex encoded)"

      size_bytes:
        type: int64
        attributes: ["@required", "@positive"]

      storage_key:
        type: string
        attributes: ["@required", "@max(1024)"]
        description: "Location in storage backend"

      storage_backend:
        type: StorageBackend
        attributes: ["@default(local)"]

      reference_count:
        type: int
        attributes: ["@required", "@min(1)", "@default(1)"]
        description: "Number of files using this content"

      first_uploaded_at:
        type: datetime
        attributes: ["@default(now)"]

      last_referenced_at:
        type: datetime
        attributes: ["@default(now)", "@updated_at"]

      # Optional: Content fingerprint for partial deduplication
      fingerprint:
        type: string?
        attributes: ["@max(128)"]
        description: "Partial content hash for similarity detection"

    indexes:
      - type: unique
        fields: [hash]
        description: "Fast hash lookup"

      - type: index
        fields: [reference_count]
        description: "For cleanup of unused content"

      - type: index
        fields: [last_referenced_at]

      - type: index
        fields: [fingerprint]
        where: "fingerprint IS NOT NULL"
        description: "For similarity searches"

entities:
  ContentHash:
    model: ContentHash
    description: "Content hash with reference counting"

    methods:
      - name: increment_reference
        mutates: true
        description: "Increment reference count"

      - name: decrement_reference
        mutates: true
        returns: bool
        description: "Decrement, return true if should delete"

      - name: can_delete
        returns: bool
        description: "Check if content can be deleted (ref_count = 0)"

      - name: is_unused
        returns: bool
        description: "Check if content has no references"

      - name: storage_saved
        returns: i64
        description: "Calculate storage saved (size * (ref_count - 1))"
```

### 4.2 Updates to Existing Schema Files

#### Update: `stored_file.model.yaml`

Add new fields to StoredFile:

```yaml
# Add to fields section:
content_hash_id:
  type: uuid?
  attributes: ["@foreign_key(ContentHash.id)"]
  description: "For deduplication"

has_video_thumbnail:
  type: bool
  attributes: ["@default(false)"]
  description: "Video thumbnail exists"

has_document_preview:
  type: bool
  attributes: ["@default(false)"]
  description: "Document preview exists"

processing_status:
  type: ProcessingStatus?
  description: "Overall processing status"

cdn_url:
  type: string?
  description: "Cached CDN URL"

cdn_url_expires_at:
  type: datetime?
  description: "CDN URL expiration"

# Add to relations:
content_hash:
  type: ContentHash
  attributes: ["@one", "@foreign_key(content_hash_id)"]
```

Add new enum:

```yaml
enums:
  - name: ProcessingStatus
    description: "File processing status"
    variants:
      - name: pending
        description: "Processing queued"
        default: true
      - name: processing
        description: "Processing in progress"
      - name: thumbnails_ready
        description: "Thumbnails generated"
      - name: scan_complete
        description: "Virus scan complete"
      - name: complete
        description: "All processing complete"
      - name: failed
        description: "Processing failed"
```

#### Update: `bucket.model.yaml`

Add new fields:

```yaml
# Add to fields:
enable_cdn:
  type: bool
  attributes: ["@default(false)"]
  description: "Enable CDN for public files"

enable_versioning:
  type: bool
  attributes: ["@default(true)"]
  description: "Enable file versioning"

enable_deduplication:
  type: bool
  attributes: ["@default(true)"]
  description: "Enable content deduplication"
```

#### Update: `index.model.yaml`

Add imports:

```yaml
imports:
  - bucket.model.yaml
  - stored_file.model.yaml
  - file_share.model.yaml
  - user_quota.model.yaml
  - file_version.model.yaml
  - thumbnail.model.yaml
  - access_log.model.yaml
  # New imports:
  - file_lock.model.yaml
  - file_comment.model.yaml
  - processing_job.model.yaml
  - conversion_job.model.yaml
  - upload_session.model.yaml
  - content_hash.model.yaml
```

Add new domain services:

```yaml
domain_services:
  LockingService:
    description: "Manages file editing locks"
    stateless: false
    dependencies:
      - file_lock_repo: FileLockRepository
      - stored_file_repo: StoredFileRepository
    methods:
      - name: acquire_lock
        async: true
        params:
          file_id: Uuid
          user_id: Uuid
          duration: Duration
        returns: "Result<FileLock, LockError>"
      - name: release_lock
        async: true
        params:
          file_id: Uuid
          user_id: Uuid
        returns: "Result<(), LockError>"
      - name: refresh_lock
        async: true
        params:
          file_id: Uuid
          user_id: Uuid
        returns: "Result<FileLock, LockError>"
      - name: break_lock
        async: true
        params:
          file_id: Uuid
          admin_id: Uuid
        returns: "Result<(), LockError>"

  DeduplicationService:
    description: "Content-based deduplication"
    stateless: true
    dependencies:
      - content_hash_repo: ContentHashRepository
      - storage_backend: StorageBackend
    methods:
      - name: find_or_create
        async: true
        params:
          hash: String
          size: i64
          content: Vec<u8>
        returns: "Result<ContentHash, DedupError>"
      - name: cleanup_orphaned
        async: true
        params:
          older_than: Duration
        returns: "Result<Vec<Uuid>, DedupError>"

  ConversionService:
    description: "File format conversion"
    stateless: true
    dependencies:
      - storage: StorageBackend
      - processor: MediaProcessor
    methods:
      - name: convert_image
        async: true
        params:
          source: Vec<u8>
          from_format: String
          to_format: String
          options: ConversionOptions
        returns: "Result<Vec<u8>, ConversionError>"
      - name: convert_document
        async: true
        params:
          source: Vec<u8>
          to_format: String
        returns: "Result<Vec<u8>, ConversionError>"

  CdnService:
    description: "CDN URL generation and caching"
    stateless: true
    dependencies:
      - cdn_provider: CdnProvider
      - config: CdnConfig
    methods:
      - name: get_or_generate_url
        async: true
        params:
          file: StoredFile
          expiry: Duration
        returns: "Result<String, CdnError>"
      - name: invalidate
        async: true
        params:
          file_id: Uuid
        returns: "Result<(), CdnError>"
```

Add new events:

```yaml
events:
  # ... existing events ...

  FileLocked:
    description: "File was locked for editing"
    aggregate: StoredFile
    version: 1
    fields:
      - name: file_id
        type: Uuid
      - name: user_id
        type: Uuid
      - name: locked_at
        type: DateTime<Utc>
      - name: expires_at
        type: DateTime<Utc>

  FileUnlocked:
    description: "File lock was released"
    aggregate: StoredFile
    version: 1
    fields:
      - name: file_id
        type: Uuid
      - name: user_id
        type: Uuid
      - name: unlocked_at
        type: DateTime<Utc>

  FileCommented:
    description: "Comment added to file"
    aggregate: FileComment
    version: 1
    fields:
      - name: comment_id
        type: Uuid
      - name: file_id
        type: Uuid
      - name: user_id
        type: Uuid
      - name: content
        type: String
      - name: mentioned_users
        type: Vec<Uuid>
      - name: commented_at
        type: DateTime<Utc>

  ConversionCompleted:
    description: "File conversion completed"
    aggregate: ConversionJob
    version: 1
    fields:
      - name: conversion_id
        type: Uuid
      - name: source_file_id
        type: Uuid
      - name: result_file_id
        type: Uuid
      - name: target_format
        type: String
      - name: completed_at
        type: DateTime<Utc>
```

---

## 5. Code Generation Tasks

### 5.1 Generation Commands

```bash
# 1. Validate all schemas
backbone schema validate bucket

# 2. Generate proto files (gRPC definitions)
backbone schema generate --target proto bucket

# 3. Generate Rust entities
backbone schema generate --target rust bucket

# 4. Generate SQL migrations
backbone schema generate --target sql bucket

# 5. Generate repository traits
backbone schema generate --target repository-trait bucket

# 6. Generate repository implementations
backbone schema generate --target repository bucket

# 7. Generate domain services
backbone schema generate --target domain-service bucket

# 8. Generate application services
backbone schema generate --target service bucket

# 9. Generate REST handlers
backbone schema generate --target handler bucket

# 10. Generate gRPC handlers
backbone schema generate --target grpc bucket

# 11. Generate OpenAPI specs
backbone schema generate --target openapi bucket

# 12. Generate all at once (recommended)
backbone schema generate --target all bucket

# 13. Verify compilation
cargo check -p backbone-bucket
```

### 5.2 Expected Generated Files

```
libs/modules/bucket/
├── proto/domain/entity/
│   ├── file_lock.proto              # NEW
│   ├── file_comment.proto           # NEW
│   ├── processing_job.proto         # NEW
│   ├── conversion_job.proto         # NEW
│   ├── upload_session.proto         # NEW
│   ├── content_hash.proto           # NEW
│   └── processing_status.proto      # NEW
│
├── src/domain/entity/
│   ├── file_lock.rs                 # NEW
│   ├── file_comment.rs              # NEW
│   ├── processing_job.rs            # NEW
│   ├── conversion_job.rs            # NEW
│   ├── upload_session.rs            # NEW
│   ├── content_hash.rs              # NEW
│   └── processing_status.rs         # NEW
│
├── src/domain/repository/
│   ├── file_lock_repository.rs      # NEW
│   ├── file_comment_repository.rs   # NEW
│   └── content_hash_repository.rs   # NEW
│
├── src/infrastructure/persistence/postgres/
│   ├── file_lock_repository.rs      # NEW
│   ├── file_comment_repository.rs   # NEW
│   └── content_hash_repository.rs   # NEW
│
├── src/domain/service/
│   ├── locking_service.rs           # NEW (custom)
│   ├── conversion_service.rs        # NEW (custom)
│   ├── deduplication_service.rs     # NEW (custom)
│   └── cdn_service.rs               # NEW (custom)
│
├── src/application/handlers/
│   ├── multipart_upload_handler.rs  # NEW (custom)
│   └── conversion_handler.rs        # NEW (custom)
│
└── migrations/
    └── XXXX_create_file_locks.sql        # NEW
    └── XXXX_create_file_comments.sql     # NEW
    └── XXXX_create_processing_jobs.sql   # NEW
    └── XXXX_create_conversion_jobs.sql   # NEW
    └── XXXX_create_upload_sessions.sql   # NEW
    └── XXXX_create_content_hashes.sql    # NEW
    └── XXXX_update_stored_files_for_v2.sql  # NEW
```

---

## 6. Custom Logic Implementation

### 6.1 File Locking Service

**Location**: `src/domain/services/locking_service.rs`

```rust
// <<< CUSTOM
use async_trait::async_trait;
use crate::domain::entity::{FileLock, StoredFile};
use crate::domain::repository::{FileLockRepository, StoredFileRepository};
use crate::domain::error::BucketError;
use std::time::Duration;
use uuid::Uuid;

#[async_trait]
pub trait LockingService: Send + Sync {
    async fn acquire_lock(&self, file_id: Uuid, user_id: Uuid, duration: Duration)
        -> Result<FileLock, BucketError>;
    async fn release_lock(&self, file_id: Uuid, user_id: Uuid) -> Result<(), BucketError>;
    async fn refresh_lock(&self, file_id: Uuid, user_id: Uuid) -> Result<FileLock, BucketError>;
    async fn break_lock(&self, file_id: Uuid, admin_id: Uuid) -> Result<(), BucketError>;
    async fn get_active_lock(&self, file_id: Uuid) -> Result<Option<FileLock>, BucketError>;
}

pub struct LockingServiceImpl<L, F>
where
    L: FileLockRepository,
    F: StoredFileRepository,
{
    lock_repo: L,
    file_repo: F,
    default_lock_duration: Duration,
}

impl<L, F> LockingServiceImpl<L, F>
where
    L: FileLockRepository,
    F: StoredFileRepository,
{
    pub fn new(lock_repo: L, file_repo: F, default_lock_duration: Duration) -> Self {
        Self {
            lock_repo,
            file_repo,
            default_lock_duration,
        }
    }

    async fn is_lock_expired(&self, lock: &FileLock) -> bool {
        chrono::Utc::now() > lock.expires_at
    }
}

#[async_trait]
impl<L, F> LockingService for LockingServiceImpl<L, F>
where
    L: FileLockRepository + Send + Sync,
    F: StoredFileRepository + Send + Sync,
{
    async fn acquire_lock(&self, file_id: Uuid, user_id: Uuid, duration: Duration)
        -> Result<FileLock, BucketError>
    {
        // Check if file exists
        let file = self.file_repo.find_by_id(file_id).await?
            .ok_or_else(|| BucketError::NotFound(file_id.to_string()))?;

        // Check existing lock
        if let Some(existing_lock) = self.lock_repo.find_by_file(file_id).await? {
            if self.is_lock_expired(&existing_lock) {
                // Auto-release expired lock
                self.lock_repo.delete(&existing_lock.id).await?;
            } else if existing_lock.user_id != user_id {
                return Err(BucketError::FileLocked {
                    file_id,
                    locked_by: existing_lock.user_id,
                    expires_at: existing_lock.expires_at,
                });
            } else {
                // Same user, just refresh
                return self.refresh_lock(file_id, user_id).await;
            }
        }

        // Create new lock
        let expires_at = chrono::Utc::now() + chrono::Duration::from_std(duration)
            .map_err(|_| BucketError::InvalidDuration)?;

        let lock = FileLock {
            id: Uuid::new_v4(),
            file_id,
            user_id,
            locked_at: chrono::Utc::now(),
            expires_at,
            refreshed_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.lock_repo.create(&lock).await?;
        Ok(lock)
    }

    async fn release_lock(&self, file_id: Uuid, user_id: Uuid) -> Result<(), BucketError> {
        if let Some(lock) = self.lock_repo.find_by_file(file_id).await? {
            if lock.user_id == user_id {
                self.lock_repo.delete(&lock.id).await?;
            } else {
                return Err(BucketError::PermissionDenied(lock.user_id));
            }
        }
        Ok(())
    }

    async fn refresh_lock(&self, file_id: Uuid, user_id: Uuid) -> Result<FileLock, BucketError> {
        let mut lock = self.lock_repo.find_by_file(file_id).await?
            .ok_or_else(|| BucketError::LockNotFound(file_id))?;

        if lock.user_id != user_id {
            return Err(BucketError::PermissionDenied(lock.user_id));
        }

        lock.expires_at = chrono::Utc::now() + chrono::Duration::from_std(self.default_lock_duration)
            .map_err(|_| BucketError::InvalidDuration)?;
        lock.refreshed_at = Some(chrono::Utc::now());
        lock.updated_at = chrono::Utc::now();

        self.lock_repo.update(&lock).await?;
        Ok(lock)
    }

    async fn break_lock(&self, file_id: Uuid, admin_id: Uuid) -> Result<(), BucketError> {
        if let Some(lock) = self.lock_repo.find_by_file(file_id).await? {
            self.lock_repo.delete(&lock.id).await?;
        }
        Ok(())
    }

    async fn get_active_lock(&self, file_id: Uuid) -> Result<Option<FileLock>, BucketError> {
        if let Some(lock) = self.lock_repo.find_by_file(file_id).await? {
            if self.is_lock_expired(&lock) {
                self.lock_repo.delete(&lock.id).await?;
                return Ok(None);
            }
            return Ok(Some(lock));
        }
        Ok(None)
    }
}
```

### 6.2 Deduplication Service

**Location**: `src/domain/services/deduplication_service.rs`

```rust
// <<< CUSTOM
use async_trait::async_trait;
use sha2::{Sha256, Digest};
use crate::domain::entity::ContentHash;
use crate::domain::repository::{ContentHashRepository, StoredFileRepository};
use crate::domain::error::BucketError;
use uuid::Uuid;

#[async_trait]
pub trait DeduplicationService: Send + Sync {
    async fn find_or_create(&self, hash: String, size: i64, content: Vec<u8>)
        -> Result<ContentHash, BucketError>;
    async fn increment_reference(&self, hash_id: Uuid) -> Result<(), BucketError>;
    async fn decrement_reference(&self, hash_id: Uuid) -> Result<bool, BucketError>;
    async fn cleanup_orphaned(&self, older_than: chrono::Duration)
        -> Result<Vec<Uuid>, BucketError>;
}

pub struct DeduplicationServiceImpl<H, S>
where
    H: ContentHashRepository,
    S: StorageBackend,
{
    hash_repo: H,
    storage: S,
}

impl<H, S> DeduplicationServiceImpl<H, S>
where
    H: ContentHashRepository,
    S: StorageBackend,
{
    pub fn new(hash_repo: H, storage: S) -> Self {
        Self { hash_repo, storage }
    }

    fn compute_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
}
```

### 6.3 Multipart Upload Handler

**Location**: `src/application/handlers/multipart_upload_handler.rs`

```rust
// <<< CUSTOM
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::application::dtos::{InitiateMultipartRequest, UploadSessionResponse, CompleteMultipartRequest};

pub async fn initiate_multipart_upload(
    State(handler): State<MultipartUploadHandler>,
    Json(req): Json<InitiateMultipartRequest>,
) -> Result<Json<UploadSessionResponse>, BucketError> {
    handler.initiate(req).await
}

pub async fn upload_chunk(
    State(handler): State<MultipartUploadHandler>,
    Path(session_id): Path<Uuid>,
    Path(part_number): Path<u32>,
    body: Vec<u8>,
) -> Result<Json<PartETagResponse>, BucketError> {
    handler.upload_chunk(session_id, part_number, body).await
}

pub async fn complete_multipart_upload(
    State(handler): State<MultipartUploadHandler>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<CompleteMultipartRequest>,
) -> Result<Json<StoredFileResponse>, BucketError> {
    handler.complete(session_id, req).await
}
```

---

## 7. Testing Strategy

### 7.1 Unit Tests

**Location**: `tests/domain_tests.rs`

Add tests for new entities and services:

```rust
#[cfg(test)]
mod file_lock_tests {
    use super::*;

    #[tokio::test]
    async fn test_acquire_lock_success() {
        // Test successful lock acquisition
    }

    #[tokio::test]
    async fn test_lock_prevents_concurrent_edit() {
        // Test that second user cannot acquire locked file
    }

    #[tokio::test]
    async fn test_lock_expiry() {
        // Test that expired locks can be acquired
    }

    #[tokio::test]
    async fn test_admin_can_break_lock() {
        // Test admin override capability
    }
}

#[cfg(test)]
mod deduplication_tests {
    use super::*;

    #[tokio::test]
    async fn test_duplicate_detection() {
        // Test SHA-256 based duplicate detection
    }

    #[tokio::test]
    async fn test_reference_counting() {
        // Test reference count increments/decrements
    }

    #[tokio::test]
    async fn test_orphan_cleanup() {
        // Test cleanup of content with ref_count = 0
    }
}
```

### 7.2 Integration Tests

**Location**: `tests/integration_tests.rs`

```rust
#[tokio::test]
async fn test_multipart_upload_workflow() {
    // Test complete multipart upload flow:
    // 1. Initiate session
    // 2. Upload chunks
    // 3. Complete upload
    // 4. Verify file created
}

#[tokio::test]
async fn test_file_conversion_workflow() {
    // Test format conversion:
    // 1. Upload image
    // 2. Request conversion to WebP
    // 3. Verify new file created
    // 4. Verify version linked
}
```

---

## 8. Migration Strategy

### 8.1 Database Migration

**New Migration**: `migrations/XXXX_update_bucket_for_v2.sql`

```sql
-- ============================================================
-- Bucket V2.0 Migration
-- Adds: file locks, comments, processing jobs, deduplication, CDN
-- ============================================================

-- New enums
DO $$ BEGIN
    CREATE TYPE processing_status AS ENUM (
        'pending', 'processing', 'thumbnails_ready',
        'scan_complete', 'complete', 'failed'
    );

    CREATE TYPE job_status AS ENUM (
        'pending', 'running', 'completed', 'failed', 'cancelled'
    );

    CREATE TYPE processing_job_type AS ENUM (
        'thumbnail_generation', 'video_thumbnail',
        'document_preview', 'compression', 'virus_scan',
        'deduplication_check'
    );

    CREATE TYPE conversion_status AS ENUM (
        'pending', 'processing', 'completed', 'failed'
    );

    CREATE TYPE upload_status AS ENUM (
        'initiated', 'uploading', 'completing', 'completed',
        'expired', 'failed'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Add new columns to stored_files
ALTER TABLE stored_files
    ADD COLUMN IF NOT EXISTS content_hash_id UUID REFERENCES content_hashes(id),
    ADD COLUMN IF NOT EXISTS has_video_thumbnail BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS has_document_preview BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS processing_status processing_status,
    ADD COLUMN IF NOT EXISTS cdn_url TEXT,
    ADD COLUMN IF NOT EXISTS cdn_url_expires_at TIMESTAMPTZ;

-- Add new columns to buckets
ALTER TABLE buckets
    ADD COLUMN IF NOT EXISTS enable_cdn BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS enable_versioning BOOLEAN DEFAULT true,
    ADD COLUMN IF NOT EXISTS enable_deduplication BOOLEAN DEFAULT true;

-- Create new tables
CREATE TABLE IF NOT EXISTS file_locks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL REFERENCES stored_files(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    locked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    refreshed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (file_id)
);

CREATE TABLE IF NOT EXISTS file_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL REFERENCES stored_files(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    parent_id UUID REFERENCES file_comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL CHECK (LENGTH(content) <= 10000),
    annotation_region JSONB,
    mentions UUID[],
    resolved BOOLEAN DEFAULT false,
    resolved_by UUID,
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS processing_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL REFERENCES stored_files(id) ON DELETE CASCADE,
    job_type processing_job_type NOT NULL,
    status job_status NOT NULL DEFAULT 'pending',
    priority INT DEFAULT 0,
    input_data JSONB,
    result_data JSONB,
    error_message TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    retry_count INT DEFAULT 0,
    max_retries INT DEFAULT 3,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS conversion_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_file_id UUID NOT NULL REFERENCES stored_files(id) ON DELETE CASCADE,
    target_format VARCHAR(50) NOT NULL,
    status conversion_status NOT NULL DEFAULT 'pending',
    conversion_options JSONB,
    result_file_id UUID REFERENCES stored_files(id) ON DELETE SET NULL,
    progress INT DEFAULT 0 CHECK (progress >= 0 AND progress <= 100),
    error_message TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS upload_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bucket_id UUID NOT NULL REFERENCES buckets(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    path TEXT NOT NULL,
    filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(127),
    file_size BIGINT NOT NULL,
    chunk_size INT NOT NULL,
    total_chunks INT NOT NULL,
    uploaded_chunks INT DEFAULT 0,
    status upload_status NOT NULL DEFAULT 'initiated',
    storage_backend VARCHAR(50) DEFAULT 'local',
    completed_parts INT[],
    part_etags JSONB,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS content_hashes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hash VARCHAR(64) UNIQUE NOT NULL,
    size_bytes BIGINT NOT NULL,
    storage_key TEXT NOT NULL,
    storage_backend VARCHAR(50) DEFAULT 'local',
    reference_count INT NOT NULL DEFAULT 1 CHECK (reference_count >= 1),
    first_uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_referenced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    fingerprint VARCHAR(128),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_file_locks_expires ON file_locks(expires_at);
CREATE INDEX IF NOT EXISTS idx_file_comments_file ON file_comments(file_id);
CREATE INDEX IF NOT EXISTS idx_file_comments_parent ON file_comments(parent_id);
CREATE INDEX IF NOT EXISTS idx_file_comments_user ON file_comments(user_id);
CREATE INDEX IF NOT EXISTS idx_processing_jobs_file_status ON processing_jobs(file_id, status);
CREATE INDEX IF NOT EXISTS idx_processing_jobs_queue ON processing_jobs(status, priority);
CREATE INDEX IF NOT EXISTS idx_conversion_jobs_source ON conversion_jobs(source_file_id, status);
CREATE INDEX IF NOT EXISTS idx_upload_sessions_user ON upload_sessions(user_id, status);
CREATE INDEX IF NOT EXISTS idx_upload_sessions_expires ON upload_sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_content_hashes_refs ON content_hashes(reference_count);
CREATE INDEX IF NOT EXISTS idx_content_hashes_referenced ON content_hashes(last_referenced_at);
CREATE INDEX IF NOT EXISTS idx_content_hashes_fingerprint ON content_hashes(fingerprint) WHERE fingerprint IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_stored_files_content_hash ON stored_files(content_hash_id);

-- Triggers
CREATE OR REPLACE FUNCTION update_last_referenced_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_referenced_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_content_hashes_last_referenced
    BEFORE UPDATE ON stored_files
    FOR EACH ROW
    WHEN (NEW.content_hash_id IS DISTINCT FROM OLD.content_hash_id AND NEW.content_hash_id IS NOT NULL)
    EXECUTE FUNCTION update_referenced_at_content();
```

### 8.2 Backward Compatibility

- All existing entities remain compatible
- New fields are nullable or have defaults
- Existing API endpoints unchanged
- New endpoints added under `/api/v2/` prefix for breaking changes

---

## 9. Dependencies & Risks

### 9.1 External Dependencies

| Dependency | Version | Purpose | Risk |
|------------|---------|---------|------|
| `sha2` | 0.10 | SHA-256 hashing | Low |
| `ffmpeg` (system) | 4.x+ | Video processing | Medium |
| `ImageMagick` (system) | 7.x+ | Image conversion | Low |
| `LibreOffice` (system) | 7.x+ | Document conversion | Medium |

### 9.2 Risk Mitigation

| Risk | Mitigation |
|------|------------|
| FFmpeg not available | Feature flag, graceful degradation |
| Large migration time | Staged rollout, backward compatible |
| Performance impact from deduplication | Async processing, caching |
| Storage backend compatibility | Abstraction layer, adapters |

---

## 10. Implementation Checklist

### Schema Creation
- [x] Create `file_lock.model.yaml`
- [x] Create `file_comment.model.yaml`
- [x] Create `processing_job.model.yaml`
- [x] Create `conversion_job.model.yaml`
- [x] Create `upload_session.model.yaml`
- [x] Create `content_hash.model.yaml`
- [x] Update `stored_file.model.yaml` with new fields
- [x] Update `bucket.model.yaml` with CDN/versioning/deduplication
- [x] Update `index.model.yaml` with imports and services
- [x] Validate all schemas: `backbone schema validate bucket`

### Code Generation
- [x] Generate proto files
- [x] Generate Rust entities
- [x] Generate SQL migrations
- [x] Generate repositories
- [x] Generate handlers
- [x] Generate all targets: `backbone schema generate --target all bucket`
- [x] Verify compilation: `cargo check -p backbone-bucket`

### Custom Implementation
- [ ] Implement `LockingService`
- [ ] Implement `DeduplicationService`
- [ ] Implement `ConversionService`
- [ ] Implement `CdnService`
- [ ] Implement multipart upload handlers
- [ ] Implement video thumbnail processor
- [ ] Implement document preview generator

### Testing
- [x] Unit tests for new entities
- [x] Integration tests for workflows
- [x] Performance benchmarks
- [x] API endpoint tests (generated)

### Documentation
- [ ] Update OpenAPI specs
- [ ] Write migration guide
- [ ] Update README
- [ ] Create usage examples

---

**End of Implementation Plan**
