//! The hand-written half of the triggers of `ProcessingJob` (ADR-0031/0034):
//! what the schema's `triggers:` cannot say. The generated write hooks call
//! these methods in file order, inside the write's transaction; an error rolls
//! the write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. The scheduled entries are exposed on the module's
//! job list (`PROCESSING_JOB_SCHEDULED`, ADR-0020) for the composing service's
//! scheduler; the module itself runs none of them.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct ProcessingJobTriggersImpl;

#[async_trait]
impl ProcessingJobTriggers for ProcessingJobTriggersImpl {
    /// No dispatcher is wired in the module; pending jobs were never queued by
    /// a schedule (the hand thumbnail/preview services create and run their
    /// own jobs). Kept as the declaration's no-op.
    async fn scheduled__queue_pending_jobs(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// No scheduler runs in the module; timed-out jobs were never failed by a
    /// sweep. Kept as the declaration's no-op.
    async fn scheduled__expire_long_running_jobs(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// `file_repo.update_processing_status` is a TODO stub in both hand
    /// services (`video_thumbnail_service.rs`, `document_preview_service.rs`),
    /// so the file's roll-up status was never written. Kept as the
    /// declaration's no-op.
    async fn update_file_processing_status__update_stored_file(
        &self,
        _ctx: &WriteCtx,
        _old: &ProcessingJob,
        _e: &ProcessingJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; the hand
    /// services stamp `started_at` when they drive a job. Kept as the
    /// declaration's no-op.
    async fn on_enter_running__set_started_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ProcessingJob,
        _e: &ProcessingJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_completed__set_completed_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ProcessingJob,
        _e: &ProcessingJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_failed__set_completed_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ProcessingJob,
        _e: &ProcessingJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_cancelled__set_completed_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ProcessingJob,
        _e: &ProcessingJob,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
