//! File Locking Service
//!
//! Hand-written — NOT generated. This file is safe from regeneration.
//!
//! Manages file editing locks to prevent concurrent edits.
//! Supports lock acquisition, release, refresh, admin break, and expired lock cleanup.

use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;

use super::error::{ServiceError, ServiceResult};
use crate::domain::entity::{FileLock, LockStatus};
use crate::infrastructure::persistence::FileLockRepository;
use crate::infrastructure::persistence::StoredFileRepository;

/// Default lock duration: 30 minutes
const DEFAULT_LOCK_DURATION_MINUTES: i64 = 30;

/// Service for managing file editing locks.
pub struct LockingService {
    lock_repo: Arc<FileLockRepository>,
    file_repo: Arc<StoredFileRepository>,
}

impl LockingService {
    pub fn new(lock_repo: Arc<FileLockRepository>, file_repo: Arc<StoredFileRepository>) -> Self {
        Self { lock_repo, file_repo }
    }

    /// Acquire an editing lock on a file.
    ///
    /// If the file is already locked by the same user, refreshes the lock.
    /// If locked by another user and the lock is still valid, returns an error.
    /// Expired locks are automatically cleaned up.
    pub async fn acquire_lock(
        &self,
        file_id: Uuid,
        user_id: Uuid,
        duration_minutes: Option<i64>,
    ) -> ServiceResult<FileLock> {
        // Verify file exists
        let _file = self.file_repo
            .find_by_id(&file_id.to_string())
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        // Check existing lock
        if let Some(existing) = self.lock_repo.find_by_file_id(file_id).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))? {
            if existing.is_expired() {
                // Expired — remove it
                self.lock_repo.delete(&existing.id.to_string()).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;
            } else if existing.user_id == user_id {
                // Same user — refresh
                return self.refresh_lock(file_id, user_id).await;
            } else {
                // Different user — conflict
                return Err(ServiceError::AlreadyExists(format!(
                    "File {} is locked by user {} until {}",
                    file_id, existing.user_id, existing.expires_at
                )));
            }
        }

        let minutes = duration_minutes.unwrap_or(DEFAULT_LOCK_DURATION_MINUTES);
        let now = Utc::now();
        let expires_at = now + Duration::minutes(minutes);

        let lock = FileLock::new(file_id, user_id, now, expires_at, LockStatus::Active);

        let created = self.lock_repo
            .create(&lock)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;

        Ok(created)
    }

    /// Release a lock. Only the lock owner can release it.
    pub async fn release_lock(&self, file_id: Uuid, user_id: Uuid) -> ServiceResult<()> {
        let lock = self.lock_repo
            .find_by_file_id(file_id)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        if lock.user_id != user_id {
            return Err(ServiceError::Validation(
                format!("Lock on file {} is owned by a different user", file_id)
            ));
        }

        self.lock_repo.delete(&lock.id.to_string()).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;
        Ok(())
    }

    /// Refresh an existing lock's expiry time.
    pub async fn refresh_lock(&self, file_id: Uuid, user_id: Uuid) -> ServiceResult<FileLock> {
        let mut lock = self.lock_repo
            .find_by_file_id(file_id)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        if lock.user_id != user_id {
            return Err(ServiceError::Validation(
                format!("Lock on file {} is owned by a different user", file_id)
            ));
        }

        lock.expires_at = Utc::now() + Duration::minutes(DEFAULT_LOCK_DURATION_MINUTES);
        lock.refreshed_at = Some(Utc::now());
        lock.metadata.touch();

        let id_str = lock.id.to_string();
        let updated = self.lock_repo
            .update(&id_str, &lock)
            .await
            .map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?
            .ok_or(ServiceError::NotFound)?;

        Ok(updated)
    }

    /// Admin-only: forcibly break a lock regardless of owner.
    pub async fn break_lock(&self, file_id: Uuid) -> ServiceResult<()> {
        if let Some(lock) = self.lock_repo.find_by_file_id(file_id).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))? {
            self.lock_repo.delete(&lock.id.to_string()).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;
        }
        Ok(())
    }

    /// Get the active (non-expired) lock for a file, if any.
    /// Automatically cleans up expired locks.
    pub async fn get_active_lock(&self, file_id: Uuid) -> ServiceResult<Option<FileLock>> {
        if let Some(lock) = self.lock_repo.find_by_file_id(file_id).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))? {
            if lock.is_expired() {
                self.lock_repo.delete(&lock.id.to_string()).await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))?;
                return Ok(None);
            }
            return Ok(Some(lock));
        }
        Ok(None)
    }

    /// Cleanup all expired locks. Returns the number of locks removed.
    pub async fn cleanup_expired(&self) -> ServiceResult<u64> {
        self.lock_repo.empty_trash().await.map_err(|e| ServiceError::Repository(backbone_core::RepositoryError::DatabaseError(e.to_string())))
    }
}
