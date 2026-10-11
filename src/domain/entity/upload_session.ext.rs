//! The extension of `upload_session.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl UploadSession {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if session has expired
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    /// Add completed part
    pub fn add_part(&mut self, part_number: i32, etag: String) -> Result<(), UploadError> {
        if self.is_expired() {
            return Err(UploadError::ValidationFailed("Upload session has expired".to_string()));
        }

        if self.completed_parts.contains(&part_number) {
            return Err(UploadError::Conflict(format!("Part {} already uploaded", part_number)));
        }

        if self.uploaded_chunks >= self.total_chunks {
            return Err(UploadError::ValidationFailed("All chunks already uploaded".to_string()));
        }

        self.completed_parts.push(part_number);
        self.uploaded_chunks += 1;

        // Store etag in part_etags JSON
        let etags = self.part_etags.get_or_insert_with(|| serde_json::json!({}));
        if let Some(obj) = etags.as_object_mut() {
            obj.insert(part_number.to_string(), serde_json::Value::String(etag));
        }

        self.status = UploadStatus::Uploading;
        self.metadata.updated_at = Some(Utc::now());

        Ok(())
    }

    /// Check if all parts uploaded
    pub fn is_complete(&self) -> bool {
        self.uploaded_chunks >= self.total_chunks
    }

    /// Get upload progress percentage
    pub fn calculate_progress(&self) -> i32 {
        if self.total_chunks <= 0 {
            return 0;
        }
        ((self.uploaded_chunks as f64 / self.total_chunks as f64) * 100.0) as i32
    }

    /// Get number of remaining chunks
    pub fn remaining_chunks(&self) -> i32 {
        (self.total_chunks - self.uploaded_chunks).max(0)
    }

    /// Check if upload can be resumed
    pub fn can_resume(&self) -> bool {
        if self.is_expired() {
            return false;
        }
        matches!(self.status, UploadStatus::Initiated | UploadStatus::Uploading)
    }

    /// Mark session as completing
    pub fn mark_complete(&mut self) -> Result<(), UploadError> {
        if !self.is_complete() {
            return Err(UploadError::ValidationFailed(
                format!("Upload not complete: {}/{} chunks uploaded", self.uploaded_chunks, self.total_chunks),
            ));
        }
        self.status = UploadStatus::Completed;
        self.metadata.updated_at = Some(Utc::now());
        Ok(())
    }

    /// Mark session as failed
    pub fn mark_failed(&mut self, error: String) -> Result<(), UploadError> {
        self.status = UploadStatus::Failed;
        // Store error message in part_etags JSON under "_error" key
        let etags = self.part_etags.get_or_insert_with(|| serde_json::json!({}));
        if let Some(obj) = etags.as_object_mut() {
            obj.insert("_error".to_string(), serde_json::Value::String(error));
        }
        self.metadata.updated_at = Some(Utc::now());
        Ok(())
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        // Invariant 1: uploaded_chunks <= total_chunks
        if self.uploaded_chunks > self.total_chunks {
            errors.push("uploaded_chunks must not exceed total_chunks");
        }
        // Invariant 2: uploaded_chunks == completed_parts.length
        if self.uploaded_chunks != self.completed_parts.len() as i32 {
            errors.push("uploaded_chunks must equal completed_parts length");
        }
        // Invariant 3: file_size > 0 and total_chunks > 0
        if self.file_size <= 0 {
            errors.push("file_size must be greater than 0");
        }
        if self.total_chunks <= 0 {
            errors.push("total_chunks must be greater than 0");
        }
        // Invariant 4: chunk_size >= 5 * 1024 * 1024 (minimum 5MB)
        if self.chunk_size < 5 * 1024 * 1024 {
            errors.push("chunk_size must be at least 5MB (5242880 bytes)");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
