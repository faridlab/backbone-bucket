//! Multipart Upload Service
//!
//! Hand-written — NOT generated. This file is safe from regeneration.

use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;

use super::error::{ServiceError, ServiceResult};
use crate::domain::entity::{UploadSession, UploadStatus, StorageBackend};
use crate::infrastructure::persistence::UploadSessionRepository;
use crate::infrastructure::persistence::BucketRepository;
use crate::infrastructure::persistence::UserQuotaRepository;

const DEFAULT_SESSION_EXPIRY_HOURS: i64 = 24;
const DEFAULT_CHUNK_SIZE: i32 = 5 * 1024 * 1024;

pub struct MultipartUploadService {
    session_repo: Arc<UploadSessionRepository>,
    bucket_repo: Arc<BucketRepository>,
    quota_repo: Arc<UserQuotaRepository>,
}

impl MultipartUploadService {
    pub fn new(
        session_repo: Arc<UploadSessionRepository>,
        bucket_repo: Arc<BucketRepository>,
        quota_repo: Arc<UserQuotaRepository>,
    ) -> Self {
        Self { session_repo, bucket_repo, quota_repo }
    }

    pub async fn initiate(
        &self,
        bucket_id: Uuid,
        user_id: Uuid,
        path: &str,
        filename: &str,
        mime_type: Option<&str>,
        file_size: i64,
        chunk_size: Option<i32>,
    ) -> ServiceResult<UploadSession> {
        let _bucket = self.bucket_repo
            .find_by_id(&bucket_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        if file_size <= 0 {
            return Err(ServiceError::Validation("file_size must be positive".into()));
        }

        let chunk = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
        if chunk <= 0 {
            return Err(ServiceError::Validation("chunk_size must be positive".into()));
        }
        let total_chunks = ((file_size as f64) / (chunk as f64)).ceil() as i32;

        let mut builder = UploadSession::builder()
            .bucket_id(bucket_id)
            .user_id(user_id)
            .path(path.to_string())
            .filename(filename.to_string())
            .file_size(file_size)
            .chunk_size(chunk)
            .total_chunks(total_chunks)
            .uploaded_chunks(0)
            .status(UploadStatus::Initiated)
            .storage_backend(StorageBackend::Local)
            .expires_at(Utc::now() + Duration::hours(DEFAULT_SESSION_EXPIRY_HOURS));

        if let Some(mt) = mime_type {
            builder = builder.mime_type(mt.to_string());
        }

        let session = builder.build()
            .map_err(|e| ServiceError::Validation(e))?;

        let created = self.session_repo
            .create(&session)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;

        Ok(created)
    }

    pub async fn record_part(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        part_number: i32,
    ) -> ServiceResult<UploadSession> {
        let session = self.get_active_session(session_id, user_id).await?;

        if part_number < 1 || part_number > session.total_chunks {
            return Err(ServiceError::Validation(
                format!("part_number must be between 1 and {}", session.total_chunks)
            ));
        }

        if session.completed_parts.contains(&part_number) {
            return Err(ServiceError::AlreadyExists(
                format!("Part {} already uploaded", part_number)
            ));
        }

        // TODO: session_repo.record_part — implement custom repository method
        // For now just update uploaded_chunks
        let mut updated_session = session;
        updated_session.uploaded_chunks += 1;
        if updated_session.status == UploadStatus::Initiated {
            updated_session.status = UploadStatus::Uploading;
        }
        updated_session.metadata.touch();
        let id_str = updated_session.id.to_string();
        self.session_repo.update(&id_str, &updated_session).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;

        self.session_repo
            .find_by_id(&session_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }

    pub async fn complete(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> ServiceResult<UploadSession> {
        let mut session = self.get_active_session(session_id, user_id).await?;

        if session.uploaded_chunks < session.total_chunks {
            return Err(ServiceError::Validation(format!(
                "Not all parts uploaded: {}/{} complete",
                session.uploaded_chunks, session.total_chunks
            )));
        }

        session.status = UploadStatus::Completing;
        session.metadata.touch();

        let id_str = session.id.to_string();
        self.session_repo
            .update(&id_str, &session)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)
    }

    pub async fn abort(&self, session_id: Uuid, user_id: Uuid) -> ServiceResult<()> {
        let mut session = self.get_active_session(session_id, user_id).await?;
        session.status = UploadStatus::Failed;
        session.metadata.touch();
        let id_str = session.id.to_string();
        self.session_repo.update(&id_str, &session).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;
        Ok(())
    }

    pub async fn mark_completed(&self, session_id: Uuid) -> ServiceResult<()> {
        let mut session = self.session_repo
            .find_by_id(&session_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        session.status = UploadStatus::Completed;
        session.metadata.touch();
        let id_str = session.id.to_string();
        self.session_repo.update(&id_str, &session).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;
        Ok(())
    }

    pub async fn cleanup_expired(&self) -> ServiceResult<u64> {
        // TODO: session_repo.expire_stale_sessions — implement custom method
        Err(ServiceError::Internal("expire_stale_sessions not yet implemented".to_string()))
    }

    pub async fn list_active(&self, _user_id: Uuid, _bucket_id: Uuid) -> ServiceResult<Vec<UploadSession>> {
        // TODO: session_repo.find_by_user_and_bucket — implement custom method
        Err(ServiceError::Internal("list_active not yet implemented".to_string()))
    }

    async fn get_active_session(&self, session_id: Uuid, user_id: Uuid) -> ServiceResult<UploadSession> {
        // TODO: session_repo.find_active_by_id — implement custom method
        let session = self.session_repo
            .find_by_id(&session_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        if session.user_id != user_id {
            return Err(ServiceError::Validation("Upload session belongs to a different user".into()));
        }

        if session.expires_at < Utc::now() {
            return Err(ServiceError::Validation("Upload session has expired".into()));
        }

        Ok(session)
    }
}
