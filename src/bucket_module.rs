//! Custom BucketModule extensions — Phase 6: Child Entity Collapse
//!
//! This file is NEVER touched by backbone-schema generators.
//! It adds http_routes() to BucketModule, which returns only the 10 active
//! CRUD stacks (Thumbnail, FileVersion, and AccessLog are intentionally excluded
//! as they are child entities with no independent lifecycle).
//!
//! The generated routes() in lib.rs is always regenerated with all 13 entities.
//! The app MUST call http_routes() instead.

use std::sync::Arc;
use axum::{response::IntoResponse, Router};

use crate::auth::{ArcAuthzPolicy, AuthExtractor, HasOwnerId};
use crate::error::{BucketError, BucketResult};
use crate::presentation::http::{
    create_bucket_routes,
    create_content_hash_routes,
    create_conversion_job_routes,
    create_file_comment_routes,
    create_file_lock_routes,
    create_file_share_routes,
    create_processing_job_routes,
    create_stored_file_routes,
    create_upload_session_routes,
    create_user_quota_routes,
    serving_router as build_serving_router,
    upload_router as build_upload_router,
    ServingContext,
    UploadConfig,
    UploadContext,
};
use crate::BucketModule;

impl BucketModule {
    /// HTTP routes for the 10 active CRUD stacks.
    ///
    /// Excludes Thumbnail, FileVersion, AccessLog — child entities managed through
    /// their parent (StoredFile/Bucket) and not exposed as independent endpoints.
    pub fn http_routes(&self) -> Router {
        Router::new()
            .merge(create_bucket_routes(self.bucket_service.clone()))
            .merge(create_content_hash_routes(self.content_hash_service.clone()))
            .merge(create_conversion_job_routes(self.conversion_job_service.clone()))
            .merge(create_file_comment_routes(self.file_comment_service.clone()))
            .merge(create_file_lock_routes(self.file_lock_service.clone()))
            .merge(create_file_share_routes(self.file_share_service.clone()))
            .merge(create_processing_job_routes(self.processing_job_service.clone()))
            .merge(create_stored_file_routes(self.stored_file_service.clone()))
            .merge(create_upload_session_routes(self.upload_session_service.clone()))
            .merge(create_user_quota_routes(self.user_quota_service.clone()))
    }

    /// The 10 generated CRUD routers, composed.
    ///
    /// Alias for [`Self::http_routes`]. Returns the existing
    /// `/api/v1/bucket/*` router — unchanged from before the serving
    /// handler was added. Named to pair symmetrically with
    /// [`Self::serving_router`].
    pub fn crud_router(&self) -> Router {
        self.http_routes()
    }

    /// Build the mode-B serving router.
    ///
    /// `A` is the consumer's [`AuthExtractor`] — and, since Axum
    /// extractors yield themselves, also the identity type the
    /// [`crate::auth::AuthzPolicy`] decides on.
    ///
    /// Returns `Err` when `.with_storage()` / `.with_config()` were not
    /// called on the builder — the serving handler can't operate without
    /// both.
    pub fn serving_router<A>(&self, authz: ArcAuthzPolicy<A>) -> BucketResult<Router>
    where
        A: AuthExtractor<()> + HasOwnerId + Clone + 'static,
        A::Rejection: IntoResponse,
    {
        let storage = self
            .storage
            .clone()
            .ok_or_else(|| BucketError::Config("storage backend not configured".into()))?;
        let config = self
            .bucket_config
            .clone()
            .ok_or_else(|| BucketError::Config("bucket config not provided".into()))?;
        let ctx = ServingContext::<A> {
            storage,
            file_repo: self.stored_file_repository.clone(),
            authz,
            config,
        };
        Ok(build_serving_router::<A>(ctx))
    }

    /// Build the multipart upload router.
    ///
    /// Mounts five routes (see [`crate::presentation::http::upload`]):
    /// single-shot `POST /uploads`, and the resumable session lifecycle
    /// under `/uploads/sessions`. The consumer's [`AuthExtractor`] `A`
    /// must also implement [`HasOwnerId`] so `owner_id` is derived from
    /// the authenticated identity instead of being trusted from input.
    ///
    /// Returns `Err` when `.with_storage()` / `.with_config()` were not
    /// configured — uploads cannot run without an [`ObjectStorage`]
    /// backend and the wired-up [`crate::FileService`].
    pub fn upload_router<A>(&self, config: UploadConfig) -> BucketResult<Router>
    where
        A: AuthExtractor<()> + HasOwnerId + Clone + 'static,
        A::Rejection: IntoResponse,
    {
        let storage = self
            .storage
            .clone()
            .ok_or_else(|| BucketError::Config("storage backend not configured".into()))?;
        let file_service = self
            .file_service
            .clone()
            .ok_or_else(|| BucketError::Config("file service not configured (call .with_storage() and .with_config())".into()))?;
        let ctx = UploadContext {
            file_service,
            multipart_service: self.multipart_upload_service.clone(),
            bucket_service: self.bucket_service.clone(),
            storage,
        };
        Ok(build_upload_router::<A>(ctx, config))
    }

    /// One-call composition: CRUD + upload + serving routers merged.
    ///
    /// Pick this when the consumer just wants "everything the module
    /// exposes" under a single nest point. Each sub-router is still
    /// reachable individually via [`Self::crud_router`] /
    /// [`Self::upload_router`] / [`Self::serving_router`] for advanced
    /// composition.
    ///
    /// The merged router pairs CRUD endpoints with the upload surface
    /// at the same prefix the consumer chooses (e.g. `/api/v1/bucket`),
    /// and nests the serving handler under `serving_prefix` (default
    /// `/cdn`). Passing `RouterOptions::default()` plus a configured
    /// `BucketModule` is the smallest possible wiring:
    ///
    /// ```ignore
    /// let app = Router::new()
    ///     .nest("/api/v1/bucket", bucket.router::<MyUser>(opts)?);
    /// ```
    ///
    /// Returns `Err` for the same reasons [`Self::serving_router`] and
    /// [`Self::upload_router`] do — `.with_storage()` and
    /// `.with_config()` are required on the builder.
    pub fn router<A>(&self, opts: RouterOptions<A>) -> BucketResult<Router>
    where
        A: AuthExtractor<()> + HasOwnerId + Clone + 'static,
        A::Rejection: IntoResponse,
    {
        let RouterOptions {
            upload_config,
            authz,
            serving_prefix,
        } = opts;
        let serving = self.serving_router::<A>(authz)?;
        let uploads = self.upload_router::<A>(upload_config)?;
        Ok(self
            .crud_router()
            .merge(uploads)
            .nest(&serving_prefix, serving))
    }
}

/// Options for [`BucketModule::router`].
///
/// `authz` is the only required field — the policy decides whether an
/// authenticated caller may read a given file. Defaults: stock
/// [`UploadConfig`] (256 MiB single-shot / 16 MiB per chunk) and
/// `/cdn` as the serving prefix.
pub struct RouterOptions<A> {
    pub upload_config: UploadConfig,
    pub authz: ArcAuthzPolicy<A>,
    pub serving_prefix: String,
}

impl<A> RouterOptions<A> {
    /// Construct with the policy and module-default values.
    pub fn new(authz: ArcAuthzPolicy<A>) -> Self {
        Self {
            upload_config: UploadConfig::default(),
            authz,
            serving_prefix: "/cdn".to_string(),
        }
    }

    /// Override the single-shot / chunk body limits.
    pub fn with_upload_config(mut self, cfg: UploadConfig) -> Self {
        self.upload_config = cfg;
        self
    }

    /// Override the serving-router mount path (default `/cdn`).
    pub fn with_serving_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.serving_prefix = prefix.into();
        self
    }
}
