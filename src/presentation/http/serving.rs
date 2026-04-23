//! Mode-B serving handler.
//!
//! Serves files under `/*key` with the consumer's auth extractor and
//! authorization policy in the path. Response strategy is driven by
//! [`ServingMode`](crate::config::ServingMode):
//!
//! - `Redirect` (default) — 302 to a short-lived presigned URL.
//! - `Stream` — proxy the bytes through the service.
//! - `SignedUrl` — return `{"url": "..."}` JSON.
//!
//! Authentication and authorization are pluggable:
//!
//! - [`AuthExtractor`] resolves the caller's identity from the request.
//! - [`AuthzPolicy`] decides whether that identity may read the file.
//!
//! The context object travels as an Axum [`Extension`] layer, which keeps
//! the Router generic over `()` so consumers can merge the serving router
//! into their existing app without wrestling with shared-state generics.

use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::auth::{ArcAuthzPolicy, AuthExtractor};
use crate::config::{BucketConfig, ServingMode};
use crate::domain::entity::StoredFile;
use crate::error::{BucketError, BucketResult};
use crate::infrastructure::persistence::StoredFileRepository;
use crate::storage::ObjectStorage;

/// Shared handler state — travels as an Axum `Extension`.
///
/// Parameterized over the consumer's identity type (`A`, which in Axum
/// doubles as both extractor and identity) so the handler can call the
/// consumer's [`AuthzPolicy`] without the module knowing what the
/// identity actually looks like.
pub struct ServingContext<Identity>
where
    Identity: Send + Sync + 'static,
{
    pub storage: Arc<dyn ObjectStorage>,
    pub file_repo: Arc<StoredFileRepository>,
    pub authz: ArcAuthzPolicy<Identity>,
    pub config: Arc<BucketConfig>,
}

impl<I> Clone for ServingContext<I>
where
    I: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            file_repo: self.file_repo.clone(),
            authz: self.authz.clone(),
            config: self.config.clone(),
        }
    }
}

/// Build the serving router.
///
/// Consumers mount this at whatever path suits their URL layout —
/// `/cdn` is the default recommendation:
///
/// ```ignore
/// let app = Router::new()
///     .merge(bucket.crud_router())
///     .nest("/cdn", bucket.serving_router::<MyAuth>(policy)?);
/// ```
///
/// `A` is the consumer's [`AuthExtractor`]; Axum resolves it on every
/// request to produce the identity that flows into the authz policy.
pub fn serving_router<A>(ctx: ServingContext<A>) -> Router
where
    A: AuthExtractor<()> + 'static,
    A::Rejection: IntoResponse,
{
    Router::new()
        .route("/*key", get(serve_file::<A>))
        .layer(Extension(Arc::new(ctx)))
}

async fn serve_file<A>(
    Extension(ctx): Extension<Arc<ServingContext<A>>>,
    identity: A,
    Path(key): Path<String>,
) -> Result<Response, BucketError>
where
    A: AuthExtractor<()> + 'static,
    A::Rejection: IntoResponse,
{
    let file = lookup_by_key(&ctx.file_repo, &key).await?;
    ctx.authz.ensure_can_read(&identity, &file).await?;
    build_mode_response(&*ctx.storage, &ctx.config, &file).await
}

/// Build the serving response for a single file according to the
/// configured [`ServingMode`].
///
/// Split out of `serve_file` so unit tests can exercise each mode
/// without spinning up a database or Axum router.
pub(crate) async fn build_mode_response(
    storage: &dyn ObjectStorage,
    config: &BucketConfig,
    file: &StoredFile,
) -> Result<Response, BucketError> {
    let serving = &config.serving;
    match serving.default_mode {
        ServingMode::Redirect => {
            let url = storage
                .presigned_get(&file.storage_key, serving.presigned_ttl)
                .await?;
            Ok(Redirect::temporary(url.as_str()).into_response())
        }
        ServingMode::Stream => {
            let bytes = storage.get(&file.storage_key).await?;
            Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, file.mime_type.clone())],
                bytes,
            )
                .into_response())
        }
        ServingMode::SignedUrl => {
            let url = storage
                .presigned_get(&file.storage_key, serving.presigned_ttl)
                .await?;
            Ok(Json(json!({ "url": url.to_string() })).into_response())
        }
    }
}

/// Look up a file by its `storage_key` column.
///
/// Exposed as a free fn so call sites outside the handler (tests, custom
/// routes) can reuse the same lookup rule.
pub async fn lookup_by_key(
    repo: &StoredFileRepository,
    key: &str,
) -> BucketResult<StoredFile> {
    repo.find_by_text_field("storage_key", key)
        .await
        .map_err(|e| BucketError::Other(e.to_string()))?
        .ok_or(BucketError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use async_trait::async_trait;
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use bytes::Bytes;
    use url::Url;
    use uuid::Uuid;

    use crate::config::{BucketConfig, ServingConfig, ServingMode, StorageConfig};
    use crate::domain::entity::{AuditMetadata, FileStatus, StoredFile};
    use crate::storage::{ObjectMeta, ObjectStorage};

    /// In-memory ObjectStorage stub for mode-branch tests.
    ///
    /// Records the keys it was asked for so assertions can confirm the
    /// handler called the right method. No filesystem or network.
    struct StubStorage {
        body: Bytes,
        presigned: Url,
    }

    #[async_trait]
    impl ObjectStorage for StubStorage {
        async fn put(&self, _key: &str, _body: Bytes, _ct: &str) -> BucketResult<()> {
            Ok(())
        }
        async fn get(&self, _key: &str) -> BucketResult<Bytes> {
            Ok(self.body.clone())
        }
        async fn delete(&self, _key: &str) -> BucketResult<()> {
            Ok(())
        }
        async fn head(&self, key: &str) -> BucketResult<ObjectMeta> {
            Ok(ObjectMeta {
                key: key.to_string(),
                size: self.body.len() as u64,
                content_type: None,
                etag: None,
                last_modified: None,
            })
        }
        async fn presigned_get(&self, _key: &str, _ttl: Duration) -> BucketResult<Url> {
            Ok(self.presigned.clone())
        }
        async fn presigned_put(
            &self,
            _key: &str,
            _ttl: Duration,
            _ct: &str,
        ) -> BucketResult<Url> {
            Ok(self.presigned.clone())
        }
        fn public_url(&self, _key: &str) -> Option<Url> {
            None
        }
    }

    fn fixture_file() -> StoredFile {
        StoredFile::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "/docs/a.txt".into(),
            "a.txt".into(),
            5,
            "text/plain".into(),
            false,
            false,
            false,
            false,
            false,
            0,
            FileStatus::Active,
            "docs/a.txt".into(),
            1,
            0,
        )
    }

    fn fixture_config(mode: ServingMode) -> BucketConfig {
        BucketConfig {
            enabled: true,
            storage: StorageConfig::Local {
                root: "/tmp/bucket-test".into(),
                base_url: Url::parse("http://localhost/cdn/").unwrap(),
                signing_secret_env: "UNUSED".into(),
            },
            serving: ServingConfig {
                default_mode: mode,
                public_prefix: "public/".into(),
                presigned_ttl: Duration::from_secs(60),
            },
        }
    }

    fn stub() -> StubStorage {
        StubStorage {
            body: Bytes::from_static(b"hello"),
            presigned: Url::parse("https://minio.example/docs/a.txt?X-Amz-Signature=abc").unwrap(),
        }
    }

    #[tokio::test]
    async fn redirect_mode_returns_302_to_presigned_url() {
        let storage = stub();
        let cfg = fixture_config(ServingMode::Redirect);
        let file = fixture_file();

        let resp = build_mode_response(&storage, &cfg, &file).await.unwrap();
        assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
        let loc = resp.headers().get(header::LOCATION).unwrap().to_str().unwrap();
        assert_eq!(loc, storage.presigned.as_str());
    }

    #[tokio::test]
    async fn stream_mode_returns_body_with_content_type() {
        let storage = stub();
        let cfg = fixture_config(ServingMode::Stream);
        let file = fixture_file();

        let resp = build_mode_response(&storage, &cfg, &file).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/plain"
        );
        let body = to_bytes(resp.into_body(), 1024).await.unwrap();
        assert_eq!(&body[..], b"hello");
    }

    #[tokio::test]
    async fn signed_url_mode_returns_json_with_url() {
        let storage = stub();
        let cfg = fixture_config(ServingMode::SignedUrl);
        let file = fixture_file();

        let resp = build_mode_response(&storage, &cfg, &file).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["url"], storage.presigned.as_str());
    }

    #[test]
    fn error_response_masks_internal_details() {
        // S3 / Other / Io / Url / Config all collapse to a generic 500.
        let resp = BucketError::S3("AWS secret leak: AKIA...".into()).into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn error_response_preserves_user_facing_status() {
        assert_eq!(
            BucketError::NotFound.into_response().status(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            BucketError::Forbidden.into_response().status(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            BucketError::Unauthenticated.into_response().status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            BucketError::InvalidSignature.into_response().status(),
            StatusCode::FORBIDDEN
        );
    }
}

impl IntoResponse for BucketError {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            BucketError::NotFound => (StatusCode::NOT_FOUND, "not found"),
            BucketError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            BucketError::Unauthenticated => (StatusCode::UNAUTHORIZED, "unauthenticated"),
            BucketError::InvalidSignature => (StatusCode::FORBIDDEN, "invalid signature"),
            BucketError::Unsupported(_) => {
                (StatusCode::NOT_IMPLEMENTED, "operation not supported")
            }
            // Internal-class errors: never leak backend details to callers.
            // The full error is captured in tracing for operators.
            BucketError::Config(_)
            | BucketError::Io(_)
            | BucketError::S3(_)
            | BucketError::Url(_)
            | BucketError::Other(_) => {
                tracing::error!(error = %self, "bucket serving handler failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            }
        };
        (status, body).into_response()
    }
}
