// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

#[cfg(feature = "auth")]
pub mod auth;
pub mod file_share_password;
pub mod serving;
pub mod upload;
pub use serving::{lookup_by_key, serving_router, ServingContext};
pub use upload::{
    upload_router, UploadConfig, UploadContext, DEFAULT_CHUNK_BODY_LIMIT, DEFAULT_UPLOAD_BODY_LIMIT,
};
