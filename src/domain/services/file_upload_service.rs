//! File Upload Service
//!
//! Orchestrates the complete file upload workflow including:
//! - Bucket validation
//! - Quota enforcement
//! - Virus scanning
//! - Image compression
//! - Storage operations

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::domain::entity::{
    Bucket, StoredFile, UserQuota, FileStatus, ThreatLevel,
};
use super::storage_service::{StorageService, StorageError};
use super::virus_scanner::{VirusScannerService, VirusScanResult};
use super::image_compressor::{ImageCompressorService, CompressionResult, ImageCompressionError};

/// Request to upload a file
#[derive(Debug, Clone)]
pub struct UploadRequest {
    pub bucket_id: Uuid,
    pub owner_id: Uuid,
    pub path: String,
    pub filename: String,
    pub content: Vec<u8>,
    pub mime_type: String,
    pub metadata: Option<serde_json::Value>,
}

/// Result of a successful file upload
#[derive(Debug)]
pub struct UploadResult {
    pub file: StoredFile,
    pub was_compressed: bool,
    pub compression_ratio: Option<f64>,
    pub scan_result: VirusScanResult,
}

/// Errors that can occur during file upload
#[derive(Debug)]
pub enum UploadError {
    /// Bucket validation failed
    BucketValidation(String),
    /// Quota exceeded
    QuotaExceeded(String),
    /// Virus detected - upload blocked
    VirusDetected {
        filename: String,
        threats: Vec<String>,
    },
    /// Storage error
    Storage(StorageError),
    /// Image compression error
    Compression(ImageCompressionError),
    /// File quarantined (not blocked, but flagged)
    Quarantined {
        file: StoredFile,
        reason: String,
    },
}

impl std::fmt::Display for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BucketValidation(msg) => write!(f, "Bucket validation failed: {}", msg),
            Self::QuotaExceeded(msg) => write!(f, "Quota exceeded: {}", msg),
            Self::VirusDetected { filename, threats } => {
                write!(f, "Virus detected in '{}': {:?}", filename, threats)
            }
            Self::Storage(e) => write!(f, "Storage error: {}", e),
            Self::Compression(e) => write!(f, "Compression error: {}", e),
            Self::Quarantined { reason, .. } => write!(f, "File quarantined: {}", reason),
        }
    }
}

impl std::error::Error for UploadError {}

impl From<StorageError> for UploadError {
    fn from(err: StorageError) -> Self {
        UploadError::Storage(err)
    }
}

impl From<ImageCompressionError> for UploadError {
    fn from(err: ImageCompressionError) -> Self {
        UploadError::Compression(err)
    }
}

/// File Upload Service
///
/// Orchestrates the complete file upload workflow:
/// 1. Validate bucket restrictions
/// 2. Check user quota
/// 3. Scan for viruses
/// 4. Compress images (if applicable)
/// 5. Store file
/// 6. Update quota
#[derive(Clone)]
pub struct FileUploadService {
    storage: Arc<StorageService>,
    scanner: Arc<VirusScannerService>,
    compressor: Arc<ImageCompressorService>,
    /// Whether to auto-compress images
    auto_compress_images: bool,
    /// Thumbnail size
    thumbnail_size: u32,
}

impl FileUploadService {
    /// Create a new file upload service
    pub fn new(
        storage: Arc<StorageService>,
        scanner: Arc<VirusScannerService>,
        compressor: Arc<ImageCompressorService>,
    ) -> Self {
        Self {
            storage,
            scanner,
            compressor,
            auto_compress_images: true,
            thumbnail_size: 256,
        }
    }

    /// Create with custom settings
    pub fn with_settings(
        storage: Arc<StorageService>,
        scanner: Arc<VirusScannerService>,
        compressor: Arc<ImageCompressorService>,
        auto_compress: bool,
        thumbnail_size: u32,
    ) -> Self {
        Self {
            storage,
            scanner,
            compressor,
            auto_compress_images: auto_compress,
            thumbnail_size,
        }
    }

    /// Upload a file with full workflow
    pub async fn upload(
        &self,
        request: UploadRequest,
        bucket: &Bucket,
        quota: &mut UserQuota,
    ) -> Result<UploadResult, UploadError> {
        let content_size = request.content.len() as i64;

        // 1. Validate bucket restrictions
        bucket
            .can_upload(content_size, &request.mime_type)
            .map_err(UploadError::BucketValidation)?;

        // 2. Check quota
        quota
            .can_upload_file(content_size)
            .map_err(|e| UploadError::QuotaExceeded(e.to_string()))?;

        // 3. Scan for viruses
        let scan_result = self.scanner.scan(&request.content, &request.filename);

        if self.scanner.should_block(&scan_result) {
            return Err(UploadError::VirusDetected {
                filename: request.filename,
                threats: scan_result.threats,
            });
        }

        // 4. Compress if image and auto-compression is enabled
        let (final_content, compression) = if self.auto_compress_images
            && self.compressor.is_image(&request.content)
        {
            let result = self.compressor.compress(&request.content)?;
            let was_compressed = result.was_compressed;
            (result.content.clone(), Some(result))
        } else {
            (request.content.clone(), None)
        };

        // 5. Store file
        let storage_key = self
            .storage
            .store_file(&bucket.slug, &request.path, &final_content)
            .await?;

        // 6. Generate thumbnail for images
        let (has_thumbnail, thumbnail_path) = if self.compressor.is_image(&final_content) {
            match self.generate_thumbnail(&final_content).await {
                Ok(path) => (true, Some(path)),
                Err(_) => (false, None),
            }
        } else {
            (false, None)
        };

        // 7. Create entity
        let file_id = Uuid::new_v4();
        let final_size = final_content.len() as i64;
        let checksum = compute_checksum(&final_content);

        let status = if self.scanner.should_quarantine(&scan_result) {
            FileStatus::Quarantined
        } else {
            FileStatus::Active
        };

        let mut file = StoredFile {
            id: file_id,
            bucket_id: request.bucket_id,
            owner_id: request.owner_id,
            path: request.path,
            original_name: request.filename.clone(),
            size_bytes: final_size,
            mime_type: request.mime_type,
            checksum: Some(checksum),
            is_compressed: compression.as_ref().map(|c| c.was_compressed).unwrap_or(false),
            original_size: compression.as_ref().filter(|c| c.was_compressed).map(|c| c.original_size as i64),
            compression_algorithm: compression.as_ref().and_then(|c| c.algorithm.clone()),
            is_scanned: true,
            scan_result: Some(scan_result.scan_details.clone()),
            threat_level: Some(scan_result.threat_level),
            has_thumbnail,
            thumbnail_path,
            status,
            storage_key,
            version: 1,
            previous_version_id: None,
            download_count: 0,
            last_accessed_at: None,
            metadata: request.metadata.unwrap_or(serde_json::json!({})),
        };

        // 8. Update quota (use actual stored size)
        if let Err(e) = quota.add_usage(final_size) {
            // Rollback: delete stored file
            let _ = self.storage.delete_file(&file.storage_key).await;
            return Err(UploadError::QuotaExceeded(e.to_string()));
        }

        // 9. Handle quarantine case
        if file.status == FileStatus::Quarantined {
            return Err(UploadError::Quarantined {
                file,
                reason: "File flagged for review".to_string(),
            });
        }

        Ok(UploadResult {
            file,
            was_compressed: compression.as_ref().map(|c| c.was_compressed).unwrap_or(false),
            compression_ratio: compression.as_ref().filter(|c| c.was_compressed).map(|c| c.compression_ratio()),
            scan_result,
        })
    }

    /// Generate a thumbnail for the file
    async fn generate_thumbnail(&self, content: &[u8]) -> Result<String, UploadError> {
        let thumbnail_content = self
            .compressor
            .generate_thumbnail(content, self.thumbnail_size)?;

        let file_id = Uuid::new_v4();
        let path = self
            .storage
            .store_thumbnail(file_id, &thumbnail_content)
            .await?;

        Ok(path)
    }

    /// Upload a new version of an existing file
    pub async fn upload_version(
        &self,
        request: UploadRequest,
        bucket: &Bucket,
        quota: &mut UserQuota,
        previous_file: &StoredFile,
    ) -> Result<UploadResult, UploadError> {
        let mut result = self.upload(request, bucket, quota).await?;

        // Link to previous version
        result.file.previous_version_id = Some(previous_file.id);
        result.file.version = previous_file.version + 1;

        Ok(result)
    }
}

/// Compute a checksum for file content
fn compute_checksum(content: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::{BucketType, BucketStatus, StorageBackend};

    fn create_test_bucket() -> Bucket {
        Bucket {
            id: Uuid::new_v4(),
            name: "Test Bucket".to_string(),
            slug: "test-bucket".to_string(),
            description: None,
            owner_id: Uuid::new_v4(),
            bucket_type: BucketType::User,
            status: BucketStatus::Active,
            storage_backend: StorageBackend::Local,
            root_path: "/tmp/test".to_string(),
            file_count: 0,
            total_size_bytes: 0,
            max_file_size: Some(10 * 1024 * 1024), // 10MB
            allowed_mime_types: vec![],
            auto_delete_after_days: None,
            metadata: serde_json::json!({}),
        }
    }

    fn create_test_quota() -> UserQuota {
        UserQuota {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            limit_bytes: 1024 * 1024 * 1024, // 1GB
            used_bytes: 0,
            file_count: 0,
            max_file_size: None,
            max_file_count: None,
            tier: "free".to_string(),
            warning_threshold_percent: 80,
            last_warning_sent_at: None,
            peak_usage_bytes: 0,
            peak_usage_at: None,
            metadata: serde_json::json!({}),
        }
    }

    #[test]
    fn test_compute_checksum() {
        let checksum1 = compute_checksum(b"Hello, World!");
        let checksum2 = compute_checksum(b"Hello, World!");
        let checksum3 = compute_checksum(b"Different content");

        assert_eq!(checksum1, checksum2);
        assert_ne!(checksum1, checksum3);
    }

    #[test]
    fn test_bucket_validation() {
        let bucket = create_test_bucket();

        // Valid upload
        assert!(bucket.can_upload(1000, "text/plain").is_ok());

        // File too large
        assert!(bucket.can_upload(20 * 1024 * 1024, "text/plain").is_err());
    }

    #[test]
    fn test_quota_validation() {
        let mut quota = create_test_quota();

        // Valid upload
        assert!(quota.can_upload_file(1000).is_ok());

        // Exhaust quota
        quota.used_bytes = quota.limit_bytes;
        assert!(quota.can_upload_file(1000).is_err());
    }
}
