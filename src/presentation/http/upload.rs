//! Multipart HTTP upload surface (custom — never regenerated).
//!
//! Two flows, mirroring `docs/serving.md` plus `schema/workflows/multipart_upload.workflow.yaml`:
//!
//! 1. **Single-shot** — `POST /uploads`, `multipart/form-data`. Whole file
//!    in one request. Routes through [`FileService::upload`] which writes
//!    bytes to the configured [`ObjectStorage`] and persists a
//!    `stored_files` row. Suitable for files up to a few hundred MB.
//!
//! 2. **Resumable / chunked** — four endpoints backed by
//!    [`MultipartUploadService`]:
//!      - `POST   /uploads/sessions`              JSON, initiate
//!      - `POST   /uploads/sessions/:id/parts/:n` multipart, upload chunk N
//!      - `POST   /uploads/sessions/:id/complete` JSON, finalize
//!      - `DELETE /uploads/sessions/:id`          abort
//!
//! Chunks are stored under `sessions/{session_id}/parts/{n}` in the
//! private bucket; `complete` concatenates parts in order, writes the
//! final object under the caller-supplied path, registers a `stored_files`
//! row, and deletes the staged parts. Backends that expose a native
//! multipart API (e.g. S3) can later replace this assembly step without
//! changing the HTTP contract — see the TODO in [`complete_session`].
//!
//! Auth is pluggable the same way [`crate::presentation::http::serving`]
//! does it: the route is generic over an [`AuthExtractor`] that also
//! implements [`HasOwnerId`], so `owner_id` is derived from the
//! authenticated identity rather than trusted from the request body.

use std::sync::Arc;

use axum::extract::{DefaultBodyLimit, Extension, Multipart, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, post};
use axum::{Json, Router};
use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::service::BucketService;
use crate::application::service::FileMeta;
use crate::application::service::FileService;
use crate::application::service::MultipartUploadService;
use crate::auth::{AuthExtractor, HasOwnerId};
use crate::domain::entity::{Bucket, BucketStatus};
use crate::error::{BucketError, BucketResult};
use crate::storage::ObjectStorage;

/// Default per-request body limit for single-shot uploads (256 MiB).
///
/// Resumable chunks use a separate, smaller limit (defaulting to 16 MiB
/// of slack over the configured chunk size). Both are overridable via
/// [`UploadConfig`] when constructing the router.
pub const DEFAULT_UPLOAD_BODY_LIMIT: usize = 256 * 1024 * 1024;
pub const DEFAULT_CHUNK_BODY_LIMIT: usize = 16 * 1024 * 1024;

/// Shared handler state — travels as an Axum [`Extension`].
///
/// `bucket_service` is required so the handler can enforce per-bucket
/// policy: `status` (Readonly/Archived/Deleted reject writes),
/// `max_file_size`, and `allowed_mime_types`. Without these checks the
/// upload surface silently violates the schema-declared invariants.
pub struct UploadContext {
    pub file_service: Arc<FileService>,
    pub multipart_service: Arc<MultipartUploadService>,
    pub bucket_service: Arc<BucketService>,
    pub storage: Arc<dyn ObjectStorage>,
}

impl Clone for UploadContext {
    fn clone(&self) -> Self {
        Self {
            file_service: self.file_service.clone(),
            multipart_service: self.multipart_service.clone(),
            bucket_service: self.bucket_service.clone(),
            storage: self.storage.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UploadConfig {
    pub single_shot_limit: usize,
    pub chunk_limit: usize,
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            single_shot_limit: DEFAULT_UPLOAD_BODY_LIMIT,
            chunk_limit: DEFAULT_CHUNK_BODY_LIMIT,
        }
    }
}

/// Build the upload router.
///
/// `A` is the consumer's [`AuthExtractor`] (and identity type). It MUST
/// also implement [`HasOwnerId`] so the handler can stamp `owner_id` on
/// new files and verify session ownership.
pub fn upload_router<A>(ctx: UploadContext, config: UploadConfig) -> Router
where
    A: AuthExtractor<()> + HasOwnerId + Clone + 'static,
    A::Rejection: IntoResponse,
{
    Router::new()
        .route(
            "/uploads",
            post(single_shot_upload::<A>).layer(DefaultBodyLimit::max(config.single_shot_limit)),
        )
        .route("/uploads/sessions", post(initiate_session::<A>))
        .route(
            "/uploads/sessions/:id/parts/:part_number",
            post(upload_part::<A>).layer(DefaultBodyLimit::max(config.chunk_limit)),
        )
        .route(
            "/uploads/sessions/:id/complete",
            post(complete_session::<A>),
        )
        .route("/uploads/sessions/:id", delete(abort_session::<A>))
        .layer(Extension(Arc::new(ctx)))
}

// ─── Single-shot ──────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SingleShotResponse {
    pub id: Uuid,
    pub storage_key: String,
    pub size_bytes: i64,
    pub mime_type: String,
}

#[tracing::instrument(
    name = "bucket.upload.single_shot",
    skip_all,
    fields(owner_id = %identity.owner_id())
)]
async fn single_shot_upload<A>(
    Extension(ctx): Extension<Arc<UploadContext>>,
    identity: A,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<SingleShotResponse>), BucketError>
where
    A: AuthExtractor<()> + HasOwnerId + 'static,
    A::Rejection: IntoResponse,
{
    let owner_id = identity.owner_id();

    let mut bucket_id: Option<Uuid> = None;
    let mut path: Option<String> = None;
    let mut owner_module: Option<String> = None;
    let mut owner_entity: Option<String> = None;
    let mut owner_entity_id: Option<Uuid> = None;
    let mut storage_key: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_mime: Option<String> = None;
    let mut file_bytes: Option<Bytes> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| BucketError::Other(format!("multipart: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "bucket_id" => bucket_id = Some(parse_uuid_field(field).await?),
            "path" => path = Some(read_text_field(field).await?),
            "owner_module" => owner_module = Some(read_text_field(field).await?),
            "owner_entity" => owner_entity = Some(read_text_field(field).await?),
            "owner_entity_id" => owner_entity_id = Some(parse_uuid_field(field).await?),
            "storage_key" => storage_key = Some(read_text_field(field).await?),
            "file" => {
                file_name = field.file_name().map(str::to_string);
                file_mime = field.content_type().map(str::to_string);
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| BucketError::Other(format!("read file field: {e}")))?;
                file_bytes = Some(bytes);
            }
            _ => {
                let _ = field.bytes().await;
            }
        }
    }

    let bucket_id = bucket_id.ok_or_else(|| BucketError::InvalidInput("missing bucket_id".into()))?;
    let path = path.ok_or_else(|| BucketError::InvalidInput("missing path".into()))?;
    let body = file_bytes.ok_or_else(|| BucketError::InvalidInput("missing file field".into()))?;
    let original_name = file_name
        .ok_or_else(|| BucketError::InvalidInput("file field has no filename".into()))?;
    let mime_type = file_mime.unwrap_or_else(|| "application/octet-stream".to_string());

    // Bucket policy gate: status, size, mime. Pre-flight before any
    // storage write so rejections cost nothing.
    let bucket = load_bucket(&ctx.bucket_service, bucket_id).await?;
    enforce_bucket_policy(&bucket, body.len() as i64, &mime_type)?;

    // Quota gate. No row → no limit (admin-opt-in).
    ctx.multipart_service
        .check_capacity(owner_id, body.len() as i64)
        .await
        .map_err(service_to_bucket_error)?;

    let body_len = body.len() as i64;
    let meta = FileMeta {
        bucket_id,
        owner_id,
        original_name,
        mime_type: mime_type.clone(),
        path,
        owner_module,
        owner_entity,
        owner_entity_id,
    };

    let file = match storage_key {
        Some(key) => ctx.file_service.upload_with_key(&key, body, meta).await?,
        None => ctx.file_service.upload(body, meta).await?,
    };

    // Post-commit quota bookkeeping. Best-effort; the row is already
    // durable so a failure here is logged, not surfaced.
    if let Err(e) = ctx
        .multipart_service
        .record_completed_usage(owner_id, body_len)
        .await
    {
        tracing::warn!(%owner_id, error = ?e, "record_completed_usage failed after single-shot upload");
    }

    Ok((
        StatusCode::CREATED,
        Json(SingleShotResponse {
            id: file.id,
            storage_key: file.storage_key,
            size_bytes: file.size_bytes,
            mime_type: file.mime_type,
        }),
    ))
}

// ─── Resumable ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct InitiateRequest {
    pub bucket_id: Uuid,
    pub path: String,
    pub filename: String,
    pub mime_type: Option<String>,
    pub file_size: i64,
    pub chunk_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct InitiateResponse {
    pub session_id: Uuid,
    pub chunk_size: i32,
    pub total_chunks: i32,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[tracing::instrument(
    name = "bucket.upload.initiate",
    skip_all,
    fields(
        owner_id = %identity.owner_id(),
        bucket_id = %req.bucket_id,
        file_size = req.file_size,
    )
)]
async fn initiate_session<A>(
    Extension(ctx): Extension<Arc<UploadContext>>,
    identity: A,
    Json(req): Json<InitiateRequest>,
) -> Result<(StatusCode, Json<InitiateResponse>), BucketError>
where
    A: AuthExtractor<()> + HasOwnerId + 'static,
    A::Rejection: IntoResponse,
{
    let owner_id = identity.owner_id();

    // Pre-check the bucket policy and quota before allocating a session
    // — fails fast instead of accepting chunks for a doomed upload.
    let bucket = load_bucket(&ctx.bucket_service, req.bucket_id).await?;
    enforce_bucket_policy(
        &bucket,
        req.file_size,
        req.mime_type.as_deref().unwrap_or("application/octet-stream"),
    )?;
    ctx.multipart_service
        .check_capacity(owner_id, req.file_size)
        .await
        .map_err(service_to_bucket_error)?;

    let session = ctx
        .multipart_service
        .initiate(
            req.bucket_id,
            owner_id,
            &req.path,
            &req.filename,
            req.mime_type.as_deref(),
            req.file_size,
            req.chunk_size,
        )
        .await
        .map_err(service_to_bucket_error)?;

    Ok((
        StatusCode::CREATED,
        Json(InitiateResponse {
            session_id: session.id,
            chunk_size: session.chunk_size,
            total_chunks: session.total_chunks,
            expires_at: session.expires_at,
        }),
    ))
}

#[derive(Debug, Serialize)]
pub struct PartResponse {
    pub session_id: Uuid,
    pub part_number: i32,
    pub uploaded_chunks: i32,
    pub total_chunks: i32,
}

#[tracing::instrument(
    name = "bucket.upload.part",
    skip_all,
    fields(
        owner_id = %identity.owner_id(),
        session_id = %session_id,
        part_number,
    )
)]
async fn upload_part<A>(
    Extension(ctx): Extension<Arc<UploadContext>>,
    identity: A,
    Path((session_id, part_number)): Path<(Uuid, i32)>,
    mut multipart: Multipart,
) -> Result<Json<PartResponse>, BucketError>
where
    A: AuthExtractor<()> + HasOwnerId + 'static,
    A::Rejection: IntoResponse,
{
    let owner_id = identity.owner_id();

    let mut chunk: Option<Bytes> = None;
    let mut content_type: Option<String> = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| BucketError::Other(format!("multipart: {e}")))?
    {
        if field.name() == Some("chunk") || field.name() == Some("file") {
            content_type = field.content_type().map(str::to_string);
            chunk = Some(
                field
                    .bytes()
                    .await
                    .map_err(|e| BucketError::Other(format!("read chunk: {e}")))?,
            );
            break;
        } else {
            let _ = field.bytes().await;
        }
    }
    let chunk = chunk.ok_or_else(|| BucketError::Other("missing chunk field".into()))?;

    let part_key = part_storage_key(session_id, part_number);
    ctx.storage
        .put(
            &part_key,
            chunk,
            content_type.as_deref().unwrap_or("application/octet-stream"),
        )
        .await?;

    let session = ctx
        .multipart_service
        .record_part(session_id, owner_id, part_number)
        .await
        .map_err(service_to_bucket_error)?;

    Ok(Json(PartResponse {
        session_id: session.id,
        part_number,
        uploaded_chunks: session.uploaded_chunks,
        total_chunks: session.total_chunks,
    }))
}

#[derive(Debug, Deserialize, Default)]
pub struct CompleteRequest {
    /// Optional explicit storage key. When omitted, an auto-generated
    /// UUID-prefixed key is used (same convention as [`FileService::upload`]).
    pub storage_key: Option<String>,
    pub owner_module: Option<String>,
    pub owner_entity: Option<String>,
    pub owner_entity_id: Option<Uuid>,
}

#[tracing::instrument(
    name = "bucket.upload.complete",
    skip_all,
    fields(owner_id = %identity.owner_id(), session_id = %session_id)
)]
async fn complete_session<A>(
    Extension(ctx): Extension<Arc<UploadContext>>,
    identity: A,
    Path(session_id): Path<Uuid>,
    body: Option<Json<CompleteRequest>>,
) -> Result<(StatusCode, Json<SingleShotResponse>), BucketError>
where
    A: AuthExtractor<()> + HasOwnerId + 'static,
    A::Rejection: IntoResponse,
{
    let req = body.map(|Json(r)| r).unwrap_or_default();
    let owner_id = identity.owner_id();

    let session = ctx
        .multipart_service
        .complete(session_id, owner_id)
        .await
        .map_err(service_to_bucket_error)?;

    // TODO: when the storage backend supports native multipart (S3
    // CompleteMultipartUpload), short-circuit this assembly. The
    // ObjectStorage trait would need a `compose(parts) -> key` method.
    let mut assembled = BytesMut::with_capacity(session.file_size.max(0) as usize);
    for part_number in 1..=session.total_chunks {
        let key = part_storage_key(session_id, part_number);
        let part = ctx.storage.get(&key).await?;
        assembled.extend_from_slice(&part);
    }
    let body = assembled.freeze();

    let mime_type = session
        .mime_type
        .clone()
        .unwrap_or_else(|| "application/octet-stream".to_string());
    let meta = FileMeta {
        bucket_id: session.bucket_id,
        owner_id,
        original_name: session.filename.clone(),
        mime_type: mime_type.clone(),
        path: session.path.clone(),
        owner_module: req.owner_module,
        owner_entity: req.owner_entity,
        owner_entity_id: req.owner_entity_id,
    };
    let assembled_size = body.len() as i64;
    let file = match req.storage_key {
        Some(key) => ctx.file_service.upload_with_key(&key, body, meta).await?,
        None => ctx.file_service.upload(body, meta).await?,
    };

    // Post-commit bookkeeping. All three are best-effort: the file row is
    // already durable, so any failure here is logged for operator follow-up
    // and does not roll back the upload.
    if let Err(e) = ctx
        .multipart_service
        .record_completed_usage(owner_id, assembled_size)
        .await
    {
        tracing::warn!(%session_id, error = ?e, "record_completed_usage failed after assembly");
    }
    for part_number in 1..=session.total_chunks {
        let key = part_storage_key(session_id, part_number);
        if let Err(e) = ctx.storage.delete(&key).await {
            tracing::warn!(%session_id, part_number, error = %e, "failed to delete staged part");
        }
    }
    if let Err(e) = ctx.multipart_service.mark_completed(session_id).await {
        tracing::warn!(%session_id, error = ?e, "mark_completed failed after assembly");
    }

    Ok((
        StatusCode::CREATED,
        Json(SingleShotResponse {
            id: file.id,
            storage_key: file.storage_key,
            size_bytes: file.size_bytes,
            mime_type: file.mime_type,
        }),
    ))
}

#[tracing::instrument(
    name = "bucket.upload.abort",
    skip_all,
    fields(owner_id = %identity.owner_id(), session_id = %session_id)
)]
async fn abort_session<A>(
    Extension(ctx): Extension<Arc<UploadContext>>,
    identity: A,
    Path(session_id): Path<Uuid>,
) -> Result<StatusCode, BucketError>
where
    A: AuthExtractor<()> + HasOwnerId + 'static,
    A::Rejection: IntoResponse,
{
    ctx.multipart_service
        .abort(session_id, identity.owner_id())
        .await
        .map_err(service_to_bucket_error)?;
    Ok(StatusCode::NO_CONTENT)
}

// ─── Helpers ──────────────────────────────────────────────────────────

fn part_storage_key(session_id: Uuid, part_number: i32) -> String {
    format!("sessions/{session_id}/parts/{part_number:08}")
}

async fn read_text_field(field: axum::extract::multipart::Field<'_>) -> BucketResult<String> {
    field
        .text()
        .await
        .map_err(|e| BucketError::Other(format!("read text field: {e}")))
}

async fn parse_uuid_field(field: axum::extract::multipart::Field<'_>) -> BucketResult<Uuid> {
    let s = read_text_field(field).await?;
    Uuid::parse_str(s.trim()).map_err(|e| BucketError::Other(format!("invalid uuid: {e}")))
}

fn service_to_bucket_error(e: crate::application::service::error::ServiceError) -> BucketError {
    use crate::application::service::error::ServiceError;
    match e {
        ServiceError::NotFound => BucketError::NotFound,
        // Validation strings carry domain meaning ("quota exceeded",
        // "session expired", "Not all parts uploaded", …); route them
        // to 400 InvalidInput so callers see the message verbatim
        // instead of a generic 500.
        ServiceError::Validation(m) => BucketError::InvalidInput(m),
        ServiceError::AlreadyExists(m) => BucketError::Conflict(m),
        other => BucketError::Other(other.to_string()),
    }
}

/// Look up a bucket by id, mapping a missing row to [`BucketError::NotFound`].
async fn load_bucket(service: &BucketService, bucket_id: Uuid) -> BucketResult<Bucket> {
    service
        .find_by_id(&bucket_id.to_string())
        .await
        .map_err(|e| BucketError::Other(format!("bucket lookup: {e}")))?
        .ok_or(BucketError::NotFound)
}

/// Enforce the schema-declared per-bucket invariants on a candidate
/// upload. The bucket's own `status`, `max_file_size`, and
/// `allowed_mime_types` columns are the source of truth; this function
/// turns each violation into the appropriate HTTP-coded error.
///
/// `size` is i64 to match `Bucket.max_file_size` and the wire shape of
/// resumable `file_size` (which can be declared up-front, before bytes
/// have arrived).
pub(crate) fn enforce_bucket_policy(
    bucket: &Bucket,
    size: i64,
    mime_type: &str,
) -> BucketResult<()> {
    match bucket.status {
        BucketStatus::Active => {}
        BucketStatus::Readonly => {
            return Err(BucketError::Conflict(format!(
                "bucket {} is read-only",
                bucket.id
            )))
        }
        BucketStatus::Archived => {
            return Err(BucketError::Conflict(format!(
                "bucket {} is archived",
                bucket.id
            )))
        }
        BucketStatus::Deleted => {
            return Err(BucketError::NotFound);
        }
    }
    if bucket.is_deleted() {
        return Err(BucketError::NotFound);
    }
    if let Some(max) = bucket.max_file_size {
        if size > max {
            return Err(BucketError::PayloadTooLarge(format!(
                "{} bytes exceeds bucket limit of {} bytes",
                size, max
            )));
        }
    }
    if !bucket.allowed_mime_types.is_empty()
        && !bucket
            .allowed_mime_types
            .iter()
            .any(|m| m.eq_ignore_ascii_case(mime_type))
    {
        return Err(BucketError::UnsupportedMediaType(format!(
            "mime type `{}` is not allowed for bucket {}",
            mime_type, bucket.id
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_storage_key_is_stable_and_zero_padded() {
        let sid = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        assert_eq!(
            part_storage_key(sid, 1),
            "sessions/00000000-0000-0000-0000-000000000001/parts/00000001"
        );
        assert_eq!(
            part_storage_key(sid, 12345),
            "sessions/00000000-0000-0000-0000-000000000001/parts/00012345"
        );
    }

    #[test]
    fn part_storage_keys_sort_lexicographically_by_part_number() {
        // The 8-digit zero pad guarantees that listing the part keys in
        // lex order produces them in numeric order — load-bearing for the
        // assembly loop in `complete_session`.
        let sid = Uuid::nil();
        let mut keys: Vec<String> =
            [1, 2, 10, 100, 9, 11].iter().map(|n| part_storage_key(sid, *n)).collect();
        keys.sort();
        let nums: Vec<i32> = keys
            .iter()
            .map(|k| {
                k.rsplit('/').next().unwrap().parse::<i32>().unwrap()
            })
            .collect();
        assert_eq!(nums, vec![1, 2, 9, 10, 11, 100]);
    }

    #[test]
    fn upload_config_defaults_match_documented_limits() {
        let c = UploadConfig::default();
        assert_eq!(c.single_shot_limit, DEFAULT_UPLOAD_BODY_LIMIT);
        assert_eq!(c.chunk_limit, DEFAULT_CHUNK_BODY_LIMIT);
        assert_eq!(DEFAULT_UPLOAD_BODY_LIMIT, 256 * 1024 * 1024);
        assert_eq!(DEFAULT_CHUNK_BODY_LIMIT, 16 * 1024 * 1024);
    }

    #[test]
    fn service_error_not_found_maps_to_bucket_not_found() {
        use crate::application::service::error::ServiceError;
        let mapped = service_to_bucket_error(ServiceError::NotFound);
        assert!(matches!(mapped, BucketError::NotFound));
    }

    #[test]
    fn service_error_validation_maps_to_invalid_input() {
        use crate::application::service::error::ServiceError;
        let mapped = service_to_bucket_error(ServiceError::Validation("oops".into()));
        assert!(matches!(mapped, BucketError::InvalidInput(m) if m == "oops"));
    }

    // ─── enforce_bucket_policy ─────────────────────────────────────────

    fn test_bucket() -> Bucket {
        use crate::domain::entity::{BucketType, StorageBackend};
        Bucket::new(
            "test".into(),
            "test".into(),
            Uuid::new_v4(),
            BucketType::default(),
            BucketStatus::Active,
            StorageBackend::Local,
            "test".into(),
            0,
            0,
            Vec::new(),
            false,
            false,
            false,
        )
    }

    #[test]
    fn policy_allows_active_bucket_with_no_constraints() {
        let b = test_bucket();
        assert!(enforce_bucket_policy(&b, 100, "image/png").is_ok());
    }

    #[test]
    fn policy_rejects_readonly_bucket_with_409() {
        let mut b = test_bucket();
        b.transition_to(crate::domain::state_machine::BucketState::Readonly).ok();
        let err = enforce_bucket_policy(&b, 100, "image/png").unwrap_err();
        assert!(matches!(err, BucketError::Conflict(_)));
    }

    #[test]
    fn policy_rejects_archived_bucket_with_409() {
        let mut b = test_bucket();
        b.transition_to(crate::domain::state_machine::BucketState::Archived).ok();
        let err = enforce_bucket_policy(&b, 100, "image/png").unwrap_err();
        assert!(matches!(err, BucketError::Conflict(_)));
    }

    #[test]
    fn policy_rejects_size_over_max_file_size_with_413() {
        let mut b = test_bucket();
        b.max_file_size = Some(1024);
        let err = enforce_bucket_policy(&b, 2048, "image/png").unwrap_err();
        assert!(matches!(err, BucketError::PayloadTooLarge(_)));
    }

    #[test]
    fn policy_allows_size_at_max_boundary() {
        let mut b = test_bucket();
        b.max_file_size = Some(1024);
        assert!(enforce_bucket_policy(&b, 1024, "image/png").is_ok());
    }

    #[test]
    fn policy_rejects_mime_not_in_allowlist_with_415() {
        let mut b = test_bucket();
        b.allowed_mime_types = vec!["image/png".into(), "image/jpeg".into()];
        let err = enforce_bucket_policy(&b, 100, "application/pdf").unwrap_err();
        assert!(matches!(err, BucketError::UnsupportedMediaType(_)));
    }

    #[test]
    fn policy_mime_match_is_case_insensitive() {
        let mut b = test_bucket();
        b.allowed_mime_types = vec!["image/PNG".into()];
        assert!(enforce_bucket_policy(&b, 100, "IMAGE/png").is_ok());
    }

    #[test]
    fn policy_empty_allowlist_means_any_mime_allowed() {
        let b = test_bucket();
        assert!(b.allowed_mime_types.is_empty());
        assert!(enforce_bucket_policy(&b, 100, "anything/goes").is_ok());
    }
}
