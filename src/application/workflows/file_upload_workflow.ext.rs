// This file's extension (hand-written; ADR-0031). The generated workflow file
// pulls it in with #[path], so its lines resolve against that file.

//! The hand half of `FileUpload` (ADR-0035): checksum, mime sniffing, the
//! threat scan, object storage and the CDN URL. Writes go through the bound
//! services or the storage port.

use std::collections::HashMap;

use async_trait::async_trait;
use backbone_core::service::{ServiceError, ServiceResult};
use backbone_core::write_hooks::WriteCtx;
use serde_json::{json, Value};

use super::super::{content_bytes, services, sniff_mime, storage, threat_scan};
use super::*;

#[derive(Default)]
pub struct FileUploadActionsImpl;

impl FileUploadActionsImpl {
    /// Run a StoredFile verb through the generated service as a transition:
    /// the machine's check and its on_enter effects apply, and the row guard
    /// allows the state move.
    async fn file_transition(
        &self,
        id: &str,
        verb: &str,
        status: &str,
        fields: HashMap<String, Value>,
    ) -> ServiceResult<Value> {
        let service = &services()?.stored_file;
        let mut fields = fields;
        fields.insert("status".to_string(), Value::String(status.to_string()));
        let moved = backbone_core::write_guard::as_transition(
            verb,
            service.partial_update(id, fields),
        )
        .await?
        .ok_or(ServiceError::NotFound)?;
        serde_json::to_value(&moved).map_err(|e| ServiceError::Internal(e.to_string()))
    }

    /// The step's parameters, minus `id`, as the fields to write.
    fn fields_of(params: &Value) -> HashMap<String, Value> {
        params
            .as_object()
            .into_iter()
            .flatten()
            .filter(|(k, _)| k.as_str() != "id")
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

#[async_trait]
impl FileUploadActions for FileUploadActionsImpl {
    /// The file machine's `complete_upload` verb: uploading -> processing,
    /// carrying the processing status.
    async fn begin_processing(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = params["id"].as_str().ok_or_else(|| ServiceError::Validation("id is required".into()))?;
        self.file_transition(id, "complete_upload", "processing", Self::fields_of(params)).await
    }

    /// The file machine's `mark_safe` verb: processing -> active. The verb's
    /// guard demands a clean scan, so a pending scan refuses (the workflow's
    /// scan_pending branch).
    async fn activate_file(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = params["id"].as_str().ok_or_else(|| ServiceError::Validation("id is required".into()))?;
        self.file_transition(id, "mark_safe", "active", Self::fields_of(params)).await
    }

    /// The file machine's `quarantine` verb: processing -> quarantined, with
    /// the scan fields written first; the verb's guard demands a threat level
    /// that is not safe.
    async fn quarantine_file(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let id = params["id"].as_str().ok_or_else(|| ServiceError::Validation("id is required".into()))?;
        let service = &services()?.stored_file;
        let scan_fields: HashMap<String, Value> = ["is_scanned", "scan_result", "threat_level", "processing_status"]
            .into_iter()
            .filter_map(|key| params.get(key).map(|value| (key.to_string(), value.clone())))
            .collect();
        if !scan_fields.is_empty() {
            service.partial_update(id, scan_fields).await?.ok_or(ServiceError::NotFound)?;
        }
        let moved = self.file_transition(id, "quarantine", "quarantined", HashMap::new()).await?;
        Ok(moved)
    }

    /// sha256 of the step's base64 `content`.
    async fn calculate_sha256(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        use sha2::{Digest, Sha256};
        let bytes = content_bytes(params)?;
        let hash = Sha256::digest(&bytes);
        Ok(Value::String(
            hash.iter().map(|b| format!("{b:02x}")).collect(),
        ))
    }

    /// Sniff the content type from the bytes, falling back to the filename.
    async fn detect_mime(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let bytes = content_bytes(params)?;
        let filename = params["filename"].as_str().unwrap_or_default();
        Ok(Value::String(sniff_mime(&bytes, filename)))
    }

    /// Store the step's base64 `content` under `key`.
    async fn store_file(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let key = params["key"].as_str().ok_or_else(|| ServiceError::Validation("key is required".into()))?;
        let body = content_bytes(params)?;
        storage()?
            .put(key, body, "application/octet-stream")
            .await
            .map_err(|e| ServiceError::Internal(format!("store `{key}`: {e}")))?;
        Ok(json!({ "key": key, "stored": true }))
    }

    /// The in-process threat heuristic over the step's content.
    async fn scan_for_threats(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        let bytes = content_bytes(params)?;
        let filename = params["filename"].as_str().unwrap_or_default();
        Ok(threat_scan(&bytes, filename))
    }

    /// A download URL for the stored object: the backend's public URL when it
    /// has one, else a one-hour presigned GET.
    async fn generate_cdn_url(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        let key = params["storage_key"].as_str().ok_or_else(|| ServiceError::Validation("storage_key is required".into()))?;
        let backend = storage()?;
        if let Some(url) = backend.public_url(key) {
            return Ok(json!({ "url": url.as_str(), "expires_at": Value::Null }));
        }
        let url = backend
            .presigned_get(key, std::time::Duration::from_secs(3600))
            .await
            .map_err(|e| ServiceError::Internal(format!("presign `{key}`: {e}")))?;
        let expires = chrono::Utc::now() + chrono::Duration::hours(1);
        Ok(json!({ "url": url.as_str(), "expires_at": expires.to_rfc3339() }))
    }

    /// Compensation: drop a stored object.
    async fn delete_file(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let key = params["key"].as_str().ok_or_else(|| ServiceError::Validation("key is required".into()))?;
        match storage()?.delete(key).await {
            Ok(()) => Ok(json!({ "key": key, "deleted": true })),
            Err(e) => Err(ServiceError::Internal(format!("delete `{key}`: {e}"))),
        }
    }

    /// Compensation: take one reference off a content hash, through the
    /// content-hash service so the write joins the step's transaction.
    async fn decrement_reference(
        &self,
        _ctx: &WriteCtx,
        params: &Value,
        _wf: &Value,
    ) -> ServiceResult<Value> {
        let id = params["id"].as_str().ok_or_else(|| ServiceError::Validation("id is required".into()))?;
        let service = &services()?.content_hash;
        let Some(hash) = service.get_by_id(id).await? else {
            return Err(ServiceError::NotFound);
        };
        let next = hash.reference_count.saturating_sub(1);
        let mut fields = HashMap::new();
        fields.insert("reference_count".to_string(), Value::from(next));
        service
            .partial_update(id, fields)
            .await?
            .map(|saved| json!({ "id": id, "reference_count": saved.reference_count }))
            .ok_or(ServiceError::NotFound)
    }
}
