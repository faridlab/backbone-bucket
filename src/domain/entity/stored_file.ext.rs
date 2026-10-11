//! The extension of `stored_file.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl StoredFile {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if file is accessible (active and not deleted)
    pub fn is_accessible(&self) -> bool {
        !self.is_deleted() && self.status == FileStatus::Active
    }

    /// Check if file passed security scan
    pub fn is_safe(&self) -> bool {
        self.is_scanned
            && match &self.threat_level {
                None => true,
                Some(ThreatLevel::Safe) => true,
                _ => false,
            }
    }

    /// Check if file needs processing (not yet scanned)
    pub fn needs_processing(&self) -> bool {
        !self.is_scanned
    }

    /// Increment download count and update last_accessed_at
    pub fn record_access(&mut self) {
        self.download_count += 1;
        self.last_accessed_at = Some(Utc::now());
    }

    /// Mark file as deleted (move to trash)
    pub fn soft_delete(&mut self) {
        self.status = FileStatus::Deleted;
        self.metadata.deleted_at = Some(Utc::now());
    }

    /// Restore file from trash
    pub fn restore(&mut self) {
        self.status = FileStatus::Active;
        self.metadata.deleted_at = None;
    }

    /// Mark file as quarantined due to threats
    pub fn quarantine(&mut self, threats: Vec<String>) {
        self.status = FileStatus::Quarantined;
        self.scan_result = Some(serde_json::json!(threats));
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        if self.size_bytes <= 0 {
            errors.push("size_bytes must be greater than 0");
        }

        if self.path.contains("..") {
            errors.push("path must not contain '..' (path traversal)");
        }

        if self.original_name.is_empty() {
            errors.push("original_name must not be empty");
        }

        if self.storage_key.is_empty() {
            errors.push("storage_key must not be empty");
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
