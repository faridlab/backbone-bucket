# Bucket Module

**Domain:** File Storage & Content Management (Bucket)

A complete Domain-Driven Design (DDD) bounded context module built on **Backbone Framework**. This module follows Clean Architecture principles with a **schema-first** approach where YAML schema files are the single source of truth.

## Module Overview

The Bucket module provides comprehensive file storage and content management capabilities:

- **File Storage:** Upload, download, streaming, versioning
- **Organization:** Buckets, folders, collections, metadata
- **Access Control:** Permissions, sharing, locks, ACLs
- **Content Processing:** Conversions, thumbnails, processing jobs
- **Security:** Hash verification, access logs, quotas
- **Collaboration:** Comments, shares, notifications

### Key Statistics

- **13 Core Entities** - Focused file management domain
- **15 Migration Files** - Complete database schema evolution
- **Generated Code** - Full CRUD, gRPC, webapp support

## Core Domain Entities

### Storage Organization
| Entity | Description | Key Features |
|---------|-------------|---------------|
| **Bucket** | Container for files | Folders, collections, metadata |
| **StoredFile** | File storage | Versions, hashes, thumbnails |
| **UploadSession** | Chunked uploads | Progress tracking, resumability |
| **FileVersion** | File versioning | History, diffs, restoration |
| **Thumbnail** | Image thumbnails | Multiple sizes, generation |
| **ProcessingJob** | Async processing | Status, results, errors |

### Access & Security
| Entity | Description | Key Features |
|---------|-------------|---------------|
| **FileShare** | Shared links | Expiration, permissions, password protection |
| **FileLock** | File locking | Exclusive access, timeout, automatic release |
| **FileComment** | File comments | Mentions, threading, notifications |
| **FileAccessLog** | Access tracking | Audit trail, user, action, timestamp |
| **UserQuota** | Storage quotas | Limits, usage, enforcement |

### Content Management
| Entity | Description | Key Features |
|---------|-------------|---------------|
| **ContentHash** | Content deduplication | SHA-256, verification, dedup |
| **ConversionJob** | Format conversion | PDF, images, video, status |
| **AccessAction** | Access tracking | View, download, share, delete |

## Architecture Overview

```
bucket/
├── schema/                          # SCHEMA DEFINITIONS
│   ├── models/                     # Entity schema definitions
│   │   ├── bucket.model.yaml
│   │   ├── stored_file.model.yaml
│   │   ├── upload_session.model.yaml
│   │   └── ...
│   ├── hooks/
│   ├── workflows/
│   └── openapi/
│
├── proto/                        # PROTOBUF DEFINITIONS
│   ├── domain/
│   │   ├── entity/              # Bucket, StoredFile, UploadSession...
│   │   ├── value_object/         # Metadata, Hash, Quota...
│   │   ├── repository/           # Repository interfaces
│   │   ├── usecase/              # CQRS commands & queries
│   │   ├── service/              # Domain services
│   │   ├── event/                # Domain events
│   │   └── ...
│   └── services/                  # Service definitions
│
├── src/
│   ├── domain/
│   │   ├── entity/               # 13 entity implementations
│   │   ├── value_object/         # Metadata, ContentHash, Thumbnail...
│   │   ├── event/                # Domain events
│   │   ├── repositories/          # Repository traits
│   │   ├── services/             # Domain services
│   │   ├── specifications/        # Business rules
│   │   └── mod.rs
│   │
│   ├── application/
│   │   ├── commands/             # 13 command handlers
│   │   ├── queries/              # 13 query handlers
│   │   ├── services/             # 13 application services
│   │   ├── bulk_operations/      # 6 bulk operation files
│   │   ├── triggers/             # 3 trigger files
│   │   ├── validation/           # 6 validator files
│   │   ├── workflows/            # 3 workflow files
│   │   └── mod.rs
│   │
│   ├── infrastructure/
│   │   ├── persistence/          # 13 repository implementations
│   │   ├── projections/         # 13 projection files
│   │   ├── event_store/         # Event store
│   │   └── mod.rs
│   │
│   ├── presentation/
│   │   ├── http/                # 13 HTTP handlers
│   │   ├── dto/                 # 13 DTO files
│   │   ├── grpc/                # gRPC services
│   │   ├── auth/                # 6 auth files
│   │   └── mod.rs
│   │
│   ├── integration/
│   │   ├── context_map.rs        # Integration context
│   │   └── mod.rs
│   │
│   ├── routes/                   # HTTP routes
│   ├── exports/                  # Public API
│   └── lib.rs
│
├── migrations/                   # DATABASE MIGRATIONS
│   ├── 001_create_enums.up.sql
│   ├── 003_create_bucket_table.up.sql
│   ├── ...
│   ├── 015_create_user_quota_table.up.sql
│   └── down.sql
│
├── tests/
│   ├── integration/
│   └── integration_tests.rs
│
├── config/
│   ├── application.yml
│   └── openapi/
│
├── Cargo.toml
└── README.md
```

## Database

**Database:** `bucket` (configured per module)
**Tables:** 13 main tables
**Migrations:** 15 migration files

### Connection Configuration

```bash
# Uses app-level database or module-specific override
# Via config/application.yml
database:
  url: postgresql://root:password@localhost:5432/bucket

# Or via environment
DATABASE_URL=postgresql://root:password@localhost:5432/bucket
```

## Quick Start

### 1. Generate Code from Schema

```bash
# Build the schema generator
cargo build --release --bin backbone-schema

# Generate all code (31 targets)
./target/release/backbone-schema schema generate bucket --target all --force
```

### 2. Run Database Migrations

```bash
# Set database URL
export DATABASE_URL="postgresql://root:password@localhost:5432/bucket"

# Run migrations
./target/debug/backbone migration run --module bucket

# Check migration status
./target/debug/backbone migration status --module bucket
```

### 3. Use the Module

```rust
use backbone_bucket::BucketModule;

// Create module instance with builder pattern
let module = BucketModule::builder()
    .with_database(pool)
    .with_config(config)
    .build()?;

// Get HTTP routes
let routes = module.routes();

// Get services
let bucket_service = module.bucket_service();
let file_service = module.stored_file_service();
```

## API Endpoints

All entities have 11 standard CRUD endpoints under `/api/v1/bucket/{collection}`:

| Method | Endpoint | Description |
|--------|------------|-------------|
| `GET` | `/api/v1/bucket/buckets` | List buckets |
| `POST` | `/api/v1/bucket/buckets` | Create bucket |
| `GET` | `/api/v1/bucket/files` | List files (with filters) |
| `POST` | `/api/v1/bucket/files` | Upload file |
| `GET` | `/api/v1/bucket/files/:id` | Get file metadata |
| `PATCH` | `/api/v1/bucket/files/:id` | Update file |
| `DELETE` | `/api/v1/bucket/files/:id` | Soft delete file |
| `POST` | `/api/v1/bucket/files/bulk` | Batch upload |
| `GET` | `/api/v1/bucket/files/trash` | List deleted files |
| `POST` | `/api/v1/bucket/files/:id/restore` | Restore file |
| `DELETE` | `/api/v1/bucket/files/empty` | Empty trash |

### Key Features

#### Chunked File Upload

Supports large file uploads through chunked sessions:

```bash
# 1. Create upload session
POST /api/v1/bucket/uploads
{
  "file_name": "large-video.mp4",
  "file_size": 1073741824,
  "content_type": "video/mp4",
  "chunk_size": 5242880  # 5MB chunks
}

# 2. Upload chunks
PUT /api/v1/bucket/uploads/{session_id}/chunks/{chunk_index}
Content-Range: bytes {start}-{end}

# 3. Complete upload
POST /api/v1/bucket/uploads/{session_id}/complete
```

#### Content Deduplication

Automatic content deduplication using SHA-256 hashes:

```yaml
# ContentHash entity tracks file content
models:
  - name: ContentHash
    fields:
      hash:
        type: string
        attributes: ["@unique"]  # SHA-256
      algorithm:
        type: string
        attributes: ["@default(sha-256)"]
      size:
        type: int64
```

#### File Versioning

Track all changes to files:

```yaml
# FileVersion entity
models:
  - name: FileVersion
    fields:
      file_id:           # Reference to StoredFile
      version_number:    # Sequential version
      change_reason:      # Why changed
      storage_path:       # Location of this version
      created_at:         # When created
```

## Development Workflow

### Schema Example (Stored File)

```yaml
models:
  - name: StoredFile
    collection: files
    soft_delete: true
    extends: [Metadata]

    fields:
      id:
        type: uuid
        attributes: ["@id", "@default(uuid)"]

      bucket_id:
        type: uuid
        attributes: ["@required", "@foreign_key(Bucket.id)", "@on_delete(cascade)"]

      file_name:
        type: string
        attributes: ["@required", "@max(255)"]

      file_path:
        type: string
        attributes: ["@required"]  # Internal storage path

      content_hash:
        type: string
        attributes: ["@foreign_key(ContentHash.id)"]

      file_size:
        type: int64
        attributes: ["@non_negative"]

      mime_type:
        type: string
        attributes: ["@max(100)"]

    relations:
      bucket:
        type: Bucket
        attributes: ["@one", "@foreign_key(bucket_id)", "@on_delete(cascade)"]

      versions:
        type: FileVersion[]
        attributes: ["@one_to_many", "@on_delete(cascade)"]

      thumbnails:
        type: Thumbnail[]
        attributes: ["@one_to_many", "@on_delete(cascade)"]

    indexes:
      - type: index
        fields: [bucket_id]
      - type: index
        fields: [content_hash]
      - type: index
        fields: [file_name]
```

## Configuration

### Application Configuration

```yaml
# config/application.yml
database:
  url: postgresql://root:password@localhost:5432/bucket
  max_connections: 20
  min_connections: 5

entities:
  bucket:
    enabled: true
    collection: buckets
    cache_ttl: 300
```

### Environment Variables

```bash
# Override database URL
DATABASE_URL=postgresql://user:pass@localhost:5432/bucket

# Override storage path
STORAGE_PATH=/data/storage

# Override max file size
MAX_FILE_SIZE=1073741824  # 1GB default
```

## Dependencies

This module depends on:
- `backbone-core` - Core framework utilities
- `backbone-orm` - ORM and database traits
- `backbone-auth` - Authentication and authorization
- `backbone-messaging` - Event messaging
- `backbone-storage` - Storage abstraction layer

## Documentation

- [Framework Documentation](../../docs/technical/FRAMEWORK.md)
- [Schema Reference](../../docs/technical/SCHEMA_REFERENCE.md)
- [API Reference](../../docs/api/API_REFERENCE.md)
- [Relation Attributes](../../docs/technical/RELATION_ATTRIBUTES.md)

## License

Part of Backbone Framework.
