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

        // Quota gate. Best-effort: a quota row may not exist for every
        // user (admin-provisioned, opt-in) — absence means "no limit",
        // not "deny". When a row exists, hard-reject if completing would
        // push `used_bytes` past `limit_bytes`. The actual increment
        // happens in `record_completed_usage` after the bytes land.
        if let Some(quota) = self
            .quota_repo
            .find_by_user_id(user_id)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
        {
            let projected = quota.used_bytes.saturating_add(session.file_size);
            if projected > quota.limit_bytes {
                return Err(ServiceError::Validation(format!(
                    "user quota exceeded: {} + {} > {} bytes",
                    quota.used_bytes, session.file_size, quota.limit_bytes
                )));
            }
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

    /// Check (without mutating) whether `user_id` has capacity for
    /// `bytes` more. Returns `Ok(())` when there is no quota row (no
    /// limit configured) or when `used_bytes + bytes <= limit_bytes`.
    ///
    /// Called by the single-shot upload handler before writing to
    /// storage. The resumable flow calls into [`Self::complete`] which
    /// performs the same check.
    pub async fn check_capacity(&self, user_id: Uuid, bytes: i64) -> ServiceResult<()> {
        let Some(quota) = self
            .quota_repo
            .find_by_user_id(user_id)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
        else {
            return Ok(());
        };
        let projected = quota.used_bytes.saturating_add(bytes);
        if projected > quota.limit_bytes {
            return Err(ServiceError::Validation(format!(
                "user quota exceeded: {} + {} > {} bytes",
                quota.used_bytes, bytes, quota.limit_bytes
            )));
        }
        Ok(())
    }

    /// Record post-upload usage on the user's quota row. Best-effort:
    /// when no quota row exists this is a no-op. Call AFTER the storage
    /// `put` and DB row commit succeed.
    pub async fn record_completed_usage(
        &self,
        user_id: Uuid,
        bytes: i64,
    ) -> ServiceResult<()> {
        let Some(mut quota) = self
            .quota_repo
            .find_by_user_id(user_id)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
        else {
            return Ok(());
        };
        quota.used_bytes = quota.used_bytes.saturating_add(bytes);
        quota.file_count = quota.file_count.saturating_add(1);
        if quota.used_bytes > quota.peak_usage_bytes {
            quota.peak_usage_bytes = quota.used_bytes;
            quota.peak_usage_at = Some(Utc::now());
        }
        quota.metadata.touch();
        let id_str = quota.id.to_string();
        self.quota_repo
            .update(&id_str, &quota)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;
        Ok(())
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
