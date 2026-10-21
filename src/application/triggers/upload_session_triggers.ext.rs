//! The hand-written half of the triggers of `UploadSession` (ADR-0031/0034):
//! what the schema's `triggers:` cannot say. The generated write hooks call
//! these methods in file order, inside the write's transaction; an error rolls
//! the write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. The scheduled entries are exposed on the module's
//! job list (`UPLOAD_SESSION_SCHEDULED`, ADR-0020) for the composing service's
//! scheduler; the module itself runs none of them.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct UploadSessionTriggersImpl;

#[async_trait]
impl UploadSessionTriggers for UploadSessionTriggersImpl {
    /// The hand `MultipartUploadService` checks expiry on use
    /// (`get_active_session`); the scheduled sweep itself never ran. Kept as
    /// the declaration's no-op.
    async fn scheduled__expire_old_sessions(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// `MultipartUploadService::cleanup_expired` is the living idea, and it is
    /// a TODO stub that refuses (`expire_stale_sessions not yet implemented`);
    /// the sweep never ran. Kept as the declaration's no-op.
    async fn scheduled__cleanup_expired_parts(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// No scheduler or notification delivery runs in the module; expiring
    /// sessions never notified their owners. Kept as the declaration's no-op.
    async fn scheduled__notify_expiring_sessions(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `file_id: $context.file_id` names the assembled file, which
    /// the row cannot know; nothing staged the event under the legacy flag.
    /// Kept as the declaration's no-op.
    async fn on_enter_completed__emit_upload_completed_event(
        &self,
        _ctx: &WriteCtx,
        _old: &UploadSession,
        _e: &UploadSession,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `error: $context.error_message` names the failure, which the
    /// row cannot know; nothing staged the event under the legacy flag. Kept
    /// as the declaration's no-op.
    async fn on_enter_failed__emit_upload_failed_event(
        &self,
        _ctx: &WriteCtx,
        _old: &UploadSession,
        _e: &UploadSession,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No queue infrastructure is wired in the module; failed parts were never
    /// cleaned up from a transition. Kept as the declaration's no-op.
    async fn on_enter_failed__queue_cleanup_upload_parts(
        &self,
        _ctx: &WriteCtx,
        _old: &UploadSession,
        _e: &UploadSession,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No queue infrastructure is wired in the module; expired parts were
    /// never cleaned up from a transition. Kept as the declaration's no-op.
    async fn on_enter_expired__queue_cleanup_upload_parts(
        &self,
        _ctx: &WriteCtx,
        _old: &UploadSession,
        _e: &UploadSession,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
