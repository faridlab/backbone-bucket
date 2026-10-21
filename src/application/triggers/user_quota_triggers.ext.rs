//! The hand-written half of the triggers of `UserQuota` (ADR-0031/0034): what
//! the schema's `triggers:` cannot say. The generated write hooks call these
//! methods in file order, inside the write's transaction; an error rolls the
//! write back with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`; no hand code in the module holds a living
//! implementation for any of them, so each says so and does nothing. The
//! scheduled entries are exposed on the module's job list
//! (`USER_QUOTA_SCHEDULED`, ADR-0020) for the composing service's scheduler.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct UserQuotaTriggersImpl;

#[async_trait]
impl UserQuotaTriggers for UserQuotaTriggersImpl {
    /// No tier-default table exists in the module: the schema's
    /// `QuotaEnforcementService` and the config carry no per-tier limits, and
    /// no hand code ever applied one (a create states its own `limit_bytes`).
    /// Kept as the declaration's no-op.
    async fn before_create__apply_tier_defaults(
        &self,
        _ctx: &WriteCtx,
        _e: &mut UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `max_storage_bytes` names a field the model has never had
    /// (its limit field is `limit_bytes`), so the payload cannot be built from
    /// the row; nothing staged the event under the legacy flag. Kept as the
    /// declaration's no-op.
    async fn after_create__emit_user_quota_created_event(
        &self,
        _ctx: &WriteCtx,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No tier-default table exists (see
    /// [`Self::before_create__apply_tier_defaults`]). Kept as the declaration's
    /// no-op.
    async fn before_update_tier__apply_tier_defaults(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &mut UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No mail leaves the module: an effect outside the database is a
    /// `NotificationRequested` the compiled half would stage, but this
    /// action's arguments (`user.email`, the formatted sizes) are relations
    /// and computed fields the row cannot resolve. Kept as the declaration's
    /// no-op.
    async fn after_update_tier__send_email_tier_changed(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `expires_at` names a field the model has never had, so the
    /// payload cannot be built from the row; nothing staged the event under
    /// the legacy flag. Kept as the declaration's no-op.
    async fn after_update_tier__emit_user_quota_tier_changed_event(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No warning was ever sent at a threshold: `last_warning_sent_at` is
    /// written by nothing in the module. Kept as the declaration's no-op.
    async fn after_update_used_bytes__check_warning_threshold(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// `quota_status` is moved by nothing in the module: the state machine's
    /// `exceed`/`restore` verbs are `hand` and no hand verb runs them, so the
    /// status a create states is the status the row keeps. Kept as the
    /// declaration's no-op.
    async fn after_update_used_bytes__update_exceeded_status(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The model has no bandwidth fields (`used_bandwidth_today` and its kin
    /// were documented by the legacy declaration but never existed), so there
    /// is nothing to reset. Kept as the declaration's no-op.
    async fn scheduled__scheduled_reset_daily_bandwidth(
        &self,
        _ctx: &WriteCtx,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The model has no bandwidth fields (see
    /// [`Self::scheduled__scheduled_reset_daily_bandwidth`]). Kept as the
    /// declaration's no-op.
    async fn scheduled__scheduled_reset_monthly_bandwidth(
        &self,
        _ctx: &WriteCtx,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The model has no `expires_at` field (the legacy declaration's expiry
    /// quota never existed), so there is nothing to expire. Kept as the
    /// declaration's no-op.
    async fn scheduled__scheduled_expire_quotas(&self, _ctx: &WriteCtx) -> ServiceResult<()> {
        Ok(())
    }

    /// The model has no `expires_at` field and no notification delivery runs
    /// in the module. Kept as the declaration's no-op.
    async fn scheduled__scheduled_send_expiry_warnings(
        &self,
        _ctx: &WriteCtx,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row;
    /// `last_warning_sent_at` is written by nothing. Kept as the declaration's
    /// no-op.
    async fn on_enter_normal__set_last_warning_sent_at(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No mail leaves the module: the action's arguments (`user.email`, the
    /// formatted sizes) are relations and computed fields the row cannot
    /// resolve, and no delivery ran under the legacy flag. Kept as the
    /// declaration's no-op.
    async fn on_enter_exceeded__send_email_quota_exceeded(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// The emit's `used_storage_bytes`/`max_storage_bytes` name fields the
    /// model has never had (its fields are `used_bytes`/`limit_bytes`), so the
    /// payload cannot be built from the row; the module's declared
    /// `QuotaExceeded` event (the older spelling) was staged by nothing
    /// either. Kept as the declaration's no-op.
    async fn on_enter_exceeded__emit_quota_exceeded_event(
        &self,
        _ctx: &WriteCtx,
        _old: &UserQuota,
        _e: &UserQuota,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
