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
use axum::Router;

use crate::BucketModule;
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
};

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
}
