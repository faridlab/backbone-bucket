//! The module's extension (hand-written; ADR-0031): what the module adds beside its
//! generated services. The generated module and builder carry `ModuleExt` and
//! `ModuleBuilderExt` and dereference to them, so their fields read as `module.field`.

#[allow(unused_imports)]
use super::*;

/// State the module adds; read through the generated module's `Deref`.
pub struct ModuleExt {
    // Custom business logic services
    pub locking_service: Arc<LockingService>,
    pub multipart_upload_service: Arc<MultipartUploadService>,
    pub conversion_service: Arc<ConversionService>,
    #[allow(deprecated)]
    pub cdn_service: Arc<CdnService>,
    pub video_thumbnail_service: Arc<VideoThumbnailService>,
    pub document_preview_service: Arc<DocumentPreviewService>,
    /// Storage backend (mode-B serving + uploads). None until `.with_storage()` is called.
    pub storage: Option<Arc<dyn ObjectStorage>>,
    /// Bucket-module config (storage + serving). None until `.with_config()` is called.
    pub bucket_config: Option<Arc<BucketConfig>>,
    /// High-level file operations service, wired when both `storage` and `bucket_config` are set.
    pub file_service: Option<Arc<FileService>>,
    /// Direct handle to the StoredFile repository — used by the mode-B
    /// serving handler to look up files by `storage_key`.
    pub stored_file_repository: Arc<infrastructure::persistence::StoredFileRepository>,
}

/// State the builder adds.
pub struct ModuleBuilderExt {
    // bucket-serving fields (see docs/serving.md)
    storage: Option<Arc<dyn ObjectStorage>>,
    bucket_config: Option<BucketConfig>,
}

impl Default for ModuleBuilderExt {
    fn default() -> Self {
        Self {
            storage: None,
            bucket_config: None,
        }
    }
}

impl ModuleBuilderExt {
    /// Build the extension's state from what the generated build made.
    #[allow(unused_variables, clippy::redundant_clone)]
    pub(crate) fn build(self, parts: &ModuleParts<'_>) -> anyhow::Result<ModuleExt> {
        let db_pool = parts.db_pool.clone();
        let stored_file_repository = parts.stored_file_repository.clone();
        // The quota read and write the stored-file triggers' bookkeeping uses
        // (ADR-0034): bound here, weakly, because the stored-file service owns
        // the hooks that own the triggers impl.
        crate::application::triggers::bind_quota_bookkeeping(
            parts.user_quota_service,
            parts.user_quota_repository,
        );
        let bucket_repository = parts.bucket_repository.clone();
        let conversion_job_repository = parts.conversion_job_repository.clone();
        let file_lock_repository = parts.file_lock_repository.clone();
        let processing_job_repository = parts.processing_job_repository.clone();
        let thumbnail_repository = parts.thumbnail_repository.clone();
        let upload_session_repository = parts.upload_session_repository.clone();
        let user_quota_repository = parts.user_quota_repository.clone();
        // Custom business logic services
        let locking_service = Arc::new(LockingService::new(
            file_lock_repository, stored_file_repository.clone(),
        ));
        let multipart_upload_service = Arc::new(MultipartUploadService::new(
            upload_session_repository, bucket_repository.clone(), user_quota_repository,
        ));
        let conversion_service = Arc::new(ConversionService::new(
            conversion_job_repository, stored_file_repository.clone(),
        ));
        #[allow(deprecated)]
        let cdn_service = Arc::new(CdnService::new(
            stored_file_repository.clone(), bucket_repository,
        ));
        let video_thumbnail_service = Arc::new(VideoThumbnailService::new(
            processing_job_repository.clone(), thumbnail_repository.clone(), stored_file_repository.clone(),
        ));
        let document_preview_service = Arc::new(DocumentPreviewService::new(
            processing_job_repository, thumbnail_repository, stored_file_repository.clone(),
        ));
        // Bucket-serving wiring (see docs/serving.md)
        let bucket_config = self.bucket_config.map(Arc::new);
        let storage = self.storage;
        let file_service = match (&storage, &bucket_config) {
            (Some(s), Some(c)) => Some(Arc::new(FileService::new(
                s.clone(),
                stored_file_repository.clone(),
                c.clone(),
            ))),
            _ => None,
        };
        Ok(ModuleExt {
            locking_service,
            multipart_upload_service,
            conversion_service,
            cdn_service,
            video_thumbnail_service,
            document_preview_service,
            storage,
            bucket_config,
            file_service,
            stored_file_repository,
        })
    }
}

impl crate::ThisModule {
    /// What [`upload_router`] needs, built from this module's own services.
    ///
    /// `None` until the module has both a storage backend and a config (the
    /// file service is wired only then). A host mounts uploads with
    /// `upload_router::<I>(bucket.upload_context()?, config)` and never holds
    /// the generic bucket service itself.
    pub fn upload_context(&self) -> Option<UploadContext> {
        Some(UploadContext {
            file_service: self.file_service.clone()?,
            multipart_service: self.multipart_upload_service.clone(),
            bucket_service: self.bucket_service.clone(),
            storage: self.storage.clone()?,
        })
    }
}

impl crate::ThisModuleBuilder {
    // Bucket-serving builder methods (see docs/serving.md)
    /// Attach the bucket-module runtime config (storage + serving).
    pub fn with_config(mut self, config: BucketConfig) -> Self {
        self.bucket_config = Some(config);
        self
    }
    /// Attach the object-storage backend (`LocalStorage`, `S3Storage`, …).
    pub fn with_storage(mut self, storage: Arc<dyn ObjectStorage>) -> Self {
        self.storage = Some(storage);
        self
    }
}
