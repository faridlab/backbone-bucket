//! The module's engine workflows (ADR-0035), driven the way they run: a delivered
//! outbox event starts an instance, the steps write through the generated services,
//! and the bespoke verbs run against the runtime the module's build binds. The live
//! test runs on a fresh database (`sqlx::test`) with the module migrations applied.
//!
//! What each test pins:
//!
//! - a delivered `ProcessingJobCreatedEvent` starts one `FileProcessing` instance,
//!   runs the virus-scan branch through the real services (the scan verb read the
//!   object from the bound storage), and the job ends `completed` with the file
//!   marked scanned — a replay of the same event id starts nothing;
//! - the Actions methods the hand halves carry (checksum, mime sniffing, the threat
//!   heuristic, chunk sizing, the retry backoff, the recipient passthrough and the
//!   password gate) answer what their declarations promise.

use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use backbone_bucket::application::service as svc;
use backbone_bucket::application::workflows::{
    base64_encode, bind_workflow_runtime, build_background_tasks, FileProcessingActions,
    FileProcessingActionsImpl, FileUploadActions, FileUploadActionsImpl, MultipartUploadActions,
    MultipartUploadActionsImpl, ShareCreationActions, ShareCreationActionsImpl, WorkflowServices,
};
use backbone_bucket::domain::event::declared;
use backbone_bucket::infrastructure::persistence as repo;
use backbone_bucket::storage::{InMemoryStorage, ObjectStorage};
use backbone_core::write_hooks::WriteCtx;
use backbone_outbox::OutboxRecord;
use serde_json::{json, Value};

fn services(pool: &PgPool, storage: Arc<InMemoryStorage>) -> Arc<WorkflowServices> {
    let services = Arc::new(WorkflowServices {
        access_log: Arc::new(svc::build_access_log_service(Arc::new(repo::AccessLogRepository::new(pool.clone())))),
        bucket: Arc::new(svc::build_bucket_service(Arc::new(repo::BucketRepository::new(pool.clone())))),
        content_hash: Arc::new(svc::build_content_hash_service(Arc::new(repo::ContentHashRepository::new(pool.clone())))),
        file_share: Arc::new(svc::build_file_share_service(Arc::new(repo::FileShareRepository::new(pool.clone())))),
        processing_job: Arc::new(svc::build_processing_job_service(Arc::new(repo::ProcessingJobRepository::new(pool.clone())))),
        stored_file: Arc::new(svc::build_stored_file_service(Arc::new(repo::StoredFileRepository::new(pool.clone())))),
        upload_session: Arc::new(svc::build_upload_session_service(Arc::new(repo::UploadSessionRepository::new(pool.clone())))),
        user_quota: Arc::new(svc::build_user_quota_service(Arc::new(repo::UserQuotaRepository::new(pool.clone())))),
    });
    bind_workflow_runtime(Some(storage), services.clone());
    services
}

fn clone_shallow(s: &WorkflowServices) -> WorkflowServices {
    WorkflowServices {
        access_log: s.access_log.clone(),
        bucket: s.bucket.clone(),
        content_hash: s.content_hash.clone(),
        file_share: s.file_share.clone(),
        processing_job: s.processing_job.clone(),
        stored_file: s.stored_file.clone(),
        upload_session: s.upload_session.clone(),
        user_quota: s.user_quota.clone(),
    }
}

/// Seed the rows a virus-scan instance acts on, through the generated services,
/// putting the object into the same storage the Actions impls read.
async fn seed_scan_target(services: &WorkflowServices, storage: &InMemoryStorage) -> Uuid {
    let bucket = services
        .bucket
        .create(
            serde_json::from_value(json!({
                "name": "wf-test", "slug": format!("wf-{}", Uuid::new_v4().simple()),
                "owner_id": Uuid::new_v4().simple().to_string(), "root_path": "/wf",
                "allowed_mime_types": [],
            }))
            .unwrap(),
        )
        .await
        .unwrap();
    let object = b"clean payload, nothing to see".to_vec();
    storage.put("wf/scan.bin", object.clone().into(), "application/octet-stream").await.unwrap();
    let file = services
        .stored_file
        .create(
            serde_json::from_value(json!({
                "bucket_id": bucket.id.to_string(), "owner_id": Uuid::new_v4().simple().to_string(),
                "path": "scan.bin", "original_name": "scan.bin", "size_bytes": object.len() as i64,
                "mime_type": "application/octet-stream", "storage_key": "wf/scan.bin",
            }))
            .unwrap(),
        )
        .await
        .unwrap();
    file.id
}

fn job_created(job_id: Uuid, file_id: Uuid) -> OutboxRecord {
    OutboxRecord {
        id: Uuid::new_v4(),
        event_type: "ProcessingJobCreatedEvent".into(),
        aggregate_type: "ProcessingJob".into(),
        aggregate_id: job_id.to_string(),
        company_id: Uuid::new_v4(),
        payload: json!({ "job_id": job_id, "file_id": file_id, "job_type": "virus_scan" }),
        occurred_at: chrono::Utc::now(),
        correlation_id: None,
        causation_id: None,
        version: 1,
        org_unit_id: None,
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn a_delivered_job_event_runs_file_processing_once(pool: PgPool) {
    declared::migrate_outbox(&pool).await.unwrap();
    let storage = Arc::new(InMemoryStorage::new());
    let services = services(&pool, storage.clone());
    let file_id = seed_scan_target(&services, &storage).await;

    let job = services
        .processing_job
        .create(
            serde_json::from_value(json!({ "file_id": file_id.to_string(), "job_type": "virus_scan" })).unwrap(),
        )
        .await
        .unwrap();

    let tasks = build_background_tasks(pool.clone(), clone_shallow(&services));
    let event = job_created(job.id, file_id);
    tasks.deliver(event.clone()).await.unwrap();

    // The instance ran the scan branch end to end through the real services.
    let reloaded = services.processing_job.get_by_id(&job.id.to_string()).await.unwrap().unwrap();
    assert_eq!(reloaded.status().to_string(), "completed", "the job ran to completion");
    let scanned = services.stored_file.get_by_id(&file_id.to_string()).await.unwrap().unwrap();
    assert!(scanned.is_scanned, "the scan step wrote through the file service");
    assert_eq!(scanned.threat_level.as_ref().map(|level| level.to_string()), Some("safe".to_string()));

    let (instances, status): (i64, String) = sqlx::query_as(
        "SELECT count(*), (array_agg(status::text))[1] FROM bucket.bb_workflow_instances WHERE workflow = 'FileProcessing'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(instances, 1, "one instance per trigger event");
    assert_eq!(status, "completed");

    // The staged completion event the processing-job hooks wrote is on the outbox.
    let staged: i64 =
        sqlx::query_scalar("SELECT count(*) FROM bucket.outbox_events WHERE event_type = 'JobCompletedEvent'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(staged, 1, "entering the completed state staged the job event");

    // A replay of the same event id starts nothing.
    tasks.deliver(event).await.unwrap();
    let again: i64 =
        sqlx::query_scalar("SELECT count(*) FROM bucket.bb_workflow_instances WHERE workflow = 'FileProcessing'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(again, 1, "a replayed start is deduped");
}

#[tokio::test]
async fn the_upload_actions_answer_what_their_declarations_promise() {
    let actions = FileUploadActionsImpl::default();
    let ctx = WriteCtx::current(None);

    // sha256 of "hello" (base64 "aGVsbG8=").
    let sum = actions
        .calculate_sha256(&ctx, &json!({ "content": "aGVsbG8=" }), &Value::Null)
        .await
        .unwrap();
    assert_eq!(
        sum,
        Value::String("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".into())
    );

    // The mime sniffer reads PNG magic.
    let png = actions
        .detect_mime(&ctx, &json!({ "content": "iVBORw0KGgo=", "filename": "x.bin" }), &Value::Null)
        .await
        .unwrap();
    assert_eq!(png, Value::String("image/png".into()));

    // The threat heuristic: EICAR is unsafe, a plain payload is safe.
    let eicar = b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";
    let found = actions
        .scan_for_threats(&ctx, &json!({ "content": base64_encode(eicar), "filename": "eicar.com" }), &Value::Null)
        .await
        .unwrap();
    assert_eq!(found["is_safe"], Value::Bool(false));
    let clean = actions
        .scan_for_threats(&ctx, &json!({ "content": "cxlZWQ=", "filename": "notes.txt" }), &Value::Null)
        .await
        .unwrap();
    assert_eq!(clean["is_safe"], Value::Bool(true));
}

#[tokio::test]
async fn the_chunking_and_the_retry_backoff_answer_their_declarations() {
    let ctx = WriteCtx::current(None);
    let chunks = MultipartUploadActionsImpl::default()
        .calculate_chunk_size(
            &ctx,
            &json!({ "file_size": 629_145_600i64, "min_chunk_size": 5_242_880i64, "max_chunk_size": 5_368_709_120i64, "max_parts": 10_000i64 }),
            &Value::Null,
        )
        .await
        .unwrap();
    assert_eq!(chunks["chunk_size"], json!(5_242_880), "a 600MB upload chunks at the minimum");
    assert_eq!(chunks["total_chunks"], json!(120));

    let backoff = FileProcessingActionsImpl::default()
        .fn_exponential_backoff(&ctx, &[json!(2)])
        .await
        .unwrap();
    let at = chrono::DateTime::parse_from_rfc3339(backoff.as_str().unwrap()).unwrap().with_timezone(&chrono::Utc);
    let minutes = (at - chrono::Utc::now()).num_minutes();
    assert!((3..=6).contains(&minutes), "the third retry backs off about four minutes, got {minutes}");
}

#[tokio::test]
async fn the_share_actions_pass_recipients_through_and_gate_passwords() {
    let ctx = WriteCtx::current(None);
    let actions = ShareCreationActionsImpl::default();
    // The recipient passthrough: a user id comes back, an e-mail resolves nobody.
    let recipient = actions
        .find_share_recipient(&ctx, &json!({ "user_id": "5f0d5e4a-3f9f-4baf-9d34-1f5a9bf5b4de" }), &Value::Null)
        .await
        .unwrap();
    assert_eq!(recipient["id"], json!("5f0d5e4a-3f9f-4baf-9d34-1f5a9bf5b4de"));
    let by_email = actions
        .find_share_recipient(&ctx, &json!({ "email": "someone@example.org" }), &Value::Null)
        .await
        .unwrap();
    assert!(by_email.is_null());

    // A short password is refused before anything is written.
    let too_short = actions
        .create_share(
            &ctx,
            &json!({
                "file_id": "8f8b7c2e-1a4e-4b1f-9c6e-2d3f4a5b6c7d",
                "shared_by_id": "9e8d7c6b-5a4f-4e3d-2c1b-0a9b8c7d6e5f",
                "shared_with_user_id": "5f0d5e4a-3f9f-4baf-9d34-1f5a9bf5b4de",
                "password": "short",
            }),
            &Value::Null,
        )
        .await;
    assert!(too_short.is_err(), "a short share password is refused");
}
