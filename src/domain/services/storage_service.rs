//! Storage Service
//!
//! Provides filesystem operations for file storage.
//! Handles file read/write, trash management, and storage key generation.

use std::path::{Path, PathBuf};
use std::io;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use chrono::Utc;

/// Storage Service
///
/// Manages filesystem operations for the bucket module.
/// Handles file storage, retrieval, deletion, and trash management.
#[derive(Debug, Clone)]
pub struct StorageService {
    /// Root path for all storage
    root_path: PathBuf,
    /// Directory for bucket storage
    buckets_dir: PathBuf,
    /// Directory for deleted files (trash)
    trash_dir: PathBuf,
    /// Directory for thumbnails
    thumbnails_dir: PathBuf,
    /// Directory for temporary files
    temp_dir: PathBuf,
}

impl StorageService {
    /// Create a new storage service
    pub fn new(root: impl AsRef<Path>) -> Self {
        let root_path = root.as_ref().to_path_buf();
        Self {
            buckets_dir: root_path.join("buckets"),
            trash_dir: root_path.join("trash"),
            thumbnails_dir: root_path.join("thumbnails"),
            temp_dir: root_path.join("temp"),
            root_path,
        }
    }

    /// Initialize storage directories
    pub async fn init(&self) -> Result<(), StorageError> {
        fs::create_dir_all(&self.buckets_dir).await?;
        fs::create_dir_all(&self.trash_dir).await?;
        fs::create_dir_all(&self.thumbnails_dir).await?;
        fs::create_dir_all(&self.temp_dir).await?;
        Ok(())
    }

    /// Generate a unique storage key for a file
    pub fn generate_storage_key(&self, bucket_slug: &str, path: &str) -> String {
        let timestamp = Utc::now().format("%Y/%m/%d");
        let unique_id = Uuid::new_v4().to_string()[..8].to_string();
        let sanitized_path = sanitize_path(path);
        format!("{}/{}/{}-{}", bucket_slug, timestamp, unique_id, sanitized_path)
    }

    /// Get the full filesystem path for a storage key
    pub fn get_file_path(&self, storage_key: &str) -> PathBuf {
        self.buckets_dir.join(storage_key)
    }

    /// Store file content
    pub async fn store_file(
        &self,
        bucket_slug: &str,
        path: &str,
        content: &[u8],
    ) -> Result<String, StorageError> {
        let storage_key = self.generate_storage_key(bucket_slug, path);
        let file_path = self.get_file_path(&storage_key);

        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write file content
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(content).await?;
        file.flush().await?;

        Ok(storage_key)
    }

    /// Read file content
    pub async fn read_file(&self, storage_key: &str) -> Result<Vec<u8>, StorageError> {
        let file_path = self.get_file_path(storage_key);

        if !file_path.exists() {
            return Err(StorageError::FileNotFound(storage_key.to_string()));
        }

        let mut file = fs::File::open(&file_path).await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;

        Ok(content)
    }

    /// Delete file permanently
    pub async fn delete_file(&self, storage_key: &str) -> Result<(), StorageError> {
        let file_path = self.get_file_path(storage_key);

        if file_path.exists() {
            fs::remove_file(&file_path).await?;
        }

        // Try to remove empty parent directories
        self.cleanup_empty_dirs(&file_path).await;

        Ok(())
    }

    /// Move file to trash
    pub async fn move_to_trash(
        &self,
        storage_key: &str,
        file_id: Uuid,
    ) -> Result<String, StorageError> {
        let source_path = self.get_file_path(storage_key);

        if !source_path.exists() {
            return Err(StorageError::FileNotFound(storage_key.to_string()));
        }

        // Generate trash path with timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let trash_key = format!("{}/{}_{}",
            Utc::now().format("%Y/%m/%d"),
            file_id,
            timestamp
        );
        let trash_path = self.trash_dir.join(&trash_key);

        // Create parent directories if needed
        if let Some(parent) = trash_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Move file to trash
        fs::rename(&source_path, &trash_path).await?;

        // Cleanup empty source directories
        self.cleanup_empty_dirs(&source_path).await;

        Ok(trash_key)
    }

    /// Restore file from trash
    pub async fn restore_from_trash(
        &self,
        trash_key: &str,
        original_storage_key: &str,
    ) -> Result<(), StorageError> {
        let trash_path = self.trash_dir.join(trash_key);

        if !trash_path.exists() {
            return Err(StorageError::FileNotFound(trash_key.to_string()));
        }

        let restore_path = self.get_file_path(original_storage_key);

        // Create parent directories if needed
        if let Some(parent) = restore_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Move file back from trash
        fs::rename(&trash_path, &restore_path).await?;

        // Cleanup empty trash directories
        self.cleanup_empty_dirs(&trash_path).await;

        Ok(())
    }

    /// Store a thumbnail
    pub async fn store_thumbnail(
        &self,
        file_id: Uuid,
        content: &[u8],
    ) -> Result<String, StorageError> {
        let thumbnail_key = format!("{}/{}.jpg",
            Utc::now().format("%Y/%m"),
            file_id
        );
        let thumbnail_path = self.thumbnails_dir.join(&thumbnail_key);

        // Create parent directories if needed
        if let Some(parent) = thumbnail_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write thumbnail
        let mut file = fs::File::create(&thumbnail_path).await?;
        file.write_all(content).await?;
        file.flush().await?;

        Ok(thumbnail_key)
    }

    /// Read a thumbnail
    pub async fn read_thumbnail(&self, thumbnail_key: &str) -> Result<Vec<u8>, StorageError> {
        let thumbnail_path = self.thumbnails_dir.join(thumbnail_key);

        if !thumbnail_path.exists() {
            return Err(StorageError::FileNotFound(thumbnail_key.to_string()));
        }

        let mut file = fs::File::open(&thumbnail_path).await?;
        let mut content = Vec::new();
        file.read_to_end(&mut content).await?;

        Ok(content)
    }

    /// Delete a thumbnail
    pub async fn delete_thumbnail(&self, thumbnail_key: &str) -> Result<(), StorageError> {
        let thumbnail_path = self.thumbnails_dir.join(thumbnail_key);

        if thumbnail_path.exists() {
            fs::remove_file(&thumbnail_path).await?;
        }

        Ok(())
    }

    /// Check if a file exists
    pub async fn file_exists(&self, storage_key: &str) -> bool {
        self.get_file_path(storage_key).exists()
    }

    /// Get file size
    pub async fn get_file_size(&self, storage_key: &str) -> Result<u64, StorageError> {
        let file_path = self.get_file_path(storage_key);
        let metadata = fs::metadata(&file_path).await?;
        Ok(metadata.len())
    }

    /// Clean up empty directories
    async fn cleanup_empty_dirs(&self, start_path: &Path) {
        let mut current = start_path.parent();

        while let Some(dir) = current {
            // Stop at root directories
            if dir == self.buckets_dir || dir == self.trash_dir || dir == self.root_path {
                break;
            }

            // Try to remove directory (will fail if not empty)
            if fs::remove_dir(dir).await.is_err() {
                break;
            }

            current = dir.parent();
        }
    }

    /// Purge old trash files
    pub async fn purge_trash_older_than(
        &self,
        days: i64,
    ) -> Result<PurgeResult, StorageError> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        let mut purged_count = 0u64;
        let mut purged_bytes = 0u64;

        self.purge_dir_recursive(&self.trash_dir, cutoff, &mut purged_count, &mut purged_bytes).await?;

        Ok(PurgeResult {
            files_purged: purged_count,
            bytes_freed: purged_bytes,
        })
    }

    /// Recursively purge old files in a directory
    async fn purge_dir_recursive(
        &self,
        dir: &Path,
        cutoff: chrono::DateTime<Utc>,
        count: &mut u64,
        bytes: &mut u64,
    ) -> Result<(), StorageError> {
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;

            if metadata.is_dir() {
                Box::pin(self.purge_dir_recursive(&path, cutoff, count, bytes)).await?;
                // Try to remove empty directory
                let _ = fs::remove_dir(&path).await;
            } else if metadata.is_file() {
                if let Ok(modified) = metadata.modified() {
                    let modified_utc: chrono::DateTime<Utc> = modified.into();
                    if modified_utc < cutoff {
                        *bytes += metadata.len();
                        fs::remove_file(&path).await?;
                        *count += 1;
                    }
                }
            }
        }

        Ok(())
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> Result<StorageStats, StorageError> {
        let buckets_size = self.calculate_dir_size(&self.buckets_dir).await?;
        let trash_size = self.calculate_dir_size(&self.trash_dir).await?;
        let thumbnails_size = self.calculate_dir_size(&self.thumbnails_dir).await?;

        Ok(StorageStats {
            buckets_bytes: buckets_size,
            trash_bytes: trash_size,
            thumbnails_bytes: thumbnails_size,
            total_bytes: buckets_size + trash_size + thumbnails_size,
        })
    }

    /// Calculate directory size recursively
    async fn calculate_dir_size(&self, dir: &Path) -> Result<u64, StorageError> {
        let mut total = 0u64;

        if !dir.exists() {
            return Ok(0);
        }

        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if metadata.is_dir() {
                total += Box::pin(self.calculate_dir_size(&entry.path())).await?;
            } else {
                total += metadata.len();
            }
        }

        Ok(total)
    }
}

/// Sanitize a file path to remove dangerous characters
fn sanitize_path(path: &str) -> String {
    path.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            c if c.is_ascii_control() => '_',
            c => c,
        })
        .collect()
}

/// Result of trash purge operation
#[derive(Debug, Clone)]
pub struct PurgeResult {
    pub files_purged: u64,
    pub bytes_freed: u64,
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub buckets_bytes: u64,
    pub trash_bytes: u64,
    pub thumbnails_bytes: u64,
    pub total_bytes: u64,
}

/// Storage operation errors
#[derive(Debug)]
pub enum StorageError {
    /// IO error
    Io(io::Error),
    /// File not found
    FileNotFound(String),
    /// Path traversal attempt
    PathTraversal(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::FileNotFound(key) => write!(f, "File not found: {}", key),
            Self::PathTraversal(path) => write!(f, "Path traversal attempt: {}", path),
        }
    }
}

impl std::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_path() {
        assert_eq!(sanitize_path("file.txt"), "file.txt");
        assert_eq!(sanitize_path("path/to/file.txt"), "path_to_file.txt");
        assert_eq!(sanitize_path("file:name.txt"), "file_name.txt");
        assert_eq!(sanitize_path("file<>name.txt"), "file__name.txt");
    }

    #[test]
    fn test_generate_storage_key() {
        let storage = StorageService::new("/tmp/test");
        let key = storage.generate_storage_key("my-bucket", "document.pdf");
        assert!(key.starts_with("my-bucket/"));
        assert!(key.contains("document.pdf"));
    }

    #[tokio::test]
    async fn test_file_operations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = StorageService::new(temp_dir.path());
        storage.init().await.unwrap();

        // Store file
        let content = b"Hello, World!";
        let key = storage.store_file("test-bucket", "test.txt", content).await.unwrap();

        // Read file
        let read_content = storage.read_file(&key).await.unwrap();
        assert_eq!(read_content, content);

        // File exists
        assert!(storage.file_exists(&key).await);

        // Delete file
        storage.delete_file(&key).await.unwrap();
        assert!(!storage.file_exists(&key).await);
    }
}
