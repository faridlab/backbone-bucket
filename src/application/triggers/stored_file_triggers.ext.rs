//! The hand-written half of the triggers of `StoredFile` (ADR-0031/0034): what
//! the schema's `triggers:` cannot say. The generated write hooks call these
//! methods in file order, inside the write's transaction; an error rolls the
//! write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. The one piece with a living implementation — the
//! quota bookkeeping of a file becoming active or deleted — is ported here
//! from `MultipartUploadService::record_completed_usage`, the module's own
//! accounting. Everything else never ran, and says so.

use std::sync::{Arc, OnceLock, Weak};

use super::*;

use crate::application::service::UserQuotaService;
use crate::infrastructure::persistence::UserQuotaRepository;
use crate::presentation::dto::UpdateUserQuotaDto;

/// The quota read and write the bookkeeping trigger uses, bound by the module's
/// build step (`lib.ext.rs`). Held weakly: the stored-file service owns the
/// hooks that own this.
static QUOTA_SERVICE: OnceLock<Weak<UserQuotaService>> = OnceLock::new();
static QUOTA_REPOSITORY: OnceLock<Weak<UserQuotaRepository>> = OnceLock::new();

/// Bind the quota service and repository the bookkeeping triggers use.
pub fn bind_quota_bookkeeping(
    service: &Arc<UserQuotaService>,
    repository: &Arc<UserQuotaRepository>,
) {
    let _ = QUOTA_SERVICE.set(Arc::downgrade(service));
    let _ = QUOTA_REPOSITORY.set(Arc::downgrade(repository));
}

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct StoredFileTriggersImpl;

impl StoredFileTriggersImpl {
    /// `used_bytes ± size_bytes, file_count ± 1`, peak usage kept current —
    /// the exact accounting of
    /// [`MultipartUploadService::record_completed_usage`], written through the
    /// quota's generated service so it joins this write's transaction
    /// (ADR-0034 decision 5). The hand upload path keeps its own post-commit
    /// call (a repository write fires no hooks), so no path counts twice.
    async fn account_quota(&self, file: &StoredFile, sign: i64) -> ServiceResult<()> {
        let (Some(service), Some(repository)) = (
            QUOTA_SERVICE.get().and_then(Weak::upgrade),
            QUOTA_REPOSITORY.get().and_then(Weak::upgrade),
        ) else {
            // Built outside the module (no binding): the legacy behaviour was
            // to do nothing, so nothing is what happens.
            tracing::debug!(
                "quota bookkeeping skipped: the module's quota service is not bound"
            );
            return Ok(());
        };
        let Some(mut quota) = repository
            .find_by_user_id(file.owner_id)
            .await
            .map_err(|e| ServiceError::Internal(format!("read the user's quota: {e}")))?
        else {
            // No quota row: no limit configured, exactly as the hand path.
            return Ok(());
        };
        quota.used_bytes = if sign < 0 {
            quota.used_bytes.saturating_sub(file.size_bytes)
        } else {
            quota.used_bytes.saturating_add(file.size_bytes)
        };
        quota.file_count = if sign < 0 {
            quota.file_count.saturating_sub(1)
        } else {
            quota.file_count.saturating_add(1)
        };
        if quota.used_bytes > quota.peak_usage_bytes {
            quota.peak_usage_bytes = quota.used_bytes;
            quota.peak_usage_at = Some(chrono::Utc::now());
        }
        let id = quota.id.to_string();
        service
            .update(
                &id,
                UpdateUserQuotaDto {
                    user_id: quota.user_id,
                    limit_bytes: quota.limit_bytes,
                    used_bytes: quota.used_bytes,
                    file_count: quota.file_count,
                    max_file_size: quota.max_file_size,
                    max_file_count: quota.max_file_count,
                    tier: quota.tier.clone(),
                    quota_status: quota.quota_status.clone(),
                    warning_threshold_percent: quota.warning_threshold_percent,
                    last_warning_sent_at: quota.last_warning_sent_at,
                    peak_usage_bytes: quota.peak_usage_bytes,
                    peak_usage_at: quota.peak_usage_at,
                },
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl StoredFileTriggers for StoredFileTriggersImpl {
    /// No access-log row was ever written on an upload (the upload route
    /// writes only the file). Kept as the declaration's no-op.
    async fn after_create__create_file_access_log(
        &self,
        _ctx: &WriteCtx,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; the
    /// download route does not stamp `last_accessed_at` today. Kept as the
    /// declaration's no-op.
    async fn after_update_download_count__set_last_accessed_at(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `user_id`/`share_id`/`ip_address` name the request, which
    /// the row cannot know; nothing staged the event under the legacy flag.
    /// Kept as the declaration's no-op.
    async fn after_update_download_count__emit_file_downloaded_event(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// Shares and access logs of a deleted file were never removed (no hand
    /// path did it). Kept as the declaration's no-op.
    async fn after_delete__delete_file_share(
        &self,
        _ctx: &WriteCtx,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// Shares and access logs of a deleted file were never removed (no hand
    /// path did it). Kept as the declaration's no-op.
    async fn after_delete__delete_file_access_log(
        &self,
        _ctx: &WriteCtx,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_uploading__set_processing_status(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No queue infrastructure is wired in the module; the scan was never
    /// queued from a transition. Kept as the declaration's no-op.
    async fn on_enter_processing__queue_scan_file(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No queue infrastructure is wired in the module; the thumbnail job was
    /// never queued from a transition (the hand services create their own).
    /// Kept as the declaration's no-op.
    async fn on_enter_processing__queue_generate_thumbnail(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No queue infrastructure is wired in the module; the deduplication check
    /// was never queued from a transition. Kept as the declaration's no-op.
    async fn on_enter_processing__queue_check_deduplication(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_processing__set_processing_status(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// `update: Bucket(file_count: file_count + 1, total_size_bytes:
    /// total_size_bytes + size_bytes)` — nothing in the module maintains the
    /// bucket counters (they are read by exports only), so the activation
    /// never bumped them. Kept as the declaration's no-op.
    async fn on_enter_active__update_bucket(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// `update: UserQuota(user_id: owner_id, used_bytes: used_bytes +
    /// size_bytes)` — ported from the module's real accounting
    /// (`MultipartUploadService::record_completed_usage`): `used_bytes` and
    /// `file_count` rise by the file, peak usage is kept current. See
    /// [`Self::account_quota`].
    async fn on_enter_active__update_user_quota(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        e: &StoredFile,
    ) -> ServiceResult<()> {
        self.account_quota(e, 1).await
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_active__set_processing_status(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `threat_name: $context.threat_name` names the scan's result,
    /// which the row cannot know; nothing staged the event under the legacy
    /// flag. Kept as the declaration's no-op.
    async fn on_enter_quarantined__emit_file_quarantined_event(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No notification delivery runs in the module; admins were never paged on
    /// a quarantine. Kept as the declaration's no-op.
    async fn on_enter_quarantined__notify(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_quarantined__set_processing_status(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row (the soft
    /// delete already stamps `deleted_at`); nothing ran it under the legacy
    /// flag. Kept as the declaration's no-op.
    async fn on_enter_deleted__set_metadata_deleted_at(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// Nothing maintains the bucket counters (see
    /// [`Self::on_enter_active__update_bucket`]). Kept as the declaration's
    /// no-op.
    async fn on_enter_deleted__update_bucket(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The symmetric accounting of [`Self::on_enter_active__update_user_quota`],
    /// ported from `record_completed_usage`'s decrements.
    async fn on_enter_deleted__update_user_quota(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        e: &StoredFile,
    ) -> ServiceResult<()> {
        self.account_quota(e, -1).await
    }

    /// The CDN invalidation the declaration names has no caller: the hand
    /// `CdnService` only mints and verifies URLs. Kept as the declaration's
    /// no-op.
    async fn on_enter_deleted__invalidate_cdn(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// Object deletion never ran from a transition: no hand path calls the
    /// storage backend's delete. Kept as the declaration's no-op.
    async fn on_enter_purged__delete_storage(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The deduplication reference count was never decremented (the hand
    /// `DeduplicationService` the declaration names is schema-only). Kept as
    /// the declaration's no-op.
    async fn on_enter_purged__dedup_decrement(
        &self,
        _ctx: &WriteCtx,
        _old: &StoredFile,
        _e: &StoredFile,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
