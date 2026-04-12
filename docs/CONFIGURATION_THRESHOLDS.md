# Bucket Configuration Thresholds

**Version**: 1.0
**Last Updated**: 2025-12-24

This document describes all configurable thresholds, constants, and default values in the Bucket module.

---

## Table of Contents

1. [Virus Scanner Service](#1-virus-scanner-service)
2. [Image Compressor Service](#2-image-compressor-service)
3. [User Quota System](#3-user-quota-system)
4. [Storage Service](#4-storage-service)
5. [Bucket Configuration](#5-bucket-configuration)
6. [File Share](#6-file-share)
7. [Summary Table](#7-summary-table)

---

## 1. Virus Scanner Service

**File**: `src/domain/services/virus_scanner.rs`

### 1.1 Blocked File Extensions

Files with these extensions are immediately blocked (ThreatLevel::Critical):

| Category | Extensions |
|----------|------------|
| **Windows Executables** | `exe`, `bat`, `cmd`, `com`, `scr`, `pif`, `msi`, `msp` |
| **Scripts** | `vbs`, `vbe`, `js`, `jse`, `ws`, `wsf`, `wsc`, `wsh` |
| **PowerShell** | `ps1`, `ps1xml`, `ps2`, `ps2xml`, `psc1`, `psc2` |
| **Libraries/Drivers** | `dll`, `sys`, `drv`, `ocx`, `cpl` |
| **Shortcuts** | `lnk`, `url` |
| **Other Dangerous** | `hta`, `reg` |

**Total**: 27 blocked extensions

### 1.2 Magic Byte Detection

Binary files are detected by their magic bytes regardless of extension:

| Binary Type | Magic Bytes | Threat Level |
|------------|-------------|--------------|
| Windows PE | `MZ` (0x4D5A) | High |
| Linux ELF | `\x7fELF` | High |
| macOS Mach-O (32-bit) | `0xfeedface` | High |
| macOS Mach-O (64-bit) | `0xfeedfacf` | High |
| macOS Mach-O (reversed) | `0xcefaedfe`, `0xcffaedfe` | High |
| Java Class | `0xcafebabe` | Medium |

### 1.3 Heuristic Detection

| Check | Pattern | Threat Level |
|-------|---------|--------------|
| Double Extensions | `.pdf.exe`, `.doc.exe`, `.xls.exe`, `.jpg.exe`, `.png.exe` | Critical |
| Double Extensions (SCR) | `.pdf.scr`, `.doc.scr`, `.xls.scr`, `.jpg.scr`, `.png.scr` | Critical |
| Embedded Scripts | `<script` or `javascript:` in SVG/HTML | Medium |

### 1.4 Threat Level Actions

| Threat Level | Action |
|--------------|--------|
| Critical | Block upload, reject file |
| High | Block upload, reject file |
| Medium | Quarantine file (flagged but stored) |
| Low | Allow with logging |
| Safe | Allow |

---

## 2. Image Compressor Service

**File**: `src/domain/services/image_compressor.rs`

### 2.1 Default Configuration

| Parameter | Default Value | Description | Range |
|-----------|---------------|-------------|-------|
| `quality` | **85** | JPEG compression quality | 1-100 |
| `min_size_for_compression` | **100,000 bytes** (100 KB) | Minimum file size before compression is attempted | 0+ |
| `convert_to_jpeg` | **false** | Whether to convert all images to JPEG | boolean |
| `max_dimension` | **None** | Maximum width/height for resizing | None or pixels |

### 2.2 Thumbnail Generation

| Parameter | Value | Description |
|-----------|-------|-------------|
| `thumbnail_quality` | **75** | JPEG quality for generated thumbnails |
| `thumbnail_method` | Lanczos3 | Resize filter algorithm |

### 2.3 Format Auto-Conversion

Some formats are automatically converted to JPEG for better compression:

| Original Format | Output Format |
|-----------------|---------------|
| BMP | JPEG |
| TIFF | JPEG |
| Other formats | Original (no conversion) |

### 2.4 Usage Examples

```rust
// Default settings
let compressor = ImageCompressorService::new();

// Custom quality
let compressor = ImageCompressorService::with_quality(70);

// Configure compression threshold
let mut compressor = ImageCompressorService::new();
compressor.set_min_size(50_000); // 50KB minimum

// Enable JPEG conversion
compressor.set_convert_to_jpeg(true);

// Set maximum dimension
compressor.set_max_dimension(2048); // Max 2048px
```

---

## 3. User Quota System

**File**: `src/domain/entity/user_quota.rs`

### 3.1 Quota Fields

| Field | Type | Description |
|-------|------|-------------|
| `limit_bytes` | i64 | Maximum storage allowed |
| `used_bytes` | i64 | Current storage used |
| `file_count` | i32 | Number of files stored |
| `max_file_size` | Option<i64> | Maximum size per file (optional) |
| `max_file_count` | Option<i32> | Maximum number of files (optional) |
| `warning_threshold_percent` | i32 | Percentage at which warnings trigger |
| `tier` | String | Quota tier (e.g., "free", "pro", "enterprise") |

### 3.2 Warning System

| Parameter | Value | Description |
|-----------|-------|-------------|
| Warning Threshold | **Configurable** (typically 80%) | Usage percentage that triggers warnings |
| Warning Cooldown | **24 hours** | Minimum time between warning notifications |

### 3.3 Quota Tier Examples

| Tier | Storage Limit | Max File Size | Max Files |
|------|---------------|---------------|-----------|
| Free | 1 GB | 50 MB | 1,000 |
| Pro | 100 GB | 500 MB | 10,000 |
| Enterprise | 1 TB | 5 GB | Unlimited |

### 3.4 Quota Check Priority

When `can_upload_file()` is called, checks are performed in this order:

1. **File Size Limit** - Is file too large for single file limit?
2. **File Count Limit** - Has user exceeded file count?
3. **Storage Quota** - Does user have space remaining?

---

## 4. Storage Service

**File**: `src/domain/services/storage_service.rs`

### 4.1 Directory Structure

| Directory | Purpose |
|-----------|---------|
| `{root}/buckets/` | Active file storage |
| `{root}/trash/` | Soft-deleted files |
| `{root}/thumbnails/` | Generated thumbnails |
| `{root}/temp/` | Temporary files |

### 4.2 Storage Key Format

```
{bucket_slug}/{YYYY}/{MM}/{DD}/{uuid8}-{sanitized_filename}
```

Example: `my-bucket/2025/12/24/a1b2c3d4-document.pdf`

### 4.3 Path Sanitization

The following characters are replaced with underscore (`_`):

| Character | ASCII Code | Description |
|-----------|------------|-------------|
| `/` | 0x2F | Forward slash |
| `\` | 0x5C | Backslash |
| `:` | 0x3A | Colon |
| `*` | 0x2A | Asterisk |
| `?` | 0x3F | Question mark |
| `"` | 0x22 | Double quote |
| `<` | 0x3C | Less than |
| `>` | 0x3E | Greater than |
| `\|` | 0x7C | Pipe |
| `\0` | 0x00 | Null character |
| Control chars | 0x00-0x1F | ASCII control characters |

### 4.4 Trash Management

| Operation | Description |
|-----------|-------------|
| `move_to_trash()` | Moves file to trash with timestamp |
| `restore_from_trash()` | Restores file to original location |
| `purge_trash_older_than(days)` | Permanently deletes files older than N days |

Trash key format: `{YYYY}/{MM}/{DD}/{file_id}_{YYYYMMDD_HHMMSS}`

---

## 5. Bucket Configuration

**File**: `src/domain/entity/bucket.rs`

### 5.1 Bucket Fields

| Field | Type | Description |
|-------|------|-------------|
| `max_file_size` | Option<i64> | Maximum file size allowed in bucket |
| `allowed_mime_types` | Vec<String> | List of allowed MIME types (supports wildcards) |
| `auto_delete_after_days` | Option<i32> | Days after which files are auto-deleted |

### 5.2 MIME Type Wildcards

Supports wildcard patterns for MIME type matching:

| Pattern | Matches |
|---------|---------|
| `image/*` | `image/jpeg`, `image/png`, `image/gif`, etc. |
| `video/*` | `video/mp4`, `video/webm`, etc. |
| `application/pdf` | Exact match only |

### 5.3 Bucket Status

| Status | Can Upload | Can Download | Description |
|--------|------------|--------------|-------------|
| Active | Yes | Yes | Normal operation |
| Readonly | No | Yes | Downloads only |
| Archived | No | No | Long-term storage |
| Deleted | No | No | Marked for deletion |

---

## 6. File Share

**File**: `src/domain/entity/file_share.rs`

### 6.1 Share Limits

| Parameter | Description |
|-----------|-------------|
| `max_downloads` | Optional limit on number of downloads |
| `expires_at` | Optional expiration timestamp |

### 6.2 Password Protection

| Method | Algorithm |
|--------|-----------|
| Hashing | bcrypt |
| Cost Factor | Default (10 rounds) |

---

## 7. Summary Table

### All Configurable Thresholds

| Service | Parameter | Default | Unit | Configurable |
|---------|-----------|---------|------|--------------|
| **ImageCompressor** | quality | 85 | 1-100 | Yes |
| **ImageCompressor** | min_size_for_compression | 100,000 | bytes | Yes |
| **ImageCompressor** | thumbnail_quality | 75 | 1-100 | No |
| **ImageCompressor** | max_dimension | None | pixels | Yes |
| **VirusScanner** | blocked_extensions | 27 types | - | Yes (addable) |
| **VirusScanner** | min_magic_bytes | 4 | bytes | No |
| **UserQuota** | warning_threshold_percent | configurable | % | Yes |
| **UserQuota** | warning_cooldown | 24 | hours | No |
| **Storage** | trash_retention | configurable | days | Yes |

### Size Constants Reference

| Size | Bytes | Common Use |
|------|-------|------------|
| 1 KB | 1,024 | - |
| 100 KB | 102,400 | Image compression threshold |
| 1 MB | 1,048,576 | - |
| 50 MB | 52,428,800 | Free tier max file |
| 100 MB | 104,857,600 | - |
| 1 GB | 1,073,741,824 | Free tier total quota |
| 100 GB | 107,374,182,400 | Pro tier total quota |
| 1 TB | 1,099,511,627,776 | Enterprise tier total quota |

---

## Environment Variables

For production deployment, consider exposing these as environment variables:

```bash
# Image Compression
BUCKET_IMAGE_QUALITY=85
BUCKET_MIN_COMPRESSION_SIZE=100000
BUCKET_MAX_DIMENSION=4096

# Storage
BUCKET_ROOT_PATH=/var/lib/bucket
BUCKET_TRASH_RETENTION_DAYS=30

# Quotas
BUCKET_FREE_TIER_BYTES=1073741824
BUCKET_PRO_TIER_BYTES=107374182400
BUCKET_WARNING_THRESHOLD=80
```

---

## Change History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-24 | Initial documentation |
