//! The extension of `file_comment.rs` (hand-written; ADR-0031).

#[allow(unused_imports)]
use super::*;

impl FileComment {

    // ==========================================================
    // DDD Entity Methods
    // ==========================================================

    /// Check if this is a reply to another comment
    pub fn is_reply(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Check if comment has region annotations
    pub fn has_annotations(&self) -> bool {
        self.annotation_region
            .as_ref()
            .map(|v| !v.is_null() && v != &serde_json::Value::Object(serde_json::Map::new()))
            .unwrap_or(false)
    }

    /// Get reply comments (aggregate pattern - replies loaded separately via repository)
    pub fn get_replies(&self) -> Vec<FileComment> {
        Vec::new()
    }

    /// Mark comment as resolved
    pub fn resolve(&mut self, by_user_id: Uuid) -> Result<(), CommentError> {
        if self.resolved {
            return Err(CommentError::Conflict("Comment is already resolved".to_string()));
        }
        self.resolved = true;
        self.resolved_by = Some(by_user_id);
        self.resolved_at = Some(Utc::now());
        self.metadata.touch();
        Ok(())
    }

    /// Mark comment as unresolved
    pub fn unresolve(&mut self) -> Result<(), CommentError> {
        if !self.resolved {
            return Err(CommentError::Conflict("Comment is not resolved".to_string()));
        }
        self.resolved = false;
        self.resolved_by = None;
        self.resolved_at = None;
        self.metadata.touch();
        Ok(())
    }

    /// Count mentioned users
    pub fn mention_count(&self) -> i32 {
        self.mentions.len() as i32
    }

    /// Check all business invariants
    pub fn check_invariants(&self) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();
        if self.content.len() > 10000 {
            errors.push("content must not exceed 10000 characters");
        }
        if self.parent_id == Some(self.id) {
            errors.push("parent_id must not reference self");
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
