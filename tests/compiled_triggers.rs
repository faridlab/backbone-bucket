//! The compiled hook triggers (ADR-0034) of bucket, driven the way they run: through the
//! generated services, inside the write's transaction. The live tests run on a fresh database
//! (`sqlx::test`) with the module migrations applied; `declared::migrate_outbox` creates the
//! outbox table the composing service creates at startup.
//!
//! What each test pins, against the legacy path (where nothing ran the declarations):
//!
//! - a compiled `set:` before the write fills the row it declared it would fill;
//! - a compiled `emit:` stages its declared event on the write's own transaction;
//! - an `after_update_<field>` fires only when the field changed;
//! - a scheduled entry dispatches by name to its method;
//! - the ported hand methods keep the guard and behaviour of the hand code they came from.

use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use backbone_bucket::application::service::{build_bucket_service, build_user_quota_service};
use backbone_bucket::application::triggers::{
    run_user_quota_scheduled, BucketTriggers, BucketTriggersImpl, FileLockTriggers,
    FileLockTriggersImpl, FileShareTriggers, FileShareTriggersImpl, StoredFileTriggers,
    StoredFileTriggersImpl, UserQuotaTriggersImpl, FILE_LOCK_SCHEDULED, USER_QUOTA_SCHEDULED,
};
use backbone_bucket::domain::entity::{
    Bucket, FileShare, FileStatus, SharePermission, ShareStatus, ShareType, StoredFile, UserQuota,
};
use backbone_bucket::domain::event::declared;
use backbone_bucket::infrastructure::persistence::{BucketRepository, UserQuotaRepository};
use backbone_bucket::presentation::dto::{CreateBucketDto, CreateUserQuotaDto, UpdateUserQuotaDto};
use backbone_core::write_hooks::WriteCtx;

/// The outbox rows of `aggregate`, in the order they were staged.
async fn outbox(pool: &PgPool, aggregate: &str) -> Vec<(String, String, serde_json::Value)> {
    sqlx::query_as(
        "SELECT event_type, aggregate_id, payload FROM bucket.outbox_events \
         WHERE aggregate_type = $1 ORDER BY occurred_at, id",
    )
    .bind(aggregate)
    .fetch_all(pool)
    .await
    .unwrap()
}

fn quota_create(file_count: i32) -> CreateUserQuotaDto {
    CreateUserQuotaDto {
        user_id: Uuid::new_v4(),
        limit_bytes: 1000,
        used_bytes: 100,
        file_count,
        max_file_size: None,
        max_file_count: None,
        tier: "free".to_string(),
        warning_threshold_percent: 80,
        last_warning_sent_at: None,
        peak_usage_bytes: 0,
        peak_usage_at: None,
    }
}

fn quota_update_of(quota: &UserQuota, tier: &str) -> UpdateUserQuotaDto {
    UpdateUserQuotaDto {
        user_id: quota.user_id,
        limit_bytes: quota.limit_bytes,
        used_bytes: quota.used_bytes,
        file_count: quota.file_count,
        max_file_size: quota.max_file_size,
        max_file_count: quota.max_file_count,
        tier: tier.to_string(),
        // `quota_status` is `pub(crate)`: the test flow never transitions it, so the
        // default (the state the create wrote) is what the row holds.
        quota_status: Default::default(),
        warning_threshold_percent: quota.warning_threshold_percent,
        last_warning_sent_at: quota.last_warning_sent_at,
        peak_usage_bytes: quota.peak_usage_bytes,
        peak_usage_at: quota.peak_usage_at,
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn a_quota_is_born_with_its_counters_zeroed_by_the_compiled_set(pool: PgPool) {
    declared::migrate_outbox(&pool).await.unwrap();
    let service = build_user_quota_service(Arc::new(UserQuotaRepository::new(pool.clone())));

    // The create says `file_count: 5`; the declared `set: file_count = 0` runs before the
    // write, so the row is born with the counter the declaration promised. Under the legacy
    // path nothing ran and the row stored the caller's 5.
    let quota = service.create(quota_create(5)).await.unwrap();
    assert_eq!(quota.file_count, 0, "the compiled `set: file_count = 0` ran before the store");
    assert_eq!(quota.used_bytes, 100, "no declaration zeroes used_bytes, so it is stored as given");
}

fn bucket_create(owner: Uuid, root_path: &str) -> CreateBucketDto {
    CreateBucketDto {
        name: "Shared documents".into(),
        slug: "shared-documents".into(),
        description: None,
        owner_id: owner,
        bucket_type: Default::default(),
        storage_backend: Default::default(),
        root_path: root_path.to_string(),
        file_count: 0,
        total_size_bytes: 0,
        max_file_size: None,
        allowed_mime_types: Vec::new(),
        auto_delete_after_days: None,
        enable_cdn: false,
        enable_versioning: false,
        enable_deduplication: false,
    }
}

#[sqlx::test(migrations = "./migrations")]
async fn a_buckets_creation_is_staged_in_tx_and_a_given_root_path_kept(pool: PgPool) {
    declared::migrate_outbox(&pool).await.unwrap();
    let owner = Uuid::new_v4();
    let service = build_bucket_service(Arc::new(BucketRepository::new(pool.clone())));

    // `if: root_path == null || root_path == '' { set: root_path = owner_id + '/' + slug }`
    // is a compiled `set:` of `before_create`: with a path given, its guard is false and the
    // row keeps what the caller wrote — exactly the legacy outcome for this create.
    let bucket = service.create(bucket_create(owner, "docs/root")).await.unwrap();
    assert_eq!(bucket.root_path, "docs/root", "a present root_path is never rewritten");

    // `emit: BucketCreatedEvent(...)` is compiled and stages on the write's transaction:
    // the row above committed, so the event committed with it.
    let rows = outbox(&pool, "Bucket").await;
    let (event, aggregate_id, payload) = rows.last().expect("the creation staged its event");
    assert_eq!(event, "BucketCreatedEvent");
    assert_eq!(aggregate_id, &bucket.id.to_string());
    assert_eq!(payload["bucket_id"], bucket.id.to_string(), "{payload}");
    assert_eq!(payload["owner_id"], owner.to_string(), "{payload}");
    assert_eq!(payload["name"], "Shared documents", "{payload}");
}

#[sqlx::test(migrations = "./migrations")]
async fn an_empty_root_path_is_refused_before_the_declared_fill_could_run(pool: PgPool) {
    declared::migrate_outbox(&pool).await.unwrap();
    let service = build_bucket_service(Arc::new(BucketRepository::new(pool.clone())));

    // The write guard's `@required` on `root_path` refuses an empty value BEFORE the
    // `before_create` triggers run, so the declared fill (`set: root_path = owner_id + '/'
    // slug`) is unreachable through the generic create — and identically was refused under
    // the legacy path, where nothing ran at all. The port keeps that order.
    let refused = service.create(bucket_create(Uuid::new_v4(), "")).await;
    let violations = match refused {
        Err(backbone_core::ServiceError::Violations(v)) => v,
        other => panic!("expected a refusal, got {other:?}"),
    };
    assert!(
        violations
            .iter()
            .any(|v| v.path == "root_path" && v.code == "required"),
        "{violations:?}"
    );
    assert!(
        outbox(&pool, "Bucket").await.is_empty(),
        "a refused write leaves no staged event behind"
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn a_tier_change_stages_a_notification_an_unchanged_tier_stages_none(pool: PgPool) {
    declared::migrate_outbox(&pool).await.unwrap();
    let service = build_user_quota_service(Arc::new(UserQuotaRepository::new(pool.clone())));
    let quota = service.create(quota_create(0)).await.unwrap();
    assert!(
        outbox(&pool, "UserQuota").await.is_empty(),
        "the create's compiled steps stage nothing for this aggregate"
    );

    let updated = service
        .update(&quota.id.to_string(), quota_update_of(&quota, "pro"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.tier, "pro");

    // `after_update_tier` is keyed on the field change: the compiled `notify` stages a
    // `NotificationRequested` on the write's transaction; the `send_email` and `emit` beside
    // it are hand methods, and their no-ops stage nothing.
    let rows = outbox(&pool, "UserQuota").await;
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!(rows[0].0, "NotificationRequested");
    let body = rows[0].2["body"].as_str().unwrap_or_default();
    assert!(body.contains("pro"), "the message interpolates the new tier: {}", rows[0].2);

    // The same row written again with the tier unchanged does not fire the trigger.
    let same = service
        .update(&updated.id.to_string(), quota_update_of(&updated, "pro"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(same.tier, "pro");
    assert_eq!(outbox(&pool, "UserQuota").await.len(), 1, "the field did not change");
}

#[tokio::test]
async fn scheduled_entries_dispatch_by_name_to_their_methods() {
    // The module's job list carries the scheduled trigger entries (ADR-0020); the composing
    // service's scheduler decides when. Here: the entries are listed, dispatch works, an
    // unknown name answers false, and a method runs.
    let names: Vec<&str> = USER_QUOTA_SCHEDULED.iter().map(|j| j.name).collect();
    for expected in [
        "scheduled_reset_daily_bandwidth",
        "scheduled_reset_monthly_bandwidth",
        "scheduled_expire_quotas",
        "scheduled_send_expiry_warnings",
    ] {
        assert!(names.contains(&expected), "{names:?}");
    }
    assert_eq!(FILE_LOCK_SCHEDULED.len(), 2, "{FILE_LOCK_SCHEDULED:?}");

    let hand = UserQuotaTriggersImpl;
    assert!(run_user_quota_scheduled(&hand, "scheduled_expire_quotas").await.unwrap());
    assert!(!run_user_quota_scheduled(&hand, "no_such_entry").await.unwrap());

    // The lock sweep's method is the ported no-op: it runs and changes nothing.
    let ctx = WriteCtx::current(Some("bucket"));
    FileLockTriggersImpl
        .scheduled__expire_overdue_locks(&ctx)
        .await
        .unwrap();
}

#[tokio::test]
async fn derive_slug_fills_an_empty_slug_and_keeps_a_given_one() {
    // The ported guard of `if: slug == null || slug == '' { set: slug = slugify(name) }`,
    // from `domain::utils::slugify`. (The write guard's `@required` refuses an empty slug
    // before the triggers run, so through the generic create this arm is unreachable; the
    // method is what a hand verb building a Bucket without a slug lands on.)
    let ctx = WriteCtx::current(Some("bucket"));
    let mut nameless = bucket_with("My Bucket Name!", "");
    BucketTriggersImpl
        .before_create__derive_slug(&ctx, &mut nameless)
        .await
        .unwrap();
    assert_eq!(nameless.slug, "my-bucket-name");

    let mut given = bucket_with("Other", "kept");
    BucketTriggersImpl
        .before_create__derive_slug(&ctx, &mut given)
        .await
        .unwrap();
    assert_eq!(given.slug, "kept", "a present slug is never overwritten");
}

#[tokio::test]
async fn ensure_token_mints_a_token_only_when_absent() {
    // The ported guard of `if: token == null || token == '' { set: token =
    // generate_secure_token(32) }`, from `domain::utils::generate_share_token`.
    let ctx = WriteCtx::current(Some("bucket"));
    let mut absent = share_with("");
    FileShareTriggersImpl.before_create__ensure_token(&ctx, &mut absent)
        .await
        .unwrap();
    assert_eq!(absent.token.len(), 32, "the declaration names a 32-character token");

    let mut present = share_with("kept-verbatim-0123456789abcdef");
    FileShareTriggersImpl.before_create__ensure_token(&ctx, &mut present)
        .await
        .unwrap();
    assert_eq!(present.token, "kept-verbatim-0123456789abcdef");
}

#[sqlx::test(migrations = "./migrations")]
async fn file_activation_accounts_the_quota_as_record_completed_usage_does(pool: PgPool) {
    use backbone_bucket::application::triggers::bind_quota_bookkeeping;

    declared::migrate_outbox(&pool).await.unwrap();
    let quota_repo = Arc::new(UserQuotaRepository::new(pool.clone()));
    let quota_service = Arc::new(build_user_quota_service(quota_repo.clone()));
    bind_quota_bookkeeping(&quota_service, &quota_repo);

    let mut dto = quota_create(0);
    dto.user_id = Uuid::new_v4();
    let user = dto.user_id;
    let quota = quota_service.create(dto).await.unwrap();
    // The create's compiled `set:` zeroed the counter; used_bytes was stored as given.
    assert_eq!((quota.used_bytes, quota.file_count), (100, 0));

    // A 50-byte file of that owner becomes active: the ported hand method mirrors
    // `MultipartUploadService::record_completed_usage` — usage and count rise, peak follows.
    let file = file_of(user, 50);
    let ctx = WriteCtx::current(Some("bucket"));
    StoredFileTriggersImpl
        .on_enter_active__update_user_quota(&ctx, &file, &file)
        .await
        .unwrap();
    let after = quota_repo.find_by_user_id(user).await.unwrap().unwrap();
    assert_eq!((after.used_bytes, after.file_count), (150, 1));
    assert_eq!(after.peak_usage_bytes, 150, "the peak follows the new high");

    // The file is deleted again: the symmetric decrement, peak never lowered.
    StoredFileTriggersImpl
        .on_enter_deleted__update_user_quota(&ctx, &file, &file)
        .await
        .unwrap();
    let restored = quota_repo.find_by_user_id(user).await.unwrap().unwrap();
    assert_eq!((restored.used_bytes, restored.file_count), (100, 0));
    assert_eq!(restored.peak_usage_bytes, 150, "the peak is a high-water mark");
}

fn bucket_with(name: &str, slug: &str) -> Bucket {
    Bucket::builder()
        .name(name.to_string())
        .slug(slug.to_string())
        .owner_id(Uuid::new_v4())
        .root_path("root".to_string())
        .allowed_mime_types(Vec::new())
        .build()
        .unwrap()
}

fn share_with(token: &str) -> FileShare {
    FileShare::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        token.to_string(),
        ShareType::Link,
        SharePermission::View,
        Vec::new(),
        0,
        ShareStatus::Active,
    )
}

fn file_of(owner: Uuid, size: i64) -> StoredFile {
    StoredFile::new(
        Uuid::new_v4(),
        owner,
        "docs/a.pdf".into(),
        "a.pdf".into(),
        size,
        "application/pdf".into(),
        false,
        false,
        false,
        false,
        false,
        0,
        FileStatus::Active,
        "key".into(),
        1,
        0,
    )
}
