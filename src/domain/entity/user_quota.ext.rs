//! The extension of `user_quota.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl UserQuota {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if user has space for additional bytes
    pub fn has_space_for(&self, bytes: i64) -> bool {
        self.used_bytes + bytes <= self.limit_bytes
    }

    /// Get current usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.limit_bytes <= 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.limit_bytes as f64) * 100.0
    }

    /// Check if usage exceeds warning threshold
    pub fn is_over_warning_threshold(&self) -> bool {
        self.usage_percent() >= self.warning_threshold_percent as f64
    }

    /// Add bytes to usage, returns error if exceeded
    pub fn add_usage(&mut self, bytes: i64) -> Result<(), QuotaExceeded> {
        if self.used_bytes + bytes > self.limit_bytes {
            return Err(QuotaExceeded::ValidationFailed(
                format!(
                    "Adding {} bytes would exceed quota limit ({}/{} bytes)",
                    bytes, self.used_bytes + bytes, self.limit_bytes
                ),
            ));
        }
        self.used_bytes += bytes;
        self.metadata.updated_at = Some(Utc::now());
        Ok(())
    }

    /// Subtract bytes from usage
    pub fn subtract_usage(&mut self, bytes: i64) {
        self.used_bytes = (self.used_bytes - bytes).max(0);
        self.metadata.updated_at = Some(Utc::now());
    }

    /// Update peak usage if current exceeds
    pub fn update_peak(&mut self) {
        if self.used_bytes > self.peak_usage_bytes {
            self.peak_usage_bytes = self.used_bytes;
            self.peak_usage_at = Some(Utc::now());
        }
    }

    /// Get remaining available bytes
    pub fn remaining_bytes(&self) -> i64 {
        (self.limit_bytes - self.used_bytes).max(0)
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        // Invariant 1: used_bytes <= limit_bytes
        if self.used_bytes > self.limit_bytes {
            errors.push("used_bytes must not exceed limit_bytes");
        }
        // Invariant 2: used_bytes >= 0
        if self.used_bytes < 0 {
            errors.push("used_bytes must be non-negative");
        }
        // Invariant 3: file_count >= 0
        if self.file_count < 0 {
            errors.push("file_count must be non-negative");
        }
        // Invariant 4: warning_threshold_percent in 0..=100
        if self.warning_threshold_percent < 0 || self.warning_threshold_percent > 100 {
            errors.push("warning_threshold_percent must be between 0 and 100");
        }
        // Invariant 5: limit_bytes must be positive
        if self.limit_bytes <= 0 {
            errors.push("limit_bytes must be positive");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
