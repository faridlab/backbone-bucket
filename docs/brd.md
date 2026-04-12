# Business Requirements Document (BRD)
## Bucket - File Storage Management System

**Document Version**: 1.0
**Last Updated**: December 24, 2025
**Author**: StartApp Development Team
**Status**: Final Draft
**Module**: `bucket`

---

## Table of Contents

1. [Document Information](#1-document-information)
2. [Project Overview](#2-project-overview)
3. [Business Context](#3-business-context)
4. [Stakeholders](#4-stakeholders)
5. [Functional Requirements](#5-functional-requirements)
6. [Non-Functional Requirements](#6-non-functional-requirements)
7. [User Stories & Use Cases](#7-user-stories--use-cases)
8. [Data Requirements](#8-data-requirements)
9. [Database Schema Specifications](#9-database-schema-specifications)
10. [Integration Requirements](#10-integration-requirements)
11. [API Specifications](#11-api-specifications)
12. [Technical Constraints](#12-technical-constraints)
13. [Security Requirements](#13-security-requirements)
14. [Success Criteria](#14-success-criteria)
15. [Assumptions & Dependencies](#15-assumptions--dependencies)
16. [Risks & Mitigation](#16-risks--mitigation)
17. [Glossary & References](#17-glossary--references)

---

## 1. Document Information

### 1.1 Document Control

| Attribute | Value |
|-----------|-------|
| **Document Title** | Business Requirements Document for Bucket File Storage System |
| **Version** | 1.0 |
| **Date** | December 24, 2025 |
| **Author** | StartApp Development Team |
| **Classification** | Internal - Confidential |
| **Distribution** | Project Team, Stakeholders, Management |

### 1.2 Revision History

| Version | Date | Description | Author |
|---------|------|-------------|--------|
| 1.0 | 2025-12-24 | Initial comprehensive BRD with full specifications | StartApp Team |

### 1.3 Document Purpose

This Business Requirements Document (BRD) serves as the authoritative specification for the Bucket File Storage Management System. It provides:

1. **Business Justification**: Clear articulation of business needs and expected value
2. **Functional Specifications**: Detailed requirements for all file storage capabilities
3. **Technical Specifications**: Complete database schema, API definitions, and integration points
4. **Security Specifications**: Virus scanning, quota enforcement, and access control
5. **Success Metrics**: Measurable criteria for project success

---

## 2. Project Overview

### 2.1 Executive Summary

**Bucket** is an enterprise-grade file storage management system designed to provide S3-like object storage and Google Drive-like sharing capabilities for applications. Built on Domain-Driven Design (DDD) principles, Bucket provides:

**Key Capabilities:**
- **File Storage**: Complete CRUD operations with hierarchical bucket organization
- **Security**: Virus scanning, threat detection, and file quarantine
- **Compression**: Automatic image compression and thumbnail generation
- **Sharing**: Public links, user-specific shares, and password-protected access
- **Quota Management**: Per-user storage limits with tiered plans
- **Versioning**: File version history with restore capabilities
- **Auditing**: Comprehensive access logging for compliance

**Technology Foundation:**
- **Backend**: Rust with Actix-Web framework
- **Database**: PostgreSQL for metadata storage
- **Storage**: Pluggable backends (Local, S3, GCS, MinIO)
- **Architecture**: Modular monolith with clean architecture layers
- **API**: RESTful APIs with OpenAPI documentation

### 2.2 Project Name & Branding

**Official Name**: Bucket - File Storage Management System

**Service Codename**: `bucket`

**Branding Rationale**: The name "Bucket" reflects a secure container for digital assets, emphasizing reliability and organization like a physical safe deposit box.

### 2.3 Project Summary

**Problem Being Solved:**
Modern applications require robust file storage with:
- Secure upload/download with virus scanning
- Flexible sharing with granular permissions
- Storage quota enforcement for multi-tenant environments
- Image optimization for web delivery
- Audit trails for compliance requirements

**Solution Provided:**
Bucket provides a unified file storage platform that:
- **Secures Files**: Automatic virus scanning with threat level classification
- **Optimizes Storage**: Image compression reducing storage costs by 30-50%
- **Controls Access**: Granular sharing with expiration and download limits
- **Enforces Quotas**: Tiered storage plans with usage monitoring
- **Tracks Everything**: Complete audit trail of all file operations

### 2.4 Business Goals & Objectives

#### Primary Objectives (Must-Have)

1. **Security & Protection**
   - Block 100% of known malware signatures
   - Quarantine suspicious files automatically
   - Zero unauthorized file access incidents

2. **Storage Efficiency**
   - Reduce image storage by 30-50% through compression
   - Automatic thumbnail generation for all images
   - Support files up to 5GB per upload

3. **Sharing & Collaboration**
   - Support 3 sharing modes: public links, user shares, password-protected
   - Configurable download limits and expiration dates
   - Real-time share revocation

4. **Quota Management**
   - Support multiple quota tiers (Free, Basic, Premium, Enterprise)
   - Real-time usage tracking
   - Automated warning notifications at configurable thresholds

### 2.5 Business Value Proposition

#### Quantifiable Benefits

| Metric | Target | Business Impact |
|--------|--------|-----------------|
| Storage Savings | 30-50% | Reduced infrastructure costs |
| Malware Blocked | 100% | Security breach prevention |
| Sharing Time | <1 second | Improved collaboration |
| Upload Throughput | 100+ concurrent | Scalable operations |
| API Response Time | <200ms (p99) | Better user experience |

---

## 3. Business Context

### 3.1 Current State Analysis

**Existing Challenges:**

1. **Fragmented Storage**: Files scattered across multiple systems without unified management
2. **Security Gaps**: No automated virus scanning or threat detection
3. **Manual Sharing**: Complex manual processes for sharing files externally
4. **No Quota Control**: Unlimited storage leading to cost overruns
5. **Missing Audit Trail**: Cannot track who accessed what and when

### 3.2 Problem Statement

Organizations need a centralized, secure file storage system that:
- Automatically scans uploads for malware
- Compresses images to reduce storage costs
- Provides flexible sharing with granular controls
- Enforces storage quotas by user/organization
- Maintains complete audit logs for compliance

### 3.3 Solution Overview

Bucket addresses these needs through:

1. **Unified Storage Layer**: Single API for all file operations
2. **Security Pipeline**: Upload → Scan → Process → Store workflow
3. **Flexible Sharing**: Multiple share types with fine-grained controls
4. **Quota System**: Tiered plans with real-time enforcement
5. **Audit System**: Complete logging of all access events

---

## 4. Stakeholders

### 4.1 Internal Stakeholders

| Role | Responsibilities | Interest in Bucket |
|------|-----------------|---------------------|
| **Application Developers** | Integrate file storage | Easy-to-use APIs |
| **System Administrators** | Manage storage infrastructure | Monitoring & quota management |
| **Security Team** | Ensure file security | Virus scanning & threat reports |
| **Product Managers** | Define storage features | Usage analytics & quotas |

### 4.2 External Stakeholders

| Role | Responsibilities | Interest in Bucket |
|------|-----------------|---------------------|
| **End Users** | Upload/download files | Fast, reliable storage |
| **External Partners** | Access shared files | Secure share links |
| **Compliance Officers** | Audit file access | Access logs & reports |

---

## 5. Functional Requirements

### 5.1 File Management (FR-FM)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-FM-001 | Upload files up to 5GB | Must-Have | Implemented |
| FR-FM-002 | Download files with resume support | Must-Have | Implemented |
| FR-FM-003 | Delete files (soft delete to trash) | Must-Have | Implemented |
| FR-FM-004 | Restore files from trash | Must-Have | Implemented |
| FR-FM-005 | Permanently purge deleted files | Must-Have | Implemented |
| FR-FM-006 | List files with pagination and filtering | Must-Have | Implemented |
| FR-FM-007 | Move files between buckets | Should-Have | Planned |
| FR-FM-008 | Copy files within/across buckets | Should-Have | Planned |
| FR-FM-009 | Rename files | Should-Have | Implemented |
| FR-FM-010 | Update file metadata | Must-Have | Implemented |

### 5.2 Bucket Management (FR-BM)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-BM-001 | Create buckets with unique slugs | Must-Have | Implemented |
| FR-BM-002 | Configure allowed MIME types per bucket | Must-Have | Implemented |
| FR-BM-003 | Set maximum file size per bucket | Must-Have | Implemented |
| FR-BM-004 | Archive buckets (read-only mode) | Should-Have | Implemented |
| FR-BM-005 | Delete buckets (with cascading file deletion) | Must-Have | Implemented |
| FR-BM-006 | Configure auto-delete for temporary buckets | Nice-to-Have | Implemented |
| FR-BM-007 | Track bucket statistics (file count, total size) | Must-Have | Implemented |

### 5.3 Security Features (FR-SEC)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-SEC-001 | Scan uploads for known malware signatures | Must-Have | Implemented |
| FR-SEC-002 | Block dangerous file extensions (exe, bat, dll, etc.) | Must-Have | Implemented |
| FR-SEC-003 | Detect executable binaries via magic bytes | Must-Have | Implemented |
| FR-SEC-004 | Quarantine suspicious files | Must-Have | Implemented |
| FR-SEC-005 | Classify threats by severity (Safe, Low, Medium, High, Critical) | Must-Have | Implemented |
| FR-SEC-006 | Admin review and release of quarantined files | Should-Have | Planned |
| FR-SEC-007 | Double extension detection (e.g., file.jpg.exe) | Must-Have | Implemented |
| FR-SEC-008 | SVG script detection | Should-Have | Implemented |

### 5.4 Image Processing (FR-IMG)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-IMG-001 | Detect image files by content analysis | Must-Have | Implemented |
| FR-IMG-002 | Compress images with configurable quality | Must-Have | Implemented |
| FR-IMG-003 | Generate thumbnails in multiple sizes | Must-Have | Implemented |
| FR-IMG-004 | Convert images to web-optimized formats | Should-Have | Implemented |
| FR-IMG-005 | Preserve original image if compression increases size | Must-Have | Implemented |
| FR-IMG-006 | Track compression ratio and savings | Should-Have | Implemented |

### 5.5 File Sharing (FR-SHR)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-SHR-001 | Create public share links | Must-Have | Implemented |
| FR-SHR-002 | Create user-specific shares | Must-Have | Implemented |
| FR-SHR-003 | Create password-protected shares | Must-Have | Implemented |
| FR-SHR-004 | Set share expiration date | Must-Have | Implemented |
| FR-SHR-005 | Set maximum download count | Must-Have | Implemented |
| FR-SHR-006 | Revoke shares immediately | Must-Have | Implemented |
| FR-SHR-007 | Track share download count | Must-Have | Implemented |
| FR-SHR-008 | Permission levels: View, Download, Edit | Must-Have | Implemented |

### 5.6 Quota Management (FR-QTA)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-QTA-001 | Track storage usage per user | Must-Have | Implemented |
| FR-QTA-002 | Enforce storage limits | Must-Have | Implemented |
| FR-QTA-003 | Support multiple quota tiers | Must-Have | Implemented |
| FR-QTA-004 | Send warning at configurable threshold | Should-Have | Implemented |
| FR-QTA-005 | Track peak usage for billing | Should-Have | Implemented |
| FR-QTA-006 | Limit maximum file size per user | Should-Have | Implemented |
| FR-QTA-007 | Limit maximum file count per user | Should-Have | Implemented |

### 5.7 Versioning (FR-VER)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-VER-001 | Track file version history | Should-Have | Implemented |
| FR-VER-002 | Restore previous versions | Should-Have | Implemented |
| FR-VER-003 | Auto-expire old versions | Nice-to-Have | Implemented |
| FR-VER-004 | Record version change summaries | Nice-to-Have | Implemented |
| FR-VER-005 | Mark current version | Should-Have | Implemented |

### 5.8 Access Logging (FR-LOG)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-LOG-001 | Log all file downloads | Must-Have | Implemented |
| FR-LOG-002 | Log all file uploads | Must-Have | Implemented |
| FR-LOG-003 | Log share access events | Must-Have | Implemented |
| FR-LOG-004 | Track IP address and user agent | Should-Have | Implemented |
| FR-LOG-005 | Track geographic location | Nice-to-Have | Implemented |
| FR-LOG-006 | Track bytes transferred and duration | Should-Have | Implemented |

---

## 6. Non-Functional Requirements

### 6.1 Performance Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-PERF-001 | File upload throughput | Up to 100 concurrent uploads |
| NFR-PERF-002 | File download throughput | Up to 500 concurrent downloads |
| NFR-PERF-003 | API response time (metadata) | <100ms (p95) |
| NFR-PERF-004 | Virus scan time | <5 seconds per file |
| NFR-PERF-005 | Image compression time | <10 seconds for 10MB image |
| NFR-PERF-006 | Thumbnail generation | <2 seconds per image |

### 6.2 Scalability Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-SCALE-001 | Total storage capacity | Unlimited (depends on backend) |
| NFR-SCALE-002 | Files per bucket | 1 million+ |
| NFR-SCALE-003 | Buckets per user | 1,000+ |
| NFR-SCALE-004 | Concurrent API requests | 10,000+ |

### 6.3 Availability Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-AVAIL-001 | System uptime | 99.9% (8.76 hours downtime/year) |
| NFR-AVAIL-002 | Recovery Time Objective (RTO) | <15 minutes |
| NFR-AVAIL-003 | Recovery Point Objective (RPO) | <5 minutes |

### 6.4 Security Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-SEC-001 | Data encryption at rest | AES-256 |
| NFR-SEC-002 | Data encryption in transit | TLS 1.3 |
| NFR-SEC-003 | Access token expiration | Configurable (default 1 hour) |
| NFR-SEC-004 | Share token length | 32-64 characters |

---

## 7. User Stories & Use Cases

### 7.1 User Stories

#### US-001: File Upload
**As a** user
**I want to** upload a file to my storage
**So that** I can access it from anywhere

**Acceptance Criteria:**
- File is scanned for viruses before storing
- Image files are automatically compressed
- Thumbnail is generated for image files
- User quota is updated
- Upload progress is trackable

#### US-002: File Sharing
**As a** user
**I want to** share a file via a link
**So that** others can access it without an account

**Acceptance Criteria:**
- Share link is generated with unique token
- Can optionally set password protection
- Can set expiration date
- Can limit number of downloads
- Can revoke share at any time

#### US-003: Quota Management
**As an** administrator
**I want to** set storage quotas for users
**So that** I can control resource usage

**Acceptance Criteria:**
- Can create quota tiers (Free, Basic, Premium, Enterprise)
- Users cannot upload when quota exceeded
- Warning sent when approaching limit
- Can view usage statistics

#### US-004: Security Monitoring
**As a** security officer
**I want to** review quarantined files
**So that** I can investigate potential threats

**Acceptance Criteria:**
- Can list all quarantined files
- Can view scan results and threat details
- Can release false positives
- Can permanently delete confirmed threats

### 7.2 Use Cases

#### UC-001: Upload File Workflow

```
1. User initiates file upload
2. System validates bucket access
3. System checks file size against bucket limits
4. System checks file type against allowed MIME types
5. System checks user quota
6. System scans file for viruses
   - If threat detected → Quarantine file → Notify user
   - If safe → Continue
7. System compresses if image
8. System generates thumbnail if image
9. System stores file content
10. System creates file metadata record
11. System updates user quota
12. System updates bucket statistics
13. System emits FileUploadedEvent
14. Return success response with file ID
```

#### UC-002: Create Share Workflow

```
1. User requests share creation for file
2. System validates file ownership
3. System generates unique share token
4. System creates share record with:
   - Share type (link/user/password)
   - Permission level (view/download/edit)
   - Expiration date (optional)
   - Max downloads (optional)
   - Password hash (if password-protected)
5. System returns share URL
```

#### UC-003: Access Shared File Workflow

```
1. External user accesses share URL
2. System validates share token
3. System checks share is active
4. System checks share not expired
5. System checks download count not exceeded
6. If password-protected → Prompt for password
7. System validates password
8. System logs access event
9. System increments download count
10. System returns file content
```

---

## 8. Data Requirements

### 8.1 Data Entities

| Entity | Description | Volume Estimate |
|--------|-------------|-----------------|
| **StoredFile** | File metadata and status | 10M+ records |
| **Bucket** | Storage containers | 100K+ records |
| **FileShare** | Share links and permissions | 1M+ records |
| **UserQuota** | Per-user storage limits | 100K+ records |
| **FileVersion** | File version history | 50M+ records |
| **Thumbnail** | Generated image thumbnails | 5M+ records |
| **FileAccessLog** | Access audit trail | 100M+ records |
| **AccessLog** | Detailed access audit | 500M+ records |

### 8.2 Data Retention

| Data Type | Retention Period | Archival Policy |
|-----------|-----------------|-----------------|
| Active Files | Indefinite | User-controlled |
| Deleted Files (Trash) | 30 days | Auto-purge |
| File Versions | Configurable | Auto-expire |
| Access Logs | 1 year | Archive to cold storage |
| Thumbnails | Until file deleted | Cascade delete |

---

## 9. Database Schema Specifications

### 9.1 Core Entities

#### 9.1.1 StoredFile

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | PK, NOT NULL | Unique identifier |
| bucket_id | UUID | FK, NOT NULL | Reference to bucket |
| owner_id | UUID | NOT NULL | File owner user ID |
| path | VARCHAR(1024) | NOT NULL | Relative path in bucket |
| original_name | VARCHAR(255) | NOT NULL | Original filename |
| size_bytes | BIGINT | NOT NULL, >= 0 | File size |
| mime_type | VARCHAR(127) | NOT NULL | MIME type |
| checksum | VARCHAR(128) | | SHA-256 hash |
| is_compressed | BOOLEAN | DEFAULT false | Compression flag |
| original_size | BIGINT | | Size before compression |
| compression_algorithm | VARCHAR | | Algorithm used |
| is_scanned | BOOLEAN | DEFAULT false | Scan status |
| scan_result | JSONB | | Detailed scan results |
| threat_level | ENUM | | Safe/Low/Medium/High/Critical |
| has_thumbnail | BOOLEAN | DEFAULT false | Thumbnail exists |
| thumbnail_path | VARCHAR | | Path to thumbnail |
| status | ENUM | NOT NULL | uploading/processing/active/quarantined/deleted/purged |
| storage_key | VARCHAR(1024) | NOT NULL | Backend storage key |
| version | INT | DEFAULT 1 | Version number |
| download_count | INT | DEFAULT 0 | Download counter |
| last_accessed_at | TIMESTAMP | | Last access time |
| metadata | JSONB | | Custom metadata |

**Indexes:**
- UNIQUE(bucket_id, path, deleted_at)
- INDEX(owner_id)
- INDEX(bucket_id)
- INDEX(status)
- INDEX(mime_type)
- INDEX(checksum) - for deduplication

#### 9.1.2 Bucket

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | PK, NOT NULL | Unique identifier |
| name | VARCHAR(255) | NOT NULL | Display name |
| slug | VARCHAR(255) | UNIQUE, NOT NULL | URL-safe identifier |
| description | TEXT | | Bucket description |
| owner_id | UUID | NOT NULL | Bucket owner |
| bucket_type | ENUM | DEFAULT 'user' | user/shared/system/temp |
| status | ENUM | DEFAULT 'active' | active/readonly/archived/deleted |
| storage_backend | ENUM | DEFAULT 'local' | local/s3/minio/gcs |
| root_path | VARCHAR(1024) | NOT NULL | Storage root path |
| file_count | INT | DEFAULT 0 | Files in bucket |
| total_size_bytes | BIGINT | DEFAULT 0 | Total storage used |
| max_file_size | BIGINT | | Max file size limit |
| allowed_mime_types | VARCHAR[] | | Allowed types |
| auto_delete_after_days | INT | | Auto-cleanup days |
| metadata | JSONB | | Custom metadata |

**Indexes:**
- UNIQUE(slug)
- INDEX(owner_id)
- INDEX(status)
- INDEX(bucket_type)

#### 9.1.3 FileShare

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | PK, NOT NULL | Unique identifier |
| file_id | UUID | FK, NOT NULL | Shared file |
| owner_id | UUID | NOT NULL | Share creator |
| token | VARCHAR(64) | UNIQUE, NOT NULL | Share token |
| share_type | ENUM | DEFAULT 'link' | user/link/password |
| permission | ENUM | DEFAULT 'view' | private/view/edit/full |
| shared_with | UUID[] | | Target user IDs |
| password_hash | VARCHAR | | Hashed password |
| max_downloads | INT | | Download limit |
| download_count | INT | DEFAULT 0 | Current count |
| expires_at | TIMESTAMP | | Expiration time |
| is_active | BOOLEAN | DEFAULT true | Active status |
| revoked_at | TIMESTAMP | | Revocation time |
| revoked_by | UUID | | Who revoked |
| message | TEXT | | Share message |
| metadata | JSONB | | Custom metadata |

**Indexes:**
- UNIQUE(token)
- INDEX(file_id)
- INDEX(owner_id)
- INDEX(is_active)
- INDEX(expires_at)

#### 9.1.4 UserQuota

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | PK, NOT NULL | Unique identifier |
| user_id | UUID | UNIQUE, NOT NULL | User ID |
| limit_bytes | BIGINT | NOT NULL, >= 0 | Storage limit |
| used_bytes | BIGINT | DEFAULT 0, >= 0 | Current usage |
| file_count | INT | DEFAULT 0, >= 0 | File count |
| max_file_size | BIGINT | | Per-file limit |
| max_file_count | INT | | File count limit |
| tier | VARCHAR(50) | DEFAULT 'free' | Quota tier |
| warning_threshold_percent | INT | DEFAULT 80 | Warning threshold |
| last_warning_sent_at | TIMESTAMP | | Last warning |
| peak_usage_bytes | BIGINT | DEFAULT 0 | Peak usage |
| peak_usage_at | TIMESTAMP | | Peak time |
| metadata | JSONB | | Custom metadata |

**Indexes:**
- UNIQUE(user_id)
- INDEX(tier)
- INDEX(used_bytes)

### 9.2 Supporting Entities

#### 9.2.1 FileVersion

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | PK, NOT NULL | Unique identifier |
| file_id | UUID | FK, NOT NULL | Parent file |
| version_number | INT | NOT NULL | Version number |
| version_type | ENUM | NOT NULL | upload/replace/edit/restore/auto_save |
| storage_key | VARCHAR | UNIQUE, NOT NULL | Storage location |
| name | VARCHAR(255) | NOT NULL | Filename at version |
| mime_type | VARCHAR | NOT NULL | MIME at version |
| size_bytes | BIGINT | NOT NULL | Size at version |
| checksum_sha256 | VARCHAR(64) | | SHA-256 hash |
| checksum_md5 | VARCHAR(32) | | MD5 hash |
| created_by_id | UUID | NOT NULL | Who created |
| change_summary | VARCHAR(500) | | Change description |
| is_current | BOOLEAN | DEFAULT false | Current version |
| expires_at | TIMESTAMP | | Auto-expire time |
| is_deleted | BOOLEAN | DEFAULT false | Deletion flag |

**Indexes:**
- UNIQUE(file_id, version_number)
- UNIQUE(storage_key)
- INDEX(file_id, is_current)
- INDEX(expires_at)

#### 9.2.2 Thumbnail

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | PK, NOT NULL | Unique identifier |
| file_id | UUID | FK, NOT NULL | Source file |
| size | ENUM | NOT NULL | tiny/small/medium/large/xlarge |
| width | INT | NOT NULL | Width in pixels |
| height | INT | NOT NULL | Height in pixels |
| storage_key | VARCHAR | UNIQUE, NOT NULL | Storage location |
| mime_type | VARCHAR | DEFAULT 'image/webp' | Format |
| format | VARCHAR | DEFAULT 'webp' | Image format |
| quality | INT | DEFAULT 80 | Compression quality |
| size_bytes | BIGINT | NOT NULL | Thumbnail size |
| generated_at | TIMESTAMP | DEFAULT NOW | Generation time |
| generation_time_ms | INT | | Processing time |
| source_version | INT | DEFAULT 1 | Source version |
| cdn_url | VARCHAR | | CDN URL |
| cache_expires_at | TIMESTAMP | | Cache expiry |
| is_stale | BOOLEAN | DEFAULT false | Needs regeneration |

**Indexes:**
- UNIQUE(file_id, size)
- UNIQUE(storage_key)
- INDEX(is_stale)

#### 9.2.3 AccessLog

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | PK, NOT NULL | Unique identifier |
| file_id | UUID | FK, NOT NULL | Accessed file |
| bucket_id | UUID | FK, NOT NULL | File's bucket |
| user_id | UUID | | Accessing user |
| share_id | UUID | | Share used |
| action | ENUM | NOT NULL | download/view/share/upload/delete/etc. |
| is_owner | BOOLEAN | NOT NULL | Is file owner |
| is_shared | BOOLEAN | NOT NULL | Via share link |
| is_public | BOOLEAN | NOT NULL | Public access |
| ip_address | VARCHAR(45) | | Client IP |
| user_agent | VARCHAR(500) | | Browser/client |
| referer | VARCHAR(500) | | Referrer URL |
| country_code | VARCHAR(2) | | Country code |
| city | VARCHAR(100) | | City |
| bytes_transferred | BIGINT | | Data transferred |
| duration_ms | INT | | Operation time |
| success | BOOLEAN | NOT NULL | Success status |
| error_message | VARCHAR(500) | | Error if failed |
| accessed_at | TIMESTAMP | NOT NULL | Access time |
| metadata | JSONB | | Additional data |

**Indexes:**
- INDEX(file_id)
- INDEX(user_id)
- INDEX(accessed_at)
- INDEX(bucket_id)
- INDEX(action)

---

## 10. Integration Requirements

### 10.1 Internal Integrations

| System | Integration Type | Purpose |
|--------|-----------------|---------|
| **Sapiens (User Management)** | API | User authentication & authorization |
| **Notification Service** | Events | Quota warnings, share notifications |
| **Analytics Platform** | Events | Usage analytics & reporting |

### 10.2 External Integrations

| System | Integration Type | Purpose |
|--------|-----------------|---------|
| **ClamAV** | Library/Daemon | Virus scanning |
| **AWS S3** | Storage Backend | Cloud storage |
| **Google Cloud Storage** | Storage Backend | Cloud storage |
| **MinIO** | Storage Backend | S3-compatible storage |
| **CDN (CloudFlare/AWS)** | Caching | Thumbnail delivery |

### 10.3 Events Published

| Event | Description | Payload |
|-------|-------------|---------|
| FileUploadedEvent | File successfully uploaded | file_id, bucket_id, owner_id, size |
| FileDeletedEvent | File moved to trash | file_id, owner_id |
| FilePurgedEvent | File permanently deleted | file_id, owner_id |
| ThreatDetectedEvent | Malware found in upload | file_id, threat_level, threats |
| QuotaExceededEvent | User exceeded quota | user_id, limit, used, attempted |
| ShareCreatedEvent | Share link created | share_id, file_id, share_type |
| ShareAccessedEvent | Share link accessed | share_id, file_id, accessor |

---

## 11. API Specifications

### 11.1 API Overview

| Resource | Base Path | Operations |
|----------|-----------|------------|
| Buckets | /api/v1/buckets | CRUD, Trash, Restore |
| Files | /api/v1/stored_files | CRUD, Trash, Restore |
| Shares | /api/v1/file_shares | CRUD, Trash, Restore |
| Quotas | /api/v1/user_quotas | CRUD |
| Versions | /api/v1/file_versions | CRUD |
| Thumbnails | /api/v1/thumbnails | CRUD |
| Access Logs | /api/v1/access_logs | Read |

### 11.2 Standard Endpoints (per resource)

Each resource provides 11 standard endpoints:

| Method | Path | Description |
|--------|------|-------------|
| GET | /api/v1/{resource} | List with pagination |
| POST | /api/v1/{resource} | Create new |
| GET | /api/v1/{resource}/{id} | Get by ID |
| PUT | /api/v1/{resource}/{id} | Full update |
| PATCH | /api/v1/{resource}/{id} | Partial update |
| DELETE | /api/v1/{resource}/{id} | Soft delete |
| POST | /api/v1/{resource}/bulk | Bulk create |
| POST | /api/v1/{resource}/upsert | Create or update |
| GET | /api/v1/{resource}/trash | List deleted |
| POST | /api/v1/{resource}/{id}/restore | Restore deleted |
| DELETE | /api/v1/{resource}/empty | Empty trash |

### 11.3 Custom Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | /api/v1/upload | Direct file upload |
| GET | /api/v1/download/{id} | Direct file download |
| GET | /api/v1/share/{token} | Access shared file |
| POST | /api/v1/files/{id}/share | Create share for file |
| DELETE | /api/v1/shares/{id}/revoke | Revoke share |
| GET | /api/v1/thumbnails/{file_id}/{size} | Get thumbnail |
| POST | /api/v1/files/{id}/restore-version/{version} | Restore version |

---

## 12. Technical Constraints

### 12.1 Technology Constraints

| Constraint | Description |
|------------|-------------|
| **Language** | Rust (stable) |
| **Database** | PostgreSQL 14+ |
| **Storage** | Pluggable backends (Local, S3, GCS) |
| **Framework** | Actix-Web |
| **Architecture** | Clean Architecture, DDD |

### 12.2 Operational Constraints

| Constraint | Value |
|------------|-------|
| Maximum file size | 5GB |
| Maximum filename length | 255 characters |
| Maximum path length | 1024 characters |
| Share token length | 32-64 characters |
| API rate limit | 1000 requests/minute |

---

## 13. Security Requirements

### 13.1 Authentication & Authorization

| Requirement | Implementation |
|-------------|----------------|
| API Authentication | JWT Bearer tokens |
| Share Authentication | Token + optional password |
| Bucket Access | Owner or explicit share |
| File Access | Owner, share, or bucket share |

### 13.2 Data Security

| Requirement | Implementation |
|-------------|----------------|
| Encryption at Rest | AES-256 (storage backend) |
| Encryption in Transit | TLS 1.3 |
| Password Hashing | Argon2id |
| Token Generation | Cryptographically secure random |

### 13.3 Threat Protection

| Threat | Mitigation |
|--------|------------|
| Malware Upload | Virus scanning with ClamAV |
| Executable Files | Extension and magic byte blocking |
| Path Traversal | Path sanitization, no .. allowed |
| DoS | Rate limiting, file size limits |
| Data Leakage | Access logging, share expiration |

---

## 14. Success Criteria

### 14.1 Functional Success

| Criterion | Metric | Target |
|-----------|--------|--------|
| File Upload Success Rate | % successful uploads | >99.5% |
| Virus Detection Rate | % malware detected | 100% |
| Share Creation Success | % shares created | >99.9% |
| Quota Enforcement | % violations blocked | 100% |

### 14.2 Performance Success

| Criterion | Metric | Target |
|-----------|--------|--------|
| Upload Latency | Time to upload 1MB | <2 seconds |
| Download Latency | Time to first byte | <200ms |
| API Response Time | 95th percentile | <100ms |
| Thumbnail Generation | Time per image | <2 seconds |

### 14.3 Business Success

| Criterion | Metric | Target |
|-----------|--------|--------|
| Storage Efficiency | Compression savings | 30-50% |
| User Satisfaction | NPS score | >70 |
| Security Incidents | Malware breaches | 0 |
| System Availability | Uptime | 99.9% |

---

## 15. Assumptions & Dependencies

### 15.1 Assumptions

1. Users have valid authentication tokens from Sapiens
2. Storage backends are properly configured and accessible
3. Virus scanner definitions are regularly updated
4. Network bandwidth is sufficient for file operations

### 15.2 Dependencies

| Dependency | Type | Impact |
|------------|------|--------|
| PostgreSQL | Database | Required for operation |
| Sapiens | Authentication | Required for user auth |
| Storage Backend | Infrastructure | Required for file storage |
| ClamAV | Security | Required for virus scanning |
| Image Crate | Library | Required for compression |

---

## 16. Risks & Mitigation

### 16.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Storage backend failure | Low | High | Multiple backend support |
| Virus scanner bypass | Low | Critical | Multiple detection methods |
| Data corruption | Very Low | Critical | Checksums, backups |
| Performance degradation | Medium | Medium | Caching, optimization |

### 16.2 Business Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Storage cost overrun | Medium | Medium | Quota enforcement |
| Data breach | Low | Critical | Access logging, encryption |
| Compliance violation | Low | High | Audit trails |

---

## 17. Glossary & References

### 17.1 Glossary

| Term | Definition |
|------|------------|
| **Bucket** | Container for organizing files, similar to S3 bucket or folder |
| **StoredFile** | Metadata record for a file stored in the system |
| **Quarantine** | Isolation of files suspected of containing malware |
| **Threat Level** | Classification of detected security threats |
| **Share** | Access link or permission grant for a file |
| **Quota** | Storage limit assigned to a user |
| **Thumbnail** | Smaller preview image of an original image file |

### 17.2 References

| Document | Location |
|----------|----------|
| Technical Specification | `libs/modules/bucket/docs/SPEC.md` |
| Domain Model | `libs/modules/bucket/docs/domain.md` |
| OpenAPI Specification | `libs/modules/bucket/schema/openapi/openapi.yaml` |
| Schema Definitions | `libs/modules/bucket/schema/models/` |

---

**Document End**
