//! Workflow Integration Tests for bucket V2.0
//!
//! Tests end-to-end business logic workflows using domain entities.
//! Validates that entity state machines and cross-entity interactions
//! work correctly for real-world scenarios.
//!
//! Run with: cargo test --package backbone-bucket --test workflow_tests

use chrono::{Duration, Utc};
use uuid::Uuid;

use backbone_bucket::domain::entity::*;

// ==========================================================================
// Test Helpers
// ==========================================================================

fn create_test_bucket() -> Bucket {
    Bucket::builder()
        .name("test-bucket".to_string())
        .slug("test-bucket".to_string())
        .owner_id(Uuid::new_v4())
        .bucket_type(BucketType::User)
        .status(BucketStatus::Active)
        .storage_backend(StorageBackend::Local)
        .root_path("/data/test-bucket".to_string())
        .file_count(0)
        .total_size_bytes(0)
        .max_file_size(100 * 1024 * 1024)
        .allowed_mime_types(vec![])
        .enable_cdn(true)
        .enable_versioning(true)
        .enable_deduplication(true)
        .build()
        .unwrap()
}

fn create_test_file(bucket_id: Uuid, owner_id: Uuid) -> StoredFile {
    StoredFile {
        id: Uuid::new_v4(),
        bucket_id,
        owner_id,
        path: "/docs/report.pdf".to_string(),
        original_name: "report.pdf".to_string(),
        size_bytes: 1024 * 1024,
        mime_type: "application/pdf".to_string(),
        checksum: Some("sha256:abc123".to_string()),
        is_compressed: false,
        original_size: None,
        compression_algorithm: None,
        is_scanned: false,
        scan_result: None,
        threat_level: None,
        has_thumbnail: false,
        thumbnail_path: None,
        has_video_thumbnail: false,
        has_document_preview: false,
        processing_status: None,
        content_hash_id: None,
        cdn_url: None,
        cdn_url_expires_at: None,
        owner_module: None,
        owner_entity: None,
        owner_entity_id: None,
        field_name: None,
        sort_order: 0,
        status: FileStatus::Active,
        storage_key: format!("files/{}", Uuid::new_v4()),
        version: 1,
        previous_version_id: None,
        download_count: 0,
        last_accessed_at: None,
        metadata: AuditMetadata::default(),
    }
}

fn create_test_video_file(bucket_id: Uuid, owner_id: Uuid) -> StoredFile {
    let mut file = create_test_file(bucket_id, owner_id);
    file.original_name = "video.mp4".to_string();
    file.mime_type = "video/mp4".to_string();
    file.size_bytes = 50 * 1024 * 1024;
    file.path = "/media/video.mp4".to_string();
    file
}

fn create_test_quota(user_id: Uuid) -> UserQuota {
    UserQuota::builder()
        .user_id(user_id)
        .limit_bytes(5 * 1024 * 1024 * 1024)
        .used_bytes(0)
        .file_count(0)
        .tier("free".to_string())
        .quota_status(QuotaStatus::Normal)
        .warning_threshold_percent(80)
        .peak_usage_bytes(0)
        .build()
        .unwrap()
}

// ==========================================================================
// Workflow: File Upload Lifecycle
// ==========================================================================

mod file_upload_lifecycle {
    use super::*;

    #[test]
    fn test_complete_upload_workflow() {
        // 1. Bucket has space
        let bucket = create_test_bucket();
        assert!(bucket.can_upload(1024 * 1024, "application/pdf"));

        // 2. User has quota
        let mut quota = create_test_quota(bucket.owner_id);
        assert!(quota.has_space_for(1024 * 1024));

        // 3. Create file
        let mut file = create_test_file(bucket.id, bucket.owner_id);
        assert_eq!(file.status, FileStatus::Active);
        assert!(!file.is_scanned);
        assert!(file.needs_processing());

        // 4. Update quota
        quota.add_usage(file.size_bytes).unwrap();
        assert_eq!(quota.used_bytes, file.size_bytes);

        // 5. File gets scanned - safe
        file.is_scanned = true;
        file.scan_result = Some(serde_json::json!("clean"));
        file.threat_level = Some(ThreatLevel::Safe);
        assert!(file.is_safe());
        assert!(!file.needs_processing());

        // 6. File is accessible
        assert!(file.is_accessible());

        // 7. Record access
        file.record_access();
        assert_eq!(file.download_count, 1);
        assert!(file.last_accessed_at.is_some());
    }

    #[test]
    fn test_upload_rejected_mime_type() {
        let bucket = Bucket::builder()
            .name("images-only".to_string())
            .slug("images-only".to_string())
            .owner_id(Uuid::new_v4())
            .root_path("/data/images".to_string())
            .allowed_mime_types(vec!["image/png".to_string(), "image/jpeg".to_string()])
            .build()
            .unwrap();

        assert!(bucket.can_upload(1024, "image/png"));
        assert!(!bucket.can_upload(1024, "application/pdf"));
    }

    #[test]
    fn test_upload_rejected_file_too_large() {
        let bucket = create_test_bucket();
        // max_file_size is 100MB
        assert!(!bucket.can_upload(200 * 1024 * 1024, "application/pdf"));
    }

    #[test]
    fn test_upload_rejected_quota_exceeded() {
        let mut quota = create_test_quota(Uuid::new_v4());
        quota.used_bytes = quota.limit_bytes;
        assert!(!quota.has_space_for(1));
    }

    #[test]
    fn test_upload_with_virus_detected() {
        let bucket = create_test_bucket();
        let mut file = create_test_file(bucket.id, bucket.owner_id);

        // Scan detects threat
        file.is_scanned = true;
        file.scan_result = Some(serde_json::json!("malware detected"));
        file.threat_level = Some(ThreatLevel::Critical);

        assert!(!file.is_safe());
        assert!(file.is_accessible()); // still accessible until quarantined

        // Quarantine the file
        file.quarantine(vec!["malware detected".to_string()]);
        assert_eq!(file.status, FileStatus::Quarantined);
        assert!(!file.is_accessible());
    }

    #[test]
    fn test_file_soft_delete_and_restore() {
        let bucket = create_test_bucket();
        let mut file = create_test_file(bucket.id, bucket.owner_id);
        let mut quota = create_test_quota(bucket.owner_id);
        quota.add_usage(file.size_bytes).unwrap();

        // Soft delete
        file.soft_delete();
        assert!(file.is_deleted());
        assert!(!file.is_accessible());

        // Quota decremented
        quota.subtract_usage(file.size_bytes);
        assert_eq!(quota.used_bytes, 0);

        // Restore
        file.restore();
        assert!(!file.is_deleted());
        assert!(file.is_accessible());

        // Quota re-added
        quota.add_usage(file.size_bytes).unwrap();
        assert_eq!(quota.used_bytes, file.size_bytes);
    }
}

// ==========================================================================
// Workflow: Multipart Upload Session
// ==========================================================================

mod multipart_upload_workflow {
    use super::*;

    fn create_upload_session(bucket_id: Uuid, user_id: Uuid) -> UploadSession {
        UploadSession::builder()
            .bucket_id(bucket_id)
            .user_id(user_id)
            .path("/uploads/large-file.zip".to_string())
            .filename("large-file.zip".to_string())
            .file_size(100 * 1024 * 1024)
            .chunk_size(5 * 1024 * 1024)
            .total_chunks(20)
            .uploaded_chunks(0)
            .status(UploadStatus::Initiated)
            .completed_parts(vec![])
            .storage_backend(StorageBackend::Local)
            .expires_at(Utc::now() + Duration::hours(24))
            .build()
            .unwrap()
    }

    #[test]
    fn test_multipart_upload_complete_flow() {
        let bucket = create_test_bucket();
        let mut session = create_upload_session(bucket.id, bucket.owner_id);

        assert_eq!(session.status, UploadStatus::Initiated);
        assert_eq!(session.uploaded_chunks, 0);
        assert_eq!(session.total_chunks, 20);
        assert!(!session.is_expired());
        assert!(!session.is_complete());
        assert_eq!(session.remaining_chunks(), 20);

        // Upload parts one by one
        for i in 1..=20 {
            session.add_part(i, format!("etag-{}", i)).unwrap();
        }

        assert_eq!(session.uploaded_chunks, 20);
        assert!(session.is_complete());
        assert_eq!(session.remaining_chunks(), 0);
        assert_eq!(session.calculate_progress(), 100);

        // Mark complete
        session.mark_complete().unwrap();
        assert_eq!(session.status, UploadStatus::Completed);
    }

    #[test]
    fn test_multipart_upload_partial_progress() {
        let bucket = create_test_bucket();
        let mut session = create_upload_session(bucket.id, bucket.owner_id);

        for i in 1..=10 {
            session.add_part(i, format!("etag-{}", i)).unwrap();
        }

        assert_eq!(session.uploaded_chunks, 10);
        assert!(!session.is_complete());
        assert_eq!(session.remaining_chunks(), 10);
        assert_eq!(session.calculate_progress(), 50);
        assert!(session.can_resume());
    }

    #[test]
    fn test_multipart_upload_expired_session() {
        let bucket = create_test_bucket();
        let mut session = create_upload_session(bucket.id, bucket.owner_id);
        session.expires_at = Utc::now() - Duration::hours(1);

        assert!(session.is_expired());
        assert!(!session.can_resume());
    }

    #[test]
    fn test_multipart_upload_failure() {
        let bucket = create_test_bucket();
        let mut session = create_upload_session(bucket.id, bucket.owner_id);

        session.add_part(1, "etag-1".to_string()).unwrap();
        session.add_part(2, "etag-2".to_string()).unwrap();

        session.mark_failed("Network timeout".to_string()).unwrap();
        assert_eq!(session.status, UploadStatus::Failed);
    }

    #[test]
    fn test_multipart_upload_invariants() {
        let bucket = create_test_bucket();
        let session = create_upload_session(bucket.id, bucket.owner_id);
        assert!(session.check_invariants().is_ok());

        // Invalid: uploaded > total
        let mut bad_session = create_upload_session(bucket.id, bucket.owner_id);
        bad_session.uploaded_chunks = 25;
        assert!(bad_session.check_invariants().is_err());
    }
}

// ==========================================================================
// Workflow: File Locking (Concurrent Edit Prevention)
// ==========================================================================

mod file_locking_workflow {
    use super::*;

    fn create_lock(file_id: Uuid, user_id: Uuid, minutes: i64) -> FileLock {
        let now = Utc::now();
        FileLock::new(file_id, user_id, now, now + Duration::minutes(minutes), LockStatus::Active)
    }

    #[test]
    fn test_lock_acquire_check_release() {
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let lock = create_lock(file_id, user_id, 30);

        assert!(!lock.is_expired());
        assert!(lock.is_valid());
        assert!(lock.is_owned_by(user_id));
        assert!(!lock.is_owned_by(Uuid::new_v4()));
    }

    #[test]
    fn test_lock_expired_auto_cleanup() {
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut lock = create_lock(file_id, user_id, 30);
        lock.expires_at = Utc::now() - Duration::minutes(5);

        assert!(lock.is_expired());
        assert!(!lock.is_valid());
    }

    #[test]
    fn test_lock_refresh() {
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut lock = create_lock(file_id, user_id, 5);
        assert!(lock.can_refresh());

        let before_refresh = lock.expires_at;
        lock.refresh(Duration::minutes(30)).unwrap();

        assert!(lock.expires_at > before_refresh);
        assert!(lock.refreshed_at.is_some());
        assert!(lock.is_valid());
    }

    #[test]
    fn test_lock_time_remaining() {
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let lock = create_lock(file_id, user_id, 30);
        let remaining = lock.time_remaining();
        assert!(remaining.num_minutes() >= 29);
    }

    #[test]
    fn test_lock_invariants() {
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let lock = create_lock(file_id, user_id, 30);
        assert!(lock.check_invariants().is_ok());
    }
}

// ==========================================================================
// Workflow: Content Deduplication
// ==========================================================================

mod deduplication_workflow {
    use super::*;

    fn create_content_hash(hash: &str, size: i64) -> ContentHash {
        ContentHash::builder()
            .hash(hash.to_string())
            .size_bytes(size)
            .storage_key(format!("content/{}", hash))
            .storage_backend(StorageBackend::Local)
            .reference_count(1)
            .first_uploaded_at(Utc::now())
            .last_referenced_at(Utc::now())
            .build()
            .unwrap()
    }

    #[test]
    fn test_dedup_new_content() {
        let hash = create_content_hash("sha256:abc123", 1024);

        assert_eq!(hash.reference_count, 1);
        assert!(!hash.is_unused());
        assert!(!hash.can_delete());
        assert_eq!(hash.storage_saved(), 0);
    }

    #[test]
    fn test_dedup_duplicate_detected() {
        let mut hash = create_content_hash("sha256:abc123", 1024 * 1024);

        hash.increment_reference().unwrap();
        assert_eq!(hash.reference_count, 2);
        assert_eq!(hash.storage_saved(), 1024 * 1024);

        hash.increment_reference().unwrap();
        assert_eq!(hash.reference_count, 3);
        assert_eq!(hash.storage_saved(), 2 * 1024 * 1024);
    }

    #[test]
    fn test_dedup_file_deletion_decrements_ref() {
        let mut hash = create_content_hash("sha256:abc123", 1024);
        hash.increment_reference().unwrap();
        hash.increment_reference().unwrap();
        assert_eq!(hash.reference_count, 3);

        let can_delete = hash.decrement_reference().unwrap();
        assert!(!can_delete);
        assert_eq!(hash.reference_count, 2);

        let can_delete = hash.decrement_reference().unwrap();
        assert!(!can_delete);
        assert_eq!(hash.reference_count, 1);

        let can_delete = hash.decrement_reference().unwrap();
        assert!(can_delete);
        assert_eq!(hash.reference_count, 0);
        assert!(hash.is_unused());
        assert!(hash.can_delete());
    }

    #[test]
    fn test_dedup_storage_savings_calculation() {
        let mut hash = create_content_hash("sha256:large_file", 100 * 1024 * 1024);

        for _ in 0..4 {
            hash.increment_reference().unwrap();
        }

        assert_eq!(hash.reference_count, 5);
        assert_eq!(hash.storage_saved(), 400 * 1024 * 1024);
    }

    #[test]
    fn test_dedup_invariants() {
        let hash = create_content_hash("sha256:valid", 1024);
        assert!(hash.check_invariants().is_ok());

        let bad_hash = ContentHash::builder()
            .hash("".to_string())
            .size_bytes(1024)
            .storage_key("key".to_string())
            .build()
            .unwrap();
        assert!(bad_hash.check_invariants().is_err());
    }
}

// ==========================================================================
// Workflow: File Conversion Pipeline
// ==========================================================================

mod conversion_workflow {
    use super::*;

    fn create_conversion(source_file_id: Uuid) -> ConversionJob {
        ConversionJob::builder()
            .source_file_id(source_file_id)
            .target_format("webp".to_string())
            .status(ConversionStatus::Pending)
            .progress(0)
            .build()
            .unwrap()
    }

    #[test]
    fn test_conversion_full_lifecycle() {
        let file_id = Uuid::new_v4();
        let mut job = create_conversion(file_id);

        assert_eq!(job.status, ConversionStatus::Pending);
        assert_eq!(job.progress, 0);
        assert!(!job.is_complete());

        job.status = ConversionStatus::Processing;
        job.started_at = Some(Utc::now());

        job.update_progress(25).unwrap();
        assert_eq!(job.progress, 25);
        assert_eq!(job.get_progress_percentage(), 25);

        job.update_progress(50).unwrap();
        job.update_progress(75).unwrap();

        let result_file_id = Uuid::new_v4();
        job.complete(result_file_id).unwrap();
        assert_eq!(job.status, ConversionStatus::Completed);
        assert_eq!(job.progress, 100);
        assert_eq!(job.result_file_id, Some(result_file_id));
        assert!(job.completed_at.is_some());
        assert!(job.is_complete());
    }

    #[test]
    fn test_conversion_failure_and_recovery() {
        let file_id = Uuid::new_v4();
        let mut job = create_conversion(file_id);

        job.status = ConversionStatus::Processing;
        job.started_at = Some(Utc::now());
        job.update_progress(30).unwrap();

        job.fail("Out of memory".to_string()).unwrap();
        assert_eq!(job.status, ConversionStatus::Failed);
        assert_eq!(job.error_message.as_deref(), Some("Out of memory"));

        // Retry with new job
        let mut retry_job = create_conversion(file_id);
        retry_job.status = ConversionStatus::Processing;
        retry_job.started_at = Some(Utc::now());

        let result_id = Uuid::new_v4();
        retry_job.complete(result_id).unwrap();
        assert!(retry_job.is_complete());
    }

    #[test]
    fn test_conversion_invariants() {
        let job = create_conversion(Uuid::new_v4());
        assert!(job.check_invariants().is_ok());
    }
}

// ==========================================================================
// Workflow: Processing Job Queue
// ==========================================================================

mod processing_job_workflow {
    use super::*;

    fn create_processing_job(file_id: Uuid, job_type: ProcessingJobType) -> ProcessingJob {
        ProcessingJob::builder()
            .file_id(file_id)
            .job_type(job_type)
            .status(JobStatus::Pending)
            .priority(0)
            .retry_count(0)
            .max_retries(3)
            .build()
            .unwrap()
    }

    #[test]
    fn test_job_lifecycle_success() {
        let file_id = Uuid::new_v4();
        let mut job = create_processing_job(file_id, ProcessingJobType::VideoThumbnail);

        assert_eq!(job.status, JobStatus::Pending);

        job.mark_started().unwrap();
        assert_eq!(job.status, JobStatus::Running);
        assert!(job.started_at.is_some());

        let result = serde_json::json!({"thumbnails": 3});
        job.mark_completed(result.clone()).unwrap();
        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(job.result_data, Some(result));
        assert!(job.completed_at.is_some());
        assert!(job.duration().is_some());
    }

    #[test]
    fn test_job_lifecycle_failure_with_retry() {
        let file_id = Uuid::new_v4();
        let mut job = create_processing_job(file_id, ProcessingJobType::DocumentPreview);

        job.mark_started().unwrap();
        job.mark_failed("Timeout".to_string()).unwrap();
        assert_eq!(job.status, JobStatus::Failed);
        assert!(job.can_retry());

        job.increment_retry();
        assert_eq!(job.retry_count, 1);

        job.status = JobStatus::Pending;
        job.mark_started().unwrap();
        job.mark_failed("Timeout again".to_string()).unwrap();
        assert!(job.can_retry());

        job.increment_retry();
        job.status = JobStatus::Pending;
        job.mark_started().unwrap();
        job.mark_failed("Third failure".to_string()).unwrap();

        job.increment_retry();
        assert_eq!(job.retry_count, 3);
        assert!(!job.can_retry());
    }

    #[test]
    fn test_job_cancellation() {
        let file_id = Uuid::new_v4();
        let mut job = create_processing_job(file_id, ProcessingJobType::VideoThumbnail);

        job.mark_started().unwrap();
        job.cancel().unwrap();
        assert_eq!(job.status, JobStatus::Cancelled);
    }

    #[test]
    fn test_job_invariants() {
        let mut job = create_processing_job(Uuid::new_v4(), ProcessingJobType::VideoThumbnail);
        assert!(job.check_invariants().is_ok());

        job.status = JobStatus::Running;
        job.started_at = None;
        assert!(job.check_invariants().is_err());
    }
}

// ==========================================================================
// Workflow: Video Thumbnail Generation
// ==========================================================================

mod video_thumbnail_workflow {
    use super::*;

    #[test]
    fn test_video_thumbnail_generation_flow() {
        let bucket = create_test_bucket();
        let file = create_test_video_file(bucket.id, bucket.owner_id);

        assert!(file.mime_type.starts_with("video/"));

        let mut job = ProcessingJob::builder()
            .file_id(file.id)
            .job_type(ProcessingJobType::VideoThumbnail)
            .status(JobStatus::Pending)
            .priority(0)
            .input_data(serde_json::json!({
                "sizes": [
                    {"size": "Small", "width": 64, "height": 64},
                    {"size": "Medium", "width": 128, "height": 128},
                    {"size": "Large", "width": 256, "height": 256},
                ]
            }))
            .retry_count(0)
            .max_retries(3)
            .build()
            .unwrap();

        job.mark_started().unwrap();

        let sizes = vec![
            (ThumbnailSize::Small, 64, 64),
            (ThumbnailSize::Medium, 128, 128),
            (ThumbnailSize::Large, 256, 256),
        ];

        let mut thumbnails = Vec::new();
        for (size, w, h) in sizes {
            let thumb = Thumbnail::builder()
                .file_id(file.id)
                .size(size)
                .width(w)
                .height(h)
                .storage_key(format!("thumbs/{}/{:?}", file.id, size))
                .mime_type("image/webp".to_string())
                .format("webp".to_string())
                .quality(80)
                .size_bytes(1024 * 10)
                .generated_at(Utc::now())
                .generation_time_ms(150)
                .source_version(1)
                .is_stale(false)
                .build()
                .unwrap();
            thumbnails.push(thumb);
        }

        assert_eq!(thumbnails.len(), 3);

        let result = serde_json::json!({"thumbnails_generated": true});
        job.mark_completed(result).unwrap();
        assert_eq!(job.status, JobStatus::Completed);
    }

    #[test]
    fn test_thumbnail_staleness_on_file_update() {
        let file_id = Uuid::new_v4();

        let mut thumb = Thumbnail::builder()
            .file_id(file_id)
            .size(ThumbnailSize::Medium)
            .width(128)
            .height(128)
            .storage_key("thumbs/old".to_string())
            .mime_type("image/webp".to_string())
            .format("webp".to_string())
            .quality(80)
            .size_bytes(5120)
            .generated_at(Utc::now() - Duration::hours(1))
            .generation_time_ms(100)
            .source_version(1)
            .is_stale(false)
            .build()
            .unwrap();

        assert!(!thumb.is_stale);

        // File updated -> mark thumbnails as stale
        thumb.is_stale = true;
        assert!(thumb.is_stale);

        // Regenerate with new version
        let new_thumb = Thumbnail::builder()
            .file_id(file_id)
            .size(ThumbnailSize::Medium)
            .width(128)
            .height(128)
            .storage_key("thumbs/new".to_string())
            .mime_type("image/webp".to_string())
            .format("webp".to_string())
            .quality(80)
            .size_bytes(5200)
            .generated_at(Utc::now())
            .generation_time_ms(120)
            .source_version(2)
            .is_stale(false)
            .build()
            .unwrap();

        assert!(!new_thumb.is_stale);
        assert_eq!(new_thumb.source_version, 2);
    }
}

// ==========================================================================
// Workflow: File Sharing
// ==========================================================================

mod file_sharing_workflow {
    use super::*;

    fn create_share(file_id: Uuid, owner_id: Uuid) -> FileShare {
        FileShare::builder()
            .file_id(file_id)
            .owner_id(owner_id)
            .token(Uuid::new_v4().to_string())
            .share_type(ShareType::Link)
            .share_status(ShareStatus::Active)
            .permission(SharePermission::View)
            .shared_with(vec![])
            .max_downloads(10)
            .download_count(0)
            .is_active(true)
            .expires_at(Utc::now() + Duration::days(7))
            .build()
            .unwrap()
    }

    fn create_direct_share(file_id: Uuid, owner_id: Uuid, viewer_id: Uuid) -> FileShare {
        FileShare::builder()
            .file_id(file_id)
            .owner_id(owner_id)
            .token(Uuid::new_v4().to_string())
            .share_type(ShareType::User)
            .share_status(ShareStatus::Active)
            .permission(SharePermission::View)
            .shared_with(vec![viewer_id])
            .max_downloads(5)
            .download_count(0)
            .is_active(true)
            .expires_at(Utc::now() + Duration::days(30))
            .build()
            .unwrap()
    }

    #[test]
    fn test_link_share_lifecycle() {
        let file_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let share = create_share(file_id, owner_id);

        assert!(share.is_valid());
        assert!(share.has_downloads_remaining());
        assert!(!share.is_expired());
        // Link shares are accessible by anyone
        assert!(share.can_access(None, None));
    }

    #[test]
    fn test_share_download_tracking() {
        let file_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let mut share = create_share(file_id, owner_id);

        for _ in 0..10 {
            assert!(share.record_download());
        }

        assert_eq!(share.download_count, 10);
        assert!(!share.has_downloads_remaining());
        // 11th download fails
        assert!(!share.record_download());
    }

    #[test]
    fn test_share_expiry() {
        let file_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let mut share = create_share(file_id, owner_id);

        share.expires_at = Some(Utc::now() - Duration::hours(1));
        assert!(share.is_expired());
        assert!(!share.is_valid());
    }

    #[test]
    fn test_share_revocation() {
        let file_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let mut share = create_share(file_id, owner_id);

        assert!(share.is_valid());

        share.revoke(owner_id);
        assert_eq!(share.share_status, ShareStatus::Revoked);
        assert!(!share.is_valid());
        assert!(share.revoked_at.is_some());
        assert_eq!(share.revoked_by, Some(owner_id));
    }

    #[test]
    fn test_direct_share_access_control() {
        let file_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let viewer_id = Uuid::new_v4();

        let share = create_direct_share(file_id, owner_id, viewer_id);

        // Correct user
        assert!(share.can_access(Some(viewer_id), None));

        // Wrong user
        let other_user = Uuid::new_v4();
        assert!(!share.can_access(Some(other_user), None));

        // No user
        assert!(!share.can_access(None, None));
    }

    #[test]
    fn test_share_invariants() {
        let file_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let share = create_share(file_id, owner_id);
        assert!(share.check_invariants().is_ok());
    }
}

// ==========================================================================
// Workflow: Quota Management
// ==========================================================================

mod quota_management_workflow {
    use super::*;

    #[test]
    fn test_quota_tracking_multiple_uploads() {
        let user_id = Uuid::new_v4();
        let mut quota = create_test_quota(user_id);

        let file_sizes: Vec<i64> = vec![
            1024 * 1024,       // 1MB
            5 * 1024 * 1024,   // 5MB
            10 * 1024 * 1024,  // 10MB
        ];

        for size in &file_sizes {
            assert!(quota.has_space_for(*size));
            quota.add_usage(*size).unwrap();
        }

        let total: i64 = file_sizes.iter().sum();
        assert_eq!(quota.used_bytes, total);

        quota.update_peak();
        assert_eq!(quota.peak_usage_bytes, total);
    }

    #[test]
    fn test_quota_warning_threshold() {
        let user_id = Uuid::new_v4();
        let mut quota = create_test_quota(user_id);

        // At 79% - below threshold
        quota.used_bytes = (quota.limit_bytes as f64 * 0.79) as i64;
        assert!(!quota.is_over_warning_threshold());

        // At 81% - above threshold
        quota.used_bytes = (quota.limit_bytes as f64 * 0.81) as i64;
        assert!(quota.is_over_warning_threshold());
    }

    #[test]
    fn test_quota_remaining_bytes() {
        let user_id = Uuid::new_v4();
        let mut quota = create_test_quota(user_id);
        let limit = quota.limit_bytes;

        quota.add_usage(1024 * 1024).unwrap();
        assert_eq!(quota.remaining_bytes(), limit - 1024 * 1024);
    }

    #[test]
    fn test_quota_usage_percent() {
        let user_id = Uuid::new_v4();
        let mut quota = create_test_quota(user_id);

        quota.used_bytes = quota.limit_bytes / 2;
        assert!((quota.usage_percent() - 50.0).abs() < 0.1);

        quota.used_bytes = quota.limit_bytes;
        assert!((quota.usage_percent() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_quota_invariants_after_operations() {
        let user_id = Uuid::new_v4();
        let mut quota = create_test_quota(user_id);

        quota.add_usage(1024).unwrap();
        assert!(quota.check_invariants().is_ok());

        quota.subtract_usage(512);
        assert!(quota.check_invariants().is_ok());
    }
}

// ==========================================================================
// Workflow: Bucket Lifecycle
// ==========================================================================

mod bucket_lifecycle_workflow {
    use super::*;

    #[test]
    fn test_bucket_creation_and_configuration() {
        let bucket = Bucket::builder()
            .name("media-storage".to_string())
            .slug("media-storage".to_string())
            .owner_id(Uuid::new_v4())
            .bucket_type(BucketType::Shared)
            .status(BucketStatus::Active)
            .storage_backend(StorageBackend::S3)
            .root_path("/data/media".to_string())
            .max_file_size(500 * 1024 * 1024)
            .allowed_mime_types(vec![
                "image/png".to_string(),
                "video/mp4".to_string(),
            ])
            .enable_versioning(true)
            .enable_cdn(true)
            .enable_deduplication(true)
            .build()
            .unwrap();

        assert!(bucket.is_accessible());
        assert!(bucket.can_upload(100 * 1024 * 1024, "image/png"));
        assert!(!bucket.can_upload(100 * 1024 * 1024, "application/pdf"));
        assert!(bucket.check_invariants().is_ok());
    }

    #[test]
    fn test_bucket_storage_tracking() {
        let mut bucket = create_test_bucket();

        bucket.update_stats(5 * 1024 * 1024, 1);
        assert_eq!(bucket.total_size_bytes, 5 * 1024 * 1024);
        assert_eq!(bucket.file_count, 1);

        bucket.update_stats(10 * 1024 * 1024, 1);
        assert_eq!(bucket.total_size_bytes, 15 * 1024 * 1024);
        assert_eq!(bucket.file_count, 2);

        // Delete a file (negative stats)
        bucket.update_stats(-5 * 1024 * 1024, -1);
        assert_eq!(bucket.total_size_bytes, 10 * 1024 * 1024);
        assert_eq!(bucket.file_count, 1);
    }

    #[test]
    fn test_bucket_archived_blocks_access() {
        let mut bucket = create_test_bucket();
        bucket.status = BucketStatus::Archived;

        assert!(!bucket.is_accessible());
        assert!(!bucket.can_upload(1024, "application/pdf"));
    }
}

// ==========================================================================
// Workflow: Cross-Entity Integration (End-to-End)
// ==========================================================================

mod cross_entity_integration {
    use super::*;

    #[test]
    fn test_full_file_upload_with_dedup_and_scan() {
        let user_id = Uuid::new_v4();

        // 1. Create bucket
        let mut bucket = Bucket::builder()
            .name("user-files".to_string())
            .slug("user-files".to_string())
            .owner_id(user_id)
            .root_path("/data/user-files".to_string())
            .allowed_mime_types(vec![])
            .enable_deduplication(true)
            .build()
            .unwrap();

        // 2. Initialize quota
        let mut quota = create_test_quota(user_id);

        // 3. Check upload prerequisites
        let file_size: i64 = 5 * 1024 * 1024;
        assert!(bucket.can_upload(file_size, "application/pdf"));
        assert!(quota.has_space_for(file_size));

        // 4. Check deduplication - new content
        let mut content_hash = ContentHash::builder()
            .hash("sha256:unique_hash_123".to_string())
            .size_bytes(file_size)
            .storage_key("content/sha256_unique_hash_123".to_string())
            .storage_backend(StorageBackend::Local)
            .reference_count(1)
            .first_uploaded_at(Utc::now())
            .last_referenced_at(Utc::now())
            .build()
            .unwrap();

        // 5. Create the file record
        let mut file = create_test_file(bucket.id, user_id);
        file.size_bytes = file_size;
        file.content_hash_id = Some(content_hash.id);

        // 6. Update bucket stats
        bucket.update_stats(file_size, 1);

        // 7. Update quota
        quota.add_usage(file_size).unwrap();

        // 8. Scan file
        file.is_scanned = true;
        file.scan_result = Some(serde_json::json!("clean"));
        file.threat_level = Some(ThreatLevel::Safe);
        assert!(file.is_safe());

        // 9. Another user uploads same content (dedup)
        content_hash.increment_reference().unwrap();
        assert_eq!(content_hash.reference_count, 2);
        assert_eq!(content_hash.storage_saved(), file_size);

        // 10. Verify all invariants
        assert!(bucket.check_invariants().is_ok());
        assert!(quota.check_invariants().is_ok());
        assert!(file.check_invariants().is_ok());
        assert!(content_hash.check_invariants().is_ok());
    }

    #[test]
    fn test_file_with_sharing_and_access_tracking() {
        let owner_id = Uuid::new_v4();
        let viewer_id = Uuid::new_v4();

        let bucket = create_test_bucket();
        let mut file = create_test_file(bucket.id, owner_id);
        file.is_scanned = true;
        file.threat_level = Some(ThreatLevel::Safe);

        // Share with viewer (direct share)
        let mut share = FileShare::builder()
            .file_id(file.id)
            .owner_id(owner_id)
            .token(Uuid::new_v4().to_string())
            .share_type(ShareType::User)
            .share_status(ShareStatus::Active)
            .permission(SharePermission::View)
            .shared_with(vec![viewer_id])
            .max_downloads(5)
            .download_count(0)
            .is_active(true)
            .expires_at(Utc::now() + Duration::days(30))
            .build()
            .unwrap();

        // Viewer accesses file
        assert!(share.can_access(Some(viewer_id), None));
        assert!(file.is_accessible());

        share.record_download();
        file.record_access();

        assert_eq!(share.download_count, 1);
        assert_eq!(file.download_count, 1);

        // Multiple downloads until limit
        for _ in 0..4 {
            share.record_download();
            file.record_access();
        }

        assert_eq!(share.download_count, 5);
        assert!(!share.has_downloads_remaining());
        assert_eq!(file.download_count, 5);
    }

    #[test]
    fn test_multipart_upload_with_quota_check() {
        let user_id = Uuid::new_v4();
        let bucket = create_test_bucket();
        let mut quota = create_test_quota(user_id);

        let file_size: i64 = 100 * 1024 * 1024;

        assert!(quota.has_space_for(file_size));

        let mut session = UploadSession::builder()
            .bucket_id(bucket.id)
            .user_id(user_id)
            .path("/uploads/huge.zip".to_string())
            .filename("huge.zip".to_string())
            .file_size(file_size)
            .chunk_size(10 * 1024 * 1024)
            .total_chunks(10)
            .uploaded_chunks(0)
            .status(UploadStatus::Initiated)
            .completed_parts(vec![])
            .storage_backend(StorageBackend::Local)
            .expires_at(Utc::now() + Duration::hours(24))
            .build()
            .unwrap();

        for i in 1..=10 {
            session.add_part(i, format!("etag-{}", i)).unwrap();
        }

        assert!(session.is_complete());
        session.mark_complete().unwrap();

        quota.add_usage(file_size).unwrap();
        quota.update_peak();

        assert_eq!(quota.used_bytes, file_size);
        assert_eq!(quota.peak_usage_bytes, file_size);
    }

    #[test]
    fn test_locked_file_prevents_concurrent_edit() {
        let file_id = Uuid::new_v4();
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();

        let lock = FileLock::new(
            file_id,
            user_a,
            Utc::now(),
            Utc::now() + Duration::minutes(30),
            LockStatus::Active,
        );

        assert!(lock.is_valid());
        assert!(lock.is_owned_by(user_a));
        assert!(!lock.is_owned_by(user_b));
        assert!(!lock.is_expired());
    }

    #[test]
    fn test_file_conversion_with_version_tracking() {
        let bucket = create_test_bucket();
        let file = create_test_file(bucket.id, bucket.owner_id);

        let mut conversion = ConversionJob::builder()
            .source_file_id(file.id)
            .target_format("webp".to_string())
            .status(ConversionStatus::Pending)
            .progress(0)
            .build()
            .unwrap();

        conversion.status = ConversionStatus::Processing;
        conversion.started_at = Some(Utc::now());
        conversion.update_progress(50).unwrap();

        let result_file_id = Uuid::new_v4();
        conversion.complete(result_file_id).unwrap();

        assert!(conversion.is_complete());
        assert_eq!(conversion.result_file_id, Some(result_file_id));
        assert_eq!(conversion.progress, 100);

        // The result file links back to original via previous_version_id
        let converted_file = StoredFile {
            id: result_file_id,
            bucket_id: bucket.id,
            owner_id: bucket.owner_id,
            path: "/docs/report.webp".to_string(),
            original_name: "report.webp".to_string(),
            size_bytes: 512 * 1024,
            mime_type: "image/webp".to_string(),
            checksum: None,
            is_compressed: false,
            original_size: None,
            compression_algorithm: None,
            is_scanned: false,
            scan_result: None,
            threat_level: None,
            has_thumbnail: false,
            thumbnail_path: None,
            has_video_thumbnail: false,
            has_document_preview: false,
            processing_status: None,
            content_hash_id: None,
            cdn_url: None,
            cdn_url_expires_at: None,
            owner_module: None,
            owner_entity: None,
            owner_entity_id: None,
            field_name: None,
            sort_order: 0,
            status: FileStatus::Active,
            storage_key: format!("files/{}", result_file_id),
            version: 1,
            previous_version_id: Some(file.id),
            download_count: 0,
            last_accessed_at: None,
            metadata: AuditMetadata::default(),
        };

        assert_eq!(converted_file.previous_version_id, Some(file.id));
        assert!(converted_file.is_accessible());
    }
}
