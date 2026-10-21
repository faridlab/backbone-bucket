//! The hand-written half of the triggers of `FileLock` (ADR-0031/0034): what
//! the schema's `triggers:` cannot say. The generated write hooks call these
//! methods in file order, inside the write's transaction; an error rolls the
//! write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. The scheduled entries are exposed on the module's
//! job list (`FILE_LOCK_SCHEDULED`, ADR-0020) for the composing service's
//! scheduler; the module itself runs none of them. The hand
//! `LockingService` already removes an expired lock on sight (its
//! `acquire_lock`/`get_active_lock` delete it), which is the behaviour the
//! sweep stood in for.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct FileLockTriggersImpl;

#[async_trait]
impl FileLockTriggers for FileLockTriggersImpl {
    /// No scheduler runs in the module, and the hand `LockingService` already
    /// deletes an expired lock the moment it sees one
    /// ([`crate::application::service::LockingService::acquire_lock`]), so the
    /// sweep never had work. Kept as the declaration's no-op.
    async fn scheduled__expire_overdue_locks(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// No scheduler or notification delivery runs in the module; expiring
    /// locks never notified their holders. Kept as the declaration's no-op.
    async fn scheduled__notify_lock_expiration(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }
}
