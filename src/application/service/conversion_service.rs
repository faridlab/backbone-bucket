//! Conversion Service
//!
//! Hand-written — NOT generated. This file is safe from regeneration.
//!
//! Manages file format conversion jobs (image → webp, document → pdf, etc.).
//! Creates ConversionJob records and tracks progress.

use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use super::error::{ServiceError, ServiceResult};
use crate::domain::entity::{ConversionJob, ConversionStatus};
use crate::infrastructure::persistence::ConversionJobRepository;
use crate::infrastructure::persistence::StoredFileRepository;

/// Service for managing file format conversions.
pub struct ConversionService {
    conversion_repo: Arc<ConversionJobRepository>,
    file_repo: Arc<StoredFileRepository>,
}

impl ConversionService {
    pub fn new(
        conversion_repo: Arc<ConversionJobRepository>,
        file_repo: Arc<StoredFileRepository>,
    ) -> Self {
        Self { conversion_repo, file_repo }
    }

    /// Request a format conversion for a file.
    pub async fn request_conversion(
        &self,
        source_file_id: Uuid,
        target_format: &str,
        options: Option<serde_json::Value>,
    ) -> ServiceResult<ConversionJob> {
        let _file = self.file_repo
            .find_by_id(&source_file_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        let mut builder = ConversionJob::builder()
            .source_file_id(source_file_id)
            .target_format(target_format.to_string())
            .status(ConversionStatus::Pending)
            .progress(0);

        if let Some(opts) = options {
            builder = builder.conversion_options(opts);
        }

        let job = builder.build()
            .map_err(|e| ServiceError::Validation(e))?;

        let created = self.conversion_repo
            .create(&job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;

        Ok(created)
    }

    /// Mark a conversion job as started.
    pub async fn mark_started(&self, job_id: Uuid) -> ServiceResult<ConversionJob> {
        let mut job = self.find_job(job_id).await?;

        if job.status != ConversionStatus::Pending {
            return Err(ServiceError::Validation(format!("Job {} is not in pending state", job_id)));
        }

        job.status = ConversionStatus::Processing;
        job.started_at = Some(Utc::now());
        job.metadata.touch();

        let id_str = job.id.to_string();
        self.conversion_repo
            .update(&id_str, &job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }

    /// Update conversion progress (0-100).
    pub async fn update_progress(&self, job_id: Uuid, progress: i32) -> ServiceResult<ConversionJob> {
        let mut job = self.find_job(job_id).await?;

        if progress < 0 || progress > 100 {
            return Err(ServiceError::Validation("Progress must be between 0 and 100".into()));
        }

        job.progress = progress;
        job.metadata.touch();

        let id_str = job.id.to_string();
        self.conversion_repo
            .update(&id_str, &job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }

    /// Mark a conversion job as completed with the result file.
    pub async fn mark_completed(
        &self,
        job_id: Uuid,
        result_file_id: Uuid,
    ) -> ServiceResult<ConversionJob> {
        let mut job = self.find_job(job_id).await?;

        job.status = ConversionStatus::Completed;
        job.result_file_id = Some(result_file_id);
        job.progress = 100;
        job.completed_at = Some(Utc::now());
        job.metadata.touch();

        let id_str = job.id.to_string();
        self.conversion_repo
            .update(&id_str, &job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }

    /// Mark a conversion job as failed with an error message.
    pub async fn mark_failed(
        &self,
        job_id: Uuid,
        error_message: &str,
    ) -> ServiceResult<ConversionJob> {
        let mut job = self.find_job(job_id).await?;

        job.status = ConversionStatus::Failed;
        job.error_message = Some(error_message.to_string());
        job.completed_at = Some(Utc::now());
        job.metadata.touch();

        let id_str = job.id.to_string();
        self.conversion_repo
            .update(&id_str, &job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }

    // ---- helpers ----

    async fn find_job(&self, job_id: Uuid) -> ServiceResult<ConversionJob> {
        self.conversion_repo
            .find_by_id(&job_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }
}
