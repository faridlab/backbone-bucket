//! The extension of `file_share.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl FileShare {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if share is active, not expired, and has downloads remaining
    pub fn is_valid(&self) -> bool {
        self.status == ShareStatus::Active
            && !self.is_expired()
            && self.has_downloads_remaining()
    }

    /// Check if user can access via this share
    pub fn can_access(&self, user_id: Option<Uuid>, password: Option<&str>) -> bool {
        if !self.is_valid() {
            return false;
        }

        match self.share_type {
            ShareType::Link => true,
            ShareType::User => {
                match user_id {
                    Some(uid) => self.shared_with.contains(&uid),
                    None => false,
                }
            }
            ShareType::Password => {
                match (password, &self.password_hash) {
                    (Some(pw), Some(hash)) => {
                        bcrypt::verify(pw, hash).unwrap_or(false)
                    }
                    _ => false,
                }
            }
        }
    }

    /// Increment download count, returns false if limit reached
    pub fn record_download(&mut self) -> bool {
        if !self.has_downloads_remaining() {
            return false;
        }
        self.download_count += 1;
        true
    }

    /// Revoke the share
    pub fn revoke(&mut self, by_user_id: Uuid) {
        self.status = ShareStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        self.revoked_by = Some(by_user_id);
    }

    /// Check if share has expired
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires) => expires <= Utc::now(),
            None => false,
        }
    }

    /// Check if download limit not reached
    pub fn has_downloads_remaining(&self) -> bool {
        match self.max_downloads {
            Some(max) => self.download_count < max,
            None => true,
        }
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        // Invariant: token must not be empty
        if self.token.is_empty() {
            errors.push("token must not be empty");
        }

        // Invariant: token must not contain ".."
        if self.token.contains("..") {
            errors.push("token must not contain '..'");
        }

        // Invariant: download_count must be non-negative
        if self.download_count < 0 {
            errors.push("download_count must be non-negative");
        }

        // Invariant: download_count <= max_downloads (if set)
        if let Some(max) = self.max_downloads {
            if self.download_count > max {
                errors.push("download_count must not exceed max_downloads");
            }
        }

        // Invariant: password shares must have a password_hash
        if self.share_type == ShareType::Password && self.password_hash.is_none() {
            errors.push("password share must have a password_hash");
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
