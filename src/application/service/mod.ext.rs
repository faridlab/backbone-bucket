// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

mod locking_service;
mod multipart_upload_service;
mod conversion_service;
mod cdn_service;
mod video_thumbnail_service;
mod document_preview_service;
mod file_service;
pub use locking_service::LockingService;
pub use multipart_upload_service::MultipartUploadService;
pub use conversion_service::ConversionService;
#[allow(deprecated)]
pub use cdn_service::CdnService;
pub use video_thumbnail_service::VideoThumbnailService;
pub use document_preview_service::DocumentPreviewService;
pub use file_service::{FileMeta, FileService};
