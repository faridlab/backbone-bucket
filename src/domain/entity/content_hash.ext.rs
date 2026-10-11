//! The extension of `content_hash.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl ContentHash {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Increment reference count
    pub fn increment_reference(&mut self) -> Result<(), DedupError> {
        self.reference_count += 1;
        self.last_referenced_at = Utc::now();
        self.metadata.touch();
        Ok(())
    }

    /// Decrement reference count (min 0), return true if now 0 (eligible for deletion)
    pub fn decrement_reference(&mut self) -> Result<bool, DedupError> {
        self.reference_count = (self.reference_count - 1).max(0);
        self.metadata.touch();
        Ok(self.reference_count == 0)
    }

    /// Check if content can be deleted (ref_count == 0)
    pub fn can_delete(&self) -> bool {
        self.reference_count == 0
    }

    /// Check if content has no references
    pub fn is_unused(&self) -> bool {
        self.reference_count == 0
    }

    /// Calculate storage saved by deduplication: size_bytes * (reference_count - 1)
    pub fn storage_saved(&self) -> i64 {
        self.size_bytes * (self.reference_count - 1).max(0) as i64
    }

    /// Get age in days since first upload
    pub fn age_days(&self) -> i32 {
        (Utc::now() - self.first_uploaded_at).num_days() as i32
    }

    /// Days since last reference
    pub fn days_since_last_reference(&self) -> i32 {
        (Utc::now() - self.last_referenced_at).num_days() as i32
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.reference_count < 0 {
            errors.push("reference_count must be >= 0");
        }
        if self.hash.is_empty() || self.hash.contains("..") {
            errors.push("hash must not be empty and must not contain '..'");
        }
        if self.size_bytes <= 0 {
            errors.push("size_bytes must be > 0");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
