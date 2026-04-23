//! Pluggable authentication and authorization surface.
//!
//! The bucket module owns *serving mechanics* (which bytes, how delivered)
//! but deliberately does NOT own identity or domain authorization. Those
//! are consumer concerns. This module exposes two trait slots the consumer
//! fills:
//!
//! - [`AuthExtractor`] — an Axum [`FromRequestParts`] implementation that
//!   reads whichever token/session/cookie the consumer uses and yields a
//!   typed identity.
//! - [`AuthzPolicy`] — decides whether a given identity may read a given
//!   file. The module ships [`DefaultOwnerOnlyPolicy`] as a sensible
//!   starting point; the consumer plugs in its own for richer rules.
//!
//! Both traits are kept minimal on purpose — the consumer shouldn't need
//! to pull in the module's internals to implement them.

use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRequestParts;

use crate::domain::entity::StoredFile;
use crate::error::BucketError;

/// Per-request identity extractor.
///
/// This is just a marker alias for `FromRequestParts` — any Axum extractor
/// that yields a typed identity (`User`, `SessionToken`, etc.) satisfies
/// it. Consumers choose the representation.
///
/// Rejections must map to [`BucketError::Unauthenticated`] via the
/// [`AuthExtractor::Rejection`] conversion.
pub trait AuthExtractor<S = ()>: FromRequestParts<S> + Send + Sync + 'static {}

impl<T, S> AuthExtractor<S> for T where T: FromRequestParts<S> + Send + Sync + 'static {}

/// Authorization decision for a single read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthzDecision {
    Allow,
    Deny,
}

impl AuthzDecision {
    pub fn is_allowed(self) -> bool {
        matches!(self, AuthzDecision::Allow)
    }
}

/// Domain authorization policy.
///
/// `Identity` is the type produced by the consumer's [`AuthExtractor`].
/// The policy typically calls into the consumer's auth service — e.g.
/// checking a share token, verifying workspace membership, or evaluating
/// an RBAC grant. The bucket module treats the result as opaque.
#[async_trait]
pub trait AuthzPolicy<Identity>: Send + Sync + 'static
where
    Identity: Send + Sync + 'static,
{
    async fn decide(
        &self,
        identity: &Identity,
        file: &StoredFile,
    ) -> Result<AuthzDecision, BucketError>;

    /// Convenience: return `Err(Forbidden)` when `decide` says `Deny`.
    async fn ensure_can_read(
        &self,
        identity: &Identity,
        file: &StoredFile,
    ) -> Result<(), BucketError> {
        match self.decide(identity, file).await? {
            AuthzDecision::Allow => Ok(()),
            AuthzDecision::Deny => Err(BucketError::Forbidden),
        }
    }
}

/// Default policy: the identity must equal the file's owner.
///
/// Requires the consumer's `Identity` type to expose an owner id reachable
/// via the [`HasOwnerId`] trait. Consumers with richer rules (sharing,
/// workspace membership, public files) should implement [`AuthzPolicy`]
/// directly.
pub struct DefaultOwnerOnlyPolicy;

#[async_trait]
impl<I> AuthzPolicy<I> for DefaultOwnerOnlyPolicy
where
    I: HasOwnerId + Send + Sync + 'static,
{
    async fn decide(
        &self,
        identity: &I,
        file: &StoredFile,
    ) -> Result<AuthzDecision, BucketError> {
        if identity.owner_id() == file.owner_id {
            Ok(AuthzDecision::Allow)
        } else {
            Ok(AuthzDecision::Deny)
        }
    }
}

/// Consumer identity types implement this to use [`DefaultOwnerOnlyPolicy`].
pub trait HasOwnerId {
    fn owner_id(&self) -> uuid::Uuid;
}

/// Type-erased policy holder used by the serving handler.
pub type ArcAuthzPolicy<I> = Arc<dyn AuthzPolicy<I>>;
