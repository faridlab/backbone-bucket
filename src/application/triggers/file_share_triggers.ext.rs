//! The hand-written half of the triggers of `FileShare` (ADR-0031/0034): what
//! the schema's `triggers:` cannot say. The generated write hooks call these
//! methods in file order, inside the write's transaction; an error rolls the
//! write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. Where the module's hand code held the real
//! behaviour, it is ported here; where it never existed, the method says so
//! and does nothing.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct FileShareTriggersImpl;

#[async_trait]
impl FileShareTriggers for FileShareTriggersImpl {
    /// `if: token == null || token == '' { set: token =
    /// generate_secure_token(32) }` — ported from
    /// [`crate::domain::utils::generate_share_token`], guard kept as declared.
    /// The write guard's `@required` (min 32) on `token` refuses an empty token
    /// before the triggers run, so the fallback is unreachable through the
    /// generic create today; it stands for the hand verb that builds a share
    /// without one.
    async fn before_create__ensure_token(&self, _ctx: &WriteCtx, e: &mut FileShare) -> ServiceResult<()> {
        if e.token.trim().is_empty() {
            e.token = crate::domain::utils::generate_share_token(32);
        }
        Ok(())
    }

    /// `$input.password` names the create request's body, which a trigger
    /// cannot see. The living behaviour is the password verb:
    /// [`crate::presentation::http::FileSharePassword::set`] takes the
    /// plaintext, argon2-hashes it and writes the row. Kept as the
    /// declaration's no-op.
    async fn before_create__if_share_type(
        &self,
        _ctx: &WriteCtx,
        _e: &mut FileShare,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// Nothing notified share recipients: the relation reads
    /// (`file.original_name`, `owner.email`, `generate_share_url`) never
    /// resolved and no delivery ran. Kept as the declaration's no-op.
    async fn after_create__if_share_type(
        &self,
        _ctx: &WriteCtx,
        _e: &FileShare,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No access-log row was ever written on a share download (the download
    /// route reads only). Kept as the declaration's no-op.
    async fn after_update_download_count__create_file_access_log(
        &self,
        _ctx: &WriteCtx,
        _old: &FileShare,
        _e: &FileShare,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `user_id: $context.user_id` names the downloader, which the
    /// row cannot know; nothing staged the event under the legacy flag. Kept
    /// as the declaration's no-op.
    async fn after_update_download_count__emit_share_accessed_event(
        &self,
        _ctx: &WriteCtx,
        _old: &FileShare,
        _e: &FileShare,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `shared_with_user_id: $context.shared_with` names the create
    /// request's context, which the row cannot know; nothing staged the event
    /// under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_active__emit_share_created_event(
        &self,
        _ctx: &WriteCtx,
        _old: &FileShare,
        _e: &FileShare,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; no hand
    /// revocation path stamps `revoked_at` today (the state change itself is
    /// the record). Kept as the declaration's no-op.
    async fn on_enter_revoked__set_revoked_at(
        &self,
        _ctx: &WriteCtx,
        _old: &FileShare,
        _e: &FileShare,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `revoked_by: $context.actor_id` names the request's actor,
    /// which the row cannot know; nothing staged the event under the legacy
    /// flag. Kept as the declaration's no-op.
    async fn on_enter_revoked__emit_share_revoked_event(
        &self,
        _ctx: &WriteCtx,
        _old: &FileShare,
        _e: &FileShare,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
