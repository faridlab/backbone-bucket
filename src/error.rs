//! Top-level error type for the bucket module.
//!
//! `BucketError` is the module's unified error surface — storage failures,
//! authorization denials, config/signing issues, and missing objects all
//! collapse into one enum so consumers get precise `match` coverage.
//!
//! Converts into `backbone_core::ServiceError` for compatibility with the
//! framework error plumbing used by the generated CRUD stack.

use std::io;

use thiserror::Error;

/// Unified error surface for storage, serving, and signing.
#[derive(Debug, Error)]
pub enum BucketError {
    /// Object not found in storage or database.
    #[error("not found")]
    NotFound,

    /// Caller is not permitted to perform the requested action.
    #[error("forbidden")]
    Forbidden,

    /// Request could not be authenticated.
    #[error("unauthenticated")]
    Unauthenticated,

    /// Presigned URL has expired or is malformed.
    #[error("invalid or expired signature")]
    InvalidSignature,

    /// Configured backend cannot satisfy this operation (e.g. `public_url`
    /// on a backend without a public bucket configured).
    #[error("operation not supported by backend: {0}")]
    Unsupported(String),

    /// Missing or malformed configuration.
    #[error("configuration error: {0}")]
    Config(String),

    /// Storage backend I/O error.
    #[error("storage I/O error: {0}")]
    Io(#[from] io::Error),

    /// S3/MinIO-side error.
    #[error("s3 error: {0}")]
    S3(String),

    /// URL construction / parsing error.
    #[error("url error: {0}")]
    Url(String),

    /// Catch-all for errors outside the above categories.
    #[error("{0}")]
    Other(String),
}

pub type BucketResult<T> = Result<T, BucketError>;

impl From<BucketError> for backbone_core::ServiceError {
    fn from(err: BucketError) -> Self {
        match err {
            BucketError::NotFound => backbone_core::ServiceError::NotFound,
            BucketError::Forbidden => {
                backbone_core::ServiceError::Validation("forbidden".to_string())
            }
            BucketError::Unauthenticated => {
                backbone_core::ServiceError::Validation("unauthenticated".to_string())
            }
            other => backbone_core::ServiceError::Internal(other.to_string()),
        }
    }
}

impl From<url::ParseError> for BucketError {
    fn from(err: url::ParseError) -> Self {
        BucketError::Url(err.to_string())
    }
}
