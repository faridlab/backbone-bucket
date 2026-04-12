# Bucket Module Specification

> **Module**: `bucket`
> **Version**: 2.0
> **Last Updated**: 2026-01-01
> **Status**: Enhanced Specification

## Table of Contents

1. [Module Overview](#1-module-overview)
2. [Module Dependencies](#2-module-dependencies)
3. [Business Objectives](#3-business-objectives)
4. [Use Cases](#4-use-cases)
5. [Entities (Data Models)](#5-entities-data-models)
6. [Enums (Value Types)](#6-enums-value-types)
7. [Entity Lifecycle (State Machines)](#7-entity-lifecycle-state-machines)
8. [Workflows (Multi-Step Processes)](#8-workflows-multi-step-processes)
9. [Events (Domain Events)](#9-events-domain-events)
10. [Services (Business Logic)](#10-services-business-logic)
11. [API Requirements](#11-api-requirements)
12. [Authorization & Permissions](#12-authorization--permissions)
13. [Integrations](#13-integrations)
14. [Non-Functional Requirements](#14-non-functional-requirements)
15. [Seed Data](#15-seed-data)
16. [New Features (V2.0)](#16-new-features-v20)

---

## 1. Module Overview

### 1.1 Basic Information

| Field | Value |
|-------|-------|
| **Module Name** | `bucket` |
| **Display Name** | Bucket - File Storage Management System |
| **Description** | Enterprise-grade file storage with S3-like object operations and Google Drive-like sharing capabilities. Features include virus scanning, image/video processing, quota management, file locking, and CDN integration. |
| **Business Domain** | Storage & Infrastructure |
| **Owner/Team** | Platform Team |

### 1.2 Module Dependencies

| Module | Relationship | Description |
|--------|--------------|-------------|
| `sapiens` | Uses | User authentication, authorization, and profile data |
| `corpus` | Subscribes | Document processing and indexing events |
| `notifier` | Uses | Sending quota warnings, share notifications |

### 1.3 Business Objectives

1. **Secure File Storage**: Provide secure, scalable file storage with automatic virus scanning and threat detection
2. **Smart Media Processing**: Automatic image compression, video thumbnails, document previews, and format conversion
3. **Flexible Sharing**: Multiple sharing modes (public, user-specific, password-protected) with granular permissions
4. **Quota Management**: Tiered storage plans with real-time enforcement and usage monitoring
5. **Collaboration Support**: File locking, comments, and annotations for team collaboration
6. **Storage Optimization**: Content deduplication, multipart uploads, and CDN integration
7. **Comprehensive Auditing**: Complete access logs for security and compliance

---

## 2. Module Dependencies

### 2.1 External Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| PostgreSQL | Database | Primary metadata storage |
| S3/GCS/MinIO | Storage Backend | Pluggable object storage |
| ClamAV | Security | Virus scanning daemon |
| CDN (CloudFlare/CloudFront) | Caching | Asset delivery acceleration |

---

## 3. Business Objectives

| Objective | Description | Success Metric |
|-----------|-------------|----------------|
| **Storage Efficiency** | Reduce storage costs through compression and deduplication | 40-60% reduction |
| **Security** | Block 100% of known malware signatures | Zero breaches |
| **Performance** | <200ms p99 API response time | 99.9% uptime |
| **Scalability** | Support 10,000+ concurrent uploads | Linear scaling |
| **User Experience** | Fast preview generation for all media types | <2s for images, <5s for video |

---

## 4. Use Cases

### UC-001: Upload File with Processing

**Actor**: User
**Description**: Upload a file with automatic virus scanning, compression, and thumbnail generation

**Preconditions**:
- User is authenticated
- User has sufficient quota
- Target bucket is accessible

**Postconditions**:
- File is stored securely
- Metadata is recorded
- Quota is updated
- Processing jobs are queued

**Main Flow**:
1. User initiates multipart upload
2. System validates bucket access and permissions
3. System checks file size against bucket limits
4. System checks file type against allowed MIME types
5. System checks user quota availability
6. System scans file for viruses (async)
7. If threat detected → Quarantine file → Notify user → **END**
8. System stores file content (using multipart if large)
9. System calculates checksum for deduplication
10. If duplicate exists → Link to existing content → Skip storage
11. System queues processing tasks:
    - Compress if image
    - Generate thumbnails
    - Generate video thumbnails if video
    - Generate document preview if Office/PDF
12. System creates file metadata record
13. System updates user quota
14. System updates bucket statistics
15. System emits FileUploadedEvent
16. Return file ID with processing status

**Alternative Flows**:
- **Quota exceeded**: Reject upload, suggest upgrade
- **Virus detected**: Quarantine, notify admin
- **Duplicate content**: Create reference, no additional storage

**Business Rules**:
- Max file size: 5GB (configurable per bucket)
- Allowed MIME types: Configured per bucket
- Quota check: Before storage, not after

**Related Entities**: StoredFile, Bucket, UserQuota, FileVersion, Thumbnail, ContentHash

---

### UC-002: Share File with Permissions

**Actor**: User
**Description**: Create a shareable link with access controls

**Preconditions**:
- User owns the file
- File is not quarantined

**Postconditions**:
- Share link is generated
- Access controls are in place
- Expiry is configured

**Main Flow**:
1. User requests share creation for file
2. System validates file ownership
3. User selects share type (link/user/password)
4. User configures permissions (view/download/edit)
5. User sets optional constraints:
   - Expiration date
   - Max downloads
   - Password
   - Allowed users
6. System generates unique share token
7. System creates share record
8. System emits ShareCreatedEvent
9. System returns share URL

**Business Rules**:
- Share tokens: 32-64 character cryptographically secure string
- Default expiry: None (configurable per organization)
- Max downloads: Unlimited by default

**Related Entities**: FileShare, StoredFile

---

### UC-003: Access Shared File

**Actor**: Anonymous/User
**Description**: Access a file via share link

**Preconditions**:
- Share token is valid
- Share is active

**Postconditions**:
- Access is logged
- Download count is incremented
- File content is delivered

**Main Flow**:
1. External user accesses share URL
2. System validates share token format
3. System loads share record
4. System checks share is active (not revoked)
5. System checks share not expired
6. System checks download count not exceeded
7. If password-protected → Prompt for password → Validate hash
8. If user-specific → Verify user in shared_with list
9. System checks file is accessible (not deleted/quarantined)
10. System logs access event
11. System increments download count
12. System returns file content (with CDN URL if available)

**Business Rules**:
- Failed password attempts: Lock after 5 attempts
- Download tracking: Increment after successful delivery

**Related Entities**: FileShare, StoredFile, FileAccessLog

---

### UC-004: Process File (Media Conversion)

**Actor**: System
**Description**: Async processing of uploaded files

**Preconditions**:
- File is uploaded successfully
- File passed virus scan

**Postconditions**:
- Thumbnails are generated
- Previews are created
- Original is optimized

**Main Flow**:
1. FileUploadedEvent is received
2. System determines file type
3. **If Image**:
   - Generate multiple thumbnail sizes
   - Compress if beneficial
   - Extract EXIF metadata
   - Store optimized versions
4. **If Video**:
   - Generate thumbnail at 1/4 duration
   - Extract duration/resolution metadata
   - Optional: Generate preview clip
5. **If Document** (PDF/Office):
   - Generate first page preview
   - Extract text content for search
   - Generate page thumbnails
6. **If Audio**:
   - Extract duration/metadata
   - Generate waveform visualization
7. System updates file metadata with processing results
8. System emits FileProcessedEvent

**Business Rules**:
- Thumbnail sizes: 128x128, 256x256, 512x512, 1024x1024
- Compression quality: 85% for JPEG, 90% for WebP
- Video thumbnail: Single frame at 25% of duration

**Related Entities**: StoredFile, Thumbnail, ProcessingJob

---

### UC-005: Lock File for Editing

**Actor**: User
**Description**: Lock a file to prevent concurrent edits

**Preconditions**:
- User has edit permission
- File is not already locked

**Postconditions**:
- File is locked
- Other users cannot edit
- Auto-release after timeout

**Main Flow**:
1. User requests file lock
2. System validates edit permission
3. System checks current lock status
4. If locked by another user → Deny with info
5. System creates file lock
6. System sets lock expiry (default 30 minutes)
7. System emits FileLockedEvent
8. System returns lock confirmation

**Alternative Flows**:
- **Already locked**: Return current lock owner and expiry
- **Lock expired**: Automatically release and grant new lock

**Business Rules**:
- Lock timeout: 30 minutes (refreshable)
- Auto-release: On file save or timeout
- Admin override: Can break any lock

**Related Entities**: FileLock, StoredFile

---

### UC-006: Convert File Format

**Actor**: User
**Description**: Convert file to different format

**Preconditions**:
- File is accessible
- Conversion is supported for file type

**Postconditions**:
- New file is created in target format
- Original is preserved
- Metadata links versions

**Main Flow**:
1. User requests format conversion
2. System validates source file accessibility
3. System validates target format support
4. System queues conversion job
5. **When processing**:
   - Download source file
   - Execute conversion (ImageMagick/FFmpeg/LibreOffice)
   - Upload result as new file
   - Link as new version of original
6. System emits FileConvertedEvent
7. System notifies user of completion

**Supported Conversions**:
- Image: PNG↔JPEG↔WebP↔GIF
- Document: DOCX→PDF, XLSX→PDF, PPTX→PDF
- Video: MP4→WebM, MOV→MP4

**Related Entities**: StoredFile, FileVersion, ConversionJob

---

### UC-007: Multipart Upload

**Actor**: User
**Description**: Upload large files in chunks

**Preconditions**:
- File size > threshold (default 100MB)
- Multipart upload is initiated

**Postconditions**:
- File is assembled from chunks
- Chunks are cleaned up

**Main Flow**:
1. User initiates multipart upload
2. System creates upload session with unique ID
3. System returns upload ID and chunk size (default 10MB)
4. User uploads chunks in parallel:
   - Each chunk: PUT /upload/{uploadId}/{partNumber}
   - System validates chunk and stores temporarily
5. User completes upload with part list (ETags)
6. System verifies all parts received
7. System assembles final file
8. System creates StoredFile record
9. System cleans up temporary chunks
10. System emits FileUploadedEvent

**Business Rules**:
- Minimum chunk size: 5MB
- Maximum parts: 10,000
- Session expiry: 24 hours

**Related Entities**: UploadSession, StoredFile

---

### UC-008: Deduplicate Content

**Actor**: System
**Description**: Detect and reuse existing content

**Preconditions**:
- File is being uploaded
- Checksum is calculated

**Postconditions**:
- Duplicate content is linked
- Storage is saved

**Main Flow**:
1. During upload, calculate SHA-256 hash
2. After upload, query for existing files with same hash
3. If exact match found:
   - Create new StoredFile pointing to existing content
   - Increment reference count on content
   - Skip storage backend upload
4. If no match:
   - Store content normally
   - Create new ContentHash record

**Business Rules**:
- Hash algorithm: SHA-256
- Reference counting: Track how many files use each content
- Cleanup: Delete content when reference count = 0

**Related Entities**: StoredFile, ContentHash

---

### UC-009: Add File Comment

**Actor**: User
**Description**: Add comment or annotation to file

**Preconditions**:
- User has access to file
- Comments are enabled for file type

**Postconditions**:
- Comment is recorded
- Notifications are sent

**Main Flow**:
1. User views file
2. User adds comment (text + optional position/region)
3. For images/PDFs: User can select region for annotation
4. System validates comment format
5. System creates comment record
6. System emits FileCommentedEvent
7. System notifies file owner and subscribers

**Business Rules**:
- Comment length: Max 10,000 characters
- Region annotations: x, y, width, height percentages
- Mention support: @username notifications

**Related Entities**: FileComment, StoredFile

---

### UC-010: Serve via CDN

**Actor**: System
**Description**: Generate signed CDN URLs for assets

**Preconditions**:
- File has public or shared access
- CDN is configured

**Postconditions**:
- CDN URL is returned
- URL expires after configured time

**Main Flow**:
1. Request for file access
2. System checks file accessibility
3. System checks if CDN URL exists and is valid
4. If not, generate new signed CDN URL:
   - Use CloudFront signed URLs or CloudFlare tokens
   - Set expiry (default 1 hour)
5. Cache signed URL in metadata
6. Return CDN URL to client

**Business Rules**:
- Signed URL expiry: 1 hour for public, 24 hours for shared
- Cache signed URLs: Regenerate after 80% of expiry
- Private files: Never use CDN (direct from storage)

**Related Entities**: StoredFile, CdnUrl

---

## 5. Entities (Data Models)

### 5.1 Entity List Summary

| Entity Name | Description | Type | Parent Entity |
|-------------|-------------|------|---------------|
| **StoredFile** | File metadata and lifecycle | aggregate_root | N/A |
| **Bucket** | Storage containers | aggregate_root | N/A |
| **FileShare** | Sharing links and permissions | entity | StoredFile |
| **UserQuota** | Per-user storage limits | entity | N/A |
| **FileVersion** | File version history | entity | StoredFile |
| **Thumbnail** | Generated thumbnails | entity | StoredFile |
| **AccessLog** | Detailed access audit | entity | StoredFile |
| **FileLock** | File editing locks | entity | StoredFile |
| **FileComment** | File comments and annotations | entity | StoredFile |
| **ProcessingJob** | Async processing tasks | entity | StoredFile |
| **ConversionJob** | Format conversion tasks | entity | StoredFile |
| **UploadSession** | Multipart upload sessions | entity | N/A |
| **ContentHash** | Content deduplication | entity | N/A |
| **CdnUrl** | CDN URL cache | entity | StoredFile |

### 5.2 Entity Definitions

#### Entity: `StoredFile`

**Description**: Core file metadata and lifecycle management

**Type**: aggregate_root

**Table Name**: `stored_files`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `bucket_id` | uuid | Yes | No | - | Reference to bucket |
| `owner_id` | uuid | Yes | No | - | File owner user ID |
| `path` | string | Yes | No | - | Relative path in bucket |
| `original_name` | string | Yes | No | - | Original filename |
| `size_bytes` | int64 | Yes | No | - | File size |
| `mime_type` | string | Yes | No | - | MIME type |
| `checksum` | string | No | No | - | SHA-256 hash |
| `content_hash_id` | uuid | No | No | - | For deduplication |
| `is_compressed` | bool | No | No | false | Compression flag |
| `original_size` | int64 | No | No | - | Size before compression |
| `compression_algorithm` | string | No | No | - | Algorithm used |
| `is_scanned` | bool | No | No | false | Scan status |
| `scan_result` | json | No | No | - | Detailed scan results |
| `threat_level` | ThreatLevel | No | No | safe | Threat classification |
| `has_thumbnail` | bool | No | No | false | Thumbnail exists |
| `thumbnail_path` | string | No | No | - | Path to thumbnail |
| `has_video_thumbnail` | bool | No | No | false | Video thumbnail |
| `has_document_preview` | bool | No | No | false | Document preview |
| `status` | FileStatus | Yes | No | active | Processing status |
| `storage_key` | string | Yes | No | - | Backend storage key |
| `storage_backend` | StorageBackend | Yes | No | local | Storage used |
| `version` | int | No | No | 1 | Version number |
| `previous_version_id` | uuid | No | No | - | Previous version |
| `download_count` | int | No | No | 0 | Download counter |
| `last_accessed_at` | datetime | No | No | - | Last access time |
| `cdn_url` | string | No | No | - | Cached CDN URL |
| `cdn_url_expires_at` | datetime | No | No | - | CDN URL expiry |
| `processing_status` | ProcessingStatus | No | No | - | Processing state |
| `metadata` | json | No | No | - | Custom metadata |
| `created_at` | datetime | Yes | No | now | Creation timestamp |
| `updated_at` | datetime | Yes | No | now | Last update |
| `deleted_at` | datetime | No | No | - | Soft delete |

**Relationships**:

| Relationship | Target Entity | Type | Field | Description |
|--------------|---------------|------|-------|-------------|
| `bucket` | Bucket | belongs_to | bucket_id | Parent bucket |
| `shares` | FileShare[] | has_many | file_id | Active shares |
| `versions` | FileVersion[] | has_many | file_id | Version history |
| `thumbnails` | Thumbnail[] | has_many | file_id | All thumbnails |
| `access_logs` | AccessLog[] | has_many | file_id | Access history |
| `lock` | FileLock | has_one | file_id | Active lock |
| `comments` | FileComment[] | has_many | file_id | Comments |
| `processing_jobs` | ProcessingJob[] | has_many | file_id | Processing tasks |
| `content_hash` | ContentHash | belongs_to | content_hash_id | Deduplication |

**Indexes**:

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_stored_files_bucket_path` | bucket_id, path | Yes (when deleted_at IS NULL) | Unique path in bucket |
| `idx_stored_files_owner` | owner_id | No | User's files |
| `idx_stored_files_bucket` | bucket_id | No | Files in bucket |
| `idx_stored_files_status` | status | No | By status |
| `idx_stored_files_mime` | mime_type | No | By type |
| `idx_stored_files_checksum` | checksum | No | For deduplication |
| `idx_stored_files_content_hash` | content_hash_id | No | Deduplication lookup |

**Validation Rules**:

| Field | Rule | Message |
|-------|------|---------|
| `path` | `@pattern(^[a-zA-Z0-9/_\-.]+$)` | Path contains invalid characters |
| `path` | `@not_contains(..)` | Path cannot contain parent directory reference |
| `size_bytes` | `@positive` | File size must be positive |
| `original_name` | `@max(255)` | Filename too long |

**Business Rules**:
1. Path must be unique within bucket (excluding deleted files)
2. File size cannot exceed bucket max_file_size setting
3. MIME type must be in bucket's allowed_mime_types list
4. Cannot delete file with active locks (except admin)

**Computed Fields**:

| Field Name | Type | Computation | Description |
|------------|------|-------------|-------------|
| `compression_ratio` | float | `original_size / size_bytes` | Compression achieved |
| `is_accessible` | bool | `status == active AND threat_level == safe` | Can be accessed |
| `is_processed` | bool | `has_thumbnail OR (processing_status == complete)` | Fully processed |

---

#### Entity: `Bucket`

**Description**: Storage container for organizing files

**Type**: aggregate_root

**Table Name**: `buckets`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `name` | string | Yes | No | - | Display name |
| `slug` | string | Yes | Yes | - | URL-safe identifier |
| `description` | string | No | No | - | Bucket description |
| `owner_id` | uuid | Yes | No | - | Bucket owner |
| `bucket_type` | BucketType | Yes | No | user | Type of bucket |
| `status` | BucketStatus | Yes | No | active | Current status |
| `storage_backend` | StorageBackend | Yes | No | local | Backend used |
| `root_path` | string | Yes | No | - | Storage root path |
| `file_count` | int | No | No | 0 | Files in bucket |
| `total_size_bytes` | int64 | No | No | 0 | Total storage |
| `max_file_size` | int64 | No | No | - | Per-file limit |
| `allowed_mime_types` | string[] | No | No | - | Allowed types (empty = all) |
| `auto_delete_after_days` | int | No | No | - | Auto-cleanup days |
| `enable_cdn` | bool | No | No | false | CDN enabled |
| `enable_versioning` | bool | No | No | true | Versioning enabled |
| `enable_deduplication` | bool | No | No | true | Deduplication enabled |
| `metadata` | json | No | No | - | Custom settings |
| `created_at` | datetime | Yes | No | now | Creation |
| `updated_at` | datetime | Yes | No | now | Last update |
| `deleted_at` | datetime | No | No | - | Soft delete |

**Relationships**:

| Relationship | Target Entity | Type | Field | Description |
|--------------|---------------|------|-------|-------------|
| `files` | StoredFile[] | has_many | bucket_id | Files in bucket |

**Indexes**:

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_buckets_slug` | slug | Yes | Unique slug |
| `idx_buckets_owner` | owner_id | No | User's buckets |
| `idx_buckets_status` | status | No | By status |
| `idx_buckets_type` | bucket_type | No | By type |

---

#### Entity: `FileShare`

**Description**: Sharing links and permissions for files

**Type**: entity

**Table Name**: `file_shares`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `file_id` | uuid | Yes | No | - | Shared file |
| `owner_id` | uuid | Yes | No | - | Share creator |
| `token` | string | Yes | Yes | - | Share token |
| `share_type` | ShareType | Yes | No | link | Type of share |
| `permission` | SharePermission | Yes | No | view | Permission level |
| `shared_with` | uuid[] | No | No | - | Target user IDs |
| `password_hash` | string | No | No | - | Optional password |
| `max_downloads` | int | No | No | - | Download limit |
| `download_count` | int | No | No | 0 | Current count |
| `expires_at` | datetime | No | No | - | Expiration |
| `is_active` | bool | No | No | true | Active status |
| `revoked_at` | datetime | No | No | - | Revocation time |
| `revoked_by` | uuid | No | No | - | Who revoked |
| `message` | string | No | No | - | Share message |
| `metadata` | json | No | No | - | Custom data |
| `created_at` | datetime | Yes | No | now | Created |
| `updated_at` | datetime | Yes | No | now | Updated |
| `deleted_at` | datetime | No | No | - | Soft delete |

---

#### Entity: `FileLock`

**Description**: File editing locks for collaboration

**Type**: entity

**Table Name**: `file_locks`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `file_id` | uuid | Yes | No | - | Locked file |
| `user_id` | uuid | Yes | No | - | Lock owner |
| `locked_at` | datetime | Yes | No | now | Lock time |
| `expires_at` | datetime | Yes | No | - | Expiry time |
| `refreshed_at` | datetime | No | No | - | Last refresh |
| `metadata` | json | No | No | - | Custom data |
| `created_at` | datetime | Yes | No | now | Created |
| `updated_at` | datetime | Yes | No | now | Updated |

**Indexes**:

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_file_locks_file` | file_id | Yes | One lock per file |
| `idx_file_locks_expires` | expires_at | No | Cleanup expired |

---

#### Entity: `FileComment`

**Description**: Comments and annotations on files

**Type**: entity

**Table Name**: `file_comments`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `file_id` | uuid | Yes | No | - | Commented file |
| `user_id` | uuid | Yes | No | - | Comment author |
| `parent_id` | uuid | No | No | - | Parent comment (for replies) |
| `content` | string | Yes | No | - | Comment text |
| `annotation_region` | json | No | No | - | Region for annotation |
| `mentions` | uuid[] | No | No | - | Mentioned users |
| `resolved` | bool | No | No | false | Resolved status |
| `resolved_by` | uuid | No | No | - | Who resolved |
| `resolved_at` | datetime | No | No | - | Resolution time |
| `metadata` | json | No | No | - | Custom data |
| `created_at` | datetime | Yes | No | now | Created |
| `updated_at` | datetime | Yes | No | now | Updated |
| `deleted_at` | datetime | No | No | - | Soft delete |

**Indexes**:

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_file_comments_file` | file_id | No | Comments on file |
| `idx_file_comments_parent` | parent_id | No | Replies |
| `idx_file_comments_user` | user_id | No | User's comments |

---

#### Entity: `ProcessingJob`

**Description**: Async file processing tasks

**Type**: entity

**Table Name**: `processing_jobs`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `file_id` | uuid | Yes | No | - | Target file |
| `job_type` | ProcessingJobType | Yes | No | - | Type of job |
| `status` | JobStatus | Yes | No | pending | Job status |
| `priority` | int | No | No | 0 | Job priority |
| `input_data` | json | No | No | - | Job parameters |
| `result_data` | json | No | No | - | Job result |
| `error_message` | string | No | No | - | Error if failed |
| `started_at` | datetime | No | No | - | Start time |
| `completed_at` | datetime | No | No | - | Completion |
| `retry_count` | int | No | No | 0 | Retry attempts |
| `max_retries` | int | No | No | 3 | Max retries |
| `metadata` | json | No | No | - | Custom data |
| `created_at` | datetime | Yes | No | now | Created |
| `updated_at` | datetime | Yes | No | now | Updated |

---

#### Entity: `UploadSession`

**Description**: Multipart upload sessions

**Type**: entity

**Table Name**: `upload_sessions`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `bucket_id` | uuid | Yes | No | - | Target bucket |
| `user_id` | uuid | Yes | No | - | Upload owner |
| `path` | string | Yes | No | - | Target path |
| `filename` | string | Yes | No | - | Original filename |
| `mime_type` | string | No | No | - | File type |
| `file_size` | int64 | Yes | No | - | Total file size |
| `chunk_size` | int | Yes | No | - | Chunk size |
| `total_chunks` | int | Yes | No | - | Total chunks |
| `uploaded_chunks` | int | No | No | 0 | Uploaded count |
| `status` | UploadStatus | Yes | No | initiated | Upload status |
| `storage_backend` | StorageBackend | Yes | No | local | Storage |
| `expires_at` | datetime | Yes | No | - | Session expiry |
| `metadata` | json | No | No | - | Custom data |
| `created_at` | datetime | Yes | No | now | Created |
| `updated_at` | datetime | Yes | No | now | Updated |

---

#### Entity: `ContentHash`

**Description**: Content deduplication tracking

**Type**: entity

**Table Name**: `content_hashes`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `hash` | string | Yes | Yes | - | SHA-256 hash |
| `size_bytes` | int64 | Yes | No | - | Content size |
| `storage_key` | string | Yes | No | - | Storage location |
| `storage_backend` | StorageBackend | Yes | No | - | Storage used |
| `reference_count` | int | Yes | No | 1 | File references |
| `first_uploaded_at` | datetime | Yes | No | now | First upload |
| `last_referenced_at` | datetime | Yes | No | now | Last reference |
| `metadata` | json | No | No | - | Custom data |
| `created_at` | datetime | Yes | No | now | Created |
| `updated_at` | datetime | Yes | No | now | Updated |

**Indexes**:

| Index Name | Fields | Unique | Description |
|------------|--------|--------|-------------|
| `idx_content_hashes_hash` | hash | Yes | Hash lookup |
| `idx_content_hashes_refs` | reference_count | No | For cleanup |

---

#### Entity: `UserQuota`

**Description**: Per-user storage limits and usage

**Type**: entity

**Table Name**: `user_quotas`

**Fields**:

| Field Name | Type | Required | Unique | Default | Description |
|------------|------|----------|--------|---------|-------------|
| `id` | uuid | Yes | Yes | auto | Primary key |
| `user_id` | uuid | Yes | Yes | - | User ID |
| `limit_bytes` | int64 | Yes | No | - | Storage limit |
| `used_bytes` | int64 | Yes | No | 0 | Current usage |
| `file_count` | int | Yes | No | 0 | File count |
| `max_file_size` | int64 | No | No | - | Per-file limit |
| `max_file_count` | int | No | No | - | File count limit |
| `tier` | string | Yes | No | free | Quota tier |
| `warning_threshold_percent` | int | Yes | No | 80 | Warning % |
| `last_warning_sent_at` | datetime | No | No | - | Last warning |
| `peak_usage_bytes` | int64 | Yes | No | 0 | Peak usage |
| `peak_usage_at` | datetime | No | No | - | Peak time |
| `metadata` | json | No | No | - | Custom data |
| `created_at` | datetime | Yes | No | now | Created |
| `updated_at` | datetime | Yes | No | now | Updated |

---

## 6. Enums (Value Types)

### Enum: `FileStatus`

**Description**: File lifecycle status

**Database Type**: `file_status`

| Variant | Value | Description |
|---------|-------|-------------|
| `uploading` | `uploading` | Upload in progress |
| `processing` | `processing` | Being scanned/processed |
| `active` | `active` | Active and accessible |
| `quarantined` | `quarantined` | Flagged as unsafe |
| `deleted` | `deleted` | Soft deleted |
| `purged` | `purged` | Permanently deleted |

### Enum: `ThreatLevel`

**Description**: Security threat classification

**Database Type**: `threat_level`

| Variant | Value | Description |
|---------|-------|-------------|
| `safe` | `safe` | No threats detected |
| `low` | `low` | Minor concern |
| `medium` | `medium` | Potential threat |
| `high` | `high` | Known malware |
| `critical` | `critical` | Active malware |

### Enum: `BucketType`

**Description**: Type of storage bucket

**Database Type**: `bucket_type`

| Variant | Value | Description |
|---------|-------|-------------|
| `user` | `user` | Personal storage |
| `shared` | `shared` | Team/group storage |
| `system` | `system` | System storage |
| `temp` | `temp` | Temporary storage |

### Enum: `BucketStatus`

**Description**: Bucket lifecycle status

**Database Type**: `bucket_status`

| Variant | Value | Description |
|---------|-------|-------------|
| `active` | `active` | Active and accessible |
| `readonly` | `readonly` | Read-only mode |
| `archived` | `archived` | Archived |
| `deleted` | `deleted` | Soft deleted |

### Enum: `StorageBackend`

**Description**: Storage backend type

**Database Type**: `storage_backend`

| Variant | Value | Description |
|---------|-------|-------------|
| `local` | `local` | Local filesystem |
| `s3` | `s3` | AWS S3 |
| `minio` | `minio` | MinIO |
| `gcs` | `gcs` | Google Cloud Storage |

### Enum: `ShareType`

**Description**: Share link type

**Database Type**: `share_type`

| Variant | Value | Description |
|---------|-------|-------------|
| `user` | `user` | User-specific |
| `link` | `link` | Public link |
| `password` | `password` | Password protected |

### Enum: `SharePermission`

**Description**: Share permission level

**Database Type**: `share_permission`

| Variant | Value | Description |
|---------|-------|-------------|
| `private` | `private` | Owner only |
| `view` | `view` | View/download |
| `edit` | `edit` | View and edit |
| `full` | `full` | Full access |

### Enum: `ProcessingJobType`

**Description**: Type of processing job

**Database Type**: `processing_job_type`

| Variant | Value | Description |
|---------|-------|-------------|
| `thumbnail_generation` | `thumbnail_generation` | Generate thumbnails |
| `video_thumbnail` | `video_thumbnail` | Video thumbnail |
| `document_preview` | `document_preview` | Document preview |
| `compression` | `compression` | Compress file |
| `conversion` | `conversion` | Convert format |

### Enum: `JobStatus`

**Description**: Processing job status

**Database Type**: `job_status`

| Variant | Value | Description |
|---------|-------|-------------|
| `pending` | `pending` | Waiting to process |
| `running` | `running` | Currently processing |
| `completed` | `completed` | Finished successfully |
| `failed` | `failed` | Failed with error |
| `cancelled` | `cancelled` | Cancelled |

### Enum: `UploadStatus`

**Description**: Multipart upload status

**Database Type**: `upload_status`

| Variant | Value | Description |
|---------|-------|-------------|
| `initiated` | `initiated` | Session created |
| `uploading` | `uploading` | Uploading chunks |
| `completing` | `completing` | Assembling |
| `completed` | `completed` | Complete |
| `expired` | `expired` | Session expired |
| `failed` | `failed` | Upload failed |

---

## 7. Entity Lifecycle (State Machines)

### State Machine: `StoredFile`

**Description**: File processing and lifecycle states

**State Field**: `status`

#### States

| State | Description | Entry Actions | Exit Actions |
|-------|-------------|---------------|--------------|
| `uploading` | Upload in progress | Initialize metadata | - |
| `processing` | Being scanned/processed | Queue processing jobs | - |
| `active` | Active and accessible | Publish availability event | - |
| `quarantined` | Flagged as unsafe | Notify admin, owner | - |
| `deleted` | Soft deleted | Move to trash location | Update quota |
| `purged` | Permanently deleted | Delete content, cleanup | - |

#### Transitions

| From State | To State | Trigger/Event | Guard Condition | Actions |
|------------|----------|---------------|-----------------|---------|
| `uploading` | `processing` | Upload complete | File received | Start processing |
| `processing` | `active` | Processing complete | No threats | Publish event |
| `processing` | `quarantined` | Threat detected | Threat level > safe | Notify admin |
| `active` | `deleted` | Delete requested | Owner or admin | Move to trash |
| `deleted` | `active` | Restore requested | Owner, quota OK | Restore from trash |
| `deleted` | `purged` | Purge requested | Admin or expired | Delete permanently |
| `quarantined` | `active` | Released by admin | Admin approval | Publish event |
| `quarantined` | `purged` | Delete requested | Owner or admin | Delete permanently |

#### State Diagram

```
[uploading] --complete--> [processing] --safe--> [active]
                             |
                             +--threat--> [quarantined]
                                                    |
                                                    +--release--> [active]
                                                    |
                                                    +--delete--> [purged]

[active] --delete--> [deleted] --purge--> [purged]
                           |
                           +--restore--> [active]
```

---

### State Machine: `ProcessingJob`

**Description**: Async job execution state machine

**State Field**: `status`

#### States

| State | Description | Entry Actions | Exit Actions |
|-------|-------------|---------------|--------------|
| `pending` | Queued for processing | Add to queue | - |
| `running` | Currently processing | Mark start time | Record duration |
| `completed` | Finished successfully | Store result | Notify |
| `failed` | Failed with error | Store error | Schedule retry |
| `cancelled` | Cancelled by user | Cleanup resources | - |

#### Transitions

| From State | To State | Trigger/Event | Guard Condition | Actions |
|------------|----------|---------------|-----------------|---------|
| `pending` | `running` | Job picked up | Worker available | Mark started |
| `running` | `completed` | Job succeeded | No errors | Store result |
| `running` | `failed` | Job failed | Error occurred | Store error |
| `failed` | `pending` | Retry scheduled | retry_count < max_retries | Increment retry |
| `*` | `cancelled` | Cancel requested | Job not complete | Cleanup |

---

## 8. Workflows (Multi-Step Processes)

### Workflow: `FileUploadWorkflow`

**Description**: Complete file upload processing pipeline

**Trigger**:
- Event: `UploadInitiatedEvent`
- Manual: `POST /api/v1/upload`

#### Steps

| Step | Name | Type | Description | On Success | On Failure |
|------|------|------|-------------|------------|------------|
| 1 | `validate_request` | action | Validate input data | Go to step 2 | Reject with error |
| 2 | `check_quota` | action | Verify user has space | Go to step 3 | Reject, notify |
| 3 | `check_bucket` | action | Verify bucket accessible | Go to step 4 | Reject with error |
| 4 | `initiate_upload` | action | Create upload session | Go to step 5 | Rollback |
| 5 | `receive_content` | action | Receive file content | Go to step 6 | Rollback |
| 6 | `calculate_hash` | action | Compute SHA-256 | Go to step 7 | Rollback |
| 7 | `check_duplicate` | condition | Check if content exists | Branch | Continue |
| 8 | `store_content` | action | Store in backend | Go to step 9 | Rollback |
| 9 | `scan_threats` | action | Virus scan (async) | Go to step 10 | Quarantine |
| 10 | `create_metadata` | action | Create StoredFile record | Go to step 11 | Rollback |
| 11 | `update_quota` | action | Increment usage | Go to step 12 | Rollback |
| 12 | `queue_processing` | parallel | Queue thumbnail/compression jobs | Complete | Log error |
| 13 | `emit_event` | action | Publish FileUploadedEvent | Complete | Log error |

#### Workflow Variables

| Variable | Type | Source | Description |
|----------|------|--------|-------------|
| `user_id` | uuid | `request.user_id` | Uploader |
| `bucket_id` | uuid | `request.bucket_id` | Target bucket |
| `file_size` | int64 | `request.content_length` | Size in bytes |
| `content_hash` | string | `step 6` | SHA-256 hash |
| `is_duplicate` | bool | `step 7` | Duplicate content |

---

### Workflow: `FileProcessingWorkflow`

**Description**: Media processing and thumbnail generation

**Trigger**:
- Event: `FileUploadedEvent`
- Schedule: Async worker queue

#### Steps

| Step | name | Type | Description | On Success | On Failure |
|------|------|------|-------------|------------|------------|
| 1 | `determine_type` | action | Detect file type | Go to appropriate branch | Skip |
| 2 | `process_image` | action | Handle images | Go to step 6 | Continue |
| 3 | `process_video` | action | Handle videos | Go to step 6 | Continue |
| 4 | `process_document` | action | Handle documents | Go to step 6 | Continue |
| 5 | `skip_processing` | action | Mark as processed | Complete | - |
| 6 | `update_status` | action | Mark as processed | Complete | - |

---

### Workflow: `MultipartUploadWorkflow`

**Description**: Large file upload with chunking

**Trigger**:
- Manual: `POST /api/v1/upload/multipart/initiate`

#### Steps

| Step | Name | Type | Description | On Success | On Failure |
|------|------|------|-------------|------------|------------|
| 1 | `create_session` | action | Create upload session | Return upload ID | Error |
| 2 | `receive_chunks` | wait | Receive chunk uploads | Go to step 3 | Expire session |
| 3 | `verify_chunks` | action | Verify all parts received | Go to step 4 | Fail |
| 4 | `assemble_file` | action | Combine chunks | Go to step 5 | Fail |
| 5 | `complete_upload` | action | Create StoredFile | Complete | Fail |
| 6 | `cleanup_chunks` | action | Delete temp chunks | - | - |

---

## 9. Events (Domain Events)

### Event: `FileUploaded`

**Description**: A new file was uploaded

**Publisher**: `StoredFile`

**Trigger**: File upload completed successfully

**Payload**:

| Field | Type | Description |
|-------|------|-------------|
| `file_id` | uuid | ID of the file |
| `bucket_id` | uuid | Containing bucket |
| `owner_id` | uuid | File owner |
| `path` | string | File path |
| `size_bytes` | int64 | File size |
| `mime_type` | string | MIME type |
| `content_hash` | string | SHA-256 hash |
| `is_duplicate` | bool | Was duplicate content |
| `uploaded_at` | datetime | Upload timestamp |

**Subscribers**:

| Module | Handler | Description |
|--------|---------|-------------|
| `bucket` | `FileProcessingHandler` | Queue processing jobs |
| `corpus` | `DocumentIndexHandler` | Index for search |
| `notifier` | `UploadNotificationHandler` | Notify if needed |

---

### Event: `FileProcessed`

**Description**: File processing completed

**Publisher**: `ProcessingJob`

**Trigger**: All processing jobs completed

**Payload**:

| Field | Type | Description |
|-------|------|-------------|
| `file_id` | uuid | File ID |
| `processing_results` | json | Processing outcomes |
| `thumbnails_generated` | int | Number of thumbnails |
| `compression_ratio` | float | Compression achieved |
| `processed_at` | datetime | Completion time |

---

### Event: `FileLocked`

**Description**: File was locked for editing

**Publisher**: `FileLock`

**Trigger**: User requested file lock

**Payload**:

| Field | Type | Description |
|-------|------|-------------|
| `file_id` | uuid | Locked file |
| `user_id` | uuid | Lock owner |
| `locked_at` | datetime | Lock time |
| `expires_at` | datetime | Expiry time |

---

### Event: `QuotaExceeded`

**Description**: User exceeded storage quota

**Publisher**: `UserQuota`

**Trigger**: Upload would exceed quota

**Payload**:

| Field | Type | Description |
|-------|------|-------------|
| `user_id` | uuid | Affected user |
| `limit_bytes` | int64 | Quota limit |
| `used_bytes` | int64 | Current usage |
| `attempted_bytes` | int64 | Requested additional |
| `occurred_at` | datetime | Event time |

**Subscribers**:

| Module | Handler | Description |
|--------|---------|-------------|
| `notifier` | `QuotaWarningHandler` | Send notification |

---

### Event: `ThreatDetected`

**Description**: Malware detected in upload

**Publisher**: `StoredFile`

**Trigger**: Virus scan found threat

**Payload**:

| Field | Type | Description |
|-------|------|-------------|
| `file_id` | uuid | Affected file |
| `owner_id` | uuid | File owner |
| `threat_level` | ThreatLevel | Severity |
| `threats` | string[] | Detected threats |
| `detected_at` | datetime | Detection time |

**Subscribers**:

| Module | Handler | Description |
|--------|---------|-------------|
| `notifier` | `ThreatAlertHandler` | Alert admins |
| `bucket` | `QuarantineHandler` | Quarantine file |

---

## 10. Services (Business Logic)

### Service: `FileProcessingService`

**Description**: Handles all file processing operations

**Dependencies**: `StorageBackend`, `VirusScanner`, `ImageProcessor`, `VideoProcessor`, `DocumentProcessor`

**Methods**:

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `process_file` | `file_id: Uuid` | `Result<ProcessingResult, ProcessingError>` | Process uploaded file |
| `generate_thumbnails` | `file: StoredFile, sizes: Vec<ThumbnailSize>` | `Result<Vec<Thumbnail>, ProcessingError>` | Generate thumbnails |
| `compress_image` | `content: Vec<u8>, quality: u8` | `Result<CompressedResult, ProcessingError>` | Compress image |
| `generate_video_thumbnail` | `file: StoredFile` | `Result<Thumbnail, ProcessingError>` | Extract video frame |
| `generate_document_preview` | `file: StoredFile` | `Result<DocumentPreview, ProcessingError>` | Generate document preview |
| `convert_format` | `file: StoredFile, target_format: string` | `Result<StoredFile, ConversionError>` | Convert file format |
| `scan_for_threats` | `content: Vec<u8>, filename: string` | `Result<ScanResult, ScanError>` | Scan for malware |

---

### Service: `QuotaEnforcementService`

**Description**: Enforces user storage quotas

**Dependencies**: `UserQuotaRepository`, `ContentHashRepository`

**Methods**:

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `check_quota` | `user_id: Uuid, additional_bytes: i64` | `Result<bool, QuotaError>` | Check if space available |
| `update_usage` | `user_id: Uuid, delta_bytes: i64` | `Result<UserQuota, QuotaError>` | Update usage |
| `get_usage_percentage` | `quota: UserQuota` | `f64` | Get usage % |
| `should_warn` | `quota: UserQuota` | `bool` | Check warning threshold |
| `reserve_space` | `user_id: Uuid, bytes: i64` | `Result<Reservation, QuotaError>` | Reserve space for upload |

---

### Service: `ShareLinkService`

**Description**: Manages file sharing links

**Dependencies**: `FileShareRepository`, `StoredFileRepository`

**Methods**:

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `generate_token` | - | `ShareToken` | Generate secure token |
| `generate_share_url` | `token: ShareToken, base_url: string` | `string` | Build share URL |
| `validate_access` | `share: FileShare, user_id: Option<Uuid>, password: Option<string>` | `bool` | Validate access |
| `record_access` | `share_id: Uuid, user_id: Option<Uuid>` | `Result<(), ShareError>` | Log access |
| `is_valid` | `share: FileShare` | `bool` | Check share validity |

---

### Service: `LockingService`

**Description**: Manages file editing locks

**Dependencies**: `FileLockRepository`, `StoredFileRepository`

**Methods**:

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `acquire_lock` | `file_id: Uuid, user_id: Uuid, duration: Duration` | `Result<FileLock, LockError>` | Acquire file lock |
| `release_lock` | `file_id: Uuid, user_id: Uuid` | `Result<(), LockError>` | Release lock |
| `refresh_lock` | `file_id: Uuid, user_id: Uuid` | `Result<FileLock, LockError>` | Refresh lock expiry |
| `get_lock` | `file_id: Uuid` | `Option<FileLock>` | Get active lock |
| `break_lock` | `file_id: Uuid, admin_id: Uuid` | `Result<(), LockError>` | Admin break lock |

---

### Service: `DeduplicationService`

**Description**: Content-based deduplication

**Dependencies**: `ContentHashRepository`, `StorageBackend`

**Methods**:

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `find_or_create` | `hash: string, size: i64, content: Vec<u8>` | `Result<ContentHash, DedupError>` | Find existing or create new |
| `increment_reference` | `hash_id: Uuid` | `Result<(), DedupError>` | Increment ref count |
| `decrement_reference` | `hash_id: Uuid` | `Result<bool, DedupError>` | Decrement, return true if should delete |
| `cleanup_orphaned` | `older_than: Duration` | `Result<Vec<Uuid>, DedupError>` | Find unused content |

---

### Service: `CdnService`

**Description**: CDN URL generation and caching

**Dependencies**: `CdnProvider`, `StoredFileRepository`

**Methods**:

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `get_or_generate_url` | `file: StoredFile` | `Result<string, CdnError>` | Get CDN URL |
| `generate_signed_url` | `storage_key: string, expiry: Duration` | `string` | Generate signed URL |
| `invalidate_url` | `file_id: Uuid` | `Result<(), CdnError>` | Invalidate cached URL |
| `purge_from_cdn` | `file: StoredFile` | `Result<(), CdnError>` | Purge from CDN cache |

---

## 11. API Requirements

### 11.1 Custom Endpoints

| Method | Path | Description | Request Body | Response |
|--------|------|-------------|--------------|----------|
| `POST` | `/api/v1/upload` | Single file upload | `MultipartFile` | `StoredFile` |
| `POST` | `/api/v1/upload/multipart/initiate` | Start multipart | `InitiateMultipartRequest` | `UploadSession` |
| `PUT` | `/api/v1/upload/multipart/{id}/{part}` | Upload chunk | `bytes` | `PartETag` |
| `POST` | `/api/v1/upload/multipart/{id}/complete` | Complete multipart | `CompleteMultipartRequest` | `StoredFile` |
| `GET` | `/api/v1/download/{id}` | Download file | - | `file content` |
| `GET` | `/api/v1/thumbnails/{file_id}/{size}` | Get thumbnail | - | `image content` |
| `POST` | `/api/v1/files/{id}/share` | Create share | `CreateShareRequest` | `FileShare` |
| `POST` | `/api/v1/shares/{token}/access` | Access via share | `optional password` | `file content` |
| `POST` | `/api/v1/files/{id}/lock` | Acquire lock | `LockRequest` | `FileLock` |
| `DELETE` | `/api/v1/files/{id}/lock` | Release lock | - | `void` |
| `POST` | `/api/v1/files/{id}/convert` | Convert format | `ConvertRequest` | `ConversionJob` |
| `POST` | `/api/v1/files/{id}/comments` | Add comment | `CommentRequest` | `FileComment` |
| `GET` | `/api/v1/files/{id}/processing` | Get processing status | - | `ProcessingStatus` |
| `GET` | `/api/v1/quota/{user_id}` | Get quota info | - | `UserQuota` |

### 11.2 Query Filters

**Entity: `StoredFile`**

| Filter | Type | Example | Description |
|--------|------|---------|-------------|
| `status` | enum | `?status=active` | Filter by status |
| `bucket_id` | uuid | `?bucket_id=xxx` | Files in bucket |
| `owner_id` | uuid | `?owner_id=xxx` | User's files |
| `mime_type` | string | `?mime_type=image/*` | By MIME type pattern |
| `size_gt` | int64 | `?size_gt=1048576` | Larger than |
| `size_lt` | int64 | `?size_lt=10485760` | Smaller than |
| `has_thumbnail` | bool | `?has_thumbnail=true` | Has thumbnail |
| `created_after` | date | `?created_after=2024-01-01` | Created after |
| `search` | string | `?search=name` | Search in name |

### 11.3 Sorting

| Field | Default Direction | Description |
|-------|-------------------|-------------|
| `created_at` | DESC | Creation time |
| `name` | ASC | Filename |
| `size_bytes` | DESC | File size |
| `download_count` | DESC | Popularity |

### 11.4 Pagination

| Parameter | Default | Max | Description |
|-----------|---------|-----|-------------|
| `page` | 1 | - | Page number |
| `limit` | 20 | 100 | Items per page |

---

## 12. Authorization & Permissions

### 12.1 Roles

| Role | Description | Permissions |
|------|-------------|-------------|
| `super_admin` | Full system access | All operations |
| `admin` | Module administrator | All operations in module |
| `user` | Regular user | Own files, create shares, view quota |
| `guest` | Anonymous access | Via share links only |

### 12.2 Permission Matrix

| Entity | Create | Read | Update | Delete | Special |
|--------|--------|------|--------|--------|---------|
| `StoredFile` | admin, user | owner, shared, public | owner | owner, admin | lock: owner |
| `Bucket` | admin, user | owner, shared | owner | owner, admin | - |
| `FileShare` | owner | owner | owner | owner | revoke: owner |
| `UserQuota` | admin | owner, admin | admin | admin | - |

### 12.3 Row-Level Security

| Entity | Rule | Description |
|--------|------|-------------|
| `StoredFile` | `owner_id = user.id OR has_active_share(file_id, user.id)` | Users see own or shared files |
| `Bucket` | `owner_id = user.id OR bucket_type = 'shared'` | Users see own or shared buckets |
| `FileShare` | `owner_id = user.id` | Users see own shares |

---

## 13. Integrations

### 13.1 External APIs

| System | Purpose | Auth Type | Endpoints Used |
|--------|---------|-----------|----------------|
| **ClamAV** | Virus scanning | TCP socket | SCAN, PING |
| **AWS S3** | Object storage | IAM keys | PutObject, GetObject, DeleteObject |
| **Google Cloud Storage** | Object storage | Service account | Upload, Download, Delete |
| **MinIO** | Object storage | Access/Secret key | S3-compatible API |
| **CloudFlare CDN** | Content delivery | API token | Purge, Signed URLs |
| **CloudFront** | Content delivery | AWS keys | Signed URLs, Invalidation |

### 13.2 Webhooks (Outgoing)

| Event | URL Pattern | Payload | Retry Policy |
|-------|-------------|---------|--------------|
| `FileUploaded` | Configured webhook URL | `file_id, owner_id, size` | 3 retries, exponential |
| `ThreatDetected` | Admin security endpoint | `file_id, threats` | 5 retries, immediate |
| `QuotaExceeded` | User notification endpoint | `user_id, usage, limit` | 3 retries |
| `FileProcessed` | Configured webhook URL | `file_id, results` | 3 retries |

### 13.3 Webhooks (Incoming)

| Endpoint | Source System | Validation | Handler |
|----------|---------------|------------|---------|
| `POST /webhooks/storage` | Storage backend | HMAC signature | `StorageEventHandler` |
| `POST /webhooks/cdn` | CDN provider | API key | `CdnEventHandler` |

---

## 14. Non-Functional Requirements

### 14.1 Performance

| Metric | Requirement | Notes |
|--------|-------------|-------|
| Upload throughput | Up to 1 GB/s per node | With multipart |
| Download throughput | Up to 5 GB/s per node | Via CDN |
| API response time (metadata) | <100ms (p95) | Cached queries |
| Thumbnail generation | <2s per image | Async processing |
| Video thumbnail | <5s per video | Async processing |

### 14.2 Data Retention

| Data Type | Retention Period | Archive Strategy |
|-----------|------------------|------------------|
| Active files | Indefinite | User-controlled |
| Deleted files (trash) | 30 days | Auto-purge |
| File versions | Configurable | Auto-expire |
| Access logs | 1 year | Archive to cold storage |
| Thumbnails | Until file deleted | Cascade delete |
| Upload sessions | 24 hours | Auto-expire |

### 14.3 Audit Requirements

| Entity | Audit Level | Fields to Track |
|--------|-------------|-----------------|
| `StoredFile` | Full | All field changes |
| `FileShare` | Full | Create, revoke, access |
| `UserQuota` | Basic | Limit changes, usage updates |

---

## 15. Seed Data

### 15.1 Reference Data

**Default Buckets**:

| id | name | slug | bucket_type | owner_id | description |
|----|------|------|-------------|----------|-------------|
| (system) | System Files | system | system | (system) | System-managed files |

**Default Quota Tiers**:

| Tier | Limit Bytes | Max File Size | Max File Count |
|------|-------------|---------------|----------------|
| `free` | 5 GB | 100 MB | 1000 |
| `basic` | 50 GB | 500 MB | 10000 |
| `premium` | 500 GB | 2 GB | 100000 |
| `enterprise` | 5 TB | 5 GB | Unlimited |

---

## 16. New Features (V2.0)

### 16.1 Media Processing Enhancements

| Feature | Description | Status |
|---------|-------------|--------|
| **Video Thumbnails** | Extract frame from video for preview | New |
| **Document Previews** | Generate PDF/Office document previews | New |
| **Format Conversion** | Convert between image/video/document formats | New |
| **Image Editing** | Basic crop, resize, rotate operations | New |

### 16.2 Collaboration Features

| Feature | Description | Status |
|---------|-------------|--------|
| **File Locking** | Prevent concurrent edits with timeout | New |
| **Comments & Annotations** | Add comments with region annotations | New |
| **Version Comparison** | Show diffs between file versions | Planned |

### 16.3 Storage & Performance

| Feature | Description | Status |
|---------|-------------|--------|
| **Multipart Upload** | Chunked upload with resume for large files | New |
| **Content Deduplication** | SHA-256 based content deduplication | New |
| **CDN Integration** | Signed URL generation for CloudFlare/CloudFront | New |
| **Archive Storage** | Move old files to cold storage | Planned |

### 16.4 Schema Updates Required

The following new entity model files need to be created:

- `file_lock.model.yaml` - File editing locks
- `file_comment.model.yaml` - Comments and annotations
- `processing_job.model.yaml` - Async processing jobs
- `conversion_job.model.yaml` - Format conversion jobs
- `upload_session.model.yaml` - Multipart upload sessions
- `content_hash.model.yaml` - Content deduplication

---

## Checklist Before Submission

- [x] All entities have complete field definitions
- [x] All relationships are defined with foreign keys
- [x] All enums are listed with variants
- [x] Validation rules are specified
- [x] Use cases cover main business flows
- [x] State machines defined for entities with lifecycle
- [x] Events defined for cross-module communication
- [x] Authorization requirements specified
- [x] Seed data requirements documented
- [x] New features for V2.0 documented
