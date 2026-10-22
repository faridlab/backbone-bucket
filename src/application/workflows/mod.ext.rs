// This directory's extension (hand-written; ADR-0031). The generated mod.rs pulls it
// in with include!, so its lines resolve against this directory.
//
// What the workflow Actions impls share: the object-storage backend and the
// generated services the module's build binds once (the impls themselves are
// built with `Default` and hold nothing). Held weakly, exactly as the
// stored-file triggers' quota bookkeeping is: the module owns the strong
// handles.

// `Arc` is already in scope: the generated mod.rs imports it.
use std::sync::{OnceLock, Weak};

use backbone_core::service::{ServiceError, ServiceResult};

use crate::storage::ObjectStorage;

static STORAGE: OnceLock<Option<Weak<dyn ObjectStorage>>> = OnceLock::new();
static SERVICES: OnceLock<Weak<WorkflowServices>> = OnceLock::new();

/// Bind the object storage and the workflow services the Actions impls call.
/// Called by the module's build step (`lib.ext.rs`); nothing bound means the
/// module was built without a storage backend, and every verb that needs one
/// refuses with a validation error naming it.
pub fn bind_workflow_runtime(storage: Option<Arc<dyn ObjectStorage>>, services: Arc<WorkflowServices>) {
    let _ = STORAGE.set(storage.map(|bound| Arc::downgrade(&bound)));
    let _ = SERVICES.set(Arc::downgrade(&services));
}

/// The bound object-storage backend, or a refusal naming what is missing.
pub fn storage() -> ServiceResult<Arc<dyn ObjectStorage>> {
    STORAGE
        .get()
        .and_then(|weak| weak.as_ref().and_then(Weak::upgrade))
        .ok_or_else(|| {
            ServiceError::Validation(
                "the bucket module was built without a storage backend: call .with_storage() before serving uploads".into(),
            )
        })
}

/// The bound workflow services, or the same refusal.
pub fn services() -> ServiceResult<Arc<WorkflowServices>> {
    SERVICES
        .get()
        .and_then(Weak::upgrade)
        .ok_or_else(|| ServiceError::Internal("the workflow services were never bound".into()))
}

/// Decode a step's `content` parameter (base64, as the trigger event carries
/// it) to bytes.
pub fn content_bytes(params: &serde_json::Value) -> ServiceResult<bytes::Bytes> {
    let text = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ServiceError::Validation("the step carries no content".into()))?;
    base64_decode(text).ok_or_else(|| ServiceError::Validation("the content is not valid base64".into()))
}

/// Encode bytes as base64 (the form JSON values carry binary in).
pub fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

/// Decode base64 (standard alphabet, padded or not) to bytes.
pub fn base64_decode(text: &str) -> Option<bytes::Bytes> {
    fn value(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some(u32::from(c - b'A')),
            b'a'..=b'z' => Some(u32::from(c - b'a') + 26),
            b'0'..=b'9' => Some(u32::from(c - b'0') + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let cleaned: Vec<u8> = text.bytes().filter(|b| !b" \n\r\t".contains(b)).collect();
    let trimmed = cleaned.split(|b| *b == b'=').next().unwrap_or(&cleaned);
    let mut out = Vec::with_capacity(trimmed.len() * 3 / 4);
    for chunk in trimmed.chunks(4) {
        let mut n: u32 = 0;
        for (i, c) in chunk.iter().enumerate() {
            n |= value(*c)? << (18 - 6 * i);
        }
        out.push((n >> 16) as u8);
        if chunk.len() > 2 {
            out.push((n >> 8) as u8);
        }
        if chunk.len() > 3 {
            out.push(n as u8);
        }
    }
    Some(bytes::Bytes::from(out))
}

/// The module's in-process threat scan: the EICAR test signature and a
/// blocklist of extensions no upload should carry. This is the heuristic the
/// module can run without an external antivirus; a real scanner slots in
/// behind the same verb.
pub fn threat_scan(content: &[u8], filename: &str) -> serde_json::Value {
    const EICAR: &[u8] = b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";
    const BLOCKED: &[&str] = &["exe", "bat", "cmd", "com", "scr", "vbs", "js", "jar", "sh"];
    let extension = filename.rsplit('.').next().map(str::to_ascii_lowercase);
    let blocked = extension
        .as_deref()
        .is_some_and(|ext| BLOCKED.contains(&ext) && filename.contains('.'));
    let eicar = content.windows(EICAR.len()).any(|w| w == EICAR);
    if eicar || blocked {
        serde_json::json!({
            "is_safe": false,
            "threat_level": "high",
            "threats_found": [if eicar { "eicar_test_signature" } else { "blocked_file_type" }],
            "details": if eicar { "the EICAR test signature matched" } else { "the file type is on the blocklist" },
        })
    } else {
        serde_json::json!({
            "is_safe": true,
            "threat_level": "safe",
            "threats_found": [],
            "details": "no signature matched and the type is not blocked",
        })
    }
}

/// Sniff a mime type from the bytes (the image crate's format probe), falling
/// back to the filename's extension.
pub fn sniff_mime(content: &[u8], filename: &str) -> String {
    if let Ok(format) = image::guess_format(content) {
        let kind = match format {
            image::ImageFormat::Png => "image/png",
            image::ImageFormat::Jpeg => "image/jpeg",
            image::ImageFormat::Gif => "image/gif",
            image::ImageFormat::WebP => "image/webp",
            image::ImageFormat::Bmp => "image/bmp",
            image::ImageFormat::Tiff => "image/tiff",
            image::ImageFormat::Avif => "image/avif",
            _ => "",
        };
        if !kind.is_empty() {
            return kind.to_string();
        }
    }
    match filename.rsplit('.').next().map(str::to_ascii_lowercase).as_deref() {
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain",
        Some("csv") => "text/csv",
        Some("json") => "application/json",
        Some("html") | Some("htm") => "text/html",
        Some("mp4") => "video/mp4",
        Some("mov") => "video/quicktime",
        Some("webm") => "video/webm",
        Some("mp3") => "audio/mpeg",
        Some("wav") => "audio/wav",
        Some("zip") => "application/zip",
        Some("gz") => "application/gzip",
        _ => "application/octet-stream",
    }
    .to_string()
}
