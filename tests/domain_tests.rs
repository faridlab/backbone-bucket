//! Domain Logic Tests for Bucket
//!
//! Comprehensive tests for core domain entities and services.

use backbone_bucket::domain::entity::*;
use bcrypt;
use chrono::{Duration, Utc};
use uuid::Uuid;

// =============================================================================
// StoredFile Entity Tests
// =============================================================================

mod stored_file_tests {
    use super::*;

    fn create_test_file() -> StoredFile {
        StoredFile {
            id: Uuid::new_v4(),
            bucket_id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            path: "documents/report.pdf".to_string(),
            original_name: "report.pdf".to_string(),
            size_bytes: 1024,
            mime_type: "application/pdf".to_string(),
            checksum: Some("abc123".to_string()),
            is_compressed: false,
            original_size: None,
            compression_algorithm: None,
            is_scanned: true,
            scan_result: None,
            threat_level: Some(ThreatLevel::Safe),
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
            storage_key: "bucket/2024/01/01/abc-report.pdf".to_string(),
            version: 1,
            previous_version_id: None,
            download_count: 0,
            last_accessed_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_is_accessible_active_file() {
        let file = create_test_file();
        assert!(file.is_accessible());
    }

    #[test]
    fn test_is_accessible_deleted_file() {
        let mut file = create_test_file();
        file.status = FileStatus::Deleted;
        assert!(!file.is_accessible());
    }

    #[test]
    fn test_is_accessible_quarantined_file() {
        let mut file = create_test_file();
        file.status = FileStatus::Quarantined;
        assert!(!file.is_accessible());
    }

    #[test]
    fn test_is_safe_with_safe_threat_level() {
        let file = create_test_file();
        assert!(file.is_safe());
    }

    #[test]
    fn test_is_safe_with_low_threat_level() {
        let mut file = create_test_file();
        file.threat_level = Some(ThreatLevel::Low);
        assert!(!file.is_safe()); // Low is not considered safe
    }

    #[test]
    fn test_is_safe_with_medium_threat_level() {
        let mut file = create_test_file();
        file.threat_level = Some(ThreatLevel::Medium);
        assert!(!file.is_safe());
    }

    #[test]
    fn test_is_safe_with_high_threat_level() {
        let mut file = create_test_file();
        file.threat_level = Some(ThreatLevel::High);
        assert!(!file.is_safe());
    }

    #[test]
    fn test_is_safe_unscanned_file() {
        let mut file = create_test_file();
        file.is_scanned = false;
        file.threat_level = None;
        assert!(!file.is_safe()); // Must be scanned to be safe
    }

    #[test]
    fn test_needs_processing_unscanned() {
        let mut file = create_test_file();
        file.is_scanned = false;
        assert!(file.needs_processing());
    }

    #[test]
    fn test_needs_processing_uploading() {
        let mut file = create_test_file();
        file.status = FileStatus::Uploading;
        file.is_scanned = false;
        assert!(file.needs_processing());
    }

    #[test]
    fn test_needs_processing_active_scanned() {
        let file = create_test_file();
        assert!(!file.needs_processing());
    }

    #[test]
    fn test_record_access() {
        let mut file = create_test_file();
        assert_eq!(file.download_count, 0);
        assert!(file.last_accessed_at.is_none());

        file.record_access();

        assert_eq!(file.download_count, 1);
        assert!(file.last_accessed_at.is_some());
    }

    #[test]
    fn test_soft_delete() {
        let mut file = create_test_file();
        assert_eq!(file.status, FileStatus::Active);

        file.soft_delete();

        assert_eq!(file.status, FileStatus::Deleted);
    }

    #[test]
    fn test_restore() {
        let mut file = create_test_file();
        file.status = FileStatus::Deleted;

        file.restore();

        assert_eq!(file.status, FileStatus::Active);
    }

    #[test]
    fn test_quarantine() {
        let mut file = create_test_file();
        let threats = vec!["Malware detected".to_string()];

        file.quarantine(threats);

        assert_eq!(file.status, FileStatus::Quarantined);
        assert!(file.scan_result.is_some());
    }

    #[test]
    fn test_check_invariants_valid() {
        let file = create_test_file();
        assert!(file.check_invariants().is_ok());
    }

    #[test]
    fn test_check_invariants_invalid_size() {
        let mut file = create_test_file();
        file.size_bytes = 0;
        assert!(file.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_path_traversal() {
        let mut file = create_test_file();
        file.path = "../../../etc/passwd".to_string();
        assert!(file.check_invariants().is_err());
    }
}

// =============================================================================
// Bucket Entity Tests
// =============================================================================

mod bucket_tests {
    use super::*;

    fn create_test_bucket() -> Bucket {
        Bucket {
            id: Uuid::new_v4(),
            name: "Test Bucket".to_string(),
            slug: "test-bucket".to_string(),
            description: None,
            owner_id: Uuid::new_v4(),
            bucket_type: BucketType::User,
            status: BucketStatus::Active,
            storage_backend: StorageBackend::Local,
            root_path: "/storage/test-bucket".to_string(),
            file_count: 0,
            total_size_bytes: 0,
            max_file_size: Some(10 * 1024 * 1024), // 10MB
            allowed_mime_types: vec![],
            auto_delete_after_days: None,
            enable_cdn: false,
            enable_versioning: false,
            enable_deduplication: false,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_can_upload_valid() {
        let bucket = create_test_bucket();
        assert!(bucket.can_upload(1024, "text/plain"));
    }

    #[test]
    fn test_can_upload_file_too_large() {
        let bucket = create_test_bucket();
        assert!(!bucket.can_upload(20 * 1024 * 1024, "text/plain"));
    }

    #[test]
    fn test_can_upload_inactive_bucket() {
        let mut bucket = create_test_bucket();
        bucket.status = BucketStatus::Deleted;
        assert!(!bucket.can_upload(1024, "text/plain"));
    }

    #[test]
    fn test_can_upload_mime_type_restriction() {
        let mut bucket = create_test_bucket();
        bucket.allowed_mime_types = vec!["image/png".to_string(), "image/jpeg".to_string()];

        assert!(bucket.can_upload(1024, "image/png"));
        assert!(bucket.can_upload(1024, "image/jpeg"));
        assert!(!bucket.can_upload(1024, "text/plain"));
        assert!(!bucket.can_upload(1024, "application/pdf"));
    }

    #[test]
    fn test_update_stats() {
        let mut bucket = create_test_bucket();
        assert_eq!(bucket.file_count, 0);
        assert_eq!(bucket.total_size_bytes, 0);

        bucket.update_stats(1024, 1);

        assert_eq!(bucket.file_count, 1);
        assert_eq!(bucket.total_size_bytes, 1024);

        bucket.update_stats(-1024, -1);

        assert_eq!(bucket.file_count, 0);
        assert_eq!(bucket.total_size_bytes, 0);
    }

    #[test]
    fn test_update_stats_no_negative() {
        let mut bucket = create_test_bucket();
        bucket.update_stats(-1000, -5);

        assert_eq!(bucket.file_count, 0);
        assert_eq!(bucket.total_size_bytes, 0);
    }

    #[test]
    fn test_is_accessible() {
        let mut bucket = create_test_bucket();
        assert!(bucket.is_accessible());

        bucket.status = BucketStatus::Readonly;
        assert!(bucket.is_accessible());

        bucket.status = BucketStatus::Archived;
        assert!(!bucket.is_accessible());
    }

    #[test]
    fn test_check_invariants_valid() {
        let bucket = create_test_bucket();
        assert!(bucket.check_invariants().is_ok());
    }
}

// =============================================================================
// UserQuota Entity Tests
// =============================================================================

mod user_quota_tests {
    use super::*;

    fn create_test_quota() -> UserQuota {
        UserQuota {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            limit_bytes: 1024 * 1024 * 1024, // 1GB
            used_bytes: 0,
            file_count: 0,
            max_file_size: Some(100 * 1024 * 1024), // 100MB
            max_file_count: Some(1000),
            tier: "free".to_string(),
            quota_status: QuotaStatus::Normal,
            warning_threshold_percent: 80,
            last_warning_sent_at: None,
            peak_usage_bytes: 0,
            peak_usage_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_has_space_for() {
        let quota = create_test_quota();
        assert!(quota.has_space_for(1024));
        assert!(quota.has_space_for(1024 * 1024 * 1024)); // Exactly at limit
        assert!(!quota.has_space_for(1024 * 1024 * 1024 + 1)); // Over limit
    }

    #[test]
    fn test_usage_percent() {
        let mut quota = create_test_quota();
        assert_eq!(quota.usage_percent(), 0.0);

        quota.used_bytes = 512 * 1024 * 1024; // 512MB
        assert!((quota.usage_percent() - 50.0).abs() < 0.001);

        quota.used_bytes = quota.limit_bytes;
        assert!((quota.usage_percent() - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_is_over_warning_threshold() {
        let mut quota = create_test_quota();
        quota.warning_threshold_percent = 80;

        quota.used_bytes = (0.79 * quota.limit_bytes as f64) as i64;
        assert!(!quota.is_over_warning_threshold());

        quota.used_bytes = (0.81 * quota.limit_bytes as f64) as i64;
        assert!(quota.is_over_warning_threshold());
    }

    #[test]
    fn test_add_usage_success() {
        let mut quota = create_test_quota();
        assert!(quota.add_usage(1024).is_ok());
        assert_eq!(quota.used_bytes, 1024);
    }

    #[test]
    fn test_add_usage_exceeds_quota() {
        let mut quota = create_test_quota();
        quota.used_bytes = quota.limit_bytes;

        let result = quota.add_usage(1);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtract_usage() {
        let mut quota = create_test_quota();
        quota.used_bytes = 2048;

        quota.subtract_usage(1024);

        assert_eq!(quota.used_bytes, 1024);
    }

    #[test]
    fn test_subtract_usage_no_negative() {
        let mut quota = create_test_quota();
        quota.subtract_usage(1024);

        assert_eq!(quota.used_bytes, 0);
        assert_eq!(quota.file_count, 0);
    }

    #[test]
    fn test_update_peak() {
        let mut quota = create_test_quota();
        quota.used_bytes = 100;
        quota.update_peak();
        assert_eq!(quota.peak_usage_bytes, 100);

        quota.used_bytes = 50;
        quota.update_peak();
        assert_eq!(quota.peak_usage_bytes, 100); // Should not decrease

        quota.used_bytes = 200;
        quota.update_peak();
        assert_eq!(quota.peak_usage_bytes, 200);
    }

    #[test]
    fn test_remaining_bytes() {
        let mut quota = create_test_quota();
        assert_eq!(quota.remaining_bytes(), quota.limit_bytes);

        quota.used_bytes = 100;
        assert_eq!(quota.remaining_bytes(), quota.limit_bytes - 100);
    }
}

// =============================================================================
// FileShare Entity Tests
// =============================================================================

mod file_share_tests {
    use super::*;

    fn create_test_share() -> FileShare {
        FileShare {
            id: Uuid::new_v4(),
            file_id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            token: "abc123xyz".to_string(),
            share_type: ShareType::Link,
            permission: SharePermission::View,
            shared_with: vec![],
            password_hash: None,
            max_downloads: None,
            download_count: 0,
            expires_at: None,
            share_status: ShareStatus::Active,
            is_active: true,
            revoked_at: None,
            revoked_by: None,
            message: None,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_is_valid_active_share() {
        let share = create_test_share();
        assert!(share.is_valid());
    }

    #[test]
    fn test_is_valid_revoked_share() {
        let mut share = create_test_share();
        share.share_status = ShareStatus::Revoked;
        assert!(!share.is_valid());
    }

    #[test]
    fn test_is_expired_no_expiry() {
        let share = create_test_share();
        assert!(!share.is_expired());
    }

    #[test]
    fn test_is_expired_future_expiry() {
        let mut share = create_test_share();
        share.expires_at = Some(Utc::now() + Duration::hours(24));
        assert!(!share.is_expired());
    }

    #[test]
    fn test_is_expired_past_expiry() {
        let mut share = create_test_share();
        share.expires_at = Some(Utc::now() - Duration::hours(1));
        assert!(share.is_expired());
    }

    #[test]
    fn test_has_downloads_remaining_no_limit() {
        let share = create_test_share();
        assert!(share.has_downloads_remaining());
    }

    #[test]
    fn test_has_downloads_remaining_with_limit() {
        let mut share = create_test_share();
        share.max_downloads = Some(10);
        share.download_count = 5;
        assert!(share.has_downloads_remaining());

        share.download_count = 10;
        assert!(!share.has_downloads_remaining());
    }

    #[test]
    fn test_can_access_public_link() {
        let share = create_test_share();
        assert!(share.can_access(None, None));
        assert!(share.can_access(Some(Uuid::new_v4()), None));
    }

    #[test]
    fn test_can_access_user_share() {
        let mut share = create_test_share();
        share.share_type = ShareType::User;
        let allowed_user = Uuid::new_v4();
        share.shared_with = vec![allowed_user];

        assert!(share.can_access(Some(allowed_user), None));
        assert!(!share.can_access(Some(Uuid::new_v4()), None));
        assert!(!share.can_access(None, None));
    }

    #[test]
    fn test_can_access_password_share() {
        let mut share = create_test_share();
        share.share_type = ShareType::Password;
        let hash = bcrypt::hash("secret123", bcrypt::DEFAULT_COST).unwrap();
        share.password_hash = Some(hash);

        assert!(share.can_access(None, Some("secret123")));   // correct password
        assert!(!share.can_access(None, Some("wrong")));       // wrong password
        assert!(!share.can_access(None, Some("")));             // empty password
        assert!(!share.can_access(None, None));                 // no password
    }

    #[test]
    fn test_record_download() {
        let mut share = create_test_share();
        assert!(share.record_download());
        assert_eq!(share.download_count, 1);
    }

    #[test]
    fn test_record_download_at_limit() {
        let mut share = create_test_share();
        share.max_downloads = Some(1);
        share.download_count = 1;

        assert!(!share.record_download());
        assert_eq!(share.download_count, 1);
    }

    #[test]
    fn test_revoke() {
        let mut share = create_test_share();
        let revoker = Uuid::new_v4();

        share.revoke(revoker);

        assert_eq!(share.share_status, ShareStatus::Revoked);
        assert!(!share.is_active);
        assert!(share.revoked_at.is_some());
        assert_eq!(share.revoked_by, Some(revoker));
    }

    #[test]
    fn test_remaining_downloads() {
        let mut share = create_test_share();
        assert!(share.max_downloads.is_none());

        share.max_downloads = Some(10);
        share.download_count = 3;
        assert!(share.has_downloads_remaining());
        assert_eq!(share.max_downloads.unwrap() - share.download_count, 7);
    }
}

// =============================================================================
// Property-Based Tests (using proptest)
// =============================================================================

mod property_tests {
    use super::*;
    use proptest::prelude::*;

    fn gen_bytes(min: i64, max: i64) -> impl Strategy<Value = i64> {
        min..=max
    }

    fn create_quota(limit: i64, used: i64, file_count: i32, tier: String) -> UserQuota {
        UserQuota {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            limit_bytes: limit,
            used_bytes: used.min(limit).max(0),
            file_count,
            max_file_size: Some(limit / 10),
            max_file_count: Some(1000),
            tier,
            quota_status: QuotaStatus::Normal,
            warning_threshold_percent: 80,
            last_warning_sent_at: None,
            peak_usage_bytes: used,
            peak_usage_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    proptest! {
        #[test]
        fn prop_usage_percent_in_range(
            limit in gen_bytes(1, 1_000_000_000_000i64),
            used_ratio in 0.0..=1.0f64,
        ) {
            let used = (limit as f64 * used_ratio) as i64;
            let quota = create_quota(limit, used, 10, "free".to_string());

            let percent = quota.usage_percent();
            prop_assert!(percent >= 0.0);
            prop_assert!(percent <= 100.0);
        }

        #[test]
        fn prop_has_space_for_consistency(
            limit in gen_bytes(1000, 1_000_000_000i64),
            used_ratio in 0.0..0.99f64,
        ) {
            let used = (limit as f64 * used_ratio) as i64;
            let quota = create_quota(limit, used, 10, "free".to_string());

            let remaining = quota.remaining_bytes();
            prop_assert!(quota.has_space_for(remaining));
            prop_assert!(!quota.has_space_for(remaining + 1));
        }

        #[test]
        fn prop_remaining_plus_used_equals_limit(
            limit in gen_bytes(1, 1_000_000_000_000i64),
            used_ratio in 0.0..=1.0f64,
        ) {
            let used = (limit as f64 * used_ratio) as i64;
            let quota = create_quota(limit, used, 10, "free".to_string());

            prop_assert_eq!(quota.remaining_bytes() + quota.used_bytes, quota.limit_bytes);
        }

        #[test]
        fn prop_add_usage_increases_by_exact_amount(
            limit in gen_bytes(1000, 1_000_000_000i64),
            add_bytes in gen_bytes(1, 100_000i64),
        ) {
            let mut quota = create_quota(limit, 0, 0, "free".to_string());

            let initial = quota.used_bytes;
            if quota.has_space_for(add_bytes) {
                let result = quota.add_usage(add_bytes);
                prop_assert!(result.is_ok());
                prop_assert_eq!(quota.used_bytes, initial + add_bytes);
            }
        }

        #[test]
        fn prop_subtract_never_negative(
            used in gen_bytes(0, 1_000_000_000i64),
            subtract in gen_bytes(0, 2_000_000_000i64),
        ) {
            let mut quota = create_quota(1_000_000_000, used, 10, "free".to_string());

            quota.subtract_usage(subtract);

            prop_assert!(quota.used_bytes >= 0);
        }

        #[test]
        fn prop_warning_threshold_consistency(
            limit in gen_bytes(1000, 1_000_000_000i64),
            used_ratio in 0.0..=1.0f64,
        ) {
            let used = (limit as f64 * used_ratio) as i64;
            let quota = create_quota(limit, used, 10, "free".to_string());

            let percent = quota.usage_percent();
            let over_threshold = quota.is_over_warning_threshold();

            prop_assert_eq!(over_threshold, percent >= 80.0);
        }

        #[test]
        fn prop_peak_usage_invariant(
            limit in gen_bytes(1000, 1_000_000_000i64),
        ) {
            let mut quota = create_quota(limit, 0, 0, "free".to_string());

            let _ = quota.add_usage(limit / 4);
            quota.update_peak();
            let _ = quota.add_usage(limit / 4);
            quota.update_peak();

            quota.subtract_usage(limit / 8);

            prop_assert!(quota.peak_usage_bytes >= quota.used_bytes);
        }

        #[test]
        fn prop_file_count_non_negative(
            initial_count in 0..100i32,
            subtract_count in 0..200i32,
        ) {
            let mut quota = create_quota(1_000_000_000, 0, initial_count, "free".to_string());

            for _ in 0..subtract_count {
                quota.subtract_usage(100);
            }

            prop_assert!(quota.file_count >= 0);
        }
    }

    #[test]
    fn test_zero_limit_quota() {
        let quota = create_quota(0, 0, 0, "free".to_string());
        assert_eq!(quota.usage_percent(), 0.0); // Zero limit returns 0%
        assert!(!quota.has_space_for(1));
    }

    #[test]
    fn test_exactly_at_limit() {
        let limit = 1_000_000_000i64;
        let quota = create_quota(limit, limit, 100, "premium".to_string());

        assert_eq!(quota.usage_percent(), 100.0);
        assert_eq!(quota.remaining_bytes(), 0);
        assert!(!quota.has_space_for(1));
        assert!(quota.is_over_warning_threshold());
    }

    #[test]
    fn test_exactly_at_warning_threshold() {
        let limit = 1_000_000i64;
        let used = 800_000i64; // Exactly 80%
        let quota = create_quota(limit, used, 50, "basic".to_string());

        assert!(quota.is_over_warning_threshold());
        assert!((quota.usage_percent() - 80.0).abs() < 0.01);
    }
}

// =============================================================================
// FileLock Entity Tests
// =============================================================================

mod file_lock_tests {
    use super::*;

    fn create_test_lock() -> FileLock {
        FileLock {
            id: Uuid::new_v4(),
            file_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            locked_at: Utc::now() - Duration::minutes(5),
            expires_at: Utc::now() + Duration::hours(1),
            refreshed_at: None,
            status: LockStatus::Active,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_is_valid_active_not_expired() {
        let lock = create_test_lock();
        assert!(lock.is_valid());
    }

    #[test]
    fn test_is_valid_expired() {
        let mut lock = create_test_lock();
        lock.expires_at = Utc::now() - Duration::seconds(1);
        assert!(!lock.is_valid());
    }

    #[test]
    fn test_is_valid_released() {
        let mut lock = create_test_lock();
        lock.status = LockStatus::Released;
        assert!(!lock.is_valid());
    }

    #[test]
    fn test_is_expired() {
        let mut lock = create_test_lock();
        assert!(!lock.is_expired());

        lock.expires_at = Utc::now() - Duration::seconds(1);
        assert!(lock.is_expired());
    }

    #[test]
    fn test_is_owned_by() {
        let lock = create_test_lock();
        assert!(lock.is_owned_by(lock.user_id));
        assert!(!lock.is_owned_by(Uuid::new_v4()));
    }

    #[test]
    fn test_can_refresh_valid_lock() {
        let lock = create_test_lock();
        assert!(lock.can_refresh());
    }

    #[test]
    fn test_can_refresh_expired_lock() {
        let mut lock = create_test_lock();
        lock.expires_at = Utc::now() - Duration::seconds(1);
        assert!(!lock.can_refresh());
    }

    #[test]
    fn test_refresh_extends_expiry() {
        let mut lock = create_test_lock();
        let old_expires = lock.expires_at;

        lock.refresh(Duration::hours(2)).unwrap();

        assert!(lock.expires_at > old_expires);
        assert!(lock.refreshed_at.is_some());
    }

    #[test]
    fn test_refresh_expired_lock_fails() {
        let mut lock = create_test_lock();
        lock.expires_at = Utc::now() - Duration::seconds(1);

        let result = lock.refresh(Duration::hours(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_time_remaining_active() {
        let lock = create_test_lock();
        let remaining = lock.time_remaining();
        assert!(remaining > Duration::zero());
    }

    #[test]
    fn test_time_remaining_expired() {
        let mut lock = create_test_lock();
        lock.expires_at = Utc::now() - Duration::hours(1);
        assert_eq!(lock.time_remaining(), Duration::zero());
    }

    #[test]
    fn test_check_invariants_valid() {
        let lock = create_test_lock();
        assert!(lock.check_invariants().is_ok());
    }

    #[test]
    fn test_check_invariants_expires_before_locked() {
        let mut lock = create_test_lock();
        lock.expires_at = lock.locked_at - Duration::hours(1);
        lock.status = LockStatus::Expired;
        let result = lock.check_invariants();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder() {
        let file_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let expires = Utc::now() + Duration::hours(1);

        let lock = FileLock::builder()
            .file_id(file_id)
            .user_id(user_id)
            .expires_at(expires)
            .status(LockStatus::Active)
            .build()
            .unwrap();

        assert_eq!(lock.file_id, file_id);
        assert_eq!(lock.user_id, user_id);
        assert_eq!(lock.status, LockStatus::Active);
    }

    #[test]
    fn test_builder_missing_required_field() {
        let result = FileLock::builder()
            .file_id(Uuid::new_v4())
            .build();
        assert!(result.is_err());
    }
}

// =============================================================================
// ProcessingJob Entity Tests
// =============================================================================

mod processing_job_tests {
    use super::*;

    fn create_test_job() -> ProcessingJob {
        ProcessingJob {
            id: Uuid::new_v4(),
            file_id: Uuid::new_v4(),
            job_type: ProcessingJobType::ThumbnailGeneration,
            status: JobStatus::Pending,
            priority: 0,
            input_data: None,
            result_data: None,
            error_message: None,
            started_at: None,
            completed_at: None,
            retry_count: 0,
            max_retries: 3,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_can_retry_pending_job() {
        let job = create_test_job();
        assert!(!job.can_retry());
    }

    #[test]
    fn test_can_retry_failed_with_retries() {
        let mut job = create_test_job();
        job.status = JobStatus::Failed;
        job.retry_count = 1;
        assert!(job.can_retry());
    }

    #[test]
    fn test_can_retry_failed_max_retries() {
        let mut job = create_test_job();
        job.status = JobStatus::Failed;
        job.retry_count = 3;
        assert!(!job.can_retry());
    }

    #[test]
    fn test_increment_retry() {
        let mut job = create_test_job();
        assert_eq!(job.retry_count, 0);

        job.increment_retry();
        assert_eq!(job.retry_count, 1);

        job.increment_retry();
        assert_eq!(job.retry_count, 2);
    }

    #[test]
    fn test_mark_started() {
        let mut job = create_test_job();
        job.mark_started().unwrap();

        assert_eq!(job.status, JobStatus::Running);
        assert!(job.started_at.is_some());
    }

    #[test]
    fn test_mark_completed() {
        let mut job = create_test_job();
        job.mark_started().unwrap();

        let result = serde_json::json!({"output": "done"});
        job.mark_completed(result.clone()).unwrap();

        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(job.result_data, Some(result));
        assert!(job.completed_at.is_some());
    }

    #[test]
    fn test_mark_failed() {
        let mut job = create_test_job();
        job.mark_started().unwrap();
        job.mark_failed("out of memory".to_string()).unwrap();

        assert_eq!(job.status, JobStatus::Failed);
        assert_eq!(job.error_message, Some("out of memory".to_string()));
    }

    #[test]
    fn test_cancel() {
        let mut job = create_test_job();
        job.cancel().unwrap();

        assert_eq!(job.status, JobStatus::Cancelled);
    }

    #[test]
    fn test_duration_not_started() {
        let job = create_test_job();
        assert!(job.duration().is_none());
    }

    #[test]
    fn test_duration_completed() {
        let mut job = create_test_job();
        let start = Utc::now() - Duration::seconds(10);
        let end = Utc::now();
        job.started_at = Some(start);
        job.completed_at = Some(end);

        let dur = job.duration().unwrap();
        assert!(dur.num_seconds() >= 9 && dur.num_seconds() <= 11);
    }

    #[test]
    fn test_check_invariants_valid() {
        let job = create_test_job();
        assert!(job.check_invariants().is_ok());
    }

    #[test]
    fn test_check_invariants_retry_exceeds_max() {
        let mut job = create_test_job();
        job.retry_count = 5;
        job.max_retries = 3;
        assert!(job.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_running_no_started_at() {
        let mut job = create_test_job();
        job.status = JobStatus::Running;
        job.started_at = None;
        assert!(job.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_completed_no_completed_at() {
        let mut job = create_test_job();
        job.status = JobStatus::Completed;
        job.completed_at = None;
        assert!(job.check_invariants().is_err());
    }

    #[test]
    fn test_builder() {
        let file_id = Uuid::new_v4();
        let job = ProcessingJob::builder()
            .file_id(file_id)
            .job_type(ProcessingJobType::VideoThumbnail)
            .status(JobStatus::Pending)
            .priority(5)
            .max_retries(5)
            .build()
            .unwrap();

        assert_eq!(job.file_id, file_id);
        assert_eq!(job.job_type, ProcessingJobType::VideoThumbnail);
        assert_eq!(job.priority, 5);
        assert_eq!(job.max_retries, 5);
        assert_eq!(job.retry_count, 0);
    }

    #[test]
    fn test_builder_missing_required() {
        let result = ProcessingJob::builder()
            .priority(1)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_job_lifecycle() {
        let mut job = create_test_job();

        // Pending -> Running
        job.mark_started().unwrap();
        assert_eq!(job.status, JobStatus::Running);

        // Running -> Failed
        job.mark_failed("timeout".to_string()).unwrap();
        assert_eq!(job.status, JobStatus::Failed);
        assert!(job.can_retry());

        // Retry
        job.increment_retry();
        job.status = JobStatus::Pending;
        job.mark_started().unwrap();

        // Running -> Completed
        job.mark_completed(serde_json::json!({"ok": true})).unwrap();
        assert_eq!(job.status, JobStatus::Completed);
        assert!(!job.can_retry());
    }
}

// =============================================================================
// ContentHash Entity Tests
// =============================================================================

mod content_hash_tests {
    use super::*;

    fn create_test_hash() -> ContentHash {
        ContentHash {
            id: Uuid::new_v4(),
            hash: "sha256:abcdef1234567890".to_string(),
            size_bytes: 1024,
            storage_key: "content/ab/cd/abcdef1234567890".to_string(),
            storage_backend: StorageBackend::Local,
            reference_count: 1,
            first_uploaded_at: Utc::now() - Duration::days(30),
            last_referenced_at: Utc::now(),
            fingerprint: None,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_increment_reference() {
        let mut hash = create_test_hash();
        assert_eq!(hash.reference_count, 1);

        hash.increment_reference().unwrap();
        assert_eq!(hash.reference_count, 2);
    }

    #[test]
    fn test_decrement_reference() {
        let mut hash = create_test_hash();
        hash.reference_count = 2;

        let is_zero = hash.decrement_reference().unwrap();
        assert!(!is_zero);
        assert_eq!(hash.reference_count, 1);

        let is_zero = hash.decrement_reference().unwrap();
        assert!(is_zero);
        assert_eq!(hash.reference_count, 0);
    }

    #[test]
    fn test_decrement_reference_floor_at_zero() {
        let mut hash = create_test_hash();
        hash.reference_count = 0;

        let is_zero = hash.decrement_reference().unwrap();
        assert!(is_zero);
        assert_eq!(hash.reference_count, 0);
    }

    #[test]
    fn test_can_delete() {
        let mut hash = create_test_hash();
        assert!(!hash.can_delete());

        hash.reference_count = 0;
        assert!(hash.can_delete());
    }

    #[test]
    fn test_is_unused() {
        let mut hash = create_test_hash();
        assert!(!hash.is_unused());

        hash.reference_count = 0;
        assert!(hash.is_unused());
    }

    #[test]
    fn test_storage_saved_single_ref() {
        let hash = create_test_hash();
        assert_eq!(hash.storage_saved(), 0);
    }

    #[test]
    fn test_storage_saved_multiple_refs() {
        let mut hash = create_test_hash();
        hash.reference_count = 5;
        assert_eq!(hash.storage_saved(), 1024 * 4);
    }

    #[test]
    fn test_storage_saved_zero_refs() {
        let mut hash = create_test_hash();
        hash.reference_count = 0;
        assert_eq!(hash.storage_saved(), 0);
    }

    #[test]
    fn test_age_days() {
        let hash = create_test_hash();
        let age = hash.age_days();
        assert!(age >= 29 && age <= 31);
    }

    #[test]
    fn test_days_since_last_reference() {
        let hash = create_test_hash();
        assert_eq!(hash.days_since_last_reference(), 0);
    }

    #[test]
    fn test_check_invariants_valid() {
        let hash = create_test_hash();
        assert!(hash.check_invariants().is_ok());
    }

    #[test]
    fn test_check_invariants_negative_ref_count() {
        let mut hash = create_test_hash();
        hash.reference_count = -1;
        assert!(hash.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_empty_hash() {
        let mut hash = create_test_hash();
        hash.hash = "".to_string();
        assert!(hash.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_zero_size() {
        let mut hash = create_test_hash();
        hash.size_bytes = 0;
        assert!(hash.check_invariants().is_err());
    }

    #[test]
    fn test_builder() {
        let hash = ContentHash::builder()
            .hash("sha256:test".to_string())
            .size_bytes(2048)
            .storage_key("content/te/st/test".to_string())
            .storage_backend(StorageBackend::S3)
            .reference_count(3)
            .build()
            .unwrap();

        assert_eq!(hash.hash, "sha256:test");
        assert_eq!(hash.size_bytes, 2048);
        assert_eq!(hash.storage_backend, StorageBackend::S3);
        assert_eq!(hash.reference_count, 3);
    }

    #[test]
    fn test_builder_missing_required() {
        let result = ContentHash::builder()
            .size_bytes(1024)
            .build();
        assert!(result.is_err());
    }
}

// =============================================================================
// ConversionJob Entity Tests
// =============================================================================

mod conversion_job_tests {
    use super::*;

    fn create_test_conversion() -> ConversionJob {
        ConversionJob {
            id: Uuid::new_v4(),
            source_file_id: Uuid::new_v4(),
            target_format: "webp".to_string(),
            status: ConversionStatus::Pending,
            conversion_options: None,
            result_file_id: None,
            progress: 0,
            error_message: None,
            started_at: None,
            completed_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_update_progress_valid() {
        let mut job = create_test_conversion();
        job.update_progress(50).unwrap();
        assert_eq!(job.progress, 50);
    }

    #[test]
    fn test_update_progress_invalid_over_100() {
        let mut job = create_test_conversion();
        assert!(job.update_progress(101).is_err());
    }

    #[test]
    fn test_update_progress_invalid_negative() {
        let mut job = create_test_conversion();
        assert!(job.update_progress(-1).is_err());
    }

    #[test]
    fn test_complete() {
        let mut job = create_test_conversion();
        let result_id = Uuid::new_v4();

        job.complete(result_id).unwrap();

        assert_eq!(job.status, ConversionStatus::Completed);
        assert_eq!(job.result_file_id, Some(result_id));
        assert_eq!(job.progress, 100);
        assert!(job.completed_at.is_some());
    }

    #[test]
    fn test_fail() {
        let mut job = create_test_conversion();
        job.fail("unsupported format".to_string()).unwrap();

        assert_eq!(job.status, ConversionStatus::Failed);
        assert_eq!(job.error_message, Some("unsupported format".to_string()));
        assert!(job.completed_at.is_some());
    }

    #[test]
    fn test_is_complete() {
        let mut job = create_test_conversion();
        assert!(!job.is_complete());

        job.complete(Uuid::new_v4()).unwrap();
        assert!(job.is_complete());
    }

    #[test]
    fn test_get_progress_percentage() {
        let mut job = create_test_conversion();
        assert_eq!(job.get_progress_percentage(), 0);

        job.progress = 75;
        assert_eq!(job.get_progress_percentage(), 75);
    }

    #[test]
    fn test_check_invariants_valid() {
        let job = create_test_conversion();
        assert!(job.check_invariants().is_ok());
    }

    #[test]
    fn test_check_invariants_completed_no_result() {
        let mut job = create_test_conversion();
        job.status = ConversionStatus::Completed;
        job.result_file_id = None;
        assert!(job.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_failed_no_error() {
        let mut job = create_test_conversion();
        job.status = ConversionStatus::Failed;
        job.error_message = None;
        assert!(job.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_invalid_progress() {
        let mut job = create_test_conversion();
        job.progress = 150;
        assert!(job.check_invariants().is_err());
    }

    #[test]
    fn test_conversion_lifecycle() {
        let mut job = create_test_conversion();

        job.update_progress(25).unwrap();
        job.update_progress(50).unwrap();
        job.update_progress(75).unwrap();

        let result_id = Uuid::new_v4();
        job.complete(result_id).unwrap();

        assert!(job.is_complete());
        assert_eq!(job.progress, 100);
        assert_eq!(job.result_file_id, Some(result_id));
        assert!(job.check_invariants().is_ok());
    }

    #[test]
    fn test_builder() {
        let source_id = Uuid::new_v4();
        let job = ConversionJob::builder()
            .source_file_id(source_id)
            .target_format("png".to_string())
            .status(ConversionStatus::Pending)
            .build()
            .unwrap();

        assert_eq!(job.source_file_id, source_id);
        assert_eq!(job.target_format, "png");
        assert_eq!(job.progress, 0);
    }
}

// =============================================================================
// UploadSession Entity Tests
// =============================================================================

mod upload_session_tests {
    use super::*;

    fn create_test_session() -> UploadSession {
        UploadSession {
            id: Uuid::new_v4(),
            bucket_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            path: "uploads/large-file.zip".to_string(),
            filename: "large-file.zip".to_string(),
            mime_type: Some("application/zip".to_string()),
            file_size: 100 * 1024 * 1024,
            chunk_size: 10 * 1024 * 1024,
            total_chunks: 10,
            uploaded_chunks: 0,
            status: UploadStatus::Initiated,
            storage_backend: StorageBackend::Local,
            completed_parts: vec![],
            part_etags: None,
            expires_at: Utc::now() + Duration::hours(24),
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_is_expired_not_expired() {
        let session = create_test_session();
        assert!(!session.is_expired());
    }

    #[test]
    fn test_is_expired_expired() {
        let mut session = create_test_session();
        session.expires_at = Utc::now() - Duration::seconds(1);
        assert!(session.is_expired());
    }

    #[test]
    fn test_add_part_success() {
        let mut session = create_test_session();
        session.add_part(1, "etag1".to_string()).unwrap();

        assert_eq!(session.uploaded_chunks, 1);
        assert!(session.completed_parts.contains(&1));
        assert_eq!(session.status, UploadStatus::Uploading);
    }

    #[test]
    fn test_add_part_duplicate() {
        let mut session = create_test_session();
        session.add_part(1, "etag1".to_string()).unwrap();

        let result = session.add_part(1, "etag1_dup".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_add_part_expired_session() {
        let mut session = create_test_session();
        session.expires_at = Utc::now() - Duration::seconds(1);

        let result = session.add_part(1, "etag1".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_add_part_all_chunks_uploaded() {
        let mut session = create_test_session();
        session.uploaded_chunks = 10;
        session.total_chunks = 10;

        let result = session.add_part(11, "etag".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_is_complete() {
        let mut session = create_test_session();
        assert!(!session.is_complete());

        session.uploaded_chunks = 10;
        assert!(session.is_complete());
    }

    #[test]
    fn test_calculate_progress() {
        let mut session = create_test_session();
        assert_eq!(session.calculate_progress(), 0);

        session.uploaded_chunks = 5;
        assert_eq!(session.calculate_progress(), 50);

        session.uploaded_chunks = 10;
        assert_eq!(session.calculate_progress(), 100);
    }

    #[test]
    fn test_remaining_chunks() {
        let mut session = create_test_session();
        assert_eq!(session.remaining_chunks(), 10);

        session.uploaded_chunks = 7;
        assert_eq!(session.remaining_chunks(), 3);
    }

    #[test]
    fn test_can_resume_initiated() {
        let session = create_test_session();
        assert!(session.can_resume());
    }

    #[test]
    fn test_can_resume_uploading() {
        let mut session = create_test_session();
        session.status = UploadStatus::Uploading;
        assert!(session.can_resume());
    }

    #[test]
    fn test_can_resume_expired() {
        let mut session = create_test_session();
        session.expires_at = Utc::now() - Duration::seconds(1);
        assert!(!session.can_resume());
    }

    #[test]
    fn test_can_resume_completed() {
        let mut session = create_test_session();
        session.status = UploadStatus::Completed;
        assert!(!session.can_resume());
    }

    #[test]
    fn test_mark_complete_success() {
        let mut session = create_test_session();
        session.uploaded_chunks = 10;

        session.mark_complete().unwrap();
        assert_eq!(session.status, UploadStatus::Completed);
    }

    #[test]
    fn test_mark_complete_not_done() {
        let mut session = create_test_session();
        session.uploaded_chunks = 5;

        let result = session.mark_complete();
        assert!(result.is_err());
    }

    #[test]
    fn test_mark_failed() {
        let mut session = create_test_session();
        session.mark_failed("network error".to_string()).unwrap();

        assert_eq!(session.status, UploadStatus::Failed);
    }

    #[test]
    fn test_check_invariants_valid() {
        let session = create_test_session();
        assert!(session.check_invariants().is_ok());
    }

    #[test]
    fn test_check_invariants_uploaded_exceeds_total() {
        let mut session = create_test_session();
        session.uploaded_chunks = 15;
        session.total_chunks = 10;
        assert!(session.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_chunk_count_mismatch() {
        let mut session = create_test_session();
        session.uploaded_chunks = 3;
        session.completed_parts = vec![1, 2]; // mismatch
        assert!(session.check_invariants().is_err());
    }

    #[test]
    fn test_check_invariants_small_chunk_size() {
        let mut session = create_test_session();
        session.chunk_size = 1024;
        assert!(session.check_invariants().is_err());
    }

    #[test]
    fn test_upload_lifecycle() {
        let mut session = create_test_session();
        session.total_chunks = 3;

        session.add_part(1, "etag1".to_string()).unwrap();
        assert_eq!(session.calculate_progress(), 33);

        session.add_part(2, "etag2".to_string()).unwrap();
        assert_eq!(session.calculate_progress(), 66);

        session.add_part(3, "etag3".to_string()).unwrap();
        assert!(session.is_complete());

        session.mark_complete().unwrap();
        assert_eq!(session.status, UploadStatus::Completed);
    }
}

// =============================================================================
// Thumbnail Entity Tests (Builder)
// =============================================================================

mod thumbnail_tests {
    use super::*;

    #[test]
    fn test_builder_all_required_fields() {
        let file_id = Uuid::new_v4();
        let thumb = Thumbnail::builder()
            .file_id(file_id)
            .size(ThumbnailSize::Medium)
            .width(128)
            .height(128)
            .storage_key("thumbs/abc/medium.webp".to_string())
            .size_bytes(4096)
            .build()
            .unwrap();

        assert_eq!(thumb.file_id, file_id);
        assert_eq!(thumb.size, ThumbnailSize::Medium);
        assert_eq!(thumb.width, 128);
        assert_eq!(thumb.height, 128);
        assert_eq!(thumb.mime_type, "image/webp");
        assert_eq!(thumb.format, "webp");
        assert_eq!(thumb.quality, 80);
        assert!(!thumb.is_stale);
    }

    #[test]
    fn test_builder_custom_format() {
        let thumb = Thumbnail::builder()
            .file_id(Uuid::new_v4())
            .size(ThumbnailSize::Large)
            .width(256)
            .height(256)
            .storage_key("thumbs/abc/large.png".to_string())
            .size_bytes(8192)
            .mime_type("image/png".to_string())
            .format("png".to_string())
            .quality(90)
            .is_stale(true)
            .build()
            .unwrap();

        assert_eq!(thumb.mime_type, "image/png");
        assert_eq!(thumb.format, "png");
        assert_eq!(thumb.quality, 90);
        assert!(thumb.is_stale);
    }

    #[test]
    fn test_builder_missing_required() {
        let result = Thumbnail::builder()
            .file_id(Uuid::new_v4())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_with_cdn_url() {
        let thumb = Thumbnail::builder()
            .file_id(Uuid::new_v4())
            .size(ThumbnailSize::Small)
            .width(64)
            .height(64)
            .storage_key("thumbs/small.webp".to_string())
            .size_bytes(2048)
            .cdn_url("https://cdn.example.com/thumbs/small.webp".to_string())
            .cache_expires_at(Utc::now() + Duration::hours(24))
            .build()
            .unwrap();

        assert!(thumb.cdn_url.is_some());
        assert!(thumb.cache_expires_at.is_some());
    }

    #[test]
    fn test_all_thumbnail_sizes() {
        let sizes = [
            ThumbnailSize::Tiny,
            ThumbnailSize::Small,
            ThumbnailSize::Medium,
            ThumbnailSize::Large,
            ThumbnailSize::Xlarge,
        ];

        for size in sizes {
            let thumb = Thumbnail::builder()
                .file_id(Uuid::new_v4())
                .size(size.clone())
                .width(64)
                .height(64)
                .storage_key(format!("thumbs/{:?}.webp", size))
                .size_bytes(1024)
                .build()
                .unwrap();

            assert_eq!(thumb.size, size);
        }
    }
}

// =============================================================================
// Owner Context Tests
// =============================================================================

mod owner_context_tests {
    use super::*;

    fn create_file_with_owner(
        owner_module: Option<&str>,
        owner_entity: Option<&str>,
        owner_entity_id: Option<Uuid>,
        field_name: Option<&str>,
        sort_order: i32,
    ) -> StoredFile {
        StoredFile {
            id: Uuid::new_v4(),
            bucket_id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            path: "uploads/photo.jpg".to_string(),
            original_name: "photo.jpg".to_string(),
            size_bytes: 2048,
            mime_type: "image/jpeg".to_string(),
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
            owner_module: owner_module.map(String::from),
            owner_entity: owner_entity.map(String::from),
            owner_entity_id,
            field_name: field_name.map(String::from),
            sort_order,
            status: FileStatus::Active,
            storage_key: format!("bucket/{}", Uuid::new_v4()),
            version: 1,
            previous_version_id: None,
            download_count: 0,
            last_accessed_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    #[test]
    fn test_file_with_owner_context() {
        let entity_id = Uuid::new_v4();
        let file = create_file_with_owner(
            Some("bersihir"),
            Some("OrderDelivery"),
            Some(entity_id),
            Some("proof_photos"),
            1,
        );

        assert_eq!(file.owner_module.as_deref(), Some("bersihir"));
        assert_eq!(file.owner_entity.as_deref(), Some("OrderDelivery"));
        assert_eq!(file.owner_entity_id, Some(entity_id));
        assert_eq!(file.field_name.as_deref(), Some("proof_photos"));
        assert_eq!(file.sort_order, 1);
    }

    #[test]
    fn test_file_without_owner_context() {
        let file = create_file_with_owner(None, None, None, None, 0);

        assert!(file.owner_module.is_none());
        assert!(file.owner_entity.is_none());
        assert!(file.owner_entity_id.is_none());
        assert!(file.field_name.is_none());
        assert_eq!(file.sort_order, 0);
    }

    #[test]
    fn test_multiple_files_sort_order() {
        let entity_id = Uuid::new_v4();
        let mut files: Vec<StoredFile> = (0..3)
            .map(|i| {
                create_file_with_owner(
                    Some("bersihir"),
                    Some("OrderDelivery"),
                    Some(entity_id),
                    Some("proof_photos"),
                    i,
                )
            })
            .collect();

        files.sort_by_key(|f| f.sort_order);

        assert_eq!(files[0].sort_order, 0);
        assert_eq!(files[1].sort_order, 1);
        assert_eq!(files[2].sort_order, 2);
    }

    #[test]
    fn test_owner_context_different_entities() {
        let module = "bersihir";
        let delivery_file = create_file_with_owner(
            Some(module),
            Some("OrderDelivery"),
            Some(Uuid::new_v4()),
            Some("proof_photos"),
            0,
        );
        let agent_file = create_file_with_owner(
            Some(module),
            Some("Agent"),
            Some(Uuid::new_v4()),
            Some("profile_photo"),
            0,
        );

        assert_eq!(delivery_file.owner_module, agent_file.owner_module);
        assert_ne!(delivery_file.owner_entity, agent_file.owner_entity);
        assert_ne!(delivery_file.field_name, agent_file.field_name);
    }

    #[test]
    fn test_owner_context_different_modules() {
        let entity_id = Uuid::new_v4();
        let bersihir_file = create_file_with_owner(
            Some("bersihir"),
            Some("OrderDelivery"),
            Some(entity_id),
            Some("proof_photos"),
            0,
        );
        let sapiens_file = create_file_with_owner(
            Some("sapiens"),
            Some("User"),
            Some(entity_id),
            Some("avatar"),
            0,
        );

        assert_ne!(bersihir_file.owner_module, sapiens_file.owner_module);
        assert_eq!(bersihir_file.owner_module.as_deref(), Some("bersihir"));
        assert_eq!(sapiens_file.owner_module.as_deref(), Some("sapiens"));
    }

    #[test]
    fn test_sort_order_for_before_after_photos() {
        let delivery_id = Uuid::new_v4();
        let before_photo = create_file_with_owner(
            Some("bersihir"),
            Some("OrderDelivery"),
            Some(delivery_id),
            Some("proof_photos"),
            0,
        );
        let after_photo = create_file_with_owner(
            Some("bersihir"),
            Some("OrderDelivery"),
            Some(delivery_id),
            Some("proof_photos"),
            1,
        );

        let mut photos = vec![&after_photo, &before_photo];
        photos.sort_by_key(|f| f.sort_order);

        assert_eq!(photos[0].sort_order, 0);
        assert_eq!(photos[1].sort_order, 1);
        assert_eq!(photos[0].id, before_photo.id);
        assert_eq!(photos[1].id, after_photo.id);
    }

    #[test]
    fn test_partial_owner_context() {
        let file = create_file_with_owner(Some("bersihir"), None, None, None, 0);

        assert_eq!(file.owner_module.as_deref(), Some("bersihir"));
        assert!(file.owner_entity.is_none());
        assert!(file.owner_entity_id.is_none());
        assert!(file.field_name.is_none());
        assert_eq!(file.sort_order, 0);
    }
}
