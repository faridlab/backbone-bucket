//! The hand-written half of the triggers of `FileComment` (ADR-0031/0034): what
//! the schema's `triggers:` cannot say. The generated write hooks call these
//! methods in file order, inside the write's transaction; an error rolls the
//! write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. No notification delivery exists in the module (a
//! `NotificationRequested` needs a relay sink no host has mounted for these
//! events), so both conditionals are the declarations' no-ops.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct FileCommentTriggersImpl;

#[async_trait]
impl FileCommentTriggers for FileCommentTriggersImpl {
    /// Nothing notified a file's owner or the mentioned users on a comment:
    /// the relation reads (`file.owner_id`, mentions) never resolved and no
    /// delivery ran. Kept as the declaration's no-op.
    async fn after_create__if_user_id(
        &self,
        _ctx: &WriteCtx,
        _e: &FileComment,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// Nothing notified a parent comment's author on a reply (`get_comment`
    /// never resolved, no delivery ran). Kept as the declaration's no-op.
    async fn after_create__if_parent_id(
        &self,
        _ctx: &WriteCtx,
        _e: &FileComment,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
