//! Performance Benchmarks for bucket V2.0
//!
//! Measures entity construction, builder patterns, serialization,
//! deduplication calculations, and workflow throughput.
//!
//! Run with: cargo test --package backbone-bucket --test processing_bench --release
//! (Placed in benches/ for organization but runs as integration test)

use std::time::{Duration, Instant};

use chrono::Utc;
use uuid::Uuid;

use backbone_bucket::domain::entity::*;

// ==========================================================================
// Benchmark Helpers
// ==========================================================================

fn bench<F: FnMut()>(name: &str, iterations: u64, mut f: F) {
    // Warmup
    for _ in 0..100 {
        f();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();

    let per_op = elapsed / iterations as u32;
    let ops_per_sec = if elapsed.as_secs_f64() > 0.0 {
        iterations as f64 / elapsed.as_secs_f64()
    } else {
        f64::INFINITY
    };

    println!(
        "  {:<50} {:>10} ops  {:>12.2?} total  {:>10.0?}/op  {:>12.0} ops/sec",
        name, iterations, elapsed, per_op, ops_per_sec
    );
}

// ==========================================================================
// Entity Construction Benchmarks
// ==========================================================================

#[test]
fn bench_entity_construction() {
    println!("\n=== Entity Construction Benchmarks ===");

    bench("StoredFile::new (direct)", 100_000, || {
        let _file = StoredFile {
            id: Uuid::new_v4(),
            bucket_id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            path: "/test/file.pdf".to_string(),
            original_name: "file.pdf".to_string(),
            size_bytes: 1024 * 1024,
            mime_type: "application/pdf".to_string(),
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
            storage_key: "key".to_string(),
            version: 1,
            previous_version_id: None,
            download_count: 0,
            last_accessed_at: None,
            metadata: AuditMetadata::default(),
        };
        std::hint::black_box(_file);
    });

    bench("ContentHash::builder().build()", 100_000, || {
        let _hash = ContentHash::builder()
            .hash("sha256:test".to_string())
            .size_bytes(1024)
            .storage_key("key".to_string())
            .build()
            .unwrap();
        std::hint::black_box(_hash);
    });

    bench("ProcessingJob::builder().build()", 100_000, || {
        let _job = ProcessingJob::builder()
            .file_id(Uuid::new_v4())
            .job_type(ProcessingJobType::VideoThumbnail)
            .build()
            .unwrap();
        std::hint::black_box(_job);
    });

    bench("Thumbnail::builder().build()", 100_000, || {
        let _thumb = Thumbnail::builder()
            .file_id(Uuid::new_v4())
            .size(ThumbnailSize::Medium)
            .width(128)
            .height(128)
            .storage_key("key".to_string())
            .mime_type("image/webp".to_string())
            .format("webp".to_string())
            .quality(80)
            .size_bytes(5120)
            .generated_at(Utc::now())
            .generation_time_ms(100)
            .source_version(1)
            .is_stale(false)
            .build()
            .unwrap();
        std::hint::black_box(_thumb);
    });

    bench("UploadSession::builder().build()", 100_000, || {
        let _session = UploadSession::builder()
            .bucket_id(Uuid::new_v4())
            .user_id(Uuid::new_v4())
            .path("/test".to_string())
            .filename("file.zip".to_string())
            .file_size(100 * 1024 * 1024)
            .chunk_size(5 * 1024 * 1024)
            .total_chunks(20)
            .uploaded_chunks(0)
            .status(UploadStatus::Initiated)
            .completed_parts(vec![])
            .storage_backend(StorageBackend::Local)
            .expires_at(Utc::now() + chrono::Duration::hours(24))
            .build()
            .unwrap();
        std::hint::black_box(_session);
    });

    bench("FileLock::new()", 100_000, || {
        let now = Utc::now();
        let _lock = FileLock::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            now,
            now + chrono::Duration::minutes(30),
            LockStatus::Active,
        );
        std::hint::black_box(_lock);
    });
}

// ==========================================================================
// Domain Logic Benchmarks
// ==========================================================================

#[test]
fn bench_domain_logic() {
    println!("\n=== Domain Logic Benchmarks ===");

    // ContentHash deduplication operations
    bench("ContentHash::increment_reference()", 1_000_000, || {
        let mut hash = ContentHash::builder()
            .hash("sha256:test".to_string())
            .size_bytes(1024)
            .storage_key("key".to_string())
            .build()
            .unwrap();
        hash.increment_reference().unwrap();
        std::hint::black_box(&hash);
    });

    bench("ContentHash::storage_saved()", 1_000_000, || {
        let mut hash = ContentHash::builder()
            .hash("sha256:test".to_string())
            .size_bytes(100 * 1024 * 1024)
            .storage_key("key".to_string())
            .reference_count(10)
            .build()
            .unwrap();
        let saved = hash.storage_saved();
        std::hint::black_box(saved);
    });

    bench("ContentHash::check_invariants()", 1_000_000, || {
        let hash = ContentHash::builder()
            .hash("sha256:test".to_string())
            .size_bytes(1024)
            .storage_key("key".to_string())
            .build()
            .unwrap();
        let result = hash.check_invariants();
        std::hint::black_box(result);
    });

    // StoredFile operations
    bench("StoredFile::is_safe()", 1_000_000, || {
        let mut file = StoredFile {
            id: Uuid::nil(),
            bucket_id: Uuid::nil(),
            owner_id: Uuid::nil(),
            path: "/test".to_string(),
            original_name: "test".to_string(),
            size_bytes: 1024,
            mime_type: "text/plain".to_string(),
            checksum: None,
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
            storage_key: "key".to_string(),
            version: 1,
            previous_version_id: None,
            download_count: 0,
            last_accessed_at: None,
            metadata: AuditMetadata::default(),
        };
        let safe = file.is_safe();
        std::hint::black_box(safe);
    });

    bench("StoredFile::record_access()", 1_000_000, || {
        let mut file = StoredFile {
            id: Uuid::nil(),
            bucket_id: Uuid::nil(),
            owner_id: Uuid::nil(),
            path: "/test".to_string(),
            original_name: "test".to_string(),
            size_bytes: 1024,
            mime_type: "text/plain".to_string(),
            checksum: None,
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
            storage_key: "key".to_string(),
            version: 1,
            previous_version_id: None,
            download_count: 0,
            last_accessed_at: None,
            metadata: AuditMetadata::default(),
        };
        file.record_access();
        std::hint::black_box(&file);
    });

    // UserQuota operations
    bench("UserQuota::add_usage()", 1_000_000, || {
        let mut quota = UserQuota::builder()
            .user_id(Uuid::nil())
            .limit_bytes(10 * 1024 * 1024 * 1024)
            .build()
            .unwrap();
        quota.add_usage(1024).unwrap();
        std::hint::black_box(&quota);
    });

    bench("UserQuota::usage_percent()", 1_000_000, || {
        let mut quota = UserQuota::builder()
            .user_id(Uuid::nil())
            .limit_bytes(10 * 1024 * 1024 * 1024)
            .used_bytes(5 * 1024 * 1024 * 1024)
            .build()
            .unwrap();
        let pct = quota.usage_percent();
        std::hint::black_box(pct);
    });

    // UploadSession operations
    bench("UploadSession::add_part() x20", 100_000, || {
        let mut session = UploadSession::builder()
            .bucket_id(Uuid::nil())
            .user_id(Uuid::nil())
            .path("/test".to_string())
            .filename("file.zip".to_string())
            .file_size(100 * 1024 * 1024)
            .chunk_size(5 * 1024 * 1024)
            .total_chunks(20)
            .uploaded_chunks(0)
            .status(UploadStatus::Initiated)
            .completed_parts(vec![])
            .storage_backend(StorageBackend::Local)
            .expires_at(Utc::now() + chrono::Duration::hours(24))
            .build()
            .unwrap();

        for i in 1..=20 {
            session.add_part(i, format!("etag-{}", i)).unwrap();
        }
        std::hint::black_box(&session);
    });

    // ProcessingJob state transitions
    bench("ProcessingJob lifecycle (start->complete)", 100_000, || {
        let mut job = ProcessingJob::builder()
            .file_id(Uuid::nil())
            .job_type(ProcessingJobType::VideoThumbnail)
            .build()
            .unwrap();
        job.mark_started().unwrap();
        job.mark_completed(serde_json::json!({"ok": true})).unwrap();
        std::hint::black_box(&job);
    });
}

// ==========================================================================
// Serialization Benchmarks
// ==========================================================================

#[test]
fn bench_serialization() {
    println!("\n=== Serialization Benchmarks ===");

    let file = StoredFile {
        id: Uuid::new_v4(),
        bucket_id: Uuid::new_v4(),
        owner_id: Uuid::new_v4(),
        path: "/documents/report.pdf".to_string(),
        original_name: "report.pdf".to_string(),
        size_bytes: 5 * 1024 * 1024,
        mime_type: "application/pdf".to_string(),
        checksum: Some("sha256:abcdef123456".to_string()),
        is_compressed: true,
        original_size: Some(10 * 1024 * 1024),
        compression_algorithm: Some("gzip".to_string()),
        is_scanned: true,
        scan_result: Some(serde_json::json!({"result": "clean"})),
        threat_level: Some(ThreatLevel::Safe),
        has_thumbnail: true,
        thumbnail_path: Some("/thumbs/report.webp".to_string()),
        has_video_thumbnail: false,
        has_document_preview: true,
        processing_status: Some(ProcessingStatus::Complete),
        content_hash_id: Some(Uuid::new_v4()),
        cdn_url: Some("/cdn/files/report.pdf?expires=12345".to_string()),
        cdn_url_expires_at: Some(Utc::now()),
        owner_module: Some("bersihir".to_string()),
        owner_entity: Some("Product".to_string()),
        owner_entity_id: Some(Uuid::new_v4()),
        field_name: Some("image_url".to_string()),
        sort_order: 0,
        status: FileStatus::Active,
        storage_key: "files/abcdef".to_string(),
        version: 3,
        previous_version_id: Some(Uuid::new_v4()),
        download_count: 42,
        last_accessed_at: Some(Utc::now()),
        metadata: AuditMetadata::new(),
    };

    bench("StoredFile serialize (JSON)", 100_000, || {
        let json = serde_json::to_string(&file).unwrap();
        std::hint::black_box(json);
    });

    let json = serde_json::to_string(&file).unwrap();
    bench("StoredFile deserialize (JSON)", 100_000, || {
        let f: StoredFile = serde_json::from_str(&json).unwrap();
        std::hint::black_box(f);
    });

    let hash = ContentHash::builder()
        .hash("sha256:benchmark_hash_value".to_string())
        .size_bytes(100 * 1024 * 1024)
        .storage_key("content/sha256_benchmark".to_string())
        .reference_count(5)
        .fingerprint("fp-1234".to_string())
        .build()
        .unwrap();

    bench("ContentHash serialize (JSON)", 100_000, || {
        let json = serde_json::to_string(&hash).unwrap();
        std::hint::black_box(json);
    });

    let hash_json = serde_json::to_string(&hash).unwrap();
    bench("ContentHash deserialize (JSON)", 100_000, || {
        let h: ContentHash = serde_json::from_str(&hash_json).unwrap();
        std::hint::black_box(h);
    });
}

// ==========================================================================
// Batch Operations Benchmarks
// ==========================================================================

#[test]
fn bench_batch_operations() {
    println!("\n=== Batch Operations Benchmarks ===");

    bench("Create 1000 StoredFile entities", 100, || {
        let files: Vec<StoredFile> = (0..1000)
            .map(|i| StoredFile {
                id: Uuid::new_v4(),
                bucket_id: Uuid::nil(),
                owner_id: Uuid::nil(),
                path: format!("/files/file_{}.dat", i),
                original_name: format!("file_{}.dat", i),
                size_bytes: 1024 * (i + 1) as i64,
                mime_type: "application/octet-stream".to_string(),
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
                storage_key: format!("key_{}", i),
                version: 1,
                previous_version_id: None,
                download_count: 0,
                last_accessed_at: None,
                metadata: AuditMetadata::default(),
            })
            .collect();
        std::hint::black_box(files);
    });

    bench("Serialize 1000 StoredFiles to JSON array", 10, || {
        let files: Vec<StoredFile> = (0..1000)
            .map(|i| StoredFile {
                id: Uuid::new_v4(),
                bucket_id: Uuid::nil(),
                owner_id: Uuid::nil(),
                path: format!("/files/file_{}.dat", i),
                original_name: format!("file_{}.dat", i),
                size_bytes: 1024 * (i + 1) as i64,
                mime_type: "application/octet-stream".to_string(),
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
                storage_key: format!("key_{}", i),
                version: 1,
                previous_version_id: None,
                download_count: 0,
                last_accessed_at: None,
                metadata: AuditMetadata::default(),
            })
            .collect();
        let json = serde_json::to_string(&files).unwrap();
        std::hint::black_box(json);
    });

    bench("Dedup: process 100 hash lookups + increments", 10_000, || {
        let mut hashes: Vec<ContentHash> = (0..100)
            .map(|i| {
                ContentHash::builder()
                    .hash(format!("sha256:hash_{}", i))
                    .size_bytes(1024 * 1024)
                    .storage_key(format!("content/{}", i))
                    .build()
                    .unwrap()
            })
            .collect();

        for hash in &mut hashes {
            hash.increment_reference().unwrap();
        }

        let total_saved: i64 = hashes.iter().map(|h| h.storage_saved()).sum();
        std::hint::black_box(total_saved);
    });

    bench("Quota: 100 sequential add_usage calls", 100_000, || {
        let mut quota = UserQuota::builder()
            .user_id(Uuid::nil())
            .limit_bytes(1024 * 1024 * 1024 * 1024)
            .build()
            .unwrap();

        for _ in 0..100 {
            quota.add_usage(1024 * 1024).unwrap();
        }
        std::hint::black_box(&quota);
    });
}

// ==========================================================================
// Invariant Check Benchmarks
// ==========================================================================

#[test]
fn bench_invariant_checks() {
    println!("\n=== Invariant Check Benchmarks ===");

    let bucket = Bucket::builder()
        .name("bench-bucket".to_string())
        .slug("bench-bucket".to_string())
        .owner_id(Uuid::new_v4())
        .root_path("/data/bench".to_string())
        .allowed_mime_types(vec![])
        .build()
        .unwrap();

    bench("Bucket::check_invariants()", 1_000_000, || {
        let result = bucket.check_invariants();
        std::hint::black_box(result);
    });

    let file = StoredFile {
        id: Uuid::nil(),
        bucket_id: Uuid::nil(),
        owner_id: Uuid::nil(),
        path: "/test".to_string(),
        original_name: "test".to_string(),
        size_bytes: 1024,
        mime_type: "text/plain".to_string(),
        checksum: None,
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
        storage_key: "key".to_string(),
        version: 1,
        previous_version_id: None,
        download_count: 0,
        last_accessed_at: None,
        metadata: AuditMetadata::default(),
    };

    bench("StoredFile::check_invariants()", 1_000_000, || {
        let result = file.check_invariants();
        std::hint::black_box(result);
    });

    let quota = UserQuota::builder()
        .user_id(Uuid::nil())
        .limit_bytes(10 * 1024 * 1024 * 1024)
        .used_bytes(5 * 1024 * 1024 * 1024)
        .build()
        .unwrap();

    bench("UserQuota::check_invariants()", 1_000_000, || {
        let result = quota.check_invariants();
        std::hint::black_box(result);
    });

    let share = FileShare::builder()
        .file_id(Uuid::nil())
        .owner_id(Uuid::nil())
        .token("test-token".to_string())
        .shared_with(vec![])
        .max_downloads(100)
        .build()
        .unwrap();

    bench("FileShare::check_invariants()", 1_000_000, || {
        let result = share.check_invariants();
        std::hint::black_box(result);
    });
}
