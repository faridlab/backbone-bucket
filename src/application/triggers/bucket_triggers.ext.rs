//! The hand-written half of the triggers of `Bucket` (ADR-0031/0034): what the
//! schema's `triggers:` cannot say. The generated write hooks call these methods
//! in file order, inside the write's transaction; an error rolls the write back
//! with everything it staged.
//!
//! These were declarations nothing performed while the module ran
//! `legacy_triggers: true`. Where the module's hand code held the real
//! behaviour, it is ported here; where it never existed, the method says so
//! and does nothing.

use super::*;

/// The implementation the generated service builder attaches.
#[derive(Default)]
pub struct BucketTriggersImpl;

#[async_trait]
impl BucketTriggers for BucketTriggersImpl {
    /// `if: slug == null || slug == '' { set: slug = slugify(name) }` — ported
    /// from [`crate::domain::utils::slugify`], guard kept as declared. The
    /// write guard's `@required` on `slug` refuses an empty slug before the
    /// triggers run, so the fallback is unreachable through the generic create
    /// today; it stands for the hand verb that builds a Bucket without one.
    async fn before_create__derive_slug(&self, _ctx: &WriteCtx, e: &mut Bucket) -> ServiceResult<()> {
        if e.slug.trim().is_empty() {
            e.slug = crate::domain::utils::slugify(&e.name);
        }
        Ok(())
    }

    /// No `StorageProvisioningService` exists in the module: object storage has
    /// no directories to create (a bucket's objects land under their keys), so
    /// nothing was ever provisioned. Kept as the declaration's no-op.
    async fn after_create__call_storage_provisioning_service_create_bucket_directory(
        &self,
        _ctx: &WriteCtx,
        _e: &Bucket,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No queue infrastructure is wired in the module; storage cleanup on
    /// delete never ran. Kept as the declaration's no-op.
    async fn after_delete__queue_cleanup_bucket_storage(
        &self,
        _ctx: &WriteCtx,
        _e: &Bucket,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No queue infrastructure is wired in the module; archived buckets were
    /// never compressed. Kept as the declaration's no-op.
    async fn on_enter_archived__queue_compress_archived_bucket(
        &self,
        _ctx: &WriteCtx,
        _old: &Bucket,
        _e: &Bucket,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// An after-write `set:` would need a second write of the row; the soft
    /// delete the generic path performs already stamps `deleted_at`, and
    /// nothing ran this under the legacy flag. Kept as the declaration's no-op.
    async fn on_enter_deleted__set_metadata_deleted_at(
        &self,
        _ctx: &WriteCtx,
        _old: &Bucket,
        _e: &Bucket,
    ) -> ServiceResult<()> {
        Ok(())
    }

    /// No scheduler or queue runs in the module; the 30-day purge was never
    /// scheduled. Kept as the declaration's no-op.
    async fn on_enter_deleted__queue_schedule_bucket_purge(
        &self,
        _ctx: &WriteCtx,
        _old: &Bucket,
        _e: &Bucket,
    ) -> ServiceResult<()> {
        Ok(())
    }
}
