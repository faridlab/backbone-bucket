//! The hand-written half of the triggers of `ConversionJob` (ADR-0031/0034):
//! what the schema's `triggers:` cannot say. The generated write hooks call
//! these methods in file order, inside the write's transaction; an error rolls
//! the write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. Where the module's hand code held the real
//! behaviour, it is ported here; where it never existed, the method says so
//! and does nothing. The scheduled entries are exposed on the module's job
//! list (`ConversionJobSCHEDULED`, ADR-0020) for the composing service's
//! scheduler; the module itself runs none of them.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct ConversionJobTriggersImpl;

#[async_trait]
impl ConversionJobTriggers for ConversionJobTriggersImpl {
    /// No dispatcher is wired in the module; pending conversions were never
    /// queued by a schedule (the hand `ConversionService` converts on demand).
    /// Kept as the declaration's no-op.
    async fn scheduled__queue_conversions(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// No scheduler runs in the module; timed-out conversions were never
    /// failed by a sweep. Kept as the declaration's no-op.
    async fn scheduled__timeout_running_jobs(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// Nothing in the module maintains the bucket counters (`file_count`,
    /// `total_size_bytes` are read by exports only), so the completion never
    /// bumped them. Kept as the declaration's no-op.
    async fn update_bucket_stats__update_bucket(
        &self,
        _ctx: &WriteCtx,
        _old: &ConversionJob,
        _e: &ConversionJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; the hand
    /// `ConversionService` stamps its own timestamps when it drives a job.
    /// Kept as the declaration's no-op.
    async fn on_enter_processing__set_started_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ConversionJob,
        _e: &ConversionJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_completed__set_completed_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ConversionJob,
        _e: &ConversionJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_completed__set_progress(
        &self,
        _ctx: &WriteCtx,
        _old: &ConversionJob,
        _e: &ConversionJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_failed__set_completed_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ConversionJob,
        _e: &ConversionJob,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; nothing ran
    /// it under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_cancelled__set_completed_at(
        &self,
        _ctx: &WriteCtx,
        _old: &ConversionJob,
        _e: &ConversionJob,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
