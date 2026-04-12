# Bucket Module - Migration Guide V2.0

> **From**: V1.0 (basic file storage)
> **To**: V2.0 (enhanced file storage with media processing, collaboration, deduplication, CDN)

---

## Breaking Changes

### Entity Field Renames

| Entity | V1.0 Field | V2.0 Field | Notes |
|--------|-----------|-----------|-------|
| **Bucket** | `storage_used` | `total_size_bytes` | Renamed for clarity |
| **Bucket** | `storage_limit` | _removed_ | No longer a field; use quotas |
| **Bucket** | `region` | _removed_ | Moved to storage backend config |
| **Bucket** | `public_access` | _removed_ | Controlled via sharing |
| **Bucket** | `enable_encryption` | _removed_ | Handled at storage backend |
| **Bucket** | _new_ | `slug` | Required unique slug |
| **Bucket** | _new_ | `root_path` | Required root path |
| **Bucket** | _new_ | `enable_deduplication` | Content deduplication toggle |
| **UserQuota** | `storage_used` | `used_bytes` | Renamed |
| **UserQuota** | `storage_limit` | `limit_bytes` | Renamed |
| **UserQuota** | `file_count_limit` | `max_file_count` | Renamed, now Optional |
| **UserQuota** | `warning_threshold` | `warning_threshold_percent` | Renamed for clarity |
| **UserQuota** | `peak_storage_used` | `peak_usage_bytes` | Renamed |
| **UserQuota** | `upload_bandwidth_*` | _removed_ | Removed bandwidth tracking |
| **UserQuota** | `download_bandwidth_*` | _removed_ | Removed bandwidth tracking |
| **UserQuota** | _new_ | `tier` | User tier (free, pro, etc.) |
| **UserQuota** | _new_ | `max_file_size` | Per-user file size limit |

### Entity Method Signature Changes

| Entity | V1.0 Method | V2.0 Method | Change |
|--------|-----------|-----------|--------|
| **Bucket** | `can_upload(file_size)` | `can_upload(file_size, mime_type)` | Added mime_type check |
| **StoredFile** | `quarantine()` | `quarantine(threats: Vec<String>)` | Takes threat list |
| **StoredFile** | `scan_result: Option<String>` | `scan_result: Option<serde_json::Value>` | JSON value |
| **UploadSession** | `add_part(part_number)` | `add_part(part_number, etag)` | Added etag tracking |
| **FileShare** | `shared_with: Option<Uuid>` | `shared_with: Vec<Uuid>` | Multi-user sharing |
| **FileShare** | `shared_by: Uuid` | `owner_id: Uuid` | Renamed |
| **FileShare** | `can_access(user_id)` | `can_access(Option<Uuid>, Option<&str>)` | Password support |
| **FileShare** | `revoke()` | `revoke(by_user_id: Uuid)` | Tracks who revoked |
| **FileShare** | _new_ | `token: String` | Required share token |
| **UserQuota** | `subtract_usage() -> Result` | `subtract_usage()` | No longer returns Result |

### Behavior Changes

| Entity | Method | V1.0 Behavior | V2.0 Behavior |
|--------|--------|--------------|--------------|
| **StoredFile** | `is_safe()` | Checked threat_level != Critical | Requires `is_scanned=true` AND threat_level in {Safe, None} |
| **StoredFile** | `needs_processing()` | Checked status | Only checks `!is_scanned` |
| **StoredFile** | `quarantine()` | Set threat_level | No longer sets threat_level |
| **UserQuota** | `add_usage()` | Modified file_count | No longer modifies file_count |
| **UserQuota** | `usage_percent()` | Returns 100.0 for zero limit | Returns 0.0 for zero limit |
| **FileLock** | `time_remaining()` | Returns `Option<Duration>` | Returns `Duration` directly |
| **ConversionJob** | `get_progress_percentage()` | Returns `f64` | Returns `i32` |
| **UploadSession** | `calculate_progress()` | Returns `f64` | Returns `i32` |

### Enum Changes

| Enum | V1.0 | V2.0 | Notes |
|------|------|------|-------|
| `BucketType` | `Personal, Team, ...` | `User, Shared, System, Temp` | Renamed variants |
| `QuotaStatus` | `Active, ...` | `Normal, Exceeded` | Simplified |
| `BucketStatus` | `Active, Suspended` | `Active, Readonly, Archived, Deleted` | More states |
| `ProcessingStatus` | `Completed` | `Complete` | Renamed variant |

### Metadata Changes

All entities now use `AuditMetadata` struct instead of `serde_json::json!({})`:

```rust
// V1.0
pub metadata: serde_json::Value,

// V2.0
#[sqlx(json)]
pub metadata: AuditMetadata,
```

---

## New Entities

### V2.0 Added Entities

| Entity | Purpose |
|--------|---------|
| `ConversionJob` | File format conversion tracking |
| `FileLock` | Concurrent edit prevention |
| `FileComment` | File commenting/annotations |
| `Thumbnail` | Generated thumbnail metadata |
| `UploadSession` | Multipart upload session management |

### V2.0 New Fields on Existing Entities

**StoredFile** gained:
- `has_video_thumbnail: bool`
- `has_document_preview: bool`
- `processing_status: Option<ProcessingStatus>`
- `content_hash_id: Option<Uuid>` - link to deduplication
- `cdn_url: Option<String>` - cached CDN URL
- `cdn_url_expires_at: Option<DateTime<Utc>>`

---

## New Custom Services

7 hand-written business logic services added in V2.0:

| Service | Purpose | Dependencies |
|---------|---------|-------------|
| `LockingService` | File editing lock management | FileLockRepository, StoredFileRepository |
| `DeduplicationService` | Content-hash based dedup | ContentHashRepository, StoredFileRepository |
| `MultipartUploadService` | Chunked upload session lifecycle | UploadSessionRepository, BucketRepository |
| `ConversionService` | File format conversion pipeline | ConversionJobRepository, StoredFileRepository |
| `CdnService` | CDN URL generation and caching | StoredFileRepository, BucketRepository |
| `VideoThumbnailService` | Video thumbnail generation queue | ProcessingJobRepository, ThumbnailRepository, StoredFileRepository |
| `DocumentPreviewService` | Document preview generation queue | ProcessingJobRepository, ThumbnailRepository, StoredFileRepository |

---

## Migration Steps

### 1. Database Migration

Run the new V2.0 migrations:

```bash
DATABASE_URL="postgresql://user:pass@localhost:5432/backbonedb" \
  ./target/debug/backbone migration run --module bucket
```

### 2. Code Updates

#### Bucket construction

```rust
// V1.0
let bucket = Bucket {
    storage_used: 0,
    storage_limit: 10_000_000_000,
    region: Some("us-east-1".to_string()),
    ..
};

// V2.0
let bucket = Bucket::builder()
    .name("my-bucket".to_string())
    .slug("my-bucket".to_string())
    .owner_id(user_id)
    .root_path("/data/my-bucket".to_string())
    .allowed_mime_types(vec![])
    .build()
    .unwrap();
```

#### UserQuota field mapping

```rust
// V1.0
quota.storage_used += bytes;
quota.storage_limit;

// V2.0
quota.add_usage(bytes)?;
quota.limit_bytes;
quota.used_bytes;
```

#### File scanning check

```rust
// V1.0
if file.threat_level != Some(ThreatLevel::Critical) { /* safe */ }

// V2.0
if file.is_safe() { /* scanned and safe */ }
```

#### Upload session parts

```rust
// V1.0
session.add_part(part_number)?;

// V2.0
session.add_part(part_number, etag)?;
```

### 3. Test Updates

All domain tests have been rewritten in V2.0 to match new entity definitions:

- `tests/domain_tests.rs` - 147 unit tests
- `tests/workflow_tests.rs` - 49 workflow integration tests
- `tests/processing_bench.rs` - 5 performance benchmark suites

Run all tests:

```bash
cargo test --package backbone-bucket
```

---

## Schema Regeneration

After schema changes, regenerate with:

```bash
./target/debug/backbone-schema schema generate bucket --target all --force
```

Custom code in `// <<< CUSTOM` blocks is preserved during regeneration.
