// This file's extension (hand-written; ADR-0031). The generated workflow file
// pulls it in with #[path], so its lines resolve against that file.

//! The hand half of `FileProcessing` (ADR-0035): the media verbs that need the
//! object-storage backend and the image crate. Every write goes through the
//! bound services or the storage port — no SQL here, and the step's
//! transaction rides whatever the repositories join.

use async_trait::async_trait;
use backbone_core::service::{ServiceError, ServiceResult};
use backbone_core::write_hooks::WriteCtx;
use serde_json::{json, Value};

use super::super::{base64_encode, content_bytes, services, storage, threat_scan};
use super::*;

#[derive(Default)]
pub struct FileProcessingActionsImpl;

impl FileProcessingActionsImpl {
    /// The bytes of a stored object.
    async fn object(&self, key: &str) -> ServiceResult<bytes::Bytes> {
        storage()?
            .get(key)
            .await
            .map_err(|e| ServiceError::Internal(format!("read `{key}` from storage: {e}")))
    }

    /// The refusal for a media kind this module cannot process in-process.
    fn unsupported(what: &str) -> ServiceError {
        ServiceError::Validation(format!(
            "this bucket module carries no {what}; the processing job is refused instead of pretending to run"
        ))
    }

    /// Run a ProcessingJob verb through the generated service as a transition:
    /// the machine's check and its on_enter sets and events all apply, and the
    /// row guard allows the state move.
    async fn job_transition(
        &self,
        id: &str,
        verb: &str,
        fields: std::collections::HashMap<String, Value>,
    ) -> ServiceResult<Value> {
        let service = &services()?.processing_job;
        let mut fields = fields;
        fields.insert("status".into(), Value::String(verb_target(verb).into()));
        let moved = backbone_core::write_guard::as_transition(
            verb,
            service.partial_update(id, fields),
        )
        .await?
        .ok_or(ServiceError::NotFound)?;
        serde_json::to_value(&moved).map_err(|e| ServiceError::Internal(e.to_string()))
    }
}

/// The state each ProcessingJob verb lands in (the machine's own targets).
fn verb_target(verb: &str) -> &'static str {
    match verb {
        "start" => "running",
        "complete" => "completed",
        "fail" => "failed",
        "retry" => "pending",
        _ => "pending",
    }
}

#[async_trait]
impl FileProcessingActions for FileProcessingActionsImpl {
    /// The machine's `start` verb: pending -> running.
    async fn start_job(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = record_of(params)?;
        self.job_transition(&id, "start", std::collections::HashMap::new()).await
    }

    /// The machine's `complete` verb: [pending, running] -> completed, carrying
    /// the job's result data.
    async fn complete_job(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = record_of(params)?;
        let mut fields = std::collections::HashMap::new();
        if let Some(result) = params.get("result_data") {
            fields.insert("result_data".into(), result.clone());
        }
        self.job_transition(&id, "complete", fields).await
    }

    /// The machine's `fail` verb: [pending, running] -> failed, carrying the error.
    async fn fail_job(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = record_of(params)?;
        let mut fields = std::collections::HashMap::new();
        if let Some(error) = params.get("error_message").and_then(Value::as_str) {
            fields.insert("error_message".into(), Value::String(error.to_string()));
        }
        self.job_transition(&id, "fail", fields).await
    }

    /// The machine's own two moves for a retry: `fail` (running -> failed),
    /// then `retry` (failed -> pending, its guard bounded by max_retries)
    /// carrying the retry bookkeeping.
    async fn requeue_job(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = record_of(params)?;
        let mut fail_fields = std::collections::HashMap::new();
        if let Some(error) = params.get("error_message").and_then(Value::as_str) {
            fail_fields.insert("error_message".into(), Value::String(error.to_string()));
        }
        self.job_transition(&id, "fail", fail_fields).await?;
        let mut retry_fields = std::collections::HashMap::new();
        if let Some(count) = params.get("retry_count") {
            retry_fields.insert("retry_count".into(), count.clone());
        }
        if let Some(after) = params.get("retry_after") {
            retry_fields.insert("retry_after".into(), after.clone());
        }
        self.job_transition(&id, "retry", retry_fields).await
    }

    /// The file machine's `quarantine` verb (processing -> quarantined) after
    /// the scan fields are written; the guard demands a threat level that is
    /// not safe.
    async fn quarantine_file(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = record_of(params)?;
        let service = &services()?.stored_file;
        let mut fields: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
        for key in ["is_scanned", "scan_result", "threat_level", "processing_status"] {
            if let Some(value) = params.get(key) {
                fields.insert(key.to_string(), value.clone());
            }
        }
        service.partial_update(&id, fields).await?.ok_or(ServiceError::NotFound)?;
        let mut move_fields = std::collections::HashMap::new();
        move_fields.insert("status".into(), Value::String("quarantined".into()));
        let moved = backbone_core::write_guard::as_transition(
            "quarantine",
            service.partial_update(&id, move_fields),
        )
        .await?
        .ok_or(ServiceError::NotFound)?;
        serde_json::to_value(&moved).map_err(|e| ServiceError::Internal(e.to_string()))
    }

    /// Generate thumbnail sizes for a stored image. `result.thumbnail` carries
    /// the largest encoded PNG (base64); the workflow stores it beside the
    /// original.
    async fn generate_image_thumbnail(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        let key = params["storage_key"].as_str().ok_or_else(|| ServiceError::Validation("storage_key is required".into()))?;
        let bytes = self.object(key).await?;
        let img = image::load_from_memory(&bytes)
            .map_err(|e| ServiceError::Validation(format!("`{key}` is not a decodable image: {e}")))?;
        let size = params["sizes"]
            .as_array()
            .and_then(|sizes| sizes.iter().max_by_key(|s| s.as_u64().unwrap_or(0)))
            .and_then(|s| s.as_u64())
            .unwrap_or(256) as u32;
        let thumb = img.thumbnail(size, size);
        let mut png = Vec::new();
        thumb
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .map_err(|e| ServiceError::Internal(format!("encode the thumbnail: {e}")))?;
        Ok(json!({
            "thumbnail": base64_encode(&png),
            "width": thumb.width(),
            "height": thumb.height(),
        }))
    }

    /// Store step output (base64 `content`) under `key` on the named backend.
    async fn store_file(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let key = params["key"].as_str().ok_or_else(|| ServiceError::Validation("key is required".into()))?;
        let body = content_bytes(params)?;
        storage()?
            .put(key, body, "application/octet-stream")
            .await
            .map_err(|e| ServiceError::Internal(format!("store `{key}`: {e}")))?;
        Ok(json!({ "key": key, "stored": true }))
    }

    /// Extract a video frame. The module carries no video decoder, so the
    /// verb refuses: the job ends failed with this reason rather than a
    /// pretend success.
    async fn extract_video_frame(
        &self,
        _ctx: &WriteCtx,
        _params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        Err(Self::unsupported("video decoder"))
    }

    /// Render a document preview. The module carries no renderer; the verb
    /// refuses as above.
    async fn generate_document_preview(
        &self,
        _ctx: &WriteCtx,
        _params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        Err(Self::unsupported("document renderer"))
    }

    /// Compress a stored object. The module carries no compressor; the verb
    /// refuses as above.
    async fn compress_file(
        &self,
        _ctx: &WriteCtx,
        _params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        Err(Self::unsupported("stream compressor"))
    }

    /// Scan a stored object for threats: the in-process heuristic (EICAR
    /// signature plus a blocked-extension list).
    async fn scan_file_for_threats(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        let key = params["storage_key"].as_str().ok_or_else(|| ServiceError::Validation("storage_key is required".into()))?;
        let filename = params["filename"].as_str().unwrap_or(key);
        let bytes = self.object(key).await?;
        Ok(threat_scan(&bytes, filename))
    }

    /// Hash a stored object (sha256) for the deduplication branch.
    async fn calculate_file_hash(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        use sha2::{Digest, Sha256};
        let key = params["storage_key"].as_str().ok_or_else(|| ServiceError::Validation("storage_key is required".into()))?;
        let bytes = self.object(key).await?;
        let hash = Sha256::digest(&bytes);
        Ok(json!({ "hash": hex_of(&hash), "size_bytes": bytes.len() }))
    }

    /// Compensation: drop a stored object.
    async fn delete_file(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let key = params["key"].as_str().ok_or_else(|| ServiceError::Validation("key is required".into()))?;
        match storage()?.delete(key).await {
            Ok(()) => Ok(json!({ "key": key, "deleted": true })),
            Err(e) => Err(ServiceError::Internal(format!("delete `{key}`: {e}"))),
        }
    }

    /// `retry_after` for a retried job: now plus an exponential backoff
    /// (2^attempt minutes, capped at 30).
    async fn fn_exponential_backoff(&self, _ctx: &WriteCtx, args: &[Value]) -> ServiceResult<Value> {
        let attempt = args.first().and_then(Value::as_u64).unwrap_or(0);
        let minutes = 1i64 << attempt.min(5);
        let at = chrono::Utc::now() + chrono::Duration::minutes(minutes.min(30));
        Ok(Value::String(at.to_rfc3339()))
    }
}

/// The `id` parameter of a step, as a string.
fn record_of(params: &Value) -> ServiceResult<String> {
    params
        .get("id")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| ServiceError::Validation("the step names no record".into()))
}

/// Lowercase hex of a digest (the checksum spelling the module stores).
fn hex_of(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
