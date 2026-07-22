# Graph Report - backbone-bucket  (2026-07-22)

## Corpus Check
- 629 files · ~353,949 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 10788 nodes · 23322 edges · 577 communities (574 shown, 3 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 87 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0b223073`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- MockRepository
- Uuid
- BackboneId
- AccessLog
- upload.rs
- AppStateBuilder
- SortParams
- StoredEvent
- ContentHash
- Arc
- MetadataBuilder
- health_checker.rs
- backbone_handler.rs
- workflow_tests.rs
- Option
- auth_middleware.rs
- StoredFileTriggerContext
- UploadSessionTriggerContext
- ProcessingJob
- FileShare
- ConversionJobTriggerContext
- ProcessingJobTriggerContext
- Bucket
- EventHandler
- events.rs
- Self
- Arc
- backbone_specifications.rs
- PostgresqlBackboneRepository
- ComputedFieldValues
- repositories/backbone_repository.rs
- Arc
- Self
- ApiVersion
- StateMachineError
- StorageError
- Self
- validator/mod.rs
- Changelog
- StoredFileState
- FileAccessLogEntry
- TestSuiteResult
- Bucket Module - Code Quality Report
- FileUploadFlowInstance
- MediaProcessingFlowInstance
- MultipartUploadFlowInstance
- ShareCreationFlowInstance
- BucketType
- ShareStatus
- BucketState
- ConversionJobState
- FileCommentState
- FileLockState
- FileShareState
- UploadSessionState
- UserQuotaState
- Metadata
- bucket_config.rs
- file_version_handler.rs
- ApiTest
- StoredFile
- ImageCompressorService
- bucket.rs
- Thumbnail
- CommandHandler
- Self
- LocalStorage
- TestSetupManager
- QuotaStatus
- ThumbnailSize
- UserQuota
- VersionType
- file_upload_service.rs
- BucketModule
- CommonUtils
- FileCommentAfterCreateHandler1
- file_comment_commands.rs
- CreateFileShareCommand
- CreateUploadSessionCommand
- FileVersion
- routes.rs
- serving.rs
- Database Migrations for bucket
- conversion_job_commands.rs
- file_lock_commands.rs
- file_version_commands.rs
- processing_job_commands.rs
- CreateStoredFileCommand
- thumbnail_commands.rs
- user_quota_commands.rs
- conversion_job_handler.rs
- Bucket Configuration Thresholds
- bucket_commands.rs
- content_hash_commands.rs
- file_access_log_commands.rs
- bucket_queries.rs
- file_lock_queries.rs
- ListFileShareQuery
- file_version_queries.rs
- thumbnail_queries.rs
- user_quota_queries.rs
- ConversionJob
- UploadSessionId
- UploadSessionBuilder
- processing_job_handler.rs
- stored_file_handler.rs
- InMemoryStorage
- app_config.rs
- content_hash_queries.rs
- FileCommentBuilder
- FileComment
- FileLock
- PaginatedResult
- ConversionJobResponseDto
- FileCommentResponseDto
- ProcessingJobResponseDto
- StoredFileResponseDto
- UploadSessionResponseDto
- HttpServices
- T
- S3Storage
- QueryHandler
- BucketStatus
- ConversionStatus
- FileLockId
- FileStatus
- JobStatus
- AccessLogResponseDto
- ContentHashResponseDto
- AuthContext
- bucket_handler.rs
- file_comment_handler.rs
- config/generated.rs
- FileProcessingFlowInstance
- ThumbnailBuilder
- UploadSession
- VirusScanResult
- FileLockResponseDto
- domain_tests.rs
- require_permission
- UploadMultipartTest
- FileCommentId
- StorageBackend
- StoredFileId
- file_lock_handler.rs
- file_share_handler.rs
- TestResult
- ListAccessLogQuery
- ListConversionJobQuery
- ListFileCommentQuery
- ListProcessingJobQuery
- ListStoredFileQuery
- ListUploadSessionQuery
- Self
- user_quota_handler.rs
- ObjectStorage
- AccessAction
- Backbone
- StubStorage
- Bucket Integration Tests
- ThumbnailId
- DocumentPreviewService
- VideoThumbnailService
- FileVersionId
- FileAccessLogResponseDto
- JwtTokenManager
- ExampleUser
- create_file_comment.rs
- CreateFileShareInput
- CreateUploadSessionInput
- update_file_comment.rs
- UpdateFileShareInput
- UpdateUploadSessionInput
- .http_routes
- FileShareId
- http/auth/access_log_auth.rs
- http/auth/bucket_auth.rs
- http/auth/content_hash_auth.rs
- http/auth/conversion_job_auth.rs
- http/auth/file_comment_auth.rs
- http/auth/file_lock_auth.rs
- http/auth/file_share_auth.rs
- http/auth/file_version_auth.rs
- http/auth/processing_job_auth.rs
- http/auth/stored_file_auth.rs
- http/auth/upload_session_auth.rs
- http/auth/user_quota_auth.rs
- IntegrationRegistry
- Breaking Changes
- BulkOperationResult
- create_conversion_job.rs
- CreateProcessingJobInput
- CreateStoredFileInput
- create_user_quota.rs
- update_access_log.rs
- update_content_hash.rs
- update_conversion_job.rs
- UpdateProcessingJobInput
- UpdateStoredFileInput
- update_user_quota.rs
- String
- String
- Action
- create_test_session
- CdnService
- create_access_log.rs
- CreateBucketInput
- create_content_hash.rs
- create_file_lock.rs
- create_thumbnail.rs
- UpdateBucketInput
- update_file_lock.rs
- update_file_version.rs
- update_thumbnail.rs
- create_content_hash_routes
- Bucket Module
- ConversionService
- create_file_version.rs
- ConversionJobId
- ProcessingJobId
- UserQuotaId
- access_log_handler.rs
- Trait Abstraction Analysis for Bucket Entities
- .new
- .new
- .new
- .new
- BulkOperationConfig
- .new
- .new
- .new
- .new
- .new
- .new
- .new
- .new
- .new
- DeduplicationService
- LockingService
- AccessLogFilter
- BucketFilter
- ConversionJobFilter
- FileCommentFilter
- FileShareFilter
- FileVersionFilter
- ProcessingJobFilter
- StoredFileFilter
- ThumbnailFilter
- UploadSessionFilter
- UserQuotaFilter
- access_log_projector.rs
- bucket_projector.rs
- content_hash_projector.rs
- conversion_job_projector.rs
- file_comment_projector.rs
- file_lock_projector.rs
- file_share_projector.rs
- file_version_projector.rs
- processing_job_projector.rs
- stored_file_projector.rs
- thumbnail_projector.rs
- upload_session_projector.rs
- user_quota_projector.rs
- FileAccessLogRepository
- list_access_log.rs
- list_bucket.rs
- list_content_hash.rs
- list_conversion_job.rs
- list_file_comment.rs
- list_file_lock.rs
- list_file_share.rs
- list_file_version.rs
- list_processing_job.rs
- list_stored_file.rs
- list_thumbnail.rs
- list_upload_session.rs
- list_user_quota.rs
- BucketPermissions
- ContentHashFilter
- repositories/file_lock_repository.rs
- FileAccessLogSpecification
- FileLockRepository
- UserQuotaRepository
- Seeder
- create_test_quota
- create_test_job
- create_test_share
- TestDataGenerator
- presentation/dto/mod.rs
- get_access_log.rs
- get_bucket.rs
- get_content_hash.rs
- get_conversion_job.rs
- get_file_comment.rs
- get_file_lock.rs
- get_file_share.rs
- get_file_version.rs
- get_processing_job.rs
- get_stored_file.rs
- get_thumbnail.rs
- get_upload_session.rs
- get_user_quota.rs
- list_file_access_log.rs
- AuditMetadata
- BucketRepository
- ContentHashRepository
- FileShareRepository
- FileVersionRepository
- ThumbnailRepository
- create_test_hash
- Bucket File Storage System - Technical Domain Documentation
- File Serving
- delete_access_log.rs
- delete_bucket.rs
- delete_content_hash.rs
- delete_conversion_job.rs
- delete_file_comment.rs
- delete_file_lock.rs
- delete_file_share.rs
- delete_file_version.rs
- delete_processing_job.rs
- delete_stored_file.rs
- delete_thumbnail.rs
- delete_upload_session.rs
- delete_user_quota.rs
- get_file_access_log.rs
- update_file_access_log.rs
- file_processing_workflow.rs
- ProcessingStatus
- ThreatLevel
- UploadStatus
- Self
- StoredFilePermissions
- StatsService
- FileCommentProjection
- FileShareProjection
- UploadSessionProjection
- create_test_lock
- FileAccessLogApiTest
- Metaphor Domain Module
- Business Requirements Document (BRD)
- CommentStatus
- LockStatus
- SharePermission
- ShareType
- ProcessingJobType
- services.rs
- AccessLogProjection
- FileVersionProjection
- ThumbnailProjection
- UserQuotaProjection
- SeedAccessLogSeeder
- SeedBucketSeeder
- SeedContentHashSeeder
- SeedConversionJobSeeder
- SeedFileAccessLogSeeder
- SeedFileCommentSeeder
- SeedFileLockSeeder
- SeedFileVersionSeeder
- SeedProcessingJobSeeder
- SeedStoredFileSeeder
- SeedThumbnailSeeder
- SeedUploadSessionSeeder
- SeedUserQuotaSeeder
- create_test_conversion
- AccessLogApiTest
- ContentHashApiTest
- ConversionJobApiTest
- FileCommentApiTest
- FileLockApiTest
- FileShareApiTest
- FileVersionApiTest
- ProcessingJobApiTest
- StoredFileApiTest
- ThumbnailApiTest
- UploadSessionApiTest
- UserQuotaApiTest
- Bucket Module Specification
- 5.2 Entity Definitions
- create_file_access_log.rs
- ConversionType
- HashAlgorithm
- FieldRestriction
- AccessLogDomainService
- BucketDomainService
- ContentHashDomainService
- ConversionJobDomainService
- FileAccessLogDomainService
- FileCommentDomainService
- FileLockDomainService
- FileShareDomainService
- FileVersionDomainService
- ProcessingJobDomainService
- StoredFileDomainService
- ThumbnailDomainService
- UploadSessionDomainService
- UserQuotaDomainService
- FileAccessLogProjection
- FileLockProjection
- collect_proto_files
- commands/mod.rs
- 4. Use Cases
- 6. Enums (Value Types)
- queries/mod.rs
- AccessLogRepository
- ConversionJobRepository
- FileCommentRepository
- ProcessingJobRepository
- StoredFileRepository
- UploadSessionRepository
- create_file_with_owner
- 7.1 User Stories
- 9.1 Core Entities
- 4. Value Objects and Enums
- Bucket API Documentation
- BucketError
- Bucket usage — step by step
- .new
- .new
- .new
- utils/mod.rs
- create_test_bucket
- 5. Functional Requirements
- Bucket Module - Implementation Plan V2.0
- 13.1 PostgreSQL Tables
- 8.1 Commands (Write Operations)
- 2. Project Overview
- State Machine: `StoredFile`
- 8. Workflows (Multi-Step Processes)
- 12.2 Layer 2: Domain-Specific Endpoints
- 3. Entities (Aggregate Roots)
- bench
- 4.1 New Schema Files to Create
- 10. Services (Business Logic)
- .from_str
- .from_str
- .from_str
- 10. Implementation Checklist
- 3. Implementation Phases
- 9. Events (Domain Events)
- 15. Implementation Checklist
- 7. Domain Services
- 9. Repositories
- AccessLogPolicy
- BucketPolicy
- ContentHashPolicy
- ConversionJobPolicy
- FileCommentPolicy
- FileLockPolicy
- FileSharePolicy
- FileVersionPolicy
- ProcessingJobPolicy
- StoredFilePolicy
- ThumbnailPolicy
- UploadSessionPolicy
- UserQuotaPolicy
- CreateFileAccessLogInput
- FileAccessLogFilter
- 6. Non-Functional Requirements
- 4.2 Updates to Existing Schema Files
- 11. API Requirements
- 16. New Features (V2.0)
- 4. Storage Service
- 14. Security Model
- 5. Domain Events
- Bucket Module — Documentation
- Workflows
- Error
- cli/mod.rs
- 10. Integration Requirements
- 11. API Specifications
- 13. Security Requirements
- 14. Success Criteria
- 3. Business Context
- 2. Current State Analysis
- 6. Custom Logic Implementation
- 12. Authorization & Permissions
- 14. Non-Functional Requirements
- 1. Module Overview
- 11. Workflows
- 2. Domain-Driven Design Overview
- .execute
- BucketDomainPolicy
- ConversionJobDomainPolicy
- FileCommentDomainPolicy
- FileLockDomainPolicy
- FileShareDomainPolicy
- ProcessingJobDomainPolicy
- StoredFileDomainPolicy
- UploadSessionDomainPolicy
- UserQuotaDomainPolicy
- BucketEventMetadata
- extract_bearer_token
- 15. Assumptions & Dependencies
- 16. Risks & Mitigation
- 17. Glossary & References
- 8. Data Requirements
- 7. Testing Strategy
- 8. Migration Strategy
- 9. Dependencies & Risks
- seeder.rs
- gen_bytes
- context_map.rs

## God Nodes (most connected - your core abstractions)
1. `AuthContext` - 255 edges
2. `BulkOperationResult` - 73 edges
3. `AuditMetadata` - 66 edges
4. `StorageBackend` - 65 edges
5. `StoredFile` - 63 edges
6. `Uuid` - 63 edges
7. `CommonUtils` - 63 edges
8. `CommandHandler` - 59 edges
9. `QueryHandler` - 52 edges
10. `FileShare` - 51 edges

## Surprising Connections (you probably didn't know these)
- `ExampleUser` --implements--> `HasOwnerId`  [EXTRACTED]
  examples/serving/main.rs → src/auth/mod.rs
- `ExamplePolicy` --implements--> `AuthzPolicy`  [EXTRACTED]
  examples/serving/main.rs → src/auth/mod.rs
- `create_processing_job()` --references--> `ProcessingJobType`  [EXTRACTED]
  tests/workflow_tests.rs → src/domain/entity/processing_job_type.rs
- `NotSpecification` --references--> `T`  [EXTRACTED]
  src/domain/specifications/backbone_specifications.rs → src/auth/mod.rs
- `access_log_routes()` --calls--> `create_access_log_routes()`  [INFERRED]
  src/presentation/http/routes.rs → src/presentation/http/access_log_handler.rs

## Import Cycles
- 2-file cycle: `src/domain/state_machine/file_lock_state_machine.rs -> src/domain/state_machine/mod.rs -> src/domain/state_machine/file_lock_state_machine.rs`
- 2-file cycle: `src/domain/state_machine/conversion_job_state_machine.rs -> src/domain/state_machine/mod.rs -> src/domain/state_machine/conversion_job_state_machine.rs`
- 2-file cycle: `src/domain/state_machine/bucket_state_machine.rs -> src/domain/state_machine/mod.rs -> src/domain/state_machine/bucket_state_machine.rs`
- 2-file cycle: `src/domain/state_machine/mod.rs -> src/domain/state_machine/stored_file_state_machine.rs -> src/domain/state_machine/mod.rs`
- 2-file cycle: `src/domain/state_machine/mod.rs -> src/domain/state_machine/processing_job_state_machine.rs -> src/domain/state_machine/mod.rs`
- 2-file cycle: `src/domain/state_machine/file_comment_state_machine.rs -> src/domain/state_machine/mod.rs -> src/domain/state_machine/file_comment_state_machine.rs`
- 2-file cycle: `src/domain/state_machine/file_share_state_machine.rs -> src/domain/state_machine/mod.rs -> src/domain/state_machine/file_share_state_machine.rs`
- 2-file cycle: `src/domain/state_machine/mod.rs -> src/domain/state_machine/user_quota_state_machine.rs -> src/domain/state_machine/mod.rs`
- 2-file cycle: `src/domain/state_machine/mod.rs -> src/domain/state_machine/upload_session_state_machine.rs -> src/domain/state_machine/mod.rs`
- 2-file cycle: `src/application/queries/mod.rs -> src/application/queries/user_quota_queries.rs -> src/application/queries/mod.rs`
- 2-file cycle: `src/application/queries/conversion_job_queries.rs -> src/application/queries/mod.rs -> src/application/queries/conversion_job_queries.rs`
- 2-file cycle: `src/application/queries/file_share_queries.rs -> src/application/queries/mod.rs -> src/application/queries/file_share_queries.rs`
- 2-file cycle: `src/application/queries/mod.rs -> src/application/queries/upload_session_queries.rs -> src/application/queries/mod.rs`
- 2-file cycle: `src/application/queries/mod.rs -> src/application/queries/stored_file_queries.rs -> src/application/queries/mod.rs`
- 2-file cycle: `src/application/queries/content_hash_queries.rs -> src/application/queries/mod.rs -> src/application/queries/content_hash_queries.rs`
- 2-file cycle: `src/application/queries/mod.rs -> src/application/queries/processing_job_queries.rs -> src/application/queries/mod.rs`
- 2-file cycle: `src/application/queries/file_lock_queries.rs -> src/application/queries/mod.rs -> src/application/queries/file_lock_queries.rs`
- 2-file cycle: `src/application/queries/file_comment_queries.rs -> src/application/queries/mod.rs -> src/application/queries/file_comment_queries.rs`
- 2-file cycle: `src/application/queries/file_version_queries.rs -> src/application/queries/mod.rs -> src/application/queries/file_version_queries.rs`
- 2-file cycle: `src/application/queries/bucket_queries.rs -> src/application/queries/mod.rs -> src/application/queries/bucket_queries.rs`

## Communities (577 total, 3 thin omitted)

### Community 0 - "MockRepository"
Cohesion: 0.06
Nodes (41): BackboneDto, BackboneFilters, BackboneQueryHandlerFactory, DefaultGetBackboneHandler, DefaultListBackbonesHandler, DefaultSearchBackbonesHandler, GetBackboneHandler, GetBackboneQuery (+33 more)

### Community 1 - "Uuid"
Cohesion: 0.06
Nodes (62): AccessLogDto, AccessLogId, AccessLogRef, AccessLogSummary, BucketDto, BucketId, BucketRef, BucketSummary (+54 more)

### Community 2 - "BackboneId"
Cohesion: 0.04
Nodes (29): E, Metadata, BackboneCreated, BackboneDeleted, BackboneMetadataChanged, BackboneStatusChanged, BackboneTagsChanged, BackboneUpdated (+21 more)

### Community 3 - "AccessLog"
Cohesion: 0.05
Nodes (25): AccessLogBuilder, AccessLog, AccessLogBuilder, AccessLogId, AsRef, DateTime, Deref, Display (+17 more)

### Community 4 - "upload.rs"
Cohesion: 0.07
Nodes (64): Field, Multipart, MultipartUploadService, Arc, BucketRepository, Option, Self, ServiceResult (+56 more)

### Community 5 - "AppStateBuilder"
Cohesion: 0.08
Nodes (56): health_check(), IntoResponse, State, AppState, AppStateBuilder, AccessLogService, Arc, BucketModule (+48 more)

### Community 6 - "SortParams"
Cohesion: 0.08
Nodes (34): BackboneValidationService, BusinessRuleReport, ConfigurationReport, CreateBackboneCommand, CreateBackboneHandler, CreateBackboneHandlerFactory, CreateBackboneResponse, DefaultCreateBackboneHandler (+26 more)

### Community 7 - "StoredEvent"
Cohesion: 0.06
Nodes (46): Pin, EventEnvelope, EventEnvelope<T>, EventMetadata, DateTime, HashMap, Into, Option (+38 more)

### Community 8 - "ContentHash"
Cohesion: 0.06
Nodes (26): ContentHash, ContentHashBuilder, ContentHashId, DedupError, AsRef, DateTime, Deref, Display (+18 more)

### Community 9 - "Arc"
Cohesion: 0.09
Nodes (23): Arc, Option, Result, Self, TriggerHandler, Vec, user_quota_trigger_registry(), UserQuotaAfterCreateHandler2 (+15 more)

### Community 10 - "MetadataBuilder"
Cohesion: 0.06
Nodes (33): Actors, ActorsBuilder, Default, Option, Result, Self, String, Uuid (+25 more)

### Community 11 - "health_checker.rs"
Cohesion: 0.08
Nodes (28): CpuHealthCheck, DatabaseHealthCheck, DiskSpaceHealthCheck, HealthCheck, HealthChecker, HealthCheckerFactory, HealthCheckResult, HealthStatus (+20 more)

### Community 12 - "backbone_handler.rs"
Cohesion: 0.10
Nodes (53): ApplicationServices, Data, HttpResponse, Query, Responder, ServiceConfig, api_info(), ApiError (+45 more)

### Community 13 - "workflow_tests.rs"
Cohesion: 0.06
Nodes (66): create_content_hash(), create_conversion(), create_direct_share(), create_lock(), create_processing_job(), create_share(), create_test_bucket(), create_test_file() (+58 more)

### Community 14 - "Option"
Cohesion: 0.07
Nodes (42): FileAccessLogBulkCreatedEvent, FileAccessLogCreatedEvent, FileAccessLogDeletedEvent, FileAccessLogEvent, FileAccessLogEventPublisher, FileAccessLogPartiallyUpdatedEvent, FileAccessLogUpdatedEvent, DateTime (+34 more)

### Community 15 - "auth_middleware.rs"
Cohesion: 0.07
Nodes (47): AuthenticationError, Cors, Credentials, HttpAuthentication, Ready, RwLock, ServiceRequest, ServiceResponse (+39 more)

### Community 16 - "StoredFileTriggerContext"
Cohesion: 0.09
Nodes (23): Arc, Option, Result, Self, TriggerHandler, Vec, stored_file_trigger_registry(), StoredFileAfterCreateHandler1 (+15 more)

### Community 17 - "UploadSessionTriggerContext"
Cohesion: 0.10
Nodes (23): Arc, Option, Result, Self, TriggerHandler, Vec, upload_session_trigger_registry(), UploadSessionAfterCreateHandler1 (+15 more)

### Community 18 - "ProcessingJob"
Cohesion: 0.07
Nodes (18): JobError, ProcessingJob, ProcessingJobBuilder, DateTime, Duration, EntityRepoMeta, From, HashMap (+10 more)

### Community 19 - "FileShare"
Cohesion: 0.08
Nodes (15): FileShare, FileShareBuilder, DateTime, EntityRepoMeta, From, HashMap, Id, Option (+7 more)

### Community 20 - "ConversionJobTriggerContext"
Cohesion: 0.10
Nodes (22): ConversionJobActionExecutor, ConversionJobEventPublisher, ConversionJobTriggerContext, ConversionJobTriggerEvent, ConversionJobTriggerRegistry, conversion_job_trigger_registry(), ConversionJobAfterCreateHandler1, ConversionJobAfterCreateHandler2 (+14 more)

### Community 21 - "ProcessingJobTriggerContext"
Cohesion: 0.10
Nodes (22): ProcessingJobActionExecutor, ProcessingJobEventPublisher, ProcessingJobTriggerContext, ProcessingJobTriggerEvent, ProcessingJobTriggerRegistry, processing_job_trigger_registry(), ProcessingJobAfterCreateHandler1, ProcessingJobAfterCreateHandler2 (+14 more)

### Community 22 - "Bucket"
Cohesion: 0.07
Nodes (16): Bucket, BucketBuilder, DateTime, EntityRepoMeta, Err, HashMap, Id, Option (+8 more)

### Community 23 - "EventHandler"
Cohesion: 0.06
Nodes (42): AccessLogEvent, ContentHashEvent, ConversionJobEvent, FileCommentEvent, FileLockEvent, FileShareEvent, FileVersionEvent, ProcessingJobEvent (+34 more)

### Community 24 - "events.rs"
Cohesion: 0.12
Nodes (58): AccessLogId, BucketId, ContentHashId, ConversionJobId, Debug, FileCommentId, FileLockId, FileShareId (+50 more)

### Community 25 - "Self"
Cohesion: 0.10
Nodes (21): BucketActionExecutor, BucketEventPublisher, BucketTriggerContext, BucketTriggerEvent, BucketTriggerRegistry, bucket_trigger_registry(), BucketAfterCreateHandler2, BucketAfterCreateHandler3 (+13 more)

### Community 26 - "Arc"
Cohesion: 0.11
Nodes (21): FileShareActionExecutor, FileShareEventPublisher, FileShareTriggerContext, FileShareTriggerEvent, FileShareTriggerRegistry, file_share_trigger_registry(), FileShareAfterCreateHandler2, FileShareAfterCreateHandler3 (+13 more)

### Community 27 - "backbone_specifications.rs"
Cohesion: 0.08
Nodes (36): AndSpecification, AndSpecification<T, U>, BackboneCanArchiveSpecification, BackboneCanDeactivateSpecification, BackboneCanSuspendSpecification, BackboneIsActiveSpecification, BackboneMustBeRecentSpecification, BackboneMustHaveMetadataSpecification (+28 more)

### Community 28 - "PostgresqlBackboneRepository"
Cohesion: 0.13
Nodes (23): PgArgumentValue, PgRow, create_test_pool(), ensure_backbone_table_exists(), PostgresqlBackboneRepository, PostgresqlBackboneRepositoryFactory, Backbone, BackboneFilters (+15 more)

### Community 29 - "ComputedFieldValues"
Cohesion: 0.09
Nodes (17): Bucket, ComputedFieldValues, ConversionJob, FileComment, FileLock, FileShare, HasComputedFields, ProcessingJob (+9 more)

### Community 30 - "repositories/backbone_repository.rs"
Cohesion: 0.09
Nodes (27): AuditableRepository, AuditEntry, BackboneFilters, BackboneRepositoryFactory, BackboneTransaction, CacheableRepository, PaginatedResult<T>, PaginationParams (+19 more)

### Community 31 - "Arc"
Cohesion: 0.12
Nodes (19): FileLockActionExecutor, FileLockEventPublisher, FileLockTriggerContext, FileLockTriggerEvent, FileLockTriggerRegistry, file_lock_trigger_registry(), FileLockAfterCreateHandler1, FileLockAfterCreateHandler2 (+11 more)

### Community 32 - "Self"
Cohesion: 0.09
Nodes (6): HashMap, Self, String, Value, StoredFileBuilder, Uuid

### Community 33 - "ApiVersion"
Cohesion: 0.07
Nodes (31): ApiVersion, extract_version_from_header(), extract_version_from_path(), extract_version_from_query(), ResolvedVersion, HeaderMap, Next, Option (+23 more)

### Community 34 - "StateMachineError"
Cohesion: 0.12
Nodes (17): String, StateMachineError, ProcessingJobState, ProcessingJobStateMachine, ProcessingJobTransition, Default, Display, Err (+9 more)

### Community 35 - "StorageError"
Cohesion: 0.13
Nodes (19): PurgeResult, AsRef, DateTime, Display, Formatter, From, Path, PathBuf (+11 more)

### Community 36 - "Self"
Cohesion: 0.09
Nodes (13): BackboneInDateRangeSpecification, BackboneTimestamp, BackboneVersion, BackboneVersionError, DateTime, Default, Self, Utc (+5 more)

### Community 37 - "validator/mod.rs"
Cohesion: 0.05
Nodes (26): AccessLogValidator, BucketValidator, ContentHashValidator, ConversionJobValidator, FileCommentValidator, FileLockValidator, FileShareValidator, FileVersionValidator (+18 more)

### Community 38 - "Changelog"
Cohesion: 0.05
Nodes (40): [0.1.10] - 2026-06-05, [0.1.11] - 2026-06-06, [0.1.12] - 2026-06-06, [0.1.13] - 2026-06-07, [0.1.14] - 2026-06-16, [0.1.15] - 2026-06-20, [0.1.16] - 2026-06-20, [0.1.17] - 2026-06-20 (+32 more)

### Community 39 - "StoredFileState"
Cohesion: 0.12
Nodes (15): Default, Display, Err, Formatter, FromStr, Result, Self, Vec (+7 more)

### Community 40 - "FileAccessLogEntry"
Cohesion: 0.17
Nodes (14): AccessLogBuilder, AccessLoggerService, FileAccessLogEntry, DateTime, Into, Option, Self, String (+6 more)

### Community 41 - "TestSuiteResult"
Cohesion: 0.07
Nodes (20): DateTime, Display, Error, Formatter, Into, Option, Path, Result (+12 more)

### Community 42 - "Bucket Module - Code Quality Report"
Cohesion: 0.05
Nodes (39): 10. Final Scores, 11. Action Items Checklist, 12. Appendix, 1.1 Entities (StoredFile, Bucket, UserQuota, FileShare), 1.2 Domain Services, 1. Clean Code Assessment, 2.1 Naming Conventions, 2.2 Pattern Consistency (+31 more)

### Community 43 - "FileUploadFlowInstance"
Cohesion: 0.10
Nodes (24): FileUploadFlowExecutor, FileUploadFlowExecutor<H>, FileUploadFlowInstance, FileUploadFlowStatus, FileUploadFlowStep, FileUploadStepHandler, FlowError, Arc (+16 more)

### Community 44 - "MediaProcessingFlowInstance"
Cohesion: 0.10
Nodes (24): FlowError, MediaProcessingFlowExecutor, MediaProcessingFlowExecutor<H>, MediaProcessingFlowInstance, MediaProcessingFlowStatus, MediaProcessingFlowStep, MediaProcessingStepHandler, Arc (+16 more)

### Community 45 - "MultipartUploadFlowInstance"
Cohesion: 0.10
Nodes (24): FlowError, MultipartUploadFlowExecutor, MultipartUploadFlowExecutor<H>, MultipartUploadFlowInstance, MultipartUploadFlowStatus, MultipartUploadFlowStep, MultipartUploadStepHandler, Arc (+16 more)

### Community 46 - "ShareCreationFlowInstance"
Cohesion: 0.10
Nodes (24): FlowError, Arc, DateTime, H, Into, Option, Result, Self (+16 more)

### Community 47 - "BucketType"
Cohesion: 0.11
Nodes (27): BucketType, Default, Display, Err, Formatter, FromStr, Result, Self (+19 more)

### Community 48 - "ShareStatus"
Cohesion: 0.12
Nodes (27): Default, Display, Err, Formatter, FromStr, Result, Self, ShareStatus (+19 more)

### Community 49 - "BucketState"
Cohesion: 0.13
Nodes (15): BucketState, BucketStateMachine, BucketTransition, Default, Display, Err, Formatter, FromStr (+7 more)

### Community 50 - "ConversionJobState"
Cohesion: 0.13
Nodes (15): ConversionJobState, ConversionJobStateMachine, ConversionJobTransition, Default, Display, Err, Formatter, FromStr (+7 more)

### Community 51 - "FileCommentState"
Cohesion: 0.13
Nodes (15): FileCommentState, FileCommentStateMachine, FileCommentTransition, Default, Display, Err, Formatter, FromStr (+7 more)

### Community 52 - "FileLockState"
Cohesion: 0.13
Nodes (15): FileLockState, FileLockStateMachine, FileLockTransition, Default, Display, Err, Formatter, FromStr (+7 more)

### Community 53 - "FileShareState"
Cohesion: 0.13
Nodes (15): FileShareState, FileShareStateMachine, FileShareTransition, Default, Display, Err, Formatter, FromStr (+7 more)

### Community 54 - "UploadSessionState"
Cohesion: 0.13
Nodes (15): Default, Display, Err, Formatter, FromStr, Result, Self, Vec (+7 more)

### Community 55 - "UserQuotaState"
Cohesion: 0.13
Nodes (15): Default, Display, Err, Formatter, FromStr, Result, Self, Vec (+7 more)

### Community 56 - "Metadata"
Cohesion: 0.09
Nodes (13): Iterator, BackboneIdError, BackboneName, BackboneNameError, Metadata, MetadataError, Formatter, From (+5 more)

### Community 57 - "bucket_config.rs"
Cohesion: 0.15
Nodes (30): D, Ok, ConfigEnvError, default_impl_matches_local_dev_shape(), default_local_base_url(), default_local_secret_env(), default_presigned_ttl(), default_public_prefix() (+22 more)

### Community 58 - "file_version_handler.rs"
Cohesion: 0.10
Nodes (30): create_file_version_read_routes(), create_file_version_routes(), create_file_version_write_routes(), create_protected_file_version_routes(), FileVersionError, A, Arc, FileVersionService (+22 more)

### Community 59 - "ApiTest"
Cohesion: 0.15
Nodes (16): Method, ApiResponse, ApiTest, Client, Duration, Error, HashMap, HeaderMap (+8 more)

### Community 60 - "StoredFile"
Cohesion: 0.11
Nodes (7): DateTime, EntityRepoMeta, Id, Option, PersistentEntity, Utc, StoredFile

### Community 61 - "ImageCompressorService"
Cohesion: 0.11
Nodes (17): DynamicImage, ImageFormat, CompressionResult, format_to_string(), ImageCompressionError, ImageCompressorService, Default, Display (+9 more)

### Community 62 - "bucket.rs"
Cohesion: 0.07
Nodes (9): BucketId, AsRef, Deref, Display, Formatter, From, FromStr, Target (+1 more)

### Community 63 - "Thumbnail"
Cohesion: 0.13
Nodes (9): DateTime, EntityRepoMeta, HashMap, Option, PersistentEntity, Utc, Value, Thumbnail (+1 more)

### Community 64 - "CommandHandler"
Cohesion: 0.12
Nodes (26): AccessLogCommand, CreateAccessLogCommand, CreateAccessLogHandler, CreateAccessLogHandler<R>, DeleteAccessLogCommand, DeleteAccessLogHandler, DeleteAccessLogHandler<R>, Arc (+18 more)

### Community 65 - "Self"
Cohesion: 0.11
Nodes (9): ConversionError, ConversionJobBuilder, Err, HashMap, Result, Self, String, Value (+1 more)

### Community 66 - "LocalStorage"
Cohesion: 0.15
Nodes (20): allows_dotdot_inside_filename(), local_object_path(), LocalStorage, make(), rejects_path_traversal(), roundtrip(), BucketResult, Bytes (+12 more)

### Community 67 - "TestSetupManager"
Cohesion: 0.14
Nodes (15): TestError, Default, HashMap, Into, PathBuf, Result, Self, String (+7 more)

### Community 68 - "QuotaStatus"
Cohesion: 0.16
Nodes (23): QuotaStatus, Default, Display, FromStr, CreateUserQuotaDto, PatchUserQuotaDto, ApplyUpdateDto, DateTime (+15 more)

### Community 69 - "ThumbnailSize"
Cohesion: 0.16
Nodes (23): Default, Display, FromStr, ThumbnailSize, CreateThumbnailDto, PatchThumbnailDto, ApplyUpdateDto, DateTime (+15 more)

### Community 70 - "UserQuota"
Cohesion: 0.14
Nodes (7): DateTime, EntityRepoMeta, Option, PersistentEntity, Utc, UserQuota, Uuid

### Community 71 - "VersionType"
Cohesion: 0.16
Nodes (23): Default, Display, FromStr, VersionType, CreateFileVersionDto, FileVersion, FileVersionListResponseDto, FileVersionResponseDto (+15 more)

### Community 72 - "file_upload_service.rs"
Cohesion: 0.13
Nodes (25): compute_checksum(), create_test_bucket(), create_test_quota(), FileUploadService, Arc, Bucket, Display, Error (+17 more)

### Community 73 - "BucketModule"
Cohesion: 0.12
Nodes (23): BucketModule, BucketModuleBuilder, AccessLogService, Arc, BucketService, ContentHashService, ConversionJobService, Default (+15 more)

### Community 74 - "CommonUtils"
Cohesion: 0.15
Nodes (14): ApiResponse, CommonUtils, HashMap, Into, Option, Result, Self, String (+6 more)

### Community 75 - "FileCommentAfterCreateHandler1"
Cohesion: 0.16
Nodes (15): FileCommentActionExecutor, FileCommentEventPublisher, FileCommentTriggerContext, FileCommentTriggerEvent, FileCommentTriggerRegistry, file_comment_trigger_registry(), FileCommentAfterCreateHandler1, FileCommentAfterCreateHandler2 (+7 more)

### Community 76 - "file_comment_commands.rs"
Cohesion: 0.13
Nodes (24): CreateFileCommentCommand, CreateFileCommentHandler, CreateFileCommentHandler<R>, DeleteFileCommentCommand, DeleteFileCommentHandler, DeleteFileCommentHandler<R>, FileCommentCommand, Arc (+16 more)

### Community 77 - "CreateFileShareCommand"
Cohesion: 0.13
Nodes (24): CreateFileShareCommand, CreateFileShareHandler, CreateFileShareHandler<R>, DeleteFileShareCommand, DeleteFileShareHandler, DeleteFileShareHandler<R>, FileShareCommand, Arc (+16 more)

### Community 78 - "CreateUploadSessionCommand"
Cohesion: 0.13
Nodes (24): CreateUploadSessionCommand, CreateUploadSessionHandler, CreateUploadSessionHandler<R>, DeleteUploadSessionCommand, DeleteUploadSessionHandler, DeleteUploadSessionHandler<R>, Arc, DateTime (+16 more)

### Community 79 - "FileVersion"
Cohesion: 0.14
Nodes (9): FileVersion, DateTime, EntityRepoMeta, HashMap, Option, PersistentEntity, Utc, Value (+1 more)

### Community 80 - "routes.rs"
Cohesion: 0.15
Nodes (30): access_log_routes(), bucket_routes(), configure_routes(), content_hash_routes(), conversion_job_routes(), file_comment_routes(), file_lock_routes(), file_share_routes() (+22 more)

### Community 81 - "serving.rs"
Cohesion: 0.14
Nodes (27): Identity, BucketError, build_mode_response(), error_response_masks_internal_details(), fixture_config(), fixture_file(), lookup_by_key(), redirect_mode_returns_302_to_presigned_url() (+19 more)

### Community 82 - "Database Migrations for bucket"
Cohesion: 0.07
Nodes (29): 001_create_bucket_table.sql, 002_create_bucket_events_table.sql, 003_create_bucket_audit_table.sql, 004_create_bucket_indexes_and_triggers.sql, 005_create_bucket_views.sql, Audit Context, Common Issues, Database Migrations for bucket (+21 more)

### Community 83 - "conversion_job_commands.rs"
Cohesion: 0.13
Nodes (23): ConversionJobCommand, CreateConversionJobCommand, CreateConversionJobHandler, CreateConversionJobHandler<R>, DeleteConversionJobCommand, DeleteConversionJobHandler, DeleteConversionJobHandler<R>, Arc (+15 more)

### Community 84 - "file_lock_commands.rs"
Cohesion: 0.13
Nodes (23): CreateFileLockCommand, CreateFileLockHandler, CreateFileLockHandler<R>, DeleteFileLockCommand, DeleteFileLockHandler, DeleteFileLockHandler<R>, FileLockCommand, Arc (+15 more)

### Community 85 - "file_version_commands.rs"
Cohesion: 0.13
Nodes (23): CreateFileVersionCommand, CreateFileVersionHandler, CreateFileVersionHandler<R>, DeleteFileVersionCommand, DeleteFileVersionHandler, DeleteFileVersionHandler<R>, FileVersionCommand, Arc (+15 more)

### Community 86 - "processing_job_commands.rs"
Cohesion: 0.13
Nodes (23): CreateProcessingJobCommand, CreateProcessingJobHandler, CreateProcessingJobHandler<R>, DeleteProcessingJobCommand, DeleteProcessingJobHandler, DeleteProcessingJobHandler<R>, ProcessingJobCommand, Arc (+15 more)

### Community 87 - "CreateStoredFileCommand"
Cohesion: 0.13
Nodes (23): CreateStoredFileCommand, CreateStoredFileHandler, CreateStoredFileHandler<R>, DeleteStoredFileCommand, DeleteStoredFileHandler, DeleteStoredFileHandler<R>, Arc, DateTime (+15 more)

### Community 88 - "thumbnail_commands.rs"
Cohesion: 0.13
Nodes (23): CreateThumbnailCommand, CreateThumbnailHandler, CreateThumbnailHandler<R>, DeleteThumbnailCommand, DeleteThumbnailHandler, DeleteThumbnailHandler<R>, Arc, DateTime (+15 more)

### Community 89 - "user_quota_commands.rs"
Cohesion: 0.13
Nodes (23): CreateUserQuotaCommand, CreateUserQuotaHandler, CreateUserQuotaHandler<R>, DeleteUserQuotaCommand, DeleteUserQuotaHandler, DeleteUserQuotaHandler<R>, Arc, DateTime (+15 more)

### Community 90 - "conversion_job_handler.rs"
Cohesion: 0.23
Nodes (27): cancel_pending_transition(), cancel_running_transition(), complete_transition(), ConversionJobError, create_conversion_job_read_routes(), create_conversion_job_routes(), create_conversion_job_transition_routes(), create_conversion_job_write_routes() (+19 more)

### Community 91 - "Bucket Configuration Thresholds"
Cohesion: 0.07
Nodes (29): 1.1 Blocked File Extensions, 1.2 Magic Byte Detection, 1.3 Heuristic Detection, 1.4 Threat Level Actions, 1. Virus Scanner Service, 2.1 Default Configuration, 2.2 Thumbnail Generation, 2.3 Format Auto-Conversion (+21 more)

### Community 92 - "bucket_commands.rs"
Cohesion: 0.14
Nodes (22): BucketCommand, CreateBucketCommand, CreateBucketHandler, CreateBucketHandler<R>, DeleteBucketCommand, DeleteBucketHandler, DeleteBucketHandler<R>, Arc (+14 more)

### Community 93 - "content_hash_commands.rs"
Cohesion: 0.13
Nodes (22): ContentHashCommand, CreateContentHashCommand, CreateContentHashHandler, CreateContentHashHandler<R>, DeleteContentHashCommand, DeleteContentHashHandler, DeleteContentHashHandler<R>, Arc (+14 more)

### Community 94 - "file_access_log_commands.rs"
Cohesion: 0.14
Nodes (22): CreateFileAccessLogCommand, CreateFileAccessLogHandler, CreateFileAccessLogHandler<R>, DeleteFileAccessLogCommand, DeleteFileAccessLogHandler, DeleteFileAccessLogHandler<R>, FileAccessLogCommand, Arc (+14 more)

### Community 95 - "bucket_queries.rs"
Cohesion: 0.13
Nodes (21): BucketQuery, GetBucketByIdHandler, GetBucketByIdHandler<R>, GetBucketByIdQuery, GetBucketBySlugHandler, GetBucketBySlugHandler<R>, GetBucketBySlugQuery, ListBucketHandler (+13 more)

### Community 96 - "file_lock_queries.rs"
Cohesion: 0.13
Nodes (21): FileLockQuery, GetFileLockByFileIdHandler, GetFileLockByFileIdHandler<R>, GetFileLockByFileIdQuery, GetFileLockByIdHandler, GetFileLockByIdHandler<R>, GetFileLockByIdQuery, ListFileLockHandler (+13 more)

### Community 97 - "ListFileShareQuery"
Cohesion: 0.13
Nodes (21): FileShareQuery, GetFileShareByIdHandler, GetFileShareByIdHandler<R>, GetFileShareByIdQuery, GetFileShareByTokenHandler, GetFileShareByTokenHandler<R>, GetFileShareByTokenQuery, ListFileShareHandler (+13 more)

### Community 98 - "file_version_queries.rs"
Cohesion: 0.13
Nodes (21): FileVersionQuery, GetFileVersionByIdHandler, GetFileVersionByIdHandler<R>, GetFileVersionByIdQuery, GetFileVersionByStorageKeyHandler, GetFileVersionByStorageKeyHandler<R>, GetFileVersionByStorageKeyQuery, ListFileVersionHandler (+13 more)

### Community 99 - "thumbnail_queries.rs"
Cohesion: 0.13
Nodes (21): GetThumbnailByIdHandler, GetThumbnailByIdHandler<R>, GetThumbnailByIdQuery, GetThumbnailByStorageKeyHandler, GetThumbnailByStorageKeyHandler<R>, GetThumbnailByStorageKeyQuery, ListThumbnailHandler, ListThumbnailHandler<R> (+13 more)

### Community 100 - "user_quota_queries.rs"
Cohesion: 0.13
Nodes (21): GetUserQuotaByIdHandler, GetUserQuotaByIdHandler<R>, GetUserQuotaByIdQuery, GetUserQuotaByUserIdHandler, GetUserQuotaByUserIdHandler<R>, GetUserQuotaByUserIdQuery, ListUserQuotaHandler, ListUserQuotaHandler<R> (+13 more)

### Community 101 - "ConversionJob"
Cohesion: 0.15
Nodes (8): ConversionJob, DateTime, EntityRepoMeta, Id, Option, PersistentEntity, Utc, Uuid

### Community 102 - "UploadSessionId"
Cohesion: 0.10
Nodes (12): AsRef, Deref, Display, Err, Formatter, From, FromStr, Result (+4 more)

### Community 103 - "UploadSessionBuilder"
Cohesion: 0.14
Nodes (7): HashMap, Self, String, Value, Vec, UploadSessionBuilder, Uuid

### Community 104 - "processing_job_handler.rs"
Cohesion: 0.22
Nodes (26): cancel_pending_transition(), cancel_running_transition(), complete_transition(), create_processing_job_read_routes(), create_processing_job_routes(), create_processing_job_transition_routes(), create_processing_job_write_routes(), create_protected_processing_job_routes() (+18 more)

### Community 105 - "stored_file_handler.rs"
Cohesion: 0.22
Nodes (26): complete_upload_transition(), create_protected_stored_file_routes(), create_stored_file_read_routes(), create_stored_file_routes(), create_stored_file_transition_routes(), create_stored_file_write_routes(), mark_safe_by_admin_transition(), mark_safe_transition() (+18 more)

### Community 106 - "InMemoryStorage"
Cohesion: 0.15
Nodes (15): DashMap, delete_is_idempotent(), Entry, InMemoryStorage, missing_key_yields_not_found(), presigned_urls_are_synthetic(), put_get_round_trip(), Arc (+7 more)

### Community 107 - "app_config.rs"
Cohesion: 0.11
Nodes (17): P, DatabaseConfig, default_log_format(), default_log_level(), FeatureFlags, LoggingConfig, ModuleConfig, ModuleSettings (+9 more)

### Community 108 - "content_hash_queries.rs"
Cohesion: 0.14
Nodes (20): ContentHashQuery, GetContentHashByHashHandler, GetContentHashByHashHandler<R>, GetContentHashByHashQuery, GetContentHashByIdHandler, GetContentHashByIdHandler<R>, GetContentHashByIdQuery, ListContentHashHandler (+12 more)

### Community 109 - "FileCommentBuilder"
Cohesion: 0.15
Nodes (7): FileCommentBuilder, HashMap, Self, String, Value, Vec, Uuid

### Community 110 - "FileComment"
Cohesion: 0.14
Nodes (7): FileComment, DateTime, EntityRepoMeta, Id, Option, PersistentEntity, Utc

### Community 111 - "FileLock"
Cohesion: 0.14
Nodes (7): FileLock, DateTime, EntityRepoMeta, Option, PersistentEntity, Utc, Vec

### Community 112 - "PaginatedResult"
Cohesion: 0.21
Nodes (8): PaginatedResult, MockRepository, Backbone, BackboneFilters, Option, PaginationParams, RepositoryResult, Vec

### Community 113 - "ConversionJobResponseDto"
Cohesion: 0.19
Nodes (20): ConversionJob, ConversionJobListResponseDto, ConversionJobResponseDto, ConversionJobSummaryDto, CreateConversionJobDto, PatchConversionJobDto, ApplyUpdateDto, ConversionJob (+12 more)

### Community 114 - "FileCommentResponseDto"
Cohesion: 0.20
Nodes (20): CreateFileCommentDto, FileComment, FileCommentListResponseDto, FileCommentResponseDto, FileCommentSummaryDto, PatchFileCommentDto, ApplyUpdateDto, DateTime (+12 more)

### Community 115 - "ProcessingJobResponseDto"
Cohesion: 0.19
Nodes (20): CreateProcessingJobDto, PatchProcessingJobDto, ProcessingJob, ProcessingJobListResponseDto, ProcessingJobResponseDto, ProcessingJobSummaryDto, ApplyUpdateDto, DateTime (+12 more)

### Community 116 - "StoredFileResponseDto"
Cohesion: 0.19
Nodes (20): CreateStoredFileDto, PatchStoredFileDto, ApplyUpdateDto, DateTime, From, FromCreateDto, Option, Self (+12 more)

### Community 117 - "UploadSessionResponseDto"
Cohesion: 0.20
Nodes (20): CreateUploadSessionDto, PatchUploadSessionDto, ApplyUpdateDto, DateTime, From, FromCreateDto, Option, Self (+12 more)

### Community 118 - "HttpServices"
Cohesion: 0.16
Nodes (27): bucket_routes(), configure_routes(), content_hash_routes(), conversion_job_routes(), file_comment_routes(), file_lock_routes(), file_share_routes(), HttpServices (+19 more)

### Community 119 - "T"
Cohesion: 0.10
Nodes (21): I, AuthExtractor, AuthzDecision, AuthzPolicy, DefaultOwnerOnlyPolicy, BucketError, FromRequestParts, Result (+13 more)

### Community 120 - "S3Storage"
Cohesion: 0.18
Nodes (15): PresigningConfig, Option, S3Config, ServingConfig, presign_url_contains_sigv4_query_params(), public_url_routes_public_prefix_only(), BucketResult, Bytes (+7 more)

### Community 121 - "QueryHandler"
Cohesion: 0.13
Nodes (21): FileAccessLogQuery, GetFileAccessLogByIdHandler, GetFileAccessLogByIdHandler<R>, GetFileAccessLogByIdQuery, ListFileAccessLogHandler, ListFileAccessLogHandler<R>, ListFileAccessLogQuery, Arc (+13 more)

### Community 122 - "BucketStatus"
Cohesion: 0.11
Nodes (18): BucketStatus, Default, Display, Err, Formatter, FromStr, Result, Self (+10 more)

### Community 123 - "ConversionStatus"
Cohesion: 0.11
Nodes (17): ConversionStatus, Default, Display, Err, Formatter, FromStr, Result, Self (+9 more)

### Community 124 - "FileLockId"
Cohesion: 0.13
Nodes (11): FileLockBuilder, FileLockId, AsRef, Deref, Display, From, FromStr, Self (+3 more)

### Community 125 - "FileStatus"
Cohesion: 0.12
Nodes (17): FileStatus, Default, Display, Err, Formatter, FromStr, Result, Self (+9 more)

### Community 126 - "JobStatus"
Cohesion: 0.11
Nodes (17): JobStatus, Default, Display, Err, Formatter, FromStr, Result, Self (+9 more)

### Community 127 - "AccessLogResponseDto"
Cohesion: 0.19
Nodes (19): AccessLog, AccessLogListResponseDto, AccessLogResponseDto, AccessLogSummaryDto, CreateAccessLogDto, PatchAccessLogDto, AccessLog, ApplyUpdateDto (+11 more)

### Community 128 - "ContentHashResponseDto"
Cohesion: 0.19
Nodes (19): ContentHash, ContentHashListResponseDto, ContentHashResponseDto, ContentHashSummaryDto, CreateContentHashDto, PatchContentHashDto, ApplyUpdateDto, ContentHash (+11 more)

### Community 129 - "AuthContext"
Cohesion: 0.25
Nodes (22): AuthContext, AuthContextExt, all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_thumbnail() (+14 more)

### Community 130 - "bucket_handler.rs"
Cohesion: 0.21
Nodes (24): archive_transition(), BucketError, create_bucket_read_routes(), create_bucket_routes(), create_bucket_transition_routes(), create_bucket_write_routes(), create_protected_bucket_routes(), delete_transition() (+16 more)

### Community 131 - "file_comment_handler.rs"
Cohesion: 0.21
Nodes (24): create_file_comment_read_routes(), create_file_comment_routes(), create_file_comment_transition_routes(), create_file_comment_write_routes(), create_protected_file_comment_routes(), delete_active_transition(), delete_resolved_transition(), edit_transition() (+16 more)

### Community 132 - "config/generated.rs"
Cohesion: 0.14
Nodes (21): ModuleConfig, BucketModuleConfig, DatabaseConfig, expand_env_vars(), FeaturesConfig, LoggingConfig, merge_yaml(), MetricsConfig (+13 more)

### Community 133 - "FileProcessingFlowInstance"
Cohesion: 0.15
Nodes (13): FileProcessingFlowExecutor<H>, FileProcessingFlowInstance, FlowError, DateTime, Into, Option, Result, String (+5 more)

### Community 134 - "ThumbnailBuilder"
Cohesion: 0.14
Nodes (7): Err, Formatter, Id, Result, Self, String, ThumbnailBuilder

### Community 135 - "UploadSession"
Cohesion: 0.15
Nodes (7): DateTime, EntityRepoMeta, Id, Option, PersistentEntity, Utc, UploadSession

### Community 136 - "VirusScanResult"
Cohesion: 0.19
Nodes (14): Option, Self, String, Value, Vec, test_allows_safe_files(), test_blocks_bat_extension(), test_blocks_exe_extension() (+6 more)

### Community 137 - "FileLockResponseDto"
Cohesion: 0.20
Nodes (18): CreateFileLockDto, FileLock, FileLockListResponseDto, FileLockResponseDto, FileLockSummaryDto, PatchFileLockDto, ApplyUpdateDto, DateTime (+10 more)

### Community 138 - "domain_tests.rs"
Cohesion: 0.13
Nodes (18): create_test_file(), test_check_invariants_invalid_size(), test_check_invariants_path_traversal(), test_is_accessible_active_file(), test_is_accessible_deleted_file(), test_is_accessible_quarantined_file(), test_is_safe_unscanned_file(), test_is_safe_with_high_threat_level() (+10 more)

### Community 139 - "require_permission"
Cohesion: 0.25
Nodes (24): Future, all(), has_all_permissions(), has_any_permission(), has_permission(), has_role(), require_create_file_access_log(), require_delete_file_access_log() (+16 more)

### Community 140 - "UploadMultipartTest"
Cohesion: 0.21
Nodes (11): RequestBuilder, _exhaustive_use(), Client, Default, Into, Option, Self, String (+3 more)

### Community 141 - "FileCommentId"
Cohesion: 0.11
Nodes (12): CommentError, FileCommentId, AsRef, Deref, Display, Err, Formatter, From (+4 more)

### Community 142 - "StorageBackend"
Cohesion: 0.12
Nodes (17): Default, Display, Err, Formatter, FromStr, Result, Self, StorageBackend (+9 more)

### Community 143 - "StoredFileId"
Cohesion: 0.10
Nodes (12): AsRef, Deref, Display, Err, Formatter, From, FromStr, Result (+4 more)

### Community 144 - "file_lock_handler.rs"
Cohesion: 0.20
Nodes (22): create_file_lock_read_routes(), create_file_lock_routes(), create_file_lock_transition_routes(), create_file_lock_write_routes(), create_protected_file_lock_routes(), expire_transition(), FileLockError, refresh_transition() (+14 more)

### Community 145 - "file_share_handler.rs"
Cohesion: 0.20
Nodes (22): create_file_share_read_routes(), create_file_share_routes(), create_file_share_transition_routes(), create_file_share_write_routes(), create_protected_file_share_routes(), exhaust_transition(), expire_transition(), FileShareError (+14 more)

### Community 146 - "TestResult"
Cohesion: 0.24
Nodes (8): G, TestResult, CrudTestConfig, GenericCrudTest, GenericCrudTest<G>, Self, String, Vec

### Community 147 - "ListAccessLogQuery"
Cohesion: 0.14
Nodes (18): AccessLogQuery, GetAccessLogByIdHandler, GetAccessLogByIdHandler<R>, GetAccessLogByIdQuery, ListAccessLogHandler, ListAccessLogHandler<R>, ListAccessLogQuery, Arc (+10 more)

### Community 148 - "ListConversionJobQuery"
Cohesion: 0.14
Nodes (18): ConversionJobQuery, GetConversionJobByIdHandler, GetConversionJobByIdHandler<R>, GetConversionJobByIdQuery, ListConversionJobHandler, ListConversionJobHandler<R>, ListConversionJobQuery, Arc (+10 more)

### Community 149 - "ListFileCommentQuery"
Cohesion: 0.14
Nodes (18): FileCommentQuery, GetFileCommentByIdHandler, GetFileCommentByIdHandler<R>, GetFileCommentByIdQuery, ListFileCommentHandler, ListFileCommentHandler<R>, ListFileCommentQuery, Arc (+10 more)

### Community 150 - "ListProcessingJobQuery"
Cohesion: 0.14
Nodes (18): GetProcessingJobByIdHandler, GetProcessingJobByIdHandler<R>, GetProcessingJobByIdQuery, ListProcessingJobHandler, ListProcessingJobHandler<R>, ListProcessingJobQuery, ProcessingJobQuery, Arc (+10 more)

### Community 151 - "ListStoredFileQuery"
Cohesion: 0.14
Nodes (18): GetStoredFileByIdHandler, GetStoredFileByIdHandler<R>, GetStoredFileByIdQuery, ListStoredFileHandler, ListStoredFileHandler<R>, ListStoredFileQuery, Arc, Default (+10 more)

### Community 152 - "ListUploadSessionQuery"
Cohesion: 0.14
Nodes (18): GetUploadSessionByIdHandler, GetUploadSessionByIdHandler<R>, GetUploadSessionByIdQuery, ListUploadSessionHandler, ListUploadSessionHandler<R>, ListUploadSessionQuery, Arc, Default (+10 more)

### Community 153 - "Self"
Cohesion: 0.17
Nodes (4): FileVersionBuilder, Id, Self, String

### Community 154 - "user_quota_handler.rs"
Cohesion: 0.18
Nodes (21): create_protected_user_quota_routes(), create_user_quota_read_routes(), create_user_quota_routes(), create_user_quota_transition_routes(), create_user_quota_write_routes(), exceed_transition(), restore_transition(), A (+13 more)

### Community 155 - "ObjectStorage"
Cohesion: 0.17
Nodes (18): FileMeta, FileService, Arc, BucketResult, Bytes, Option, Self, StoredFile (+10 more)

### Community 156 - "AccessAction"
Cohesion: 0.11
Nodes (15): AccessAction, Default, Display, Err, Formatter, FromStr, Result, Self (+7 more)

### Community 157 - "Backbone"
Cohesion: 0.25
Nodes (5): Backbone, Error, Result, S, SpecificationEvaluator

### Community 158 - "StubStorage"
Cohesion: 0.14
Nodes (14): BucketResult, Bytes, Clone, Duration, Option, Self, Url, ServingContext<I> (+6 more)

### Community 159 - "Bucket Integration Tests"
Cohesion: 0.09
Nodes (22): Best Practices, Bucket Integration Tests, Connection refused, Console Output, Core Components, Creating Custom Tests, Directory Structure, Environment Variables (+14 more)

### Community 160 - "ThumbnailId"
Cohesion: 0.11
Nodes (8): AsRef, Deref, Display, From, FromStr, Target, Uuid, ThumbnailId

### Community 161 - "DocumentPreviewService"
Cohesion: 0.22
Nodes (13): DocumentPreviewService, is_document(), Arc, Option, ProcessingJob, ProcessingJobRepository, Self, ServiceResult (+5 more)

### Community 162 - "VideoThumbnailService"
Cohesion: 0.21
Nodes (13): is_video(), Arc, Option, ProcessingJob, ProcessingJobRepository, Self, ServiceResult, StoredFileRepository (+5 more)

### Community 163 - "FileVersionId"
Cohesion: 0.11
Nodes (11): FileVersionId, AsRef, Deref, Display, Err, Formatter, From, FromStr (+3 more)

### Community 164 - "FileAccessLogResponseDto"
Cohesion: 0.24
Nodes (16): CreateFileAccessLogDto, FileAccessLog, FileAccessLogListResponseDto, FileAccessLogResponseDto, FileAccessLogSummaryDto, PatchFileAccessLogDto, DateTime, FileAccessLog (+8 more)

### Community 165 - "JwtTokenManager"
Cohesion: 0.23
Nodes (13): Algorithm, Claims, JwtTokenManager, Default, Into, Option, Result, Self (+5 more)

### Community 166 - "ExampleUser"
Cohesion: 0.15
Nodes (16): build_app(), build_app_merged(), ExamplePolicy, ExampleUser, BucketError, FromRequestParts, PgPool, Result (+8 more)

### Community 167 - "create_file_comment.rs"
Cohesion: 0.13
Nodes (18): CreateFileCommentInput, CreateFileCommentOutput, CreateFileCommentUseCase, Arc, DateTime, FileComment, FileCommentRepository, Input (+10 more)

### Community 168 - "CreateFileShareInput"
Cohesion: 0.13
Nodes (18): CreateFileShareInput, CreateFileShareOutput, CreateFileShareUseCase, Arc, DateTime, FileShare, FileShareRepository, Input (+10 more)

### Community 169 - "CreateUploadSessionInput"
Cohesion: 0.13
Nodes (18): CreateUploadSessionInput, CreateUploadSessionOutput, CreateUploadSessionUseCase, Arc, DateTime, Input, Option, Output (+10 more)

### Community 170 - "update_file_comment.rs"
Cohesion: 0.13
Nodes (18): Arc, DateTime, FileComment, FileCommentRepository, Input, Option, Output, Result (+10 more)

### Community 171 - "UpdateFileShareInput"
Cohesion: 0.13
Nodes (18): Arc, DateTime, FileShare, FileShareRepository, Input, Option, Output, Result (+10 more)

### Community 172 - "UpdateUploadSessionInput"
Cohesion: 0.13
Nodes (18): Arc, DateTime, Input, Option, Output, Result, Self, String (+10 more)

### Community 173 - ".http_routes"
Cohesion: 0.18
Nodes (12): BucketModule, RouterOptions, RouterOptions<A>, A, ArcAuthzPolicy, BucketResult, Into, Router (+4 more)

### Community 174 - "FileShareId"
Cohesion: 0.12
Nodes (10): FileShareId, AsRef, Deref, Display, Err, Formatter, FromStr, Result (+2 more)

### Community 175 - "http/auth/access_log_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_access_log(), require_delete_access_log(), require_empty_trash_access_log() (+12 more)

### Community 176 - "http/auth/bucket_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_bucket(), require_delete_bucket(), require_empty_trash_bucket() (+12 more)

### Community 177 - "http/auth/content_hash_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_content_hash(), require_delete_content_hash(), require_empty_trash_content_hash() (+12 more)

### Community 178 - "http/auth/conversion_job_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_conversion_job(), require_delete_conversion_job(), require_empty_trash_conversion_job() (+12 more)

### Community 179 - "http/auth/file_comment_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_file_comment(), require_delete_file_comment(), require_empty_trash_file_comment() (+12 more)

### Community 180 - "http/auth/file_lock_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_file_lock(), require_delete_file_lock(), require_empty_trash_file_lock() (+12 more)

### Community 181 - "http/auth/file_share_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_file_share(), require_delete_file_share(), require_empty_trash_file_share() (+12 more)

### Community 182 - "http/auth/file_version_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_file_version(), require_delete_file_version(), require_empty_trash_file_version() (+12 more)

### Community 183 - "http/auth/processing_job_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_processing_job(), require_delete_processing_job(), require_empty_trash_processing_job() (+12 more)

### Community 184 - "http/auth/stored_file_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_stored_file(), require_delete_stored_file(), require_empty_trash_stored_file() (+12 more)

### Community 185 - "http/auth/upload_session_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_upload_session(), require_delete_upload_session(), require_empty_trash_upload_session() (+12 more)

### Community 186 - "http/auth/user_quota_auth.rs"
Cohesion: 0.34
Nodes (20): all(), has_all_roles(), has_any_permission(), has_permission(), has_role(), require_create_user_quota(), require_delete_user_quota(), require_empty_trash_user_quota() (+12 more)

### Community 187 - "IntegrationRegistry"
Cohesion: 0.15
Nodes (14): Any, IntegrationError, IntegrationRegistry, ModuleAdapter, ModuleFacade, Arc, Default, Error (+6 more)

### Community 188 - "Breaking Changes"
Cohesion: 0.10
Nodes (20): 1. Database Migration, 2. Code Updates, 3. Test Updates, Behavior Changes, Breaking Changes, Bucket construction, Bucket Module - Migration Guide V2.0, Entity Field Renames (+12 more)

### Community 189 - "BulkOperationResult"
Cohesion: 0.14
Nodes (10): BulkOperationError, BulkOperationProgress, BulkOperationResult, BulkOperationResult<T>, DateTime, Option, Self, String (+2 more)

### Community 190 - "create_conversion_job.rs"
Cohesion: 0.14
Nodes (17): CreateConversionJobInput, CreateConversionJobOutput, CreateConversionJobUseCase, Arc, ConversionJob, ConversionJobRepository, DateTime, Input (+9 more)

### Community 191 - "CreateProcessingJobInput"
Cohesion: 0.14
Nodes (17): CreateProcessingJobInput, CreateProcessingJobOutput, CreateProcessingJobUseCase, Arc, DateTime, Input, Option, Output (+9 more)

### Community 192 - "CreateStoredFileInput"
Cohesion: 0.14
Nodes (17): CreateStoredFileInput, CreateStoredFileOutput, CreateStoredFileUseCase, Arc, DateTime, Input, Option, Output (+9 more)

### Community 193 - "create_user_quota.rs"
Cohesion: 0.14
Nodes (17): CreateUserQuotaInput, CreateUserQuotaOutput, CreateUserQuotaUseCase, Arc, DateTime, Input, Option, Output (+9 more)

### Community 194 - "update_access_log.rs"
Cohesion: 0.14
Nodes (17): AccessLog, AccessLogRepository, Arc, DateTime, Input, Option, Output, Result (+9 more)

### Community 195 - "update_content_hash.rs"
Cohesion: 0.14
Nodes (17): Arc, ContentHash, ContentHashRepository, DateTime, Input, Option, Output, Result (+9 more)

### Community 196 - "update_conversion_job.rs"
Cohesion: 0.14
Nodes (17): Arc, ConversionJob, ConversionJobRepository, DateTime, Input, Option, Output, Result (+9 more)

### Community 197 - "UpdateProcessingJobInput"
Cohesion: 0.14
Nodes (17): Arc, DateTime, Input, Option, Output, ProcessingJob, ProcessingJobRepository, Result (+9 more)

### Community 198 - "UpdateStoredFileInput"
Cohesion: 0.14
Nodes (17): Arc, DateTime, Input, Option, Output, Result, Self, StoredFile (+9 more)

### Community 199 - "update_user_quota.rs"
Cohesion: 0.14
Nodes (17): Arc, DateTime, Input, Option, Output, Result, Self, String (+9 more)

### Community 200 - "String"
Cohesion: 0.12
Nodes (9): LockError, Duration, Err, Formatter, HashMap, Id, Result, String (+1 more)

### Community 201 - "String"
Cohesion: 0.13
Nodes (8): Err, Formatter, HashMap, Id, Result, String, Value, Vec

### Community 202 - "Action"
Cohesion: 0.17
Nodes (12): FileSharePermissions, FileShareRoleRules, Default, HashMap, HashSet, Option, PermissionResult, String (+4 more)

### Community 203 - "create_test_session"
Cohesion: 0.10
Nodes (20): create_test_session(), UploadSession, test_add_part_all_chunks_uploaded(), test_add_part_duplicate(), test_add_part_expired_session(), test_add_part_success(), test_calculate_progress(), test_can_resume_completed() (+12 more)

### Community 204 - "CdnService"
Cohesion: 0.20
Nodes (12): CdnService, Arc, BucketRepository, DateTime, Option, Self, ServiceResult, StoredFile (+4 more)

### Community 205 - "create_access_log.rs"
Cohesion: 0.15
Nodes (16): CreateAccessLogInput, CreateAccessLogOutput, CreateAccessLogUseCase, AccessLog, AccessLogRepository, Arc, DateTime, Input (+8 more)

### Community 206 - "CreateBucketInput"
Cohesion: 0.15
Nodes (16): CreateBucketInput, CreateBucketOutput, CreateBucketUseCase, Arc, Bucket, BucketRepository, Input, Option (+8 more)

### Community 207 - "create_content_hash.rs"
Cohesion: 0.15
Nodes (16): CreateContentHashInput, CreateContentHashOutput, CreateContentHashUseCase, Arc, ContentHash, ContentHashRepository, DateTime, Input (+8 more)

### Community 208 - "create_file_lock.rs"
Cohesion: 0.15
Nodes (16): CreateFileLockInput, CreateFileLockOutput, CreateFileLockUseCase, Arc, DateTime, FileLock, FileLockRepository, Input (+8 more)

### Community 209 - "create_thumbnail.rs"
Cohesion: 0.15
Nodes (16): CreateThumbnailInput, CreateThumbnailOutput, CreateThumbnailUseCase, Arc, DateTime, Input, Option, Output (+8 more)

### Community 210 - "UpdateBucketInput"
Cohesion: 0.15
Nodes (16): Arc, Bucket, BucketRepository, Input, Option, Output, Result, Self (+8 more)

### Community 211 - "update_file_lock.rs"
Cohesion: 0.15
Nodes (16): Arc, DateTime, FileLock, FileLockRepository, Input, Option, Output, Result (+8 more)

### Community 212 - "update_file_version.rs"
Cohesion: 0.15
Nodes (16): Arc, DateTime, FileVersion, FileVersionRepository, Input, Option, Output, Result (+8 more)

### Community 213 - "update_thumbnail.rs"
Cohesion: 0.15
Nodes (16): Arc, DateTime, Input, Option, Output, Result, Self, String (+8 more)

### Community 214 - "create_content_hash_routes"
Cohesion: 0.19
Nodes (15): ContentHashError, create_content_hash_read_routes(), create_content_hash_routes(), create_content_hash_write_routes(), create_protected_content_hash_routes(), A, Arc, ContentHashService (+7 more)

### Community 215 - "Bucket Module"
Cohesion: 0.11
Nodes (18): `application.yml`, Architecture, Bucket Module, Configuration, Documentation index, Environment, File serving, File upload (multipart HTTP) (+10 more)

### Community 216 - "ConversionService"
Cohesion: 0.28
Nodes (10): ConversionService, Arc, ConversionJob, ConversionJobRepository, Option, Self, ServiceResult, StoredFileRepository (+2 more)

### Community 217 - "create_file_version.rs"
Cohesion: 0.16
Nodes (15): CreateFileVersionInput, CreateFileVersionOutput, CreateFileVersionUseCase, Arc, DateTime, FileVersion, FileVersionRepository, Input (+7 more)

### Community 218 - "ConversionJobId"
Cohesion: 0.14
Nodes (9): ConversionJobId, AsRef, Deref, Display, Formatter, From, FromStr, Target (+1 more)

### Community 219 - "ProcessingJobId"
Cohesion: 0.14
Nodes (9): ProcessingJobId, AsRef, Deref, Display, Err, Formatter, FromStr, Target (+1 more)

### Community 220 - "UserQuotaId"
Cohesion: 0.14
Nodes (9): QuotaExceeded, AsRef, Deref, Display, From, FromStr, Target, Uuid (+1 more)

### Community 221 - "access_log_handler.rs"
Cohesion: 0.20
Nodes (15): AccessLogError, create_access_log_read_routes(), create_access_log_routes(), create_access_log_write_routes(), create_protected_access_log_routes(), A, AccessLogService, Arc (+7 more)

### Community 222 - "Trait Abstraction Analysis for Bucket Entities"
Cohesion: 0.12
Nodes (17): 1. Entity Trait (Already Implemented), 1. Invariant Checking, 1. Macros (If Needed Later), 2. Accessibility Checks, 2. Builder Pattern Enhancement, 2. HasComputedFields Trait (Already Implemented), 3. Soft Delete Pattern, 4. Status-Based State Machine (+9 more)

### Community 223 - ".new"
Cohesion: 0.29
Nodes (9): AccessLogBulkOperations, AccessLogBulkOperations<R>, AccessLog, Arc, R, Result, Self, String (+1 more)

### Community 224 - ".new"
Cohesion: 0.29
Nodes (9): BucketBulkOperations, BucketBulkOperations<R>, Arc, Bucket, R, Result, Self, String (+1 more)

### Community 225 - ".new"
Cohesion: 0.29
Nodes (9): ContentHashBulkOperations, ContentHashBulkOperations<R>, Arc, ContentHash, R, Result, Self, String (+1 more)

### Community 226 - ".new"
Cohesion: 0.29
Nodes (9): ConversionJobBulkOperations, ConversionJobBulkOperations<R>, Arc, ConversionJob, R, Result, Self, String (+1 more)

### Community 227 - "BulkOperationConfig"
Cohesion: 0.24
Nodes (11): FileAccessLogBulkOperations, FileAccessLogBulkOperations<R>, Arc, FileAccessLog, R, Result, Self, String (+3 more)

### Community 228 - ".new"
Cohesion: 0.29
Nodes (9): FileCommentBulkOperations, FileCommentBulkOperations<R>, Arc, FileComment, R, Result, Self, String (+1 more)

### Community 229 - ".new"
Cohesion: 0.29
Nodes (9): FileLockBulkOperations, FileLockBulkOperations<R>, Arc, FileLock, R, Result, Self, String (+1 more)

### Community 230 - ".new"
Cohesion: 0.29
Nodes (9): FileShareBulkOperations, FileShareBulkOperations<R>, Arc, FileShare, R, Result, Self, String (+1 more)

### Community 231 - ".new"
Cohesion: 0.29
Nodes (9): FileVersionBulkOperations, FileVersionBulkOperations<R>, Arc, FileVersion, R, Result, Self, String (+1 more)

### Community 232 - ".new"
Cohesion: 0.29
Nodes (9): ProcessingJobBulkOperations, ProcessingJobBulkOperations<R>, Arc, ProcessingJob, R, Result, Self, String (+1 more)

### Community 233 - ".new"
Cohesion: 0.29
Nodes (9): Arc, R, Result, Self, StoredFile, String, Vec, StoredFileBulkOperations (+1 more)

### Community 234 - ".new"
Cohesion: 0.29
Nodes (9): Arc, R, Result, Self, String, Thumbnail, Vec, ThumbnailBulkOperations (+1 more)

### Community 235 - ".new"
Cohesion: 0.29
Nodes (9): Arc, R, Result, Self, String, UploadSession, Vec, UploadSessionBulkOperations (+1 more)

### Community 236 - ".new"
Cohesion: 0.29
Nodes (9): Arc, R, Result, Self, String, UserQuota, Vec, UserQuotaBulkOperations (+1 more)

### Community 237 - "DeduplicationService"
Cohesion: 0.21
Nodes (9): DeduplicationService, Arc, ContentHash, ContentHashRepository, Self, ServiceResult, StoredFileRepository, Uuid (+1 more)

### Community 238 - "LockingService"
Cohesion: 0.27
Nodes (9): LockingService, Arc, FileLock, FileLockRepository, Option, Self, ServiceResult, StoredFileRepository (+1 more)

### Community 239 - "AccessLogFilter"
Cohesion: 0.12
Nodes (12): AccessLogFilter, AccessLogPaginatedResult, AccessLogPaginationParams, AccessLogRepository, AccessLog, Option, Self, Send (+4 more)

### Community 240 - "BucketFilter"
Cohesion: 0.12
Nodes (12): BucketFilter, BucketPaginatedResult, BucketPaginationParams, BucketRepository, Bucket, Option, Self, Send (+4 more)

### Community 241 - "ConversionJobFilter"
Cohesion: 0.12
Nodes (12): ConversionJobFilter, ConversionJobPaginatedResult, ConversionJobPaginationParams, ConversionJobRepository, ConversionJob, Option, Self, Send (+4 more)

### Community 242 - "FileCommentFilter"
Cohesion: 0.12
Nodes (12): FileCommentFilter, FileCommentPaginatedResult, FileCommentPaginationParams, FileCommentRepository, FileComment, Option, Self, Send (+4 more)

### Community 243 - "FileShareFilter"
Cohesion: 0.12
Nodes (12): FileShareFilter, FileSharePaginatedResult, FileSharePaginationParams, FileShareRepository, FileShare, Option, Self, Send (+4 more)

### Community 244 - "FileVersionFilter"
Cohesion: 0.12
Nodes (12): FileVersionFilter, FileVersionPaginatedResult, FileVersionPaginationParams, FileVersionRepository, FileVersion, Option, Self, Send (+4 more)

### Community 245 - "ProcessingJobFilter"
Cohesion: 0.12
Nodes (12): ProcessingJobFilter, ProcessingJobPaginatedResult, ProcessingJobPaginationParams, ProcessingJobRepository, Option, ProcessingJob, Self, Send (+4 more)

### Community 246 - "StoredFileFilter"
Cohesion: 0.12
Nodes (12): Option, Self, Send, StoredFile, String, Sync, Uuid, Vec (+4 more)

### Community 247 - "ThumbnailFilter"
Cohesion: 0.12
Nodes (12): Option, Self, Send, String, Sync, Thumbnail, Uuid, Vec (+4 more)

### Community 248 - "UploadSessionFilter"
Cohesion: 0.12
Nodes (12): Option, Self, Send, String, Sync, UploadSession, Uuid, Vec (+4 more)

### Community 249 - "UserQuotaFilter"
Cohesion: 0.12
Nodes (12): Option, Self, Send, String, Sync, UserQuota, Uuid, Vec (+4 more)

### Community 250 - "access_log_projector.rs"
Cohesion: 0.22
Nodes (10): AccessLogEventHandler, AccessLogProjectionRepository, AccessLogProjector, AccessLogProjector<R>, Arc, R, Result, Self (+2 more)

### Community 251 - "bucket_projector.rs"
Cohesion: 0.22
Nodes (10): BucketEventHandler, BucketProjectionRepository, BucketProjector, BucketProjector<R>, Arc, R, Result, Self (+2 more)

### Community 252 - "content_hash_projector.rs"
Cohesion: 0.22
Nodes (10): ContentHashEventHandler, ContentHashProjectionRepository, ContentHashProjector, ContentHashProjector<R>, Arc, R, Result, Self (+2 more)

### Community 253 - "conversion_job_projector.rs"
Cohesion: 0.22
Nodes (10): ConversionJobEventHandler, ConversionJobProjectionRepository, ConversionJobProjector, ConversionJobProjector<R>, Arc, R, Result, Self (+2 more)

### Community 254 - "file_comment_projector.rs"
Cohesion: 0.22
Nodes (10): FileCommentEventHandler, FileCommentProjectionRepository, FileCommentProjector, FileCommentProjector<R>, Arc, R, Result, Self (+2 more)

### Community 255 - "file_lock_projector.rs"
Cohesion: 0.22
Nodes (10): FileLockEventHandler, FileLockProjectionRepository, FileLockProjector, FileLockProjector<R>, Arc, R, Result, Self (+2 more)

### Community 256 - "file_share_projector.rs"
Cohesion: 0.22
Nodes (10): FileShareEventHandler, FileShareProjectionRepository, FileShareProjector, FileShareProjector<R>, Arc, R, Result, Self (+2 more)

### Community 257 - "file_version_projector.rs"
Cohesion: 0.22
Nodes (10): FileVersionEventHandler, FileVersionProjectionRepository, FileVersionProjector, FileVersionProjector<R>, Arc, R, Result, Self (+2 more)

### Community 258 - "processing_job_projector.rs"
Cohesion: 0.22
Nodes (10): ProcessingJobEventHandler, ProcessingJobProjectionRepository, ProcessingJobProjector, ProcessingJobProjector<R>, Arc, R, Result, Self (+2 more)

### Community 259 - "stored_file_projector.rs"
Cohesion: 0.22
Nodes (10): Arc, R, Result, Self, Send, Sync, StoredFileEventHandler, StoredFileProjectionRepository (+2 more)

### Community 260 - "thumbnail_projector.rs"
Cohesion: 0.22
Nodes (10): Arc, R, Result, Self, Send, Sync, ThumbnailEventHandler, ThumbnailProjectionRepository (+2 more)

### Community 261 - "upload_session_projector.rs"
Cohesion: 0.22
Nodes (10): Arc, R, Result, Self, Send, Sync, UploadSessionEventHandler, UploadSessionProjectionRepository (+2 more)

### Community 262 - "user_quota_projector.rs"
Cohesion: 0.22
Nodes (10): Arc, R, Result, Self, Send, Sync, UserQuotaEventHandler, UserQuotaProjectionRepository (+2 more)

### Community 263 - "FileAccessLogRepository"
Cohesion: 0.17
Nodes (13): DeleteFileAccessLogInput, DeleteFileAccessLogOutput, DeleteFileAccessLogUseCase, Arc, Input, Output, Result, Self (+5 more)

### Community 264 - "list_access_log.rs"
Cohesion: 0.19
Nodes (13): ListAccessLogInput, ListAccessLogOutput, ListAccessLogUseCase, AccessLog, AccessLogRepository, Arc, Input, Option (+5 more)

### Community 265 - "list_bucket.rs"
Cohesion: 0.19
Nodes (13): ListBucketInput, ListBucketOutput, ListBucketUseCase, Arc, Bucket, BucketRepository, Input, Option (+5 more)

### Community 266 - "list_content_hash.rs"
Cohesion: 0.19
Nodes (13): ListContentHashInput, ListContentHashOutput, ListContentHashUseCase, Arc, ContentHash, ContentHashRepository, Input, Option (+5 more)

### Community 267 - "list_conversion_job.rs"
Cohesion: 0.19
Nodes (13): ListConversionJobInput, ListConversionJobOutput, ListConversionJobUseCase, Arc, ConversionJob, ConversionJobRepository, Input, Option (+5 more)

### Community 268 - "list_file_comment.rs"
Cohesion: 0.19
Nodes (13): ListFileCommentInput, ListFileCommentOutput, ListFileCommentUseCase, Arc, FileComment, FileCommentRepository, Input, Option (+5 more)

### Community 269 - "list_file_lock.rs"
Cohesion: 0.19
Nodes (13): ListFileLockInput, ListFileLockOutput, ListFileLockUseCase, Arc, FileLock, FileLockRepository, Input, Option (+5 more)

### Community 270 - "list_file_share.rs"
Cohesion: 0.19
Nodes (13): ListFileShareInput, ListFileShareOutput, ListFileShareUseCase, Arc, FileShare, FileShareRepository, Input, Option (+5 more)

### Community 271 - "list_file_version.rs"
Cohesion: 0.19
Nodes (13): ListFileVersionInput, ListFileVersionOutput, ListFileVersionUseCase, Arc, FileVersion, FileVersionRepository, Input, Option (+5 more)

### Community 272 - "list_processing_job.rs"
Cohesion: 0.19
Nodes (13): ListProcessingJobInput, ListProcessingJobOutput, ListProcessingJobUseCase, Arc, Input, Option, Output, ProcessingJob (+5 more)

### Community 273 - "list_stored_file.rs"
Cohesion: 0.19
Nodes (13): ListStoredFileInput, ListStoredFileOutput, ListStoredFileUseCase, Arc, Input, Option, Output, Result (+5 more)

### Community 274 - "list_thumbnail.rs"
Cohesion: 0.19
Nodes (13): ListThumbnailInput, ListThumbnailOutput, ListThumbnailUseCase, Arc, Input, Option, Output, Result (+5 more)

### Community 275 - "list_upload_session.rs"
Cohesion: 0.19
Nodes (13): ListUploadSessionInput, ListUploadSessionOutput, ListUploadSessionUseCase, Arc, Input, Option, Output, Result (+5 more)

### Community 276 - "list_user_quota.rs"
Cohesion: 0.19
Nodes (13): ListUserQuotaInput, ListUserQuotaOutput, ListUserQuotaUseCase, Arc, Input, Option, Output, Result (+5 more)

### Community 277 - "BucketPermissions"
Cohesion: 0.19
Nodes (9): BucketPermissions, BucketRoleRules, Default, HashMap, HashSet, Option, PermissionResult, String (+1 more)

### Community 278 - "ContentHashFilter"
Cohesion: 0.12
Nodes (11): ContentHashFilter, ContentHashPaginatedResult, ContentHashPaginationParams, ContentHashRepository, ContentHash, Option, Self, Send (+3 more)

### Community 279 - "repositories/file_lock_repository.rs"
Cohesion: 0.13
Nodes (11): FileLockFilter, FileLockPaginatedResult, FileLockPaginationParams, FileLockRepository, FileLock, Option, Self, Send (+3 more)

### Community 280 - "FileAccessLogSpecification"
Cohesion: 0.28
Nodes (9): AndFileAccessLogSpec, FileAccessLogSpecification, NotFileAccessLogSpec, OrFileAccessLogSpec, Box, FileAccessLog, Self, Send (+1 more)

### Community 281 - "FileLockRepository"
Cohesion: 0.18
Nodes (11): FileLockRepository, Deref, FileLock, GenericCrudRepository, Option, PgPool, Result, Self (+3 more)

### Community 282 - "UserQuotaRepository"
Cohesion: 0.18
Nodes (11): Deref, GenericCrudRepository, Option, PgPool, Result, Self, SoftDelete, Target (+3 more)

### Community 283 - "Seeder"
Cohesion: 0.20
Nodes (7): Default, PgPool, Result, Self, SeedFileShareSeeder, Seeder, SeederType

### Community 284 - "create_test_quota"
Cohesion: 0.12
Nodes (16): create_quota(), create_test_quota(), String, UserQuota, test_add_usage_exceeds_quota(), test_add_usage_success(), test_exactly_at_limit(), test_exactly_at_warning_threshold() (+8 more)

### Community 285 - "create_test_job"
Cohesion: 0.12
Nodes (16): create_test_job(), ProcessingJob, test_can_retry_failed_max_retries(), test_can_retry_failed_with_retries(), test_can_retry_pending_job(), test_cancel(), test_check_invariants_completed_no_completed_at(), test_check_invariants_retry_exceeds_max() (+8 more)

### Community 286 - "create_test_share"
Cohesion: 0.12
Nodes (16): create_test_share(), FileShare, test_can_access_password_share(), test_can_access_public_link(), test_can_access_user_share(), test_has_downloads_remaining_no_limit(), test_has_downloads_remaining_with_limit(), test_is_expired_future_expiry() (+8 more)

### Community 287 - "TestDataGenerator"
Cohesion: 0.17
Nodes (10): BucketApiTest, BucketTestData, Default, Self, Value, Vec, test_bucket_crud(), Send (+2 more)

### Community 288 - "presentation/dto/mod.rs"
Cohesion: 0.17
Nodes (10): ApiError, ApiError, ApiResponse, ApiResponse<T>, PaginationParams, Into, Option, Self (+2 more)

### Community 289 - "get_access_log.rs"
Cohesion: 0.20
Nodes (12): GetAccessLogInput, GetAccessLogOutput, GetAccessLogUseCase, AccessLog, AccessLogRepository, Arc, Input, Output (+4 more)

### Community 290 - "get_bucket.rs"
Cohesion: 0.20
Nodes (12): GetBucketInput, GetBucketOutput, GetBucketUseCase, Arc, Bucket, BucketRepository, Input, Output (+4 more)

### Community 291 - "get_content_hash.rs"
Cohesion: 0.20
Nodes (12): GetContentHashInput, GetContentHashOutput, GetContentHashUseCase, Arc, ContentHash, ContentHashRepository, Input, Output (+4 more)

### Community 292 - "get_conversion_job.rs"
Cohesion: 0.20
Nodes (12): GetConversionJobInput, GetConversionJobOutput, GetConversionJobUseCase, Arc, ConversionJob, ConversionJobRepository, Input, Output (+4 more)

### Community 293 - "get_file_comment.rs"
Cohesion: 0.20
Nodes (12): GetFileCommentInput, GetFileCommentOutput, GetFileCommentUseCase, Arc, FileComment, FileCommentRepository, Input, Output (+4 more)

### Community 294 - "get_file_lock.rs"
Cohesion: 0.20
Nodes (12): GetFileLockInput, GetFileLockOutput, GetFileLockUseCase, Arc, FileLock, FileLockRepository, Input, Output (+4 more)

### Community 295 - "get_file_share.rs"
Cohesion: 0.20
Nodes (12): GetFileShareInput, GetFileShareOutput, GetFileShareUseCase, Arc, FileShare, FileShareRepository, Input, Output (+4 more)

### Community 296 - "get_file_version.rs"
Cohesion: 0.20
Nodes (12): GetFileVersionInput, GetFileVersionOutput, GetFileVersionUseCase, Arc, FileVersion, FileVersionRepository, Input, Output (+4 more)

### Community 297 - "get_processing_job.rs"
Cohesion: 0.20
Nodes (12): GetProcessingJobInput, GetProcessingJobOutput, GetProcessingJobUseCase, Arc, Input, Output, ProcessingJob, ProcessingJobRepository (+4 more)

### Community 298 - "get_stored_file.rs"
Cohesion: 0.20
Nodes (12): GetStoredFileInput, GetStoredFileOutput, GetStoredFileUseCase, Arc, Input, Output, Result, Self (+4 more)

### Community 299 - "get_thumbnail.rs"
Cohesion: 0.20
Nodes (12): GetThumbnailInput, GetThumbnailOutput, GetThumbnailUseCase, Arc, Input, Output, Result, Self (+4 more)

### Community 300 - "get_upload_session.rs"
Cohesion: 0.20
Nodes (12): GetUploadSessionInput, GetUploadSessionOutput, GetUploadSessionUseCase, Arc, Input, Output, Result, Self (+4 more)

### Community 301 - "get_user_quota.rs"
Cohesion: 0.20
Nodes (12): GetUserQuotaInput, GetUserQuotaOutput, GetUserQuotaUseCase, Arc, Input, Output, Result, Self (+4 more)

### Community 302 - "list_file_access_log.rs"
Cohesion: 0.19
Nodes (12): ListFileAccessLogInput, ListFileAccessLogOutput, ListFileAccessLogUseCase, Arc, FileAccessLog, Input, Option, Output (+4 more)

### Community 303 - "AuditMetadata"
Cohesion: 0.17
Nodes (6): AuditMetadata, DateTime, Option, Self, Utc, Uuid

### Community 304 - "BucketRepository"
Cohesion: 0.18
Nodes (10): BucketRepository, Bucket, Deref, GenericCrudRepository, Option, PgPool, Result, Self (+2 more)

### Community 305 - "ContentHashRepository"
Cohesion: 0.18
Nodes (10): ContentHashRepository, ContentHash, Deref, GenericCrudRepository, Option, PgPool, Result, Self (+2 more)

### Community 306 - "FileShareRepository"
Cohesion: 0.18
Nodes (10): FileShareRepository, Deref, FileShare, GenericCrudRepository, Option, PgPool, Result, Self (+2 more)

### Community 307 - "FileVersionRepository"
Cohesion: 0.18
Nodes (10): FileVersionRepository, Deref, FileVersion, GenericCrudRepository, Option, PgPool, Result, Self (+2 more)

### Community 308 - "ThumbnailRepository"
Cohesion: 0.18
Nodes (10): Deref, GenericCrudRepository, Option, PgPool, Result, Self, SoftDelete, Target (+2 more)

### Community 309 - "create_test_hash"
Cohesion: 0.13
Nodes (15): create_test_hash(), ContentHash, test_age_days(), test_can_delete(), test_check_invariants_empty_hash(), test_check_invariants_negative_ref_count(), test_check_invariants_zero_size(), test_days_since_last_reference() (+7 more)

### Community 311 - "Bucket File Storage System - Technical Domain Documentation"
Cohesion: 0.14
Nodes (14): 10.1 File Upload Specification, 10.2 File Access Specification, 10.3 Share Validity Specification, 10. Specifications, 16. Conclusion, 1.1 Purpose, 1.2 Architecture Principles, 1. Introduction (+6 more)

### Community 312 - "File Serving"
Cohesion: 0.14
Nodes (14): Authentication & authorization, `AuthExtractor`, `AuthzPolicy<Identity>`, Backends shipped, Backward compatibility, Buffering, Configuration, File Serving (+6 more)

### Community 313 - "delete_access_log.rs"
Cohesion: 0.21
Nodes (11): DeleteAccessLogInput, DeleteAccessLogOutput, DeleteAccessLogUseCase, AccessLogRepository, Arc, Input, Output, Result (+3 more)

### Community 314 - "delete_bucket.rs"
Cohesion: 0.21
Nodes (11): DeleteBucketInput, DeleteBucketOutput, DeleteBucketUseCase, Arc, BucketRepository, Input, Output, Result (+3 more)

### Community 315 - "delete_content_hash.rs"
Cohesion: 0.21
Nodes (11): DeleteContentHashInput, DeleteContentHashOutput, DeleteContentHashUseCase, Arc, ContentHashRepository, Input, Output, Result (+3 more)

### Community 316 - "delete_conversion_job.rs"
Cohesion: 0.21
Nodes (11): DeleteConversionJobInput, DeleteConversionJobOutput, DeleteConversionJobUseCase, Arc, ConversionJobRepository, Input, Output, Result (+3 more)

### Community 317 - "delete_file_comment.rs"
Cohesion: 0.21
Nodes (11): DeleteFileCommentInput, DeleteFileCommentOutput, DeleteFileCommentUseCase, Arc, FileCommentRepository, Input, Output, Result (+3 more)

### Community 318 - "delete_file_lock.rs"
Cohesion: 0.21
Nodes (11): DeleteFileLockInput, DeleteFileLockOutput, DeleteFileLockUseCase, Arc, FileLockRepository, Input, Output, Result (+3 more)

### Community 319 - "delete_file_share.rs"
Cohesion: 0.21
Nodes (11): DeleteFileShareInput, DeleteFileShareOutput, DeleteFileShareUseCase, Arc, FileShareRepository, Input, Output, Result (+3 more)

### Community 320 - "delete_file_version.rs"
Cohesion: 0.21
Nodes (11): DeleteFileVersionInput, DeleteFileVersionOutput, DeleteFileVersionUseCase, Arc, FileVersionRepository, Input, Output, Result (+3 more)

### Community 321 - "delete_processing_job.rs"
Cohesion: 0.21
Nodes (11): DeleteProcessingJobInput, DeleteProcessingJobOutput, DeleteProcessingJobUseCase, Arc, Input, Output, ProcessingJobRepository, Result (+3 more)

### Community 322 - "delete_stored_file.rs"
Cohesion: 0.21
Nodes (11): DeleteStoredFileInput, DeleteStoredFileOutput, DeleteStoredFileUseCase, Arc, Input, Output, Result, Self (+3 more)

### Community 323 - "delete_thumbnail.rs"
Cohesion: 0.21
Nodes (11): DeleteThumbnailInput, DeleteThumbnailOutput, DeleteThumbnailUseCase, Arc, Input, Output, Result, Self (+3 more)

### Community 324 - "delete_upload_session.rs"
Cohesion: 0.21
Nodes (11): DeleteUploadSessionInput, DeleteUploadSessionOutput, DeleteUploadSessionUseCase, Arc, Input, Output, Result, Self (+3 more)

### Community 325 - "delete_user_quota.rs"
Cohesion: 0.21
Nodes (11): DeleteUserQuotaInput, DeleteUserQuotaOutput, DeleteUserQuotaUseCase, Arc, Input, Output, Result, Self (+3 more)

### Community 326 - "get_file_access_log.rs"
Cohesion: 0.20
Nodes (11): GetFileAccessLogInput, GetFileAccessLogOutput, GetFileAccessLogUseCase, Arc, FileAccessLog, Input, Output, Result (+3 more)

### Community 327 - "update_file_access_log.rs"
Cohesion: 0.20
Nodes (12): Arc, DateTime, FileAccessLog, Option, Self, String, UseCase, Utc (+4 more)

### Community 328 - "file_processing_workflow.rs"
Cohesion: 0.19
Nodes (11): FileProcessingFlowExecutor, FileProcessingFlowStatus, FileProcessingFlowStep, FileProcessingStepHandler, Arc, H, Self, Send (+3 more)

### Community 329 - "ProcessingStatus"
Cohesion: 0.18
Nodes (8): ProcessingStatus, Default, Display, Err, Formatter, FromStr, Result, Self

### Community 330 - "ThreatLevel"
Cohesion: 0.18
Nodes (8): Default, Display, Err, Formatter, FromStr, Result, Self, ThreatLevel

### Community 331 - "UploadStatus"
Cohesion: 0.18
Nodes (8): Default, Display, Err, Formatter, FromStr, Result, Self, UploadStatus

### Community 333 - "StoredFilePermissions"
Cohesion: 0.23
Nodes (8): Default, HashMap, HashSet, PermissionResult, String, Vec, StoredFilePermissions, StoredFileRoleRules

### Community 334 - "StatsService"
Cohesion: 0.27
Nodes (6): Default, Result, Self, Uuid, StatsService, test_stats_service_creation()

### Community 335 - "FileCommentProjection"
Cohesion: 0.25
Nodes (10): FileCommentProjection, FileCommentSummary, DateTime, Option, Self, String, Utc, Uuid (+2 more)

### Community 336 - "FileShareProjection"
Cohesion: 0.25
Nodes (10): FileShareProjection, FileShareSummary, DateTime, Option, Self, String, Utc, Uuid (+2 more)

### Community 337 - "UploadSessionProjection"
Cohesion: 0.25
Nodes (10): DateTime, Option, Self, String, Utc, Uuid, Value, Vec (+2 more)

### Community 338 - "create_test_lock"
Cohesion: 0.14
Nodes (14): create_test_lock(), FileLock, test_can_refresh_expired_lock(), test_can_refresh_valid_lock(), test_check_invariants_expires_before_locked(), test_is_expired(), test_is_owned_by(), test_is_valid_active_not_expired() (+6 more)

### Community 340 - "FileAccessLogApiTest"
Cohesion: 0.22
Nodes (7): FileAccessLogApiTest, FileAccessLogTestData, Default, Self, Value, Vec, test_file_access_log_crud()

### Community 342 - "Metaphor Domain Module"
Cohesion: 0.15
Nodes (13): Anti-patterns, Common tasks, Deeper knowledge (load on demand), Four-layer folder cheatsheet, Golden path, graphify, Key files to read before editing, Metaphor Domain Module (+5 more)

### Community 343 - "Business Requirements Document (BRD)"
Cohesion: 0.15
Nodes (13): 12.1 Technology Constraints, 12.2 Operational Constraints, 12. Technical Constraints, 1.1 Document Control, 1.2 Revision History, 1.3 Document Purpose, 1. Document Information, 4.1 Internal Stakeholders (+5 more)

### Community 344 - "CommentStatus"
Cohesion: 0.19
Nodes (8): CommentStatus, Default, Display, Err, Formatter, FromStr, Result, Self

### Community 345 - "LockStatus"
Cohesion: 0.19
Nodes (8): LockStatus, Default, Display, Err, Formatter, FromStr, Result, Self

### Community 346 - "SharePermission"
Cohesion: 0.19
Nodes (8): Default, Display, Err, Formatter, FromStr, Result, Self, SharePermission

### Community 347 - "ShareType"
Cohesion: 0.19
Nodes (8): Default, Display, Err, Formatter, FromStr, Result, Self, ShareType

### Community 348 - "ProcessingJobType"
Cohesion: 0.19
Nodes (8): ProcessingJobType, Default, Display, Err, Formatter, FromStr, Result, Self

### Community 349 - "services.rs"
Cohesion: 0.18
Nodes (9): ExportSummary, BucketQueryService, BucketQueryServiceImpl, BucketQueryServiceImpl<R>, Arc, R, Self, Send (+1 more)

### Community 350 - "AccessLogProjection"
Cohesion: 0.27
Nodes (9): AccessLogProjection, AccessLogSummary, DateTime, Option, Self, String, Utc, Uuid (+1 more)

### Community 351 - "FileVersionProjection"
Cohesion: 0.28
Nodes (9): FileVersionProjection, FileVersionSummary, DateTime, Option, Self, String, Utc, Uuid (+1 more)

### Community 352 - "ThumbnailProjection"
Cohesion: 0.27
Nodes (9): DateTime, Option, Self, String, Utc, Uuid, Value, ThumbnailProjection (+1 more)

### Community 353 - "UserQuotaProjection"
Cohesion: 0.27
Nodes (9): DateTime, Option, Self, String, Utc, Uuid, Value, UserQuotaProjection (+1 more)

### Community 354 - "SeedAccessLogSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedAccessLogSeeder

### Community 355 - "SeedBucketSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedBucketSeeder

### Community 356 - "SeedContentHashSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedContentHashSeeder

### Community 357 - "SeedConversionJobSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedConversionJobSeeder

### Community 358 - "SeedFileAccessLogSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedFileAccessLogSeeder

### Community 359 - "SeedFileCommentSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedFileCommentSeeder

### Community 360 - "SeedFileLockSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedFileLockSeeder

### Community 361 - "SeedFileVersionSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedFileVersionSeeder

### Community 362 - "SeedProcessingJobSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedProcessingJobSeeder

### Community 363 - "SeedStoredFileSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedStoredFileSeeder

### Community 364 - "SeedThumbnailSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedThumbnailSeeder

### Community 365 - "SeedUploadSessionSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedUploadSessionSeeder

### Community 366 - "SeedUserQuotaSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedUserQuotaSeeder

### Community 367 - "create_test_conversion"
Cohesion: 0.15
Nodes (13): create_test_conversion(), ConversionJob, test_check_invariants_completed_no_result(), test_check_invariants_failed_no_error(), test_check_invariants_invalid_progress(), test_complete(), test_conversion_lifecycle(), test_fail() (+5 more)

### Community 368 - "AccessLogApiTest"
Cohesion: 0.22
Nodes (7): AccessLogApiTest, AccessLogTestData, Default, Self, Value, Vec, test_access_log_crud()

### Community 369 - "ContentHashApiTest"
Cohesion: 0.22
Nodes (7): ContentHashApiTest, ContentHashTestData, Default, Self, Value, Vec, test_content_hash_crud()

### Community 370 - "ConversionJobApiTest"
Cohesion: 0.22
Nodes (7): ConversionJobApiTest, ConversionJobTestData, Default, Self, Value, Vec, test_conversion_job_crud()

### Community 371 - "FileCommentApiTest"
Cohesion: 0.22
Nodes (7): FileCommentApiTest, FileCommentTestData, Default, Self, Value, Vec, test_file_comment_crud()

### Community 372 - "FileLockApiTest"
Cohesion: 0.22
Nodes (7): FileLockApiTest, FileLockTestData, Default, Self, Value, Vec, test_file_lock_crud()

### Community 373 - "FileShareApiTest"
Cohesion: 0.22
Nodes (7): FileShareApiTest, FileShareTestData, Default, Self, Value, Vec, test_file_share_crud()

### Community 374 - "FileVersionApiTest"
Cohesion: 0.22
Nodes (7): FileVersionApiTest, FileVersionTestData, Default, Self, Value, Vec, test_file_version_crud()

### Community 375 - "ProcessingJobApiTest"
Cohesion: 0.22
Nodes (7): ProcessingJobApiTest, ProcessingJobTestData, Default, Self, Value, Vec, test_processing_job_crud()

### Community 376 - "StoredFileApiTest"
Cohesion: 0.22
Nodes (7): Default, Self, Value, Vec, StoredFileApiTest, StoredFileTestData, test_stored_file_crud()

### Community 377 - "ThumbnailApiTest"
Cohesion: 0.22
Nodes (7): Default, Self, Value, Vec, test_thumbnail_crud(), ThumbnailApiTest, ThumbnailTestData

### Community 378 - "UploadSessionApiTest"
Cohesion: 0.22
Nodes (7): Default, Self, Value, Vec, test_upload_session_crud(), UploadSessionApiTest, UploadSessionTestData

### Community 379 - "UserQuotaApiTest"
Cohesion: 0.22
Nodes (7): Default, Self, Value, Vec, test_user_quota_crud(), UserQuotaApiTest, UserQuotaTestData

### Community 380 - "Bucket Module Specification"
Cohesion: 0.17
Nodes (12): 13.1 External APIs, 13.2 Webhooks (Outgoing), 13.3 Webhooks (Incoming), 13. Integrations, 15.1 Reference Data, 15. Seed Data, 2.1 External Dependencies, 2. Module Dependencies (+4 more)

### Community 381 - "5.2 Entity Definitions"
Cohesion: 0.17
Nodes (12): 5.1 Entity List Summary, 5.2 Entity Definitions, 5. Entities (Data Models), Entity: `Bucket`, Entity: `ContentHash`, Entity: `FileComment`, Entity: `FileLock`, Entity: `FileShare` (+4 more)

### Community 382 - "create_file_access_log.rs"
Cohesion: 0.24
Nodes (9): CreateFileAccessLogOutput, CreateFileAccessLogUseCase, Arc, FileAccessLog, Input, Output, Result, Self (+1 more)

### Community 383 - "ConversionType"
Cohesion: 0.21
Nodes (8): ConversionType, Default, Display, Err, Formatter, FromStr, Result, Self

### Community 384 - "HashAlgorithm"
Cohesion: 0.21
Nodes (8): HashAlgorithm, Default, Display, Err, Formatter, FromStr, Result, Self

### Community 386 - "FieldRestriction"
Cohesion: 0.24
Nodes (8): Option, FieldRestriction, PermissionChecker, PermissionError, PermissionRule, HashSet, Option, String

### Community 387 - "AccessLogDomainService"
Cohesion: 0.33
Nodes (5): AccessLogDomainService, AccessLog, Default, Result, Self

### Community 388 - "BucketDomainService"
Cohesion: 0.32
Nodes (5): BucketDomainService, Bucket, Default, Result, Self

### Community 389 - "ContentHashDomainService"
Cohesion: 0.32
Nodes (5): ContentHashDomainService, ContentHash, Default, Result, Self

### Community 390 - "ConversionJobDomainService"
Cohesion: 0.32
Nodes (5): ConversionJobDomainService, ConversionJob, Default, Result, Self

### Community 391 - "FileAccessLogDomainService"
Cohesion: 0.32
Nodes (5): FileAccessLogDomainService, Default, FileAccessLog, Result, Self

### Community 392 - "FileCommentDomainService"
Cohesion: 0.32
Nodes (5): FileCommentDomainService, Default, FileComment, Result, Self

### Community 393 - "FileLockDomainService"
Cohesion: 0.33
Nodes (5): FileLockDomainService, Default, FileLock, Result, Self

### Community 394 - "FileShareDomainService"
Cohesion: 0.32
Nodes (5): FileShareDomainService, Default, FileShare, Result, Self

### Community 395 - "FileVersionDomainService"
Cohesion: 0.32
Nodes (5): FileVersionDomainService, Default, FileVersion, Result, Self

### Community 396 - "ProcessingJobDomainService"
Cohesion: 0.33
Nodes (5): ProcessingJobDomainService, Default, ProcessingJob, Result, Self

### Community 397 - "StoredFileDomainService"
Cohesion: 0.32
Nodes (5): Default, Result, Self, StoredFile, StoredFileDomainService

### Community 398 - "ThumbnailDomainService"
Cohesion: 0.32
Nodes (5): Default, Result, Self, Thumbnail, ThumbnailDomainService

### Community 399 - "UploadSessionDomainService"
Cohesion: 0.32
Nodes (5): Default, Result, Self, UploadSession, UploadSessionDomainService

### Community 400 - "UserQuotaDomainService"
Cohesion: 0.32
Nodes (5): Default, Result, Self, UserQuota, UserQuotaDomainService

### Community 401 - "FileAccessLogProjection"
Cohesion: 0.29
Nodes (8): FileAccessLogProjection, FileAccessLogSummary, DateTime, Option, Self, String, Utc, Uuid

### Community 402 - "FileLockProjection"
Cohesion: 0.29
Nodes (8): FileLockProjection, FileLockSummary, DateTime, Option, Self, Utc, Uuid, Value

### Community 403 - "collect_proto_files"
Cohesion: 0.44
Nodes (10): collect_proto_files(), collect_proto_files_recursive(), compile_protos(), main(), Box, Error, Path, Result (+2 more)

### Community 404 - "commands/mod.rs"
Cohesion: 0.24
Nodes (7): C, CommandBus, Default, H, Output, Result, Self

### Community 405 - "4. Use Cases"
Cohesion: 0.18
Nodes (11): 4. Use Cases, UC-001: Upload File with Processing, UC-002: Share File with Permissions, UC-003: Access Shared File, UC-004: Process File (Media Conversion), UC-005: Lock File for Editing, UC-006: Convert File Format, UC-007: Multipart Upload (+3 more)

### Community 406 - "6. Enums (Value Types)"
Cohesion: 0.18
Nodes (11): 6. Enums (Value Types), Enum: `BucketStatus`, Enum: `BucketType`, Enum: `FileStatus`, Enum: `JobStatus`, Enum: `ProcessingJobType`, Enum: `SharePermission`, Enum: `ShareType` (+3 more)

### Community 407 - "queries/mod.rs"
Cohesion: 0.24
Nodes (7): Q, QueryBus, Default, H, Output, Result, Self

### Community 408 - "AccessLogRepository"
Cohesion: 0.22
Nodes (8): AccessLogRepository, AccessLog, Deref, GenericCrudRepository, PgPool, Self, SoftDelete, Target

### Community 409 - "ConversionJobRepository"
Cohesion: 0.22
Nodes (8): ConversionJobRepository, ConversionJob, Deref, GenericCrudRepository, PgPool, Self, SoftDelete, Target

### Community 410 - "FileCommentRepository"
Cohesion: 0.22
Nodes (8): FileCommentRepository, Deref, FileComment, GenericCrudRepository, PgPool, Self, SoftDelete, Target

### Community 411 - "ProcessingJobRepository"
Cohesion: 0.22
Nodes (8): ProcessingJobRepository, Deref, GenericCrudRepository, PgPool, ProcessingJob, Self, SoftDelete, Target

### Community 412 - "StoredFileRepository"
Cohesion: 0.22
Nodes (8): Deref, GenericCrudRepository, PgPool, Self, SoftDelete, StoredFile, Target, StoredFileRepository

### Community 413 - "UploadSessionRepository"
Cohesion: 0.22
Nodes (8): Deref, GenericCrudRepository, PgPool, Self, SoftDelete, Target, UploadSession, UploadSessionRepository

### Community 414 - "create_file_with_owner"
Cohesion: 0.18
Nodes (11): create_file_with_owner(), Option, StoredFile, Uuid, test_file_with_owner_context(), test_file_without_owner_context(), test_multiple_files_sort_order(), test_owner_context_different_entities() (+3 more)

### Community 415 - "7.1 User Stories"
Cohesion: 0.20
Nodes (10): 7.1 User Stories, 7.2 Use Cases, 7. User Stories & Use Cases, UC-001: Upload File Workflow, UC-002: Create Share Workflow, UC-003: Access Shared File Workflow, US-001: File Upload, US-002: File Sharing (+2 more)

### Community 416 - "9.1 Core Entities"
Cohesion: 0.20
Nodes (10): 9.1.1 StoredFile, 9.1.2 Bucket, 9.1.3 FileShare, 9.1.4 UserQuota, 9.1 Core Entities, 9.2.1 FileVersion, 9.2.2 Thumbnail, 9.2.3 AccessLog (+2 more)

### Community 417 - "4. Value Objects and Enums"
Cohesion: 0.20
Nodes (10): 4.1 FileStatus Enum, 4.2 BucketStatus Enum, 4.3 BucketType Enum, 4.4 StorageBackend Enum, 4.5 ThreatLevel Enum, 4.6 ShareType Enum, 4.7 SharePermission Enum, 4.8 AccessAction Enum (+2 more)

### Community 418 - "Bucket API Documentation"
Cohesion: 0.20
Nodes (9): API Base URL, Authentication, Bucket API Documentation, Common Response Formats, Error Response, Files, Generating API Documentation, HTTP Status Codes (+1 more)

### Community 419 - "BucketError"
Cohesion: 0.31
Nodes (7): ParseError, backbone_core::ServiceError, BucketError, Error, From, Self, String

### Community 420 - "Bucket usage — step by step"
Cohesion: 0.20
Nodes (10): 1. Create a bucket, 2. Read a bucket, 3. List & search buckets, 4. Update a bucket, 5. Soft-delete, restore, empty trash, 6. State transitions (lock / unlock / archive / …), 7. Bulk & upsert, 8. Counts (+2 more)

### Community 421 - ".new"
Cohesion: 0.33
Nodes (7): Self, test_admin_role(), test_create_permissions(), test_guest_role(), test_super_admin_role(), test_unknown_role(), test_user_role()

### Community 422 - ".new"
Cohesion: 0.33
Nodes (7): Self, test_admin_role(), test_create_permissions(), test_guest_role(), test_super_admin_role(), test_unknown_role(), test_user_role()

### Community 423 - ".new"
Cohesion: 0.33
Nodes (7): Self, test_admin_role(), test_create_permissions(), test_guest_role(), test_super_admin_role(), test_unknown_role(), test_user_role()

### Community 424 - "utils/mod.rs"
Cohesion: 0.29
Nodes (6): generate_share_token(), generate_storage_key(), String, slugify(), test_generate_share_token(), test_generate_storage_key()

### Community 425 - "create_test_bucket"
Cohesion: 0.20
Nodes (10): create_test_bucket(), Bucket, test_can_upload_file_too_large(), test_can_upload_inactive_bucket(), test_can_upload_mime_type_restriction(), test_can_upload_valid(), test_check_invariants_valid(), test_is_accessible() (+2 more)

### Community 426 - "5. Functional Requirements"
Cohesion: 0.22
Nodes (9): 5.1 File Management (FR-FM), 5.2 Bucket Management (FR-BM), 5.3 Security Features (FR-SEC), 5.4 Image Processing (FR-IMG), 5.5 File Sharing (FR-SHR), 5.6 Quota Management (FR-QTA), 5.7 Versioning (FR-VER), 5.8 Access Logging (FR-LOG) (+1 more)

### Community 427 - "Bucket Module - Implementation Plan V2.0"
Cohesion: 0.22
Nodes (9): 1.1 Project Overview, 1.2 V2.0 Enhancement Goals, 1.3 Success Criteria, 1. Executive Summary, 5.1 Generation Commands, 5.2 Expected Generated Files, 5. Code Generation Tasks, Bucket Module - Implementation Plan V2.0 (+1 more)

### Community 428 - "13.1 PostgreSQL Tables"
Cohesion: 0.22
Nodes (9): 13.1 PostgreSQL Tables, 13. Database Schema, access_logs Table, buckets Table, file_shares Table, file_versions Table, stored_files Table, thumbnails Table (+1 more)

### Community 429 - "8.1 Commands (Write Operations)"
Cohesion: 0.22
Nodes (9): 8.1 Commands (Write Operations), 8.2 Queries (Read Operations), 8. Use Cases (CQRS), Bucket Commands, Bucket Queries, File Commands, File Queries, Quota Queries (+1 more)

### Community 430 - "2. Project Overview"
Cohesion: 0.25
Nodes (8): 2.1 Executive Summary, 2.2 Project Name & Branding, 2.3 Project Summary, 2.4 Business Goals & Objectives, 2.5 Business Value Proposition, 2. Project Overview, Primary Objectives (Must-Have), Quantifiable Benefits

### Community 431 - "State Machine: `StoredFile`"
Cohesion: 0.25
Nodes (8): 7. Entity Lifecycle (State Machines), State Diagram, State Machine: `ProcessingJob`, State Machine: `StoredFile`, States, States, Transitions, Transitions

### Community 432 - "8. Workflows (Multi-Step Processes)"
Cohesion: 0.25
Nodes (8): 8. Workflows (Multi-Step Processes), Steps, Steps, Steps, Workflow: `FileProcessingWorkflow`, Workflow: `FileUploadWorkflow`, Workflow: `MultipartUploadWorkflow`, Workflow Variables

### Community 433 - "12.2 Layer 2: Domain-Specific Endpoints"
Cohesion: 0.25
Nodes (8): 12.1 Layer 1: Backbone Generic CRUD, 12.2 Layer 2: Domain-Specific Endpoints, 12. API Endpoints, Admin Operations, Bucket Operations, File Operations, Quota Operations, Share Operations

### Community 434 - "3. Entities (Aggregate Roots)"
Cohesion: 0.25
Nodes (8): 3.1 StoredFile Entity, 3.2 Bucket Entity, 3.3 UserQuota Entity, 3.4 FileShare Entity, 3.5 FileVersion Entity, 3.6 Thumbnail Entity, 3.7 AccessLog Entity, 3. Entities (Aggregate Roots)

### Community 435 - "bench"
Cohesion: 0.43
Nodes (7): F, bench(), bench_batch_operations(), bench_domain_logic(), bench_entity_construction(), bench_invariant_checks(), bench_serialization()

### Community 436 - "4.1 New Schema Files to Create"
Cohesion: 0.29
Nodes (7): 4.1 New Schema Files to Create, File: `content_hash.model.yaml`, File: `conversion_job.model.yaml`, File: `file_comment.model.yaml`, File: `file_lock.model.yaml`, File: `processing_job.model.yaml`, File: `upload_session.model.yaml`

### Community 437 - "10. Services (Business Logic)"
Cohesion: 0.29
Nodes (7): 10. Services (Business Logic), Service: `CdnService`, Service: `DeduplicationService`, Service: `FileProcessingService`, Service: `LockingService`, Service: `QuotaEnforcementService`, Service: `ShareLinkService`

### Community 438 - ".from_str"
Cohesion: 0.29
Nodes (4): Err, Formatter, Result, Self

### Community 439 - ".from_str"
Cohesion: 0.29
Nodes (4): Err, Formatter, Result, Self

### Community 440 - ".from_str"
Cohesion: 0.29
Nodes (4): Err, Formatter, Result, Self

### Community 441 - "10. Implementation Checklist"
Cohesion: 0.33
Nodes (6): 10. Implementation Checklist, Code Generation, Custom Implementation, Documentation, Schema Creation, Testing

### Community 442 - "3. Implementation Phases"
Cohesion: 0.33
Nodes (6): 3. Implementation Phases, Phase 1: Schema Foundation (Week 1), Phase 2: Enhanced StoredFile (Week 1-2), Phase 3: Code Generation (Week 2), Phase 4: Custom Logic Implementation (Week 2-3), Phase 5: Testing & Documentation (Week 3-4)

### Community 443 - "9. Events (Domain Events)"
Cohesion: 0.33
Nodes (6): 9. Events (Domain Events), Event: `FileLocked`, Event: `FileProcessed`, Event: `FileUploaded`, Event: `QuotaExceeded`, Event: `ThreatDetected`

### Community 444 - "15. Implementation Checklist"
Cohesion: 0.33
Nodes (6): 15. Implementation Checklist, Phase 1: Core Entities, Phase 2: Domain Services, Phase 3: Database, Phase 4: API Layer, Phase 5: Integration

### Community 445 - "7. Domain Services"
Cohesion: 0.33
Nodes (6): 7.1 StorageService, 7.2 VirusScannerService, 7.3 ImageCompressorService, 7.4 FileUploadService, 7.5 AccessLoggerService, 7. Domain Services

### Community 446 - "9. Repositories"
Cohesion: 0.33
Nodes (6): 9.1 StoredFileRepository, 9.2 BucketRepository, 9.3 UserQuotaRepository, 9.4 FileShareRepository, 9.5 AccessLogRepository, 9. Repositories

### Community 447 - "AccessLogPolicy"
Cohesion: 0.47
Nodes (4): AccessLogPolicy, AccessLog, ResourceAction, ResourcePolicy

### Community 448 - "BucketPolicy"
Cohesion: 0.47
Nodes (4): BucketPolicy, Bucket, ResourceAction, ResourcePolicy

### Community 449 - "ContentHashPolicy"
Cohesion: 0.47
Nodes (4): ContentHashPolicy, ContentHash, ResourceAction, ResourcePolicy

### Community 450 - "ConversionJobPolicy"
Cohesion: 0.47
Nodes (4): ConversionJobPolicy, ConversionJob, ResourceAction, ResourcePolicy

### Community 451 - "FileCommentPolicy"
Cohesion: 0.47
Nodes (4): FileCommentPolicy, FileComment, ResourceAction, ResourcePolicy

### Community 452 - "FileLockPolicy"
Cohesion: 0.47
Nodes (4): FileLockPolicy, FileLock, ResourceAction, ResourcePolicy

### Community 453 - "FileSharePolicy"
Cohesion: 0.47
Nodes (4): FileSharePolicy, FileShare, ResourceAction, ResourcePolicy

### Community 454 - "FileVersionPolicy"
Cohesion: 0.47
Nodes (4): FileVersionPolicy, FileVersion, ResourceAction, ResourcePolicy

### Community 455 - "ProcessingJobPolicy"
Cohesion: 0.47
Nodes (4): ProcessingJobPolicy, ProcessingJob, ResourceAction, ResourcePolicy

### Community 456 - "StoredFilePolicy"
Cohesion: 0.47
Nodes (4): ResourceAction, ResourcePolicy, StoredFile, StoredFilePolicy

### Community 457 - "ThumbnailPolicy"
Cohesion: 0.47
Nodes (4): ResourceAction, ResourcePolicy, Thumbnail, ThumbnailPolicy

### Community 458 - "UploadSessionPolicy"
Cohesion: 0.47
Nodes (4): ResourceAction, ResourcePolicy, UploadSession, UploadSessionPolicy

### Community 459 - "UserQuotaPolicy"
Cohesion: 0.47
Nodes (4): ResourceAction, ResourcePolicy, UserQuota, UserQuotaPolicy

### Community 460 - "CreateFileAccessLogInput"
Cohesion: 0.33
Nodes (6): CreateFileAccessLogInput, DateTime, Option, String, Utc, Uuid

### Community 461 - "FileAccessLogFilter"
Cohesion: 0.40
Nodes (4): FileAccessLogFilter, Option, String, Uuid

### Community 462 - "6. Non-Functional Requirements"
Cohesion: 0.40
Nodes (5): 6.1 Performance Requirements, 6.2 Scalability Requirements, 6.3 Availability Requirements, 6.4 Security Requirements, 6. Non-Functional Requirements

### Community 463 - "4.2 Updates to Existing Schema Files"
Cohesion: 0.40
Nodes (5): 4.2 Updates to Existing Schema Files, 4. Schema Updates, Update: `bucket.model.yaml`, Update: `index.model.yaml`, Update: `stored_file.model.yaml`

### Community 464 - "11. API Requirements"
Cohesion: 0.40
Nodes (5): 11.1 Custom Endpoints, 11.2 Query Filters, 11.3 Sorting, 11.4 Pagination, 11. API Requirements

### Community 465 - "16. New Features (V2.0)"
Cohesion: 0.40
Nodes (5): 16.1 Media Processing Enhancements, 16.2 Collaboration Features, 16.3 Storage & Performance, 16.4 Schema Updates Required, 16. New Features (V2.0)

### Community 466 - "4. Storage Service"
Cohesion: 0.40
Nodes (5): 4.1 Directory Structure, 4.2 Storage Key Format, 4.3 Path Sanitization, 4.4 Trash Management, 4. Storage Service

### Community 467 - "14. Security Model"
Cohesion: 0.40
Nodes (5): 14.1 File Security Layers, 14.2 Blocked File Types, 14.3 Magic Byte Detection, 14.4 Threat Response Matrix, 14. Security Model

### Community 468 - "5. Domain Events"
Cohesion: 0.40
Nodes (5): 5.1 File Events, 5.2 Security Events, 5.3 Share Events, 5.4 Quota Events, 5. Domain Events

### Community 469 - "Bucket Module — Documentation"
Cohesion: 0.40
Nodes (5): Bucket Module — Documentation, Map, Module overview, Quick links by task, What's generated vs. hand-written

### Community 470 - "Workflows"
Cohesion: 0.40
Nodes (4): Example Workflow, Running Schema Generation, What are Workflows?, Workflows

### Community 471 - "Error"
Cohesion: 0.40
Nodes (3): Error, Option, Self

### Community 472 - "cli/mod.rs"
Cohesion: 0.60
Nodes (4): empty_trash(), list_trash(), Result, String

### Community 473 - "10. Integration Requirements"
Cohesion: 0.50
Nodes (4): 10.1 Internal Integrations, 10.2 External Integrations, 10.3 Events Published, 10. Integration Requirements

### Community 474 - "11. API Specifications"
Cohesion: 0.50
Nodes (4): 11.1 API Overview, 11.2 Standard Endpoints (per resource), 11.3 Custom Endpoints, 11. API Specifications

### Community 475 - "13. Security Requirements"
Cohesion: 0.50
Nodes (4): 13.1 Authentication & Authorization, 13.2 Data Security, 13.3 Threat Protection, 13. Security Requirements

### Community 476 - "14. Success Criteria"
Cohesion: 0.50
Nodes (4): 14.1 Functional Success, 14.2 Performance Success, 14.3 Business Success, 14. Success Criteria

### Community 477 - "3. Business Context"
Cohesion: 0.50
Nodes (4): 3.1 Current State Analysis, 3.2 Problem Statement, 3.3 Solution Overview, 3. Business Context

### Community 478 - "2. Current State Analysis"
Cohesion: 0.50
Nodes (4): 2.1 Existing Entities (8), 2.2 Existing Schema Files, 2.3 Gaps Identified, 2. Current State Analysis

### Community 479 - "6. Custom Logic Implementation"
Cohesion: 0.50
Nodes (4): 6.1 File Locking Service, 6.2 Deduplication Service, 6.3 Multipart Upload Handler, 6. Custom Logic Implementation

### Community 480 - "12. Authorization & Permissions"
Cohesion: 0.50
Nodes (4): 12.1 Roles, 12.2 Permission Matrix, 12.3 Row-Level Security, 12. Authorization & Permissions

### Community 481 - "14. Non-Functional Requirements"
Cohesion: 0.50
Nodes (4): 14.1 Performance, 14.2 Data Retention, 14.3 Audit Requirements, 14. Non-Functional Requirements

### Community 482 - "1. Module Overview"
Cohesion: 0.50
Nodes (4): 1.1 Basic Information, 1.2 Module Dependencies, 1.3 Business Objectives, 1. Module Overview

### Community 483 - "11. Workflows"
Cohesion: 0.50
Nodes (4): 11.1 File Upload Workflow, 11.2 File Version Cleanup Workflow, 11.3 Quota Warning Workflow, 11. Workflows

### Community 484 - "2. Domain-Driven Design Overview"
Cohesion: 0.50
Nodes (4): 2.1 Bounded Context, 2.2 Ubiquitous Language, 2.3 Aggregates, 2. Domain-Driven Design Overview

### Community 485 - ".execute"
Cohesion: 0.50
Nodes (3): Input, Output, Result

### Community 486 - "BucketDomainPolicy"
Cohesion: 0.67
Nodes (3): BucketDomainPolicy, Bucket, DomainPolicy

### Community 487 - "ConversionJobDomainPolicy"
Cohesion: 0.67
Nodes (3): ConversionJobDomainPolicy, ConversionJob, DomainPolicy

### Community 488 - "FileCommentDomainPolicy"
Cohesion: 0.67
Nodes (3): FileCommentDomainPolicy, DomainPolicy, FileComment

### Community 489 - "FileLockDomainPolicy"
Cohesion: 0.67
Nodes (3): FileLockDomainPolicy, DomainPolicy, FileLock

### Community 490 - "FileShareDomainPolicy"
Cohesion: 0.67
Nodes (3): FileShareDomainPolicy, DomainPolicy, FileShare

### Community 491 - "ProcessingJobDomainPolicy"
Cohesion: 0.67
Nodes (3): ProcessingJobDomainPolicy, DomainPolicy, ProcessingJob

### Community 492 - "StoredFileDomainPolicy"
Cohesion: 0.67
Nodes (3): DomainPolicy, StoredFile, StoredFileDomainPolicy

### Community 493 - "UploadSessionDomainPolicy"
Cohesion: 0.67
Nodes (3): DomainPolicy, UploadSession, UploadSessionDomainPolicy

### Community 494 - "UserQuotaDomainPolicy"
Cohesion: 0.67
Nodes (3): DomainPolicy, UserQuota, UserQuotaDomainPolicy

### Community 495 - "BucketEventMetadata"
Cohesion: 0.50
Nodes (3): BucketEventMetadata, Default, Self

### Community 496 - "extract_bearer_token"
Cohesion: 0.67
Nodes (3): extract_bearer_token(), B, Request

### Community 497 - "15. Assumptions & Dependencies"
Cohesion: 0.67
Nodes (3): 15.1 Assumptions, 15.2 Dependencies, 15. Assumptions & Dependencies

### Community 498 - "16. Risks & Mitigation"
Cohesion: 0.67
Nodes (3): 16.1 Technical Risks, 16.2 Business Risks, 16. Risks & Mitigation

### Community 499 - "17. Glossary & References"
Cohesion: 0.67
Nodes (3): 17.1 Glossary, 17.2 References, 17. Glossary & References

### Community 500 - "8. Data Requirements"
Cohesion: 0.67
Nodes (3): 8.1 Data Entities, 8.2 Data Retention, 8. Data Requirements

### Community 501 - "7. Testing Strategy"
Cohesion: 0.67
Nodes (3): 7.1 Unit Tests, 7.2 Integration Tests, 7. Testing Strategy

### Community 502 - "8. Migration Strategy"
Cohesion: 0.67
Nodes (3): 8.1 Database Migration, 8.2 Backward Compatibility, 8. Migration Strategy

### Community 503 - "9. Dependencies & Risks"
Cohesion: 0.67
Nodes (3): 9.1 External Dependencies, 9.2 Risk Mitigation, 9. Dependencies & Risks

### Community 505 - "gen_bytes"
Cohesion: 0.67
Nodes (3): Strategy, gen_bytes(), Value

## Knowledge Gaps
- **472 isolated node(s):** `DeleteAccessLogOutput`, `DeleteBucketOutput`, `DeleteContentHashOutput`, `DeleteConversionJobOutput`, `DeleteFileAccessLogOutput` (+467 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `T` connect `T` to `presentation/dto/mod.rs`, `TestSetupManager`, `StoredEvent`, `ApiTest`, `IntegrationRegistry`, `PaginatedResult`, `EventHandler`, `backbone_specifications.rs`, `BulkOperationResult`, `repositories/backbone_repository.rs`?**
  _High betweenness centrality (0.287) - this node is a cross-community bridge._
- **Why does `ApiResponse` connect `presentation/dto/mod.rs` to `T`?**
  _High betweenness centrality (0.191) - this node is a cross-community bridge._
- **Why does `AuditMetadata` connect `AuditMetadata` to `ContentHashResponseDto`, `AccessLog`, `UploadSession`, `ContentHash`, `FileLockResponseDto`, `FileCommentId`, `StoredFileId`, `ProcessingJob`, `FileShare`, `Bucket`, `events.rs`, `ThumbnailId`, `FileVersionId`, `FileShareId`, `BucketType`, `ShareStatus`, `StoredFile`, `bucket.rs`, `Thumbnail`, `QuotaStatus`, `ThumbnailSize`, `UserQuota`, `VersionType`, `FileVersion`, `ConversionJobId`, `ProcessingJobId`, `UserQuotaId`, `ConversionJob`, `UploadSessionId`, `FileComment`, `FileLock`, `ConversionJobResponseDto`, `FileCommentResponseDto`, `ProcessingJobResponseDto`, `StoredFileResponseDto`, `UploadSessionResponseDto`, `FileLockId`, `AccessLogResponseDto`?**
  _High betweenness centrality (0.170) - this node is a cross-community bridge._
- **What connects `DeleteAccessLogOutput`, `DeleteBucketOutput`, `DeleteContentHashOutput` to the rest of the system?**
  _472 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MockRepository` be split into smaller, more focused modules?**
  _Cohesion score 0.05786163522012579 - nodes in this community are weakly interconnected._
- **Should `Uuid` be split into smaller, more focused modules?**
  _Cohesion score 0.061107938320959454 - nodes in this community are weakly interconnected._
- **Should `BackboneId` be split into smaller, more focused modules?**
  _Cohesion score 0.04249947401641069 - nodes in this community are weakly interconnected._