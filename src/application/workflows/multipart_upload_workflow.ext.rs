// This file's extension (hand-written; ADR-0031). The generated workflow file
// pulls it in with #[path], so its lines resolve against that file.

//! The hand half of `MultipartUpload` (ADR-0035): the chunk sizing, mirroring
//! `MultipartUploadService::initiate`'s arithmetic under the workflow's own
//! min/max/parts constraints.

use async_trait::async_trait;
use backbone_core::service::ServiceResult;
use backbone_core::write_hooks::WriteCtx;
use serde_json::{json, Value};

use super::*;

#[derive(Default)]
pub struct MultipartUploadActionsImpl;

#[async_trait]
impl MultipartUploadActions for MultipartUploadActionsImpl {
    /// The chunk size for a `file_size`-byte upload: the largest power-of-two
    /// friendly size between `min_chunk_size` and `max_chunk_size` that keeps
    /// the part count within `max_parts`.
    async fn calculate_chunk_size(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        let file_size = params["file_size"].as_i64().unwrap_or(0);
        if file_size <= 0 {
            return Err(backbone_core::service::ServiceError::Validation(
                "file_size must be positive".into(),
            ));
        }
        let min = params["min_chunk_size"].as_i64().unwrap_or(5_242_880).max(1);
        let max = params["max_chunk_size"].as_i64().unwrap_or(5_368_709_120);
        let max_parts = params["max_parts"].as_i64().unwrap_or(10_000).max(1);
        let needed = (file_size + max_parts - 1) / max_parts; // smallest size that fits the part budget
        let chunk = needed.clamp(min, max).max(min);
        let total = (file_size + chunk - 1) / chunk;
        Ok(json!({ "chunk_size": chunk, "total_chunks": total }))
    }
}
