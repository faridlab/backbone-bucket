# Bucket File Storage System - Technical Domain Documentation

**Version**: 1.0
**Date**: 2025-01-20
**Author**: StartApp Engineering Team
**Purpose**: Authoritative technical reference for implementing Bucket domain layer

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Domain-Driven Design Overview](#2-domain-driven-design-overview)
3. [Entities (Aggregate Roots)](#3-entities-aggregate-roots)
4. [Value Objects and Enums](#4-value-objects-and-enums)
5. [Domain Events](#5-domain-events)
6. [State Machines](#6-state-machines)
7. [Domain Services](#7-domain-services)
8. [Use Cases (CQRS)](#8-use-cases-cqrs)
9. [Repositories](#9-repositories)
10. [Specifications](#10-specifications)
11. [Workflows](#11-workflows)
12. [API Endpoints](#12-api-endpoints)
13. [Database Schema](#13-database-schema)
14. [Security Model](#14-security-model)

---

## 1. Introduction

### 1.1 Purpose

This document provides **complete technical specifications** for the Bucket File Storage System domain layer. It serves as:

- **Implementation guide** for repositories, use cases, and services
- **Migration blueprint** for PostgreSQL database
- **API contract** for both Backbone generic CRUD and domain-specific endpoints
- **Security reference** for virus scanning and access control

### 1.2 Architecture Principles

**Domain-Driven Design (DDD)**:
- Clean separation between domain, application, infrastructure, and presentation layers
- Schema-first approach for all domain definitions
- CQRS for read/write separation
- Event-driven architecture for file lifecycle events

**File Storage Architecture**:
- Multi-backend storage support (Local, S3, GCS, Azure Blob)
- Bucket-based organization (similar to S3)
- Quota management with tiered plans
- Virus scanning and security enforcement

**Two-Layer API Architecture**:
- **Layer 1**: Backbone Generic CRUD (auto-generated endpoints)
- **Layer 2**: Domain-Specific Use Cases (upload, download, share)

---

## 2. Domain-Driven Design Overview

### 2.1 Bounded Context

**Bucket File Storage Bounded Context** encompasses:
- File storage and retrieval operations
- Bucket organization and management
- User quota tracking and enforcement
- File sharing and access control
- Virus scanning and threat detection
- Image compression and thumbnail generation
- File versioning and history
- Access logging and audit trail

### 2.2 Ubiquitous Language

| Term | Definition |
|------|------------|
| **StoredFile** | A file uploaded and stored in the system |
| **Bucket** | A container for organizing files (similar to S3 bucket) |
| **UserQuota** | Storage limits and usage tracking for a user |
| **FileShare** | A sharing configuration for file access |
| **FileVersion** | A historical version of a file |
| **Thumbnail** | A compressed preview image of a file |
| **AccessLog** | Record of file access for audit purposes |
| **Storage Key** | Unique identifier for locating a file in storage |
| **Threat Level** | Security classification from virus scan |
| **Soft Delete** | Marking file as deleted without physical removal |
| **Quarantine** | Isolating potentially dangerous files |

### 2.3 Aggregates

**Primary Aggregates**:

1. **StoredFile Aggregate**
   - Root: StoredFile
   - Contains: Thumbnails (one-to-many)
   - Contains: FileVersions (one-to-many)
   - References: Bucket, UserQuota

2. **Bucket Aggregate**
   - Root: Bucket
   - Contains: StoredFiles (one-to-many)
   - Manages: Storage policies, MIME type restrictions

3. **UserQuota Aggregate**
   - Root: UserQuota
   - Tracks: Storage usage, file counts
   - Enforces: Storage limits, file size limits

4. **FileShare Aggregate**
   - Root: FileShare
   - References: StoredFile
   - Controls: Access permissions, download limits

5. **AccessLog Aggregate**
   - Root: AccessLog (append-only, immutable)
   - Tracks: All file operations

---

## 3. Entities (Aggregate Roots)

### 3.1 StoredFile Entity

**Location**: `libs/modules/bucket/src/domain/entity/stored_file.rs`

**Description**: Core aggregate root for file storage, tracking file metadata, security status, and compression state.

**Attributes**:

```rust
pub struct StoredFile {
    // Identity
    pub id: Uuid,
    pub bucket_id: Uuid,
    pub owner_id: Uuid,

    // File metadata
    pub path: String,                      // Virtual path within bucket
    pub original_name: String,             // Original filename
    pub size_bytes: i64,                   // File size in bytes
    pub mime_type: String,                 // MIME type
    pub checksum: Option<String>,          // File hash for integrity

    // Storage
    pub storage_key: String,               // Physical storage location

    // Compression
    pub is_compressed: bool,
    pub original_size: Option<i64>,        // Size before compression
    pub compression_algorithm: Option<String>,

    // Security
    pub is_scanned: bool,                  // Virus scan completed
    pub scan_result: Option<serde_json::Value>,
    pub threat_level: Option<ThreatLevel>,

    // Thumbnail
    pub has_thumbnail: bool,
    pub thumbnail_path: Option<String>,

    // Status
    pub status: FileStatus,

    // Versioning
    pub version: i32,
    pub previous_version_id: Option<Uuid>,

    // Access tracking
    pub download_count: i32,
    pub last_accessed_at: Option<DateTime<Utc>>,

    // Extensibility
    pub metadata: serde_json::Value,
}
```

**Business Rules**:
- Files must belong to an active bucket
- Files must pass virus scan before becoming Active
- Quarantined files cannot be downloaded
- Original name preserved, storage key generated
- Checksum required for integrity verification
- Compression only applied to eligible file types

**Invariants**:
- `id` is immutable after creation
- `bucket_id` and `owner_id` are immutable
- `storage_key` is immutable (files are copy-on-write)
- `size_bytes` must be > 0
- `version` starts at 1 and only increments
- Files in `Deleted` status cannot be re-activated directly

**Custom Methods**:

```rust
impl StoredFile {
    /// Check if file is safe to download
    pub fn is_safe(&self) -> bool;

    /// Check if file needs processing (scanning/compression)
    pub fn needs_processing(&self) -> bool;

    /// Check if file can be accessed
    pub fn is_accessible(&self) -> bool;

    /// Record a download access
    pub fn record_access(&mut self);

    /// Soft delete the file
    pub fn soft_delete(&mut self);

    /// Restore from deleted state
    pub fn restore(&mut self);

    /// Mark file as scanned with result
    pub fn mark_scanned(&mut self, threat_level: ThreatLevel, scan_result: Option<Value>);

    /// Set compression information
    pub fn set_compressed(&mut self, original_size: i64, algorithm: &str);

    /// Quarantine file after threat detection
    pub fn quarantine(&mut self, scan_result: Value, threat_level: ThreatLevel);
}
```

**PostgreSQL Table**: `stored_files`

---

### 3.2 Bucket Entity

**Location**: `libs/modules/bucket/src/domain/entity/bucket.rs`

**Description**: Container for organizing files with configurable policies.

**Attributes**:

```rust
pub struct Bucket {
    // Identity
    pub id: Uuid,
    pub owner_id: Uuid,

    // Naming
    pub name: String,                      // Display name
    pub slug: String,                      // URL-safe identifier
    pub description: Option<String>,

    // Type and status
    pub bucket_type: BucketType,
    pub status: BucketStatus,

    // Storage configuration
    pub storage_backend: StorageBackend,
    pub root_path: String,                 // Base path in storage

    // Statistics
    pub file_count: i32,
    pub total_size_bytes: i64,

    // Policies
    pub max_file_size: Option<i64>,        // Per-file size limit
    pub allowed_mime_types: Vec<String>,   // Allowed MIME patterns
    pub auto_delete_after_days: Option<i32>,

    // Extensibility
    pub metadata: serde_json::Value,
}
```

**Business Rules**:
- Slug must be unique and URL-safe
- Only Active buckets can receive uploads
- Readonly buckets allow downloads only
- Archived buckets are frozen (no operations)
- MIME type patterns support wildcards (e.g., `image/*`)
- Max file size enforced on upload

**Invariants**:
- `id` is immutable
- `slug` is immutable after creation
- `storage_backend` is immutable
- `file_count` and `total_size_bytes` must be >= 0
- Only one status transition at a time

**Custom Methods**:

```rust
impl Bucket {
    /// Validate if upload is allowed
    pub fn can_upload(&self, size: i64, mime_type: &str) -> Result<(), String>;

    /// Update bucket statistics
    pub fn update_stats(&mut self, size_delta: i64, count_delta: i32);

    /// Check if bucket is accessible
    pub fn is_accessible(&self) -> bool;

    /// Lock bucket (readonly)
    pub fn lock(&mut self);

    /// Unlock bucket
    pub fn unlock(&mut self);

    /// Archive bucket
    pub fn archive(&mut self);

    /// Restore from archived
    pub fn restore(&mut self);

    /// Check if bucket is empty
    pub fn is_empty(&self) -> bool;

    /// Calculate average file size
    pub fn average_file_size(&self) -> i64;
}
```

**PostgreSQL Table**: `buckets`

---

### 3.3 UserQuota Entity

**Location**: `libs/modules/bucket/src/domain/entity/user_quota.rs`

**Description**: Storage limits and usage tracking for a user.

**Attributes**:

```rust
pub struct UserQuota {
    // Identity
    pub id: Uuid,
    pub user_id: Uuid,

    // Limits
    pub limit_bytes: i64,                  // Total storage limit
    pub max_file_size: Option<i64>,        // Per-file size limit
    pub max_file_count: Option<i32>,       // Max number of files

    // Usage
    pub used_bytes: i64,
    pub file_count: i32,

    // Tier
    pub tier: String,                      // free, pro, enterprise

    // Warnings
    pub warning_threshold_percent: i32,    // Default: 80
    pub last_warning_sent_at: Option<DateTime<Utc>>,

    // Peak tracking
    pub peak_usage_bytes: i64,
    pub peak_usage_at: Option<DateTime<Utc>>,

    // Extensibility
    pub metadata: serde_json::Value,
}
```

**Business Rules**:
- Each user has exactly one quota record
- Usage cannot exceed limit
- Warnings sent when threshold exceeded
- Peak usage tracked for analytics
- Tier determines default limits

**Tier Limits**:
| Tier | Storage | Max File Size | Max Files |
|------|---------|---------------|-----------|
| free | 1 GB | 50 MB | 1,000 |
| pro | 100 GB | 500 MB | 50,000 |
| enterprise | 1 TB | 5 GB | Unlimited |

**Invariants**:
- `used_bytes` <= `limit_bytes`
- `file_count` >= 0
- `warning_threshold_percent` between 0-100

**Custom Methods**:

```rust
impl UserQuota {
    /// Check if space available for bytes
    pub fn has_space_for(&self, bytes: i64) -> bool;

    /// Get usage percentage
    pub fn usage_percent(&self) -> f64;

    /// Check if over warning threshold
    pub fn is_over_warning_threshold(&self) -> bool;

    /// Add usage (file upload)
    pub fn add_usage(&mut self, bytes: i64) -> Result<(), QuotaExceeded>;

    /// Subtract usage (file delete)
    pub fn subtract_usage(&mut self, bytes: i64);

    /// Get remaining bytes
    pub fn remaining_bytes(&self) -> i64;

    /// Validate file can be uploaded
    pub fn can_upload_file(&self, size: i64) -> Result<(), QuotaExceeded>;

    /// Update peak if current > peak
    pub fn update_peak(&mut self);
}
```

**PostgreSQL Table**: `user_quotas`

---

### 3.4 FileShare Entity

**Location**: `libs/modules/bucket/src/domain/entity/file_share.rs`

**Description**: Sharing configuration for file access.

**Attributes**:

```rust
pub struct FileShare {
    // Identity
    pub id: Uuid,
    pub file_id: Uuid,
    pub owner_id: Uuid,

    // Access token
    pub token: String,                     // URL-safe random token

    // Share type
    pub share_type: ShareType,             // Link, User, Password

    // Target users (for User share type)
    pub shared_with: Vec<Uuid>,

    // Password protection
    pub password_hash: Option<String>,

    // Permissions
    pub permission: SharePermission,       // View, Download, Edit

    // Limits
    pub expires_at: Option<DateTime<Utc>>,
    pub max_downloads: Option<i32>,
    pub download_count: i32,

    // Status
    pub is_active: bool,

    // Revocation
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by: Option<Uuid>,
}
```

**Business Rules**:
- Token must be unique and URL-safe (32+ chars)
- Password shares require password_hash
- User shares require at least one user in shared_with
- Download count cannot exceed max_downloads
- Expired shares are not accessible
- Revoked shares cannot be reactivated

**Invariants**:
- `token` is immutable
- `download_count` <= `max_downloads` (if set)
- `revoked_at` is null if `is_active` is true

**Custom Methods**:

```rust
impl FileShare {
    /// Check if share is still valid
    pub fn is_valid(&self) -> bool;

    /// Check if share has expired
    pub fn is_expired(&self) -> bool;

    /// Check if downloads remaining
    pub fn has_downloads_remaining(&self) -> bool;

    /// Check if user can access
    pub fn can_access(&self, user_id: Option<Uuid>) -> bool;

    /// Verify password
    pub fn verify_password(&self, password: &str) -> bool;

    /// Record download
    pub fn record_download(&mut self) -> bool;

    /// Revoke share
    pub fn revoke(&mut self, by_user_id: Uuid);

    /// Generate share URL
    pub fn generate_url(&self, base_url: &str) -> String;
}
```

**PostgreSQL Table**: `file_shares`

---

### 3.5 FileVersion Entity

**Location**: `libs/modules/bucket/src/domain/entity/file_version.rs`

**Description**: Historical version of a file.

**Attributes**:

```rust
pub struct FileVersion {
    // Identity
    pub id: Uuid,
    pub file_id: Uuid,

    // Version info
    pub version_number: i32,
    pub is_current: bool,

    // Storage
    pub storage_key: String,
    pub storage_backend: String,
    pub size_bytes: i64,
    pub checksum: Option<String>,

    // Metadata
    pub mime_type: String,
    pub original_name: String,
    pub metadata: serde_json::Value,

    // Status
    pub status: FileStatus,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
}
```

**Business Rules**:
- Only one version can be current at a time
- Versions are append-only (no updates)
- Expired versions auto-delete
- Storage key must be unique per version

**Invariants**:
- `version_number` >= 1
- `storage_key` is immutable
- Only one version per file_id with `is_current = true`

**Custom Methods**:

```rust
impl FileVersion {
    /// Check if version is expired
    pub fn is_expired(&self) -> bool;

    /// Check if version can be restored
    pub fn can_restore(&self) -> bool;

    /// Set as current version
    pub fn set_as_current(&mut self);

    /// Unset current flag
    pub fn unset_current(&mut self);

    /// Soft delete version
    pub fn soft_delete(&mut self);

    /// Get version label (e.g., "v1", "v2")
    pub fn version_label(&self) -> String;

    /// Check if version was restored
    pub fn is_restored(&self) -> bool;

    /// Get checksum or placeholder
    pub fn get_checksum(&self) -> &str;
}
```

**PostgreSQL Table**: `file_versions`

---

### 3.6 Thumbnail Entity

**Location**: `libs/modules/bucket/src/domain/entity/thumbnail.rs`

**Description**: Compressed preview image of a file.

**Attributes**:

```rust
pub struct Thumbnail {
    // Identity
    pub id: Uuid,
    pub file_id: Uuid,

    // Dimensions
    pub width: i32,
    pub height: i32,

    // Storage
    pub storage_key: String,
    pub storage_backend: String,
    pub size_bytes: i64,
    pub mime_type: String,
    pub format: String,                    // jpeg, webp, png

    // Generation
    pub generated_at: DateTime<Utc>,
    pub generation_time_ms: i32,

    // CDN
    pub cdn_url: String,
    pub cache_until: DateTime<Utc>,

    // Quality
    pub quality: i32,                      // 1-100

    // Versioning
    pub source_version: i32,               // File version this was generated from
    pub is_stale: bool,                    // Needs regeneration

    // Metadata
    pub metadata: serde_json::Value,
}
```

**Business Rules**:
- One thumbnail per file per size
- Standard sizes: 64, 128, 256, 512, 1024
- Regenerated when source file changes
- CDN URLs have TTL for cache invalidation

**Invariants**:
- `width` and `height` > 0
- `quality` between 1-100
- `source_version` must match file version when generated

**Custom Methods**:

```rust
impl Thumbnail {
    /// Check if thumbnail needs regeneration
    pub fn needs_regeneration(&self) -> bool;

    /// Mark as stale
    pub fn mark_stale(&mut self);

    /// Mark as fresh after regeneration
    pub fn mark_fresh(&mut self, source_version: i32);

    /// Check if cache expired
    pub fn is_cache_expired(&self) -> bool;

    /// Check if has valid CDN URL
    pub fn has_cdn_url(&self) -> bool;

    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f64;

    /// Get dimensions tuple
    pub fn dimensions(&self) -> (i32, i32);

    /// Get dimensions string (e.g., "256x256")
    pub fn dimensions_string(&self) -> String;

    /// Get size category (small, medium, large)
    pub fn category_size(&self) -> &str;

    /// Check if for current file version
    pub fn is_current_version(&self, file_version: i32) -> bool;
}
```

**PostgreSQL Table**: `thumbnails`

---

### 3.7 AccessLog Entity

**Location**: `libs/modules/bucket/src/domain/entity/access_log.rs`

**Description**: Immutable record of file access for audit purposes.

**Attributes**:

```rust
pub struct AccessLog {
    // Identity
    pub id: Uuid,
    pub file_id: Uuid,

    // Actor
    pub user_id: Option<Uuid>,             // Null for anonymous access
    pub share_id: Option<Uuid>,            // If accessed via share

    // Action
    pub action: AccessAction,

    // Context
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>,
    pub location: Option<serde_json::Value>,  // GeoIP data

    // Result
    pub success: bool,
    pub error_code: Option<String>,
    pub error_message: Option<String>,

    // Performance
    pub duration_ms: Option<i32>,
    pub bytes_transferred: Option<i64>,

    // Timestamp
    pub accessed_at: DateTime<Utc>,

    // Metadata
    pub metadata: serde_json::Value,
}
```

**Business Rules**:
- Append-only: No updates or deletes
- Retention: 90 days hot, 7 years archived
- All access attempts logged (success and failure)
- Anonymous access still tracked

**Invariants**:
- All fields immutable after creation
- `accessed_at` set on creation, never updated
- Either `user_id` or `share_id` should be set (or both null for anonymous)

**Custom Methods**:

```rust
impl AccessLog {
    /// Check if access was successful
    pub fn is_successful(&self) -> bool;

    /// Check if access was by owner
    pub fn is_by_owner(&self) -> bool;

    /// Check if access was via share link
    pub fn is_via_share(&self) -> bool;

    /// Check if anonymous access
    pub fn is_anonymous(&self) -> bool;

    /// Check if download action
    pub fn is_download(&self) -> bool;

    /// Check if upload action
    pub fn is_upload(&self) -> bool;

    /// Get formatted duration
    pub fn duration_string(&self) -> String;

    /// Get formatted bytes transferred
    pub fn bytes_transferred_string(&self) -> String;

    /// Get location summary
    pub fn location_summary(&self) -> Option<String>;
}
```

**PostgreSQL Table**: `access_logs`

---

## 4. Value Objects and Enums

### 4.1 FileStatus Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Uploading,      // File upload in progress
    Processing,     // Being processed (scanning, compression)
    Active,         // Ready for access
    Quarantined,    // Flagged by virus scan
    Deleted,        // Soft deleted
    Archived,       // Long-term storage
}
```

**State Transitions**:
- `Uploading` -> `Processing` (upload complete)
- `Processing` -> `Active` (scan passed)
- `Processing` -> `Quarantined` (threat detected)
- `Active` -> `Deleted` (soft delete)
- `Active` -> `Archived` (archive)
- `Deleted` -> `Active` (restore)
- `Archived` -> `Active` (unarchive)
- `Quarantined` -> `Active` (manual review passed)

---

### 4.2 BucketStatus Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketStatus {
    Active,         // Fully operational
    Readonly,       // Downloads only, no uploads
    Locked,         // No operations allowed
    Archived,       // Long-term storage, readonly
}
```

---

### 4.3 BucketType Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketType {
    User,           // User's personal bucket
    Team,           // Shared team bucket
    Project,        // Project-specific bucket
    System,         // System/application bucket
    Public,         // Publicly accessible bucket
}
```

---

### 4.4 StorageBackend Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBackend {
    Local,          // Local filesystem
    S3,             // Amazon S3
    Gcs,            // Google Cloud Storage
    AzureBlob,      // Azure Blob Storage
    Minio,          // MinIO (S3-compatible)
}
```

---

### 4.5 ThreatLevel Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    Safe,           // No threats detected
    Low,            // Minor concerns
    Medium,         // Potential threats, quarantine recommended
    High,           // Dangerous, block access
    Critical,       // Malware confirmed, immediate action required
}
```

---

### 4.6 ShareType Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareType {
    Link,           // Public link (anyone with URL)
    User,           // Specific users only
    Password,       // Password-protected link
}
```

---

### 4.7 SharePermission Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharePermission {
    View,           // Preview only
    Download,       // Download allowed
    Edit,           // Modify allowed
}
```

---

### 4.8 AccessAction Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessAction {
    Download,       // File downloaded
    View,           // File viewed/previewed
    Preview,        // Thumbnail/preview accessed
    Share,          // File shared
    ShareAccess,    // Access via share link
    Upload,         // File uploaded
    Delete,         // File deleted
    Rename,         // File renamed
    Move,           // File moved
    Copy,           // File copied
    Restore,        // File restored from trash
    MetadataUpdate, // Metadata modified
}
```

---

### 4.9 QuotaExceeded Error

```rust
#[derive(Debug, Clone)]
pub struct QuotaExceeded {
    pub user_id: Uuid,
    pub used: i64,
    pub limit: i64,
    pub requested: i64,
}

impl std::fmt::Display for QuotaExceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quota exceeded for user {}: {} + {} > {}",
            self.user_id, self.used, self.requested, self.limit)
    }
}
```

---

## 5. Domain Events

### 5.1 File Events

```rust
// File lifecycle events
pub struct FileUploadedEvent {
    pub file_id: Uuid,
    pub bucket_id: Uuid,
    pub owner_id: Uuid,
    pub file_name: String,
    pub size_bytes: i64,
    pub mime_type: String,
    pub uploaded_at: DateTime<Utc>,
}

pub struct FileDownloadedEvent {
    pub file_id: Uuid,
    pub user_id: Option<Uuid>,
    pub share_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub downloaded_at: DateTime<Utc>,
}

pub struct FileDeletedEvent {
    pub file_id: Uuid,
    pub deleted_by: Uuid,
    pub deletion_type: DeletionType,  // Soft, Hard
    pub deleted_at: DateTime<Utc>,
}

pub struct FileRestoredEvent {
    pub file_id: Uuid,
    pub restored_by: Uuid,
    pub restored_at: DateTime<Utc>,
}

pub struct FileMovedEvent {
    pub file_id: Uuid,
    pub from_bucket: Uuid,
    pub to_bucket: Uuid,
    pub from_path: String,
    pub to_path: String,
    pub moved_by: Uuid,
    pub moved_at: DateTime<Utc>,
}
```

### 5.2 Security Events

```rust
pub struct FileScannedEvent {
    pub file_id: Uuid,
    pub threat_level: ThreatLevel,
    pub threats_found: Vec<String>,
    pub scan_engine: String,
    pub scanned_at: DateTime<Utc>,
}

pub struct FileQuarantinedEvent {
    pub file_id: Uuid,
    pub threat_level: ThreatLevel,
    pub reason: String,
    pub quarantined_at: DateTime<Utc>,
}

pub struct FileClearedEvent {
    pub file_id: Uuid,
    pub cleared_by: Uuid,
    pub reason: String,
    pub cleared_at: DateTime<Utc>,
}
```

### 5.3 Share Events

```rust
pub struct FileSharedEvent {
    pub share_id: Uuid,
    pub file_id: Uuid,
    pub shared_by: Uuid,
    pub share_type: ShareType,
    pub permission: SharePermission,
    pub expires_at: Option<DateTime<Utc>>,
    pub shared_at: DateTime<Utc>,
}

pub struct ShareAccessedEvent {
    pub share_id: Uuid,
    pub file_id: Uuid,
    pub accessed_by: Option<Uuid>,
    pub ip_address: Option<String>,
    pub accessed_at: DateTime<Utc>,
}

pub struct ShareRevokedEvent {
    pub share_id: Uuid,
    pub revoked_by: Uuid,
    pub reason: Option<String>,
    pub revoked_at: DateTime<Utc>,
}
```

### 5.4 Quota Events

```rust
pub struct QuotaWarningEvent {
    pub user_id: Uuid,
    pub usage_percent: f64,
    pub used_bytes: i64,
    pub limit_bytes: i64,
    pub warned_at: DateTime<Utc>,
}

pub struct QuotaExceededEvent {
    pub user_id: Uuid,
    pub requested_bytes: i64,
    pub used_bytes: i64,
    pub limit_bytes: i64,
    pub operation: String,
    pub occurred_at: DateTime<Utc>,
}
```

---

## 6. State Machines

### 6.1 File Status State Machine

```
                    ┌──────────────┐
                    │  Uploading   │
                    └──────┬───────┘
                           │ upload_complete
                           ▼
                    ┌──────────────┐
                    │  Processing  │
                    └──────┬───────┘
                           │
          ┌────────────────┼────────────────┐
          │ scan_clean     │ threat_found   │
          ▼                ▼                │
   ┌──────────────┐ ┌──────────────┐        │
   │    Active    │ │ Quarantined  │        │
   └──────┬───────┘ └──────┬───────┘        │
          │                │                │
    ┌─────┴─────┐    manual_clear           │
    │           │          │                │
soft_delete  archive       │                │
    │           │          ▼                │
    ▼           ▼   ┌──────────────┐        │
┌──────────┐ ┌──────────────┐              │
│ Deleted  │ │  Archived    │              │
└──────┬───┘ └──────┬───────┘              │
       │            │                       │
    restore     unarchive                   │
       │            │                       │
       └────────────┴───────────────────────┘
                    │
                    ▼
             ┌──────────────┐
             │    Active    │
             └──────────────┘
```

### 6.2 Bucket Status State Machine

```
        ┌──────────────┐
        │    Active    │◄───────────────┐
        └──────┬───────┘                │
               │                        │
     ┌─────────┼─────────┐              │
     │ lock    │ archive │              │
     ▼         │         ▼              │
┌──────────┐   │   ┌──────────────┐     │
│ Readonly │   │   │   Archived   │     │
└──────┬───┘   │   └──────┬───────┘     │
       │       │          │             │
    lock       │      restore           │
       │       │          │             │
       ▼       │          │             │
┌──────────┐   │          │             │
│  Locked  │───┼──────────┴─────────────┘
└──────────┘   │         unlock
               │
            readonly
               │
               ▼
        ┌──────────┐
        │ Readonly │
        └──────────┘
```

### 6.3 Share State Machine

```
┌──────────────┐
│   Created    │
└──────┬───────┘
       │ activate
       ▼
┌──────────────┐
│    Active    │◄─────────────────────────┐
└──────┬───────┘                          │
       │                                  │
       ├──────────────┬──────────────┐    │
       │ expire       │ max_downloads│    │
       │              │ reached      │    │
       ▼              ▼              │    │
┌──────────────┐ ┌──────────────┐   │    │
│   Expired    │ │  Exhausted   │   │    │
└──────────────┘ └──────────────┘   │    │
                                    │    │
                              revoke│    │extend
                                    │    │
                                    ▼    │
                             ┌──────────────┐
                             │   Revoked    │
                             └──────────────┘
```

---

## 7. Domain Services

### 7.1 StorageService

**Location**: `libs/modules/bucket/src/domain/services/storage_service.rs`

**Description**: Handles physical file storage operations.

```rust
pub struct StorageService {
    root_path: PathBuf,
    buckets_dir: PathBuf,
    trash_dir: PathBuf,
    thumbnails_dir: PathBuf,
}

impl StorageService {
    /// Store file content
    pub async fn store_file(&self, bucket: &str, path: &str, content: &[u8])
        -> Result<String, StorageError>;

    /// Read file content
    pub async fn read_file(&self, storage_key: &str)
        -> Result<Vec<u8>, StorageError>;

    /// Delete file
    pub async fn delete_file(&self, storage_key: &str)
        -> Result<(), StorageError>;

    /// Move file to trash
    pub async fn move_to_trash(&self, storage_key: &str, file_id: Uuid)
        -> Result<String, StorageError>;

    /// Restore from trash
    pub async fn restore_from_trash(&self, file_id: Uuid, original_key: &str)
        -> Result<(), StorageError>;

    /// Store thumbnail
    pub async fn store_thumbnail(&self, file_id: Uuid, content: &[u8])
        -> Result<String, StorageError>;

    /// Generate unique storage key
    pub fn generate_storage_key(&self, bucket: &str, path: &str) -> String;
}
```

---

### 7.2 VirusScannerService

**Location**: `libs/modules/bucket/src/domain/services/virus_scanner.rs`

**Description**: Scans files for viruses and malware.

```rust
pub struct VirusScanResult {
    pub threat_level: ThreatLevel,
    pub threats: Vec<String>,
    pub scan_details: serde_json::Value,
}

pub struct VirusScannerService {
    blocked_extensions: Vec<String>,
}

impl VirusScannerService {
    /// Scan file content
    pub fn scan(&self, content: &[u8], filename: &str) -> VirusScanResult;

    /// Check if scan result should block file
    pub fn should_block(&self, result: &VirusScanResult) -> bool;

    /// Check if scan result should quarantine file
    pub fn should_quarantine(&self, result: &VirusScanResult) -> bool;
}
```

**Scanning Rules**:
1. **Extension blocking**: exe, bat, scr, dll, sys, vbs, ps1, cmd, msi, com
2. **Magic byte detection**: MZ (Windows), ELF (Linux), Mach-O (macOS)
3. **Script detection**: JavaScript, VBScript embedded in documents
4. **Archive scanning**: ZIP, RAR contents scanned recursively

---

### 7.3 ImageCompressorService

**Location**: `libs/modules/bucket/src/domain/services/image_compressor.rs`

**Description**: Compresses images and generates thumbnails.

```rust
pub struct CompressionResult {
    pub content: Vec<u8>,
    pub original_size: u64,
    pub compressed_size: u64,
    pub was_compressed: bool,
    pub algorithm: Option<String>,
}

pub struct ImageCompressorService {
    quality: u8,                     // Default: 85
    min_size_for_compression: u64,   // Default: 100KB
}

impl ImageCompressorService {
    /// Compress image
    pub fn compress(&self, content: &[u8]) -> Result<CompressionResult, ImageCompressionError>;

    /// Generate thumbnail
    pub fn generate_thumbnail(&self, content: &[u8], size: u32)
        -> Result<Vec<u8>, ImageCompressionError>;

    /// Check if content is an image
    pub fn is_image(&self, content: &[u8]) -> bool;

    /// Get image dimensions
    pub fn get_dimensions(&self, content: &[u8])
        -> Result<(u32, u32), ImageCompressionError>;
}
```

**Compression Settings**:
| Format | Quality | Algorithm |
|--------|---------|-----------|
| JPEG | 85 | MozJPEG |
| PNG | Lossless | oxipng |
| WebP | 80 | libwebp |
| GIF | - | gifsicle |

---

### 7.4 FileUploadService

**Location**: `libs/modules/bucket/src/domain/services/file_upload_service.rs`

**Description**: Orchestrates the complete file upload workflow.

```rust
pub struct FileUploadService {
    storage: Arc<StorageService>,
    scanner: Arc<VirusScannerService>,
    compressor: Arc<ImageCompressorService>,
    auto_compress_images: bool,
    thumbnail_size: u32,
}

pub struct UploadRequest {
    pub bucket_id: Uuid,
    pub owner_id: Uuid,
    pub path: String,
    pub filename: String,
    pub content: Vec<u8>,
    pub mime_type: String,
    pub metadata: Option<serde_json::Value>,
}

pub struct UploadResult {
    pub file: StoredFile,
    pub was_compressed: bool,
    pub compression_ratio: Option<f64>,
    pub scan_result: VirusScanResult,
}

impl FileUploadService {
    /// Upload file with full workflow
    pub async fn upload(
        &self,
        request: UploadRequest,
        bucket: &Bucket,
        quota: &mut UserQuota,
    ) -> Result<UploadResult, UploadError>;

    /// Upload new version of existing file
    pub async fn upload_version(
        &self,
        request: UploadRequest,
        bucket: &Bucket,
        quota: &mut UserQuota,
        previous_file: &StoredFile,
    ) -> Result<UploadResult, UploadError>;
}
```

**Upload Workflow**:
1. Validate bucket restrictions
2. Check user quota
3. Scan for viruses
4. Compress if image and enabled
5. Store file
6. Generate thumbnail if image
7. Create entity
8. Update quota
9. Return result (or quarantine)

---

### 7.5 AccessLoggerService

**Location**: `libs/modules/bucket/src/domain/services/access_logger.rs`

**Description**: Creates access log entries for audit trail.

```rust
pub struct AccessLoggerService;

impl AccessLoggerService {
    /// Log download action
    pub fn log_download(
        &self,
        file_id: Uuid,
        user_id: Option<Uuid>,
        share_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AccessLog;

    /// Log view action
    pub fn log_view(
        &self,
        file_id: Uuid,
        user_id: Option<Uuid>,
        share_id: Option<Uuid>,
    ) -> AccessLog;

    /// Log upload action
    pub fn log_upload(
        &self,
        file_id: Uuid,
        user_id: Uuid,
        size_bytes: i64,
    ) -> AccessLog;

    /// Log share action
    pub fn log_share(
        &self,
        file_id: Uuid,
        share_id: Uuid,
        user_id: Uuid,
    ) -> AccessLog;
}
```

---

## 8. Use Cases (CQRS)

### 8.1 Commands (Write Operations)

#### File Commands

```rust
// Upload file
pub struct UploadFileCommand {
    pub bucket_id: Uuid,
    pub owner_id: Uuid,
    pub path: String,
    pub filename: String,
    pub content: Vec<u8>,
    pub mime_type: String,
    pub metadata: Option<serde_json::Value>,
}

// Delete file
pub struct DeleteFileCommand {
    pub file_id: Uuid,
    pub deleted_by: Uuid,
    pub hard_delete: bool,
}

// Restore file
pub struct RestoreFileCommand {
    pub file_id: Uuid,
    pub restored_by: Uuid,
}

// Move file
pub struct MoveFileCommand {
    pub file_id: Uuid,
    pub target_bucket_id: Uuid,
    pub target_path: String,
    pub moved_by: Uuid,
}

// Copy file
pub struct CopyFileCommand {
    pub file_id: Uuid,
    pub target_bucket_id: Uuid,
    pub target_path: String,
    pub copied_by: Uuid,
}

// Rename file
pub struct RenameFileCommand {
    pub file_id: Uuid,
    pub new_name: String,
    pub renamed_by: Uuid,
}
```

#### Bucket Commands

```rust
// Create bucket
pub struct CreateBucketCommand {
    pub name: String,
    pub slug: String,
    pub owner_id: Uuid,
    pub bucket_type: BucketType,
    pub storage_backend: StorageBackend,
    pub max_file_size: Option<i64>,
    pub allowed_mime_types: Vec<String>,
}

// Update bucket
pub struct UpdateBucketCommand {
    pub bucket_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_file_size: Option<i64>,
    pub allowed_mime_types: Option<Vec<String>>,
}

// Lock bucket
pub struct LockBucketCommand {
    pub bucket_id: Uuid,
    pub locked_by: Uuid,
}

// Archive bucket
pub struct ArchiveBucketCommand {
    pub bucket_id: Uuid,
    pub archived_by: Uuid,
}
```

#### Share Commands

```rust
// Create share
pub struct CreateShareCommand {
    pub file_id: Uuid,
    pub owner_id: Uuid,
    pub share_type: ShareType,
    pub permission: SharePermission,
    pub password: Option<String>,
    pub shared_with: Vec<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_downloads: Option<i32>,
}

// Revoke share
pub struct RevokeShareCommand {
    pub share_id: Uuid,
    pub revoked_by: Uuid,
    pub reason: Option<String>,
}

// Update share
pub struct UpdateShareCommand {
    pub share_id: Uuid,
    pub permission: Option<SharePermission>,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_downloads: Option<i32>,
}
```

### 8.2 Queries (Read Operations)

#### File Queries

```rust
// Get file by ID
pub struct GetFileByIdQuery {
    pub file_id: Uuid,
}

// List files in bucket
pub struct ListFilesQuery {
    pub bucket_id: Uuid,
    pub path: Option<String>,
    pub status: Option<FileStatus>,
    pub mime_type_filter: Option<String>,
    pub page: i32,
    pub limit: i32,
    pub sort_by: String,
    pub sort_order: SortOrder,
}

// Search files
pub struct SearchFilesQuery {
    pub owner_id: Uuid,
    pub query: String,
    pub bucket_id: Option<Uuid>,
    pub mime_type: Option<String>,
    pub min_size: Option<i64>,
    pub max_size: Option<i64>,
    pub page: i32,
    pub limit: i32,
}

// Get file versions
pub struct GetFileVersionsQuery {
    pub file_id: Uuid,
    pub include_deleted: bool,
}
```

#### Bucket Queries

```rust
// Get bucket by ID
pub struct GetBucketByIdQuery {
    pub bucket_id: Uuid,
}

// Get bucket by slug
pub struct GetBucketBySlugQuery {
    pub slug: String,
}

// List user buckets
pub struct ListUserBucketsQuery {
    pub user_id: Uuid,
    pub bucket_type: Option<BucketType>,
    pub status: Option<BucketStatus>,
    pub page: i32,
    pub limit: i32,
}

// Get bucket statistics
pub struct GetBucketStatsQuery {
    pub bucket_id: Uuid,
}
```

#### Quota Queries

```rust
// Get user quota
pub struct GetUserQuotaQuery {
    pub user_id: Uuid,
}

// Get quota usage history
pub struct GetQuotaUsageHistoryQuery {
    pub user_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub granularity: Granularity,  // Day, Week, Month
}
```

---

## 9. Repositories

### 9.1 StoredFileRepository

```rust
#[async_trait]
pub trait StoredFileRepository {
    async fn save(&self, file: &StoredFile) -> Result<StoredFile, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<StoredFile>, RepositoryError>;
    async fn find_by_bucket(&self, bucket_id: Uuid, path: Option<&str>, page: i32, limit: i32)
        -> Result<(Vec<StoredFile>, i64), RepositoryError>;
    async fn find_by_owner(&self, owner_id: Uuid, page: i32, limit: i32)
        -> Result<(Vec<StoredFile>, i64), RepositoryError>;
    async fn search(&self, query: &SearchFilesQuery)
        -> Result<(Vec<StoredFile>, i64), RepositoryError>;
    async fn update(&self, file: &StoredFile) -> Result<StoredFile, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn count_by_bucket(&self, bucket_id: Uuid) -> Result<i64, RepositoryError>;
    async fn sum_size_by_bucket(&self, bucket_id: Uuid) -> Result<i64, RepositoryError>;
}
```

### 9.2 BucketRepository

```rust
#[async_trait]
pub trait BucketRepository {
    async fn save(&self, bucket: &Bucket) -> Result<Bucket, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Bucket>, RepositoryError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Bucket>, RepositoryError>;
    async fn find_by_owner(&self, owner_id: Uuid, page: i32, limit: i32)
        -> Result<(Vec<Bucket>, i64), RepositoryError>;
    async fn update(&self, bucket: &Bucket) -> Result<Bucket, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn slug_exists(&self, slug: &str) -> Result<bool, RepositoryError>;
}
```

### 9.3 UserQuotaRepository

```rust
#[async_trait]
pub trait UserQuotaRepository {
    async fn save(&self, quota: &UserQuota) -> Result<UserQuota, RepositoryError>;
    async fn find_by_user(&self, user_id: Uuid) -> Result<Option<UserQuota>, RepositoryError>;
    async fn update(&self, quota: &UserQuota) -> Result<UserQuota, RepositoryError>;
    async fn find_over_threshold(&self, threshold_percent: f64)
        -> Result<Vec<UserQuota>, RepositoryError>;
}
```

### 9.4 FileShareRepository

```rust
#[async_trait]
pub trait FileShareRepository {
    async fn save(&self, share: &FileShare) -> Result<FileShare, RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<FileShare>, RepositoryError>;
    async fn find_by_token(&self, token: &str) -> Result<Option<FileShare>, RepositoryError>;
    async fn find_by_file(&self, file_id: Uuid) -> Result<Vec<FileShare>, RepositoryError>;
    async fn find_active_by_file(&self, file_id: Uuid) -> Result<Vec<FileShare>, RepositoryError>;
    async fn update(&self, share: &FileShare) -> Result<FileShare, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn revoke_all_for_file(&self, file_id: Uuid, revoked_by: Uuid) -> Result<i64, RepositoryError>;
}
```

### 9.5 AccessLogRepository

```rust
#[async_trait]
pub trait AccessLogRepository {
    async fn save(&self, log: &AccessLog) -> Result<AccessLog, RepositoryError>;
    async fn find_by_file(&self, file_id: Uuid, page: i32, limit: i32)
        -> Result<(Vec<AccessLog>, i64), RepositoryError>;
    async fn find_by_user(&self, user_id: Uuid, page: i32, limit: i32)
        -> Result<(Vec<AccessLog>, i64), RepositoryError>;
    async fn find_by_action(&self, action: AccessAction, start: DateTime<Utc>, end: DateTime<Utc>)
        -> Result<Vec<AccessLog>, RepositoryError>;
    async fn count_downloads_by_file(&self, file_id: Uuid) -> Result<i64, RepositoryError>;
}
```

---

## 10. Specifications

### 10.1 File Upload Specification

```rust
pub struct FileUploadSpecification {
    pub max_file_size: i64,
    pub allowed_mime_types: Vec<String>,
    pub require_virus_scan: bool,
    pub auto_compress_images: bool,
    pub generate_thumbnails: bool,
}

impl FileUploadSpecification {
    pub fn is_satisfied_by(&self, file: &UploadRequest, bucket: &Bucket, quota: &UserQuota) -> bool {
        self.check_size(file.content.len() as i64)
            && self.check_mime_type(&file.mime_type)
            && bucket.can_upload(file.content.len() as i64, &file.mime_type).is_ok()
            && quota.has_space_for(file.content.len() as i64)
    }
}
```

### 10.2 File Access Specification

```rust
pub struct FileAccessSpecification {
    pub require_active_status: bool,
    pub require_safe_scan: bool,
    pub check_owner: bool,
}

impl FileAccessSpecification {
    pub fn can_access(&self, file: &StoredFile, user_id: Option<Uuid>) -> bool {
        if self.require_active_status && file.status != FileStatus::Active {
            return false;
        }
        if self.require_safe_scan && !file.is_safe() {
            return false;
        }
        if self.check_owner {
            if let Some(uid) = user_id {
                return file.owner_id == uid;
            }
            return false;
        }
        true
    }
}
```

### 10.3 Share Validity Specification

```rust
pub struct ShareValiditySpecification {
    pub check_expiry: bool,
    pub check_downloads: bool,
    pub check_revoked: bool,
}

impl ShareValiditySpecification {
    pub fn is_valid(&self, share: &FileShare) -> bool {
        if self.check_revoked && !share.is_active {
            return false;
        }
        if self.check_expiry && share.is_expired() {
            return false;
        }
        if self.check_downloads && !share.has_downloads_remaining() {
            return false;
        }
        true
    }
}
```

---

## 11. Workflows

### 11.1 File Upload Workflow

**Trigger**: `UploadFileCommand`

**Steps**:
1. **Validate Request**: Check file size, MIME type
2. **Check Quota**: Verify user has sufficient storage
3. **Scan File**: Run virus scanner
4. **Block if Threat**: Return error if critical/high threat
5. **Compress Image**: If applicable and enabled
6. **Store File**: Write to storage backend
7. **Generate Thumbnail**: If image
8. **Create Record**: Save StoredFile entity
9. **Update Quota**: Increment used bytes
10. **Emit Event**: FileUploadedEvent

**Compensations**:
- On quota failure: Delete stored file
- On scan failure: Mark as quarantined
- On storage failure: Return error, no quota update

### 11.2 File Version Cleanup Workflow

**Trigger**: Scheduled (daily at 2 AM)

**Steps**:
1. **Find Expired Versions**: Query versions past retention
2. **For Each Version**:
   - Delete from storage
   - Delete version record
   - Update bucket stats
3. **Log Cleanup Stats**: Record deleted count/size
4. **Emit Event**: VersionCleanupCompletedEvent

### 11.3 Quota Warning Workflow

**Trigger**: `FileUploadedEvent`

**Steps**:
1. **Get User Quota**: Load current quota
2. **Check Threshold**: If usage > warning_threshold
3. **Check Last Warning**: If > 24 hours since last warning
4. **Send Warning**: Email/notification to user
5. **Update Last Warning**: Set last_warning_sent_at
6. **Emit Event**: QuotaWarningEvent

---

## 12. API Endpoints

### 12.1 Layer 1: Backbone Generic CRUD

**All 7 entities get standard endpoints**:

```
GET    /api/v1/{collection}              - List (paginated, filtered, sorted)
POST   /api/v1/{collection}              - Create
GET    /api/v1/{collection}/:id          - Get by ID
PUT    /api/v1/{collection}/:id          - Full update
PATCH  /api/v1/{collection}/:id          - Partial update
DELETE /api/v1/{collection}/:id          - Soft delete
POST   /api/v1/{collection}/bulk         - Bulk create
POST   /api/v1/{collection}/upcreate     - Upsert
GET    /api/v1/{collection}/trash        - List deleted
POST   /api/v1/{collection}/:id/restore  - Restore
DELETE /api/v1/{collection}/empty        - Empty trash (hard delete)
```

**Collections**:
1. `files`
2. `buckets`
3. `user-quotas`
4. `file-shares`
5. `file-versions`
6. `thumbnails`
7. `access-logs` (read-only: GET list, GET :id only)

### 12.2 Layer 2: Domain-Specific Endpoints

#### File Operations

```
POST   /api/v1/files/upload              - Upload file (multipart)
POST   /api/v1/files/upload-chunk        - Chunked upload
GET    /api/v1/files/:id/download        - Download file
GET    /api/v1/files/:id/preview         - Preview file (images)
GET    /api/v1/files/:id/thumbnail/:size - Get thumbnail
POST   /api/v1/files/:id/move            - Move file
POST   /api/v1/files/:id/copy            - Copy file
POST   /api/v1/files/:id/rename          - Rename file
GET    /api/v1/files/:id/versions        - Get file versions
POST   /api/v1/files/:id/restore-version - Restore specific version
POST   /api/v1/files/search              - Search files
GET    /api/v1/files/:id/access-logs     - Get access history
```

#### Bucket Operations

```
GET    /api/v1/buckets/:slug             - Get bucket by slug
GET    /api/v1/buckets/:id/files         - List bucket files
GET    /api/v1/buckets/:id/stats         - Get bucket statistics
POST   /api/v1/buckets/:id/lock          - Lock bucket
POST   /api/v1/buckets/:id/unlock        - Unlock bucket
POST   /api/v1/buckets/:id/archive       - Archive bucket
POST   /api/v1/buckets/:id/restore       - Restore bucket
```

#### Share Operations

```
POST   /api/v1/files/:id/share           - Create share
GET    /api/v1/shares/:token             - Access shared file (public)
GET    /api/v1/shares/:token/download    - Download via share
POST   /api/v1/shares/:token/verify      - Verify password
GET    /api/v1/files/:id/shares          - List file shares
DELETE /api/v1/shares/:id                - Revoke share
```

#### Quota Operations

```
GET    /api/v1/users/:id/quota           - Get user quota
GET    /api/v1/users/:id/quota/usage     - Get usage history
GET    /api/v1/users/:id/storage-stats   - Get storage breakdown
```

#### Admin Operations

```
POST   /api/v1/admin/files/:id/scan      - Re-scan file
POST   /api/v1/admin/files/:id/clear     - Clear quarantine
GET    /api/v1/admin/quarantined         - List quarantined files
POST   /api/v1/admin/cleanup             - Trigger cleanup job
GET    /api/v1/admin/stats               - System statistics
```

---

## 13. Database Schema

### 13.1 PostgreSQL Tables

#### stored_files Table

```sql
CREATE TABLE stored_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bucket_id UUID NOT NULL REFERENCES buckets(id),
    owner_id UUID NOT NULL,

    path VARCHAR(1000) NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    size_bytes BIGINT NOT NULL CHECK (size_bytes > 0),
    mime_type VARCHAR(255) NOT NULL,
    checksum VARCHAR(64),
    storage_key VARCHAR(500) NOT NULL,

    is_compressed BOOLEAN NOT NULL DEFAULT FALSE,
    original_size BIGINT,
    compression_algorithm VARCHAR(50),

    is_scanned BOOLEAN NOT NULL DEFAULT FALSE,
    scan_result JSONB,
    threat_level VARCHAR(20) CHECK (threat_level IN ('safe', 'low', 'medium', 'high', 'critical')),

    has_thumbnail BOOLEAN NOT NULL DEFAULT FALSE,
    thumbnail_path VARCHAR(500),

    status VARCHAR(20) NOT NULL DEFAULT 'uploading'
        CHECK (status IN ('uploading', 'processing', 'active', 'quarantined', 'deleted', 'archived')),

    version INTEGER NOT NULL DEFAULT 1,
    previous_version_id UUID REFERENCES stored_files(id),

    download_count INTEGER NOT NULL DEFAULT 0,
    last_accessed_at TIMESTAMPTZ,

    metadata JSONB NOT NULL DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT stored_files_storage_key_unique UNIQUE (storage_key)
);

CREATE INDEX idx_stored_files_bucket ON stored_files(bucket_id, path);
CREATE INDEX idx_stored_files_owner ON stored_files(owner_id);
CREATE INDEX idx_stored_files_status ON stored_files(status);
CREATE INDEX idx_stored_files_mime_type ON stored_files(mime_type);
CREATE INDEX idx_stored_files_created_at ON stored_files(created_at DESC);
CREATE INDEX idx_stored_files_name_search ON stored_files USING gin(original_name gin_trgm_ops);
```

#### buckets Table

```sql
CREATE TABLE buckets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL,

    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    description TEXT,

    bucket_type VARCHAR(20) NOT NULL DEFAULT 'user'
        CHECK (bucket_type IN ('user', 'team', 'project', 'system', 'public')),
    status VARCHAR(20) NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'readonly', 'locked', 'archived')),

    storage_backend VARCHAR(20) NOT NULL DEFAULT 'local'
        CHECK (storage_backend IN ('local', 's3', 'gcs', 'azure_blob', 'minio')),
    root_path VARCHAR(500) NOT NULL,

    file_count INTEGER NOT NULL DEFAULT 0,
    total_size_bytes BIGINT NOT NULL DEFAULT 0,

    max_file_size BIGINT,
    allowed_mime_types TEXT[] NOT NULL DEFAULT '{}',
    auto_delete_after_days INTEGER,

    metadata JSONB NOT NULL DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT buckets_slug_unique UNIQUE (slug),
    CONSTRAINT buckets_file_count_positive CHECK (file_count >= 0),
    CONSTRAINT buckets_size_positive CHECK (total_size_bytes >= 0)
);

CREATE INDEX idx_buckets_owner ON buckets(owner_id);
CREATE INDEX idx_buckets_status ON buckets(status);
CREATE INDEX idx_buckets_type ON buckets(bucket_type);
```

#### user_quotas Table

```sql
CREATE TABLE user_quotas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,

    limit_bytes BIGINT NOT NULL DEFAULT 1073741824,  -- 1GB
    max_file_size BIGINT,
    max_file_count INTEGER,

    used_bytes BIGINT NOT NULL DEFAULT 0,
    file_count INTEGER NOT NULL DEFAULT 0,

    tier VARCHAR(20) NOT NULL DEFAULT 'free',

    warning_threshold_percent INTEGER NOT NULL DEFAULT 80,
    last_warning_sent_at TIMESTAMPTZ,

    peak_usage_bytes BIGINT NOT NULL DEFAULT 0,
    peak_usage_at TIMESTAMPTZ,

    metadata JSONB NOT NULL DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT user_quotas_user_unique UNIQUE (user_id),
    CONSTRAINT user_quotas_usage_valid CHECK (used_bytes <= limit_bytes),
    CONSTRAINT user_quotas_threshold_valid CHECK (warning_threshold_percent BETWEEN 0 AND 100)
);

CREATE INDEX idx_user_quotas_tier ON user_quotas(tier);
CREATE INDEX idx_user_quotas_usage_percent ON user_quotas((used_bytes::float / limit_bytes::float));
```

#### file_shares Table

```sql
CREATE TABLE file_shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL REFERENCES stored_files(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL,

    token VARCHAR(64) NOT NULL,

    share_type VARCHAR(20) NOT NULL DEFAULT 'link'
        CHECK (share_type IN ('link', 'user', 'password')),

    shared_with UUID[] NOT NULL DEFAULT '{}',

    password_hash VARCHAR(255),

    permission VARCHAR(20) NOT NULL DEFAULT 'view'
        CHECK (permission IN ('view', 'download', 'edit')),

    expires_at TIMESTAMPTZ,
    max_downloads INTEGER,
    download_count INTEGER NOT NULL DEFAULT 0,

    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    revoked_at TIMESTAMPTZ,
    revoked_by UUID,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT file_shares_token_unique UNIQUE (token),
    CONSTRAINT file_shares_download_valid CHECK (
        max_downloads IS NULL OR download_count <= max_downloads
    ),
    CONSTRAINT file_shares_revoked_valid CHECK (
        (is_active = TRUE AND revoked_at IS NULL) OR
        (is_active = FALSE AND revoked_at IS NOT NULL)
    )
);

CREATE INDEX idx_file_shares_file ON file_shares(file_id);
CREATE INDEX idx_file_shares_owner ON file_shares(owner_id);
CREATE INDEX idx_file_shares_active ON file_shares(is_active, expires_at);
```

#### file_versions Table

```sql
CREATE TABLE file_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL REFERENCES stored_files(id) ON DELETE CASCADE,

    version_number INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL DEFAULT FALSE,

    storage_key VARCHAR(500) NOT NULL,
    storage_backend VARCHAR(20) NOT NULL,
    size_bytes BIGINT NOT NULL,
    checksum VARCHAR(64),

    mime_type VARCHAR(255) NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',

    status VARCHAR(20) NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'deleted', 'archived')),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    created_by UUID NOT NULL,

    CONSTRAINT file_versions_storage_key_unique UNIQUE (storage_key),
    CONSTRAINT file_versions_number_positive CHECK (version_number >= 1)
);

CREATE INDEX idx_file_versions_file ON file_versions(file_id, version_number DESC);
CREATE INDEX idx_file_versions_current ON file_versions(file_id) WHERE is_current = TRUE;
CREATE INDEX idx_file_versions_expires ON file_versions(expires_at) WHERE expires_at IS NOT NULL;
```

#### thumbnails Table

```sql
CREATE TABLE thumbnails (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL REFERENCES stored_files(id) ON DELETE CASCADE,

    width INTEGER NOT NULL CHECK (width > 0),
    height INTEGER NOT NULL CHECK (height > 0),

    storage_key VARCHAR(500) NOT NULL,
    storage_backend VARCHAR(20) NOT NULL,
    size_bytes BIGINT NOT NULL CHECK (size_bytes > 0),
    mime_type VARCHAR(100) NOT NULL,
    format VARCHAR(20) NOT NULL,

    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    generation_time_ms INTEGER NOT NULL,

    cdn_url VARCHAR(500) NOT NULL,
    cache_until TIMESTAMPTZ NOT NULL,

    quality INTEGER NOT NULL CHECK (quality BETWEEN 1 AND 100),

    source_version INTEGER NOT NULL,
    is_stale BOOLEAN NOT NULL DEFAULT FALSE,

    metadata JSONB NOT NULL DEFAULT '{}',

    CONSTRAINT thumbnails_storage_key_unique UNIQUE (storage_key),
    CONSTRAINT thumbnails_file_size_unique UNIQUE (file_id, width, height)
);

CREATE INDEX idx_thumbnails_file ON thumbnails(file_id);
CREATE INDEX idx_thumbnails_stale ON thumbnails(is_stale) WHERE is_stale = TRUE;
CREATE INDEX idx_thumbnails_cache ON thumbnails(cache_until);
```

#### access_logs Table

```sql
CREATE TABLE access_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_id UUID NOT NULL,

    user_id UUID,
    share_id UUID,

    action VARCHAR(20) NOT NULL
        CHECK (action IN ('download', 'view', 'preview', 'share', 'share_access',
                          'upload', 'delete', 'rename', 'move', 'copy', 'restore', 'metadata_update')),

    ip_address INET,
    user_agent VARCHAR(500),
    referrer VARCHAR(500),
    location JSONB,

    success BOOLEAN NOT NULL DEFAULT TRUE,
    error_code VARCHAR(50),
    error_message TEXT,

    duration_ms INTEGER,
    bytes_transferred BIGINT,

    accessed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    metadata JSONB NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_access_logs_file ON access_logs(file_id, accessed_at DESC);
CREATE INDEX idx_access_logs_user ON access_logs(user_id, accessed_at DESC);
CREATE INDEX idx_access_logs_action ON access_logs(action, accessed_at DESC);
CREATE INDEX idx_access_logs_time ON access_logs(accessed_at DESC);

-- Partitioning for large-scale deployments
-- CREATE TABLE access_logs_y2025m01 PARTITION OF access_logs
--     FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');
```

---

## 14. Security Model

### 14.1 File Security Layers

1. **Upload Security**
   - MIME type validation
   - File size limits
   - Extension blocking
   - Virus scanning
   - Content inspection

2. **Access Security**
   - Owner verification
   - Share token validation
   - Password protection
   - Expiration checking
   - Download limits

3. **Storage Security**
   - Encrypted at rest
   - Secure key generation
   - Access logging
   - Quarantine isolation

### 14.2 Blocked File Types

| Category | Extensions |
|----------|------------|
| Executables | exe, bat, cmd, com, msi, scr |
| Scripts | vbs, vbe, js, jse, ws, wsf, ps1 |
| System | dll, sys, drv, ocx |
| Archives (risky) | cab, hta, msc, inf |

### 14.3 Magic Byte Detection

| Signature | Type | Action |
|-----------|------|--------|
| `MZ` | Windows PE | Block |
| `\x7fELF` | Linux ELF | Block |
| `\xCA\xFE\xBA\xBE` | macOS Mach-O | Block |
| `PK` | ZIP (scan contents) | Scan |
| `Rar!` | RAR (scan contents) | Scan |

### 14.4 Threat Response Matrix

| Threat Level | Action | User Notification |
|--------------|--------|-------------------|
| Safe | Allow | None |
| Low | Allow with warning | Warning badge |
| Medium | Quarantine | Email + in-app alert |
| High | Block | Email + admin alert |
| Critical | Block + alert admin | Email + SMS to admin |

---

## 15. Implementation Checklist

### Phase 1: Core Entities
- [x] StoredFile entity with custom methods
- [x] Bucket entity with custom methods
- [x] UserQuota entity with custom methods
- [x] FileShare entity with custom methods
- [x] FileVersion entity with custom methods
- [x] Thumbnail entity with custom methods
- [x] AccessLog entity with custom methods
- [x] All enums (10 total)

### Phase 2: Domain Services
- [x] StorageService (local filesystem)
- [x] VirusScannerService
- [x] ImageCompressorService
- [x] FileUploadService (orchestration)
- [x] AccessLoggerService

### Phase 3: Database
- [ ] PostgreSQL migrations
- [ ] Indexes and constraints
- [ ] Repository implementations

### Phase 4: API Layer
- [ ] HTTP handlers
- [ ] Request/response validation
- [ ] Error handling
- [ ] Authentication middleware

### Phase 5: Integration
- [ ] S3/cloud storage backends
- [ ] CDN integration
- [ ] Email notifications
- [ ] Admin dashboard

---

## 16. Conclusion

This technical domain documentation provides **complete specifications** for implementing the Bucket File Storage System. All 7 entities, 10 enums, domain services, and workflows are fully documented with:

- **Entity definitions** with business rules and invariants
- **State machines** for file and bucket lifecycle
- **Domain services** for storage, scanning, and compression
- **PostgreSQL schemas** with constraints and indexes
- **API endpoint specifications** (Backbone + Domain-specific)
- **Security model** for file protection

**Document Version**: 1.0 (Complete)
**Last Updated**: 2025-01-20
**Status**: Ready for implementation

---
