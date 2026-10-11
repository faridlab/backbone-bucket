//! The extension of `bucket.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl Bucket {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if file can be uploaded to this bucket
    pub fn can_upload(&self, file_size: i64, mime_type: &str) -> bool {
        if self.status != BucketStatus::Active {
            return false;
        }
        if let Some(max_size) = self.max_file_size {
            if file_size > max_size {
                return false;
            }
        }
        if !self.allowed_mime_types.is_empty() && !self.allowed_mime_types.contains(&mime_type.to_string()) {
            return false;
        }
        true
    }

    /// Update file count and total size
    pub fn update_stats(&mut self, size_delta: i64, count_delta: i32) {
        self.file_count = (self.file_count + count_delta).max(0);
        self.total_size_bytes = (self.total_size_bytes + size_delta).max(0);
    }

    /// Check if bucket is accessible (not deleted/archived)
    pub fn is_accessible(&self) -> bool {
        !self.is_deleted() && self.status != BucketStatus::Archived
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.file_count < 0 {
            errors.push("file_count must be >= 0");
        }
        if self.total_size_bytes < 0 {
            errors.push("total_size_bytes must be >= 0");
        }
        if self.name.trim().is_empty() {
            errors.push("name must not be empty");
        }
        if self.slug.trim().is_empty() {
            errors.push("slug must not be empty");
        }
        if self.root_path.contains("..") {
            errors.push("root_path must not contain path traversal");
        }
        if let Some(max_file_size) = self.max_file_size {
            if max_file_size <= 0 {
                errors.push("max_file_size must be positive when set");
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
