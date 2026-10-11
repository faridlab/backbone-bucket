//! The extension of `file_lock.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl FileLock {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if lock is still valid (active status and not yet expired)
    pub fn is_valid(&self) -> bool {
        self.status == LockStatus::Active && !self.is_expired()
    }

    /// Check if lock has expired (expires_at is at or before the current time)
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    /// Check if lock belongs to the given user
    pub fn is_owned_by(&self, user_id: Uuid) -> bool {
        self.user_id == user_id
    }

    /// Check if lock can be refreshed (must be currently valid)
    pub fn can_refresh(&self) -> bool {
        self.is_valid()
    }

    /// Refresh lock expiry time by extending it by the given duration
    pub fn refresh(&mut self, duration: Duration) -> Result<(), LockError> {
        if !self.can_refresh() {
            return Err(LockError::ValidationFailed(
                "Cannot refresh lock: lock is not active or has expired".to_string(),
            ));
        }

        let now = Utc::now();
        self.expires_at = now + duration;
        self.refreshed_at = Some(now);
        self.metadata.touch();

        Ok(())
    }

    /// Get remaining time until expiry. Returns zero duration if already expired.
    pub fn time_remaining(&self) -> Duration {
        let remaining = self.expires_at - Utc::now();
        if remaining < Duration::zero() {
            Duration::zero()
        } else {
            remaining
        }
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        // Invariant 1: expires_at must be after locked_at
        if self.expires_at <= self.locked_at {
            errors.push("expires_at must be after locked_at");
        }

        // Invariant 2: if status is Active, it must not be expired
        if self.status == LockStatus::Active && self.is_expired() {
            errors.push("active lock must not be expired (expires_at must be in the future)");
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
