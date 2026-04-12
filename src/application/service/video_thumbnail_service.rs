//! Video Thumbnail Processor Service
//!
//! Hand-written — NOT generated. This file is safe from regeneration.

use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use super::error::{ServiceError, ServiceResult};
use crate::domain::entity::{
    ProcessingJob, ProcessingJobType, JobStatus, Thumbnail, ThumbnailSize,
};
use crate::infrastructure::persistence::{
    ProcessingJobRepository, ThumbnailRepository, StoredFileRepository,
};

const VIDEO_MIME_PREFIXES: &[&str] = &["video/"];

const DEFAULT_VIDEO_THUMBNAIL_SIZES: &[(ThumbnailSize, i32, i32)] = &[
    (ThumbnailSize::Small, 64, 64),
    (ThumbnailSize::Medium, 128, 128),
    (ThumbnailSize::Large, 256, 256),
];

pub struct VideoThumbnailService {
    job_repo: Arc<ProcessingJobRepository>,
    thumb_repo: Arc<ThumbnailRepository>,
    file_repo: Arc<StoredFileRepository>,
}

impl VideoThumbnailService {
    pub fn new(
        job_repo: Arc<ProcessingJobRepository>,
        thumb_repo: Arc<ThumbnailRepository>,
        file_repo: Arc<StoredFileRepository>,
    ) -> Self {
        Self { job_repo, thumb_repo, file_repo }
    }

    pub async fn enqueue(&self, file_id: Uuid, priority: Option<i32>) -> ServiceResult<ProcessingJob> {
        let file = self.file_repo
            .find_by_id(&file_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        if !is_video(&file.mime_type) {
            return Err(ServiceError::Validation(format!(
                "File {} is not a video (mime: {})", file_id, file.mime_type
            )));
        }

        // TODO: job_repo.has_active_job — implement custom repository method
        let _ = &ProcessingJobType::VideoThumbnail;

        let input = serde_json::json!({
            "sizes": DEFAULT_VIDEO_THUMBNAIL_SIZES.iter().map(|(size, w, h)| {
                serde_json::json!({ "size": format!("{:?}", size), "width": w, "height": h })
            }).collect::<Vec<_>>(),
        });

        let job = ProcessingJob::builder()
            .file_id(file_id)
            .job_type(ProcessingJobType::VideoThumbnail)
            .status(JobStatus::Pending)
            .priority(priority.unwrap_or(0))
            .input_data(input)
            .retry_count(0)
            .max_retries(3)
            .build()
            .map_err(|e| ServiceError::Validation(e))?;

        let created = self.job_repo
            .create(&job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;

        Ok(created)
    }

    pub async fn record_thumbnail(
        &self,
        file_id: Uuid,
        size: ThumbnailSize,
        width: i32,
        height: i32,
        storage_key: &str,
        size_bytes: i64,
        generation_time_ms: Option<i32>,
    ) -> ServiceResult<Thumbnail> {
        let thumb = Thumbnail::builder()
            .file_id(file_id)
            .size(size)
            .width(width)
            .height(height)
            .storage_key(storage_key.to_string())
            .mime_type("image/webp".to_string())
            .format("webp".to_string())
            .quality(80)
            .size_bytes(size_bytes)
            .generated_at(Utc::now())
            .generation_time_ms(generation_time_ms.unwrap_or(0))
            .source_version(1)
            .is_stale(false)
            .build()
            .map_err(|e| ServiceError::Validation(e))?;

        let created = self.thumb_repo
            .create(&thumb)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;

        Ok(created)
    }

    pub async fn complete_job(&self, job_id: Uuid, _file_id: Uuid) -> ServiceResult<ProcessingJob> {
        let mut job = self.find_job(job_id).await?;

        let result = serde_json::json!({
            "completed_at": Utc::now().to_rfc3339(),
            "thumbnails_generated": true,
        });

        job.status = JobStatus::Completed;
        job.result_data = Some(result);
        job.completed_at = Some(Utc::now());
        job.metadata.touch();

        let id_str = job.id.to_string();
        let updated = self.job_repo
            .update(&id_str, &job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        // TODO: file_repo.update_processing_status — implement custom method

        Ok(updated)
    }

    pub async fn fail_job(&self, job_id: Uuid, error_message: &str) -> ServiceResult<ProcessingJob> {
        let mut job = self.find_job(job_id).await?;

        job.status = JobStatus::Failed;
        job.error_message = Some(error_message.to_string());
        job.completed_at = Some(Utc::now());
        job.metadata.touch();

        let id_str = job.id.to_string();
        self.job_repo
            .update(&id_str, &job)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }

    pub async fn get_thumbnails(&self, _file_id: Uuid) -> ServiceResult<Vec<Thumbnail>> {
        // TODO: thumb_repo.find_by_file_id — implement custom repository method
        Ok(vec![])
    }

    pub async fn regenerate(&self, file_id: Uuid) -> ServiceResult<ProcessingJob> {
        // TODO: thumb_repo.mark_stale_by_file — implement custom method
        self.enqueue(file_id, Some(1)).await
    }

    async fn find_job(&self, job_id: Uuid) -> ServiceResult<ProcessingJob> {
        self.job_repo
            .find_by_id(&job_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }
}

fn is_video(mime_type: &str) -> bool {
    VIDEO_MIME_PREFIXES.iter().any(|prefix| mime_type.starts_with(prefix))
}
