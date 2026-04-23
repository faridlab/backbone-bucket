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

use crate::auth::{ArcAuthzPolicy, AuthExtractor};
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
    ServingContext,
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
        A: AuthExtractor<()> + Clone + 'static,
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
}
