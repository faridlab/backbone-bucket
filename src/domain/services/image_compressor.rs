//! Image Compressor Service
//!
//! Provides image compression capabilities for uploaded images.
//! Supports automatic format detection and quality-based compression.

use std::io::Cursor;
use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};

/// Result of image compression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    /// The (possibly compressed) content
    pub content: Vec<u8>,
    /// Original file size in bytes
    pub original_size: u64,
    /// Final size in bytes
    pub compressed_size: u64,
    /// Whether compression was applied
    pub was_compressed: bool,
    /// Algorithm used for compression (if any)
    pub algorithm: Option<String>,
    /// Detected image format
    pub format: Option<String>,
}

impl CompressionResult {
    /// Get the compression ratio (0.0 to 1.0)
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        1.0 - (self.compressed_size as f64 / self.original_size as f64)
    }

    /// Get the size reduction in bytes
    pub fn bytes_saved(&self) -> u64 {
        self.original_size.saturating_sub(self.compressed_size)
    }
}

/// Image Compressor Service
///
/// Compresses images to reduce storage space while maintaining quality.
/// Supports JPEG, PNG, WebP, and other common formats.
#[derive(Debug, Clone)]
pub struct ImageCompressorService {
    /// JPEG quality (1-100, default: 85)
    quality: u8,
    /// Minimum file size before compression is attempted (default: 100KB)
    min_size_for_compression: u64,
    /// Whether to convert all images to JPEG
    convert_to_jpeg: bool,
    /// Maximum dimension (width or height) for resizing
    max_dimension: Option<u32>,
}

impl Default for ImageCompressorService {
    fn default() -> Self {
        Self {
            quality: 85,
            min_size_for_compression: 100_000, // 100KB
            convert_to_jpeg: false,
            max_dimension: None,
        }
    }
}

impl ImageCompressorService {
    /// Create a new image compressor service with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom quality setting
    pub fn with_quality(quality: u8) -> Self {
        Self {
            quality: quality.min(100).max(1),
            ..Default::default()
        }
    }

    /// Set the minimum size for compression
    pub fn set_min_size(&mut self, min_size: u64) {
        self.min_size_for_compression = min_size;
    }

    /// Set whether to convert all images to JPEG
    pub fn set_convert_to_jpeg(&mut self, convert: bool) {
        self.convert_to_jpeg = convert;
    }

    /// Set maximum dimension for resizing
    pub fn set_max_dimension(&mut self, max_dim: u32) {
        self.max_dimension = Some(max_dim);
    }

    /// Check if content is a valid image
    pub fn is_image(&self, content: &[u8]) -> bool {
        ImageReader::new(Cursor::new(content))
            .with_guessed_format()
            .ok()
            .and_then(|r| r.format())
            .is_some()
    }

    /// Compress image content
    pub fn compress(&self, content: &[u8]) -> Result<CompressionResult, ImageCompressionError> {
        let original_size = content.len() as u64;

        // Skip if content is too small
        if original_size < self.min_size_for_compression {
            return Ok(CompressionResult {
                content: content.to_vec(),
                original_size,
                compressed_size: original_size,
                was_compressed: false,
                algorithm: None,
                format: None,
            });
        }

        // Try to detect and decode the image
        let reader = ImageReader::new(Cursor::new(content))
            .with_guessed_format()
            .map_err(|e| ImageCompressionError::FormatDetection(e.to_string()))?;

        let format = match reader.format() {
            Some(f) => f,
            None => {
                // Not a recognized image format, return as-is
                return Ok(CompressionResult {
                    content: content.to_vec(),
                    original_size,
                    compressed_size: original_size,
                    was_compressed: false,
                    algorithm: None,
                    format: None,
                });
            }
        };

        let format_name = format_to_string(format);

        // Decode the image
        let mut img = reader
            .decode()
            .map_err(|e| ImageCompressionError::Decode(e.to_string()))?;

        // Resize if needed
        if let Some(max_dim) = self.max_dimension {
            img = self.resize_if_needed(img, max_dim);
        }

        // Determine output format
        let output_format = if self.convert_to_jpeg {
            ImageFormat::Jpeg
        } else {
            // Keep original format for most, but convert some to JPEG
            match format {
                ImageFormat::Bmp | ImageFormat::Tiff => ImageFormat::Jpeg,
                other => other,
            }
        };

        // Compress to output format
        let compressed = self.encode_image(&img, output_format)?;
        let compressed_size = compressed.len() as u64;

        // Only use compressed version if it's actually smaller
        if compressed_size < original_size {
            let algorithm = format!("{}_q{}", format_to_string(output_format), self.quality);
            Ok(CompressionResult {
                content: compressed,
                original_size,
                compressed_size,
                was_compressed: true,
                algorithm: Some(algorithm),
                format: Some(format_name),
            })
        } else {
            // Compression didn't help, return original
            Ok(CompressionResult {
                content: content.to_vec(),
                original_size,
                compressed_size: original_size,
                was_compressed: false,
                algorithm: None,
                format: Some(format_name),
            })
        }
    }

    /// Resize image if it exceeds max dimension
    fn resize_if_needed(&self, img: DynamicImage, max_dim: u32) -> DynamicImage {
        let (width, height) = (img.width(), img.height());

        if width <= max_dim && height <= max_dim {
            return img;
        }

        // Calculate new dimensions maintaining aspect ratio
        let ratio = if width > height {
            max_dim as f64 / width as f64
        } else {
            max_dim as f64 / height as f64
        };

        let new_width = (width as f64 * ratio) as u32;
        let new_height = (height as f64 * ratio) as u32;

        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    }

    /// Encode image to bytes
    fn encode_image(
        &self,
        img: &DynamicImage,
        format: ImageFormat,
    ) -> Result<Vec<u8>, ImageCompressionError> {
        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        match format {
            ImageFormat::Jpeg => {
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, self.quality);
                img.write_with_encoder(encoder)
                    .map_err(|e| ImageCompressionError::Encode(e.to_string()))?;
            }
            ImageFormat::Png => {
                img.write_to(&mut cursor, ImageFormat::Png)
                    .map_err(|e| ImageCompressionError::Encode(e.to_string()))?;
            }
            ImageFormat::WebP => {
                img.write_to(&mut cursor, ImageFormat::WebP)
                    .map_err(|e| ImageCompressionError::Encode(e.to_string()))?;
            }
            other => {
                img.write_to(&mut cursor, other)
                    .map_err(|e| ImageCompressionError::Encode(e.to_string()))?;
            }
        }

        Ok(output)
    }

    /// Generate a thumbnail
    pub fn generate_thumbnail(
        &self,
        content: &[u8],
        max_size: u32,
    ) -> Result<Vec<u8>, ImageCompressionError> {
        let reader = ImageReader::new(Cursor::new(content))
            .with_guessed_format()
            .map_err(|e| ImageCompressionError::FormatDetection(e.to_string()))?;

        let img = reader
            .decode()
            .map_err(|e| ImageCompressionError::Decode(e.to_string()))?;

        // Create thumbnail
        let thumbnail = img.thumbnail(max_size, max_size);

        // Encode as JPEG
        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, 75);
        thumbnail
            .write_with_encoder(encoder)
            .map_err(|e| ImageCompressionError::Encode(e.to_string()))?;

        Ok(output)
    }
}

/// Convert ImageFormat to string
fn format_to_string(format: ImageFormat) -> String {
    match format {
        ImageFormat::Png => "png".to_string(),
        ImageFormat::Jpeg => "jpeg".to_string(),
        ImageFormat::Gif => "gif".to_string(),
        ImageFormat::WebP => "webp".to_string(),
        ImageFormat::Bmp => "bmp".to_string(),
        ImageFormat::Tiff => "tiff".to_string(),
        ImageFormat::Ico => "ico".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Errors that can occur during image compression
#[derive(Debug, Clone)]
pub enum ImageCompressionError {
    /// Failed to detect image format
    FormatDetection(String),
    /// Failed to decode image
    Decode(String),
    /// Failed to encode image
    Encode(String),
}

impl std::fmt::Display for ImageCompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FormatDetection(msg) => write!(f, "Format detection error: {}", msg),
            Self::Decode(msg) => write!(f, "Image decode error: {}", msg),
            Self::Encode(msg) => write!(f, "Image encode error: {}", msg),
        }
    }
}

impl std::error::Error for ImageCompressionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skips_small_files() {
        let compressor = ImageCompressorService::new();
        let small_content = vec![0u8; 1000]; // 1KB
        let result = compressor.compress(&small_content).unwrap();
        assert!(!result.was_compressed);
    }

    #[test]
    fn test_detects_non_image() {
        let compressor = ImageCompressorService::new();
        assert!(!compressor.is_image(b"Hello, World!"));
    }

    #[test]
    fn test_compression_result_ratio() {
        let result = CompressionResult {
            content: vec![],
            original_size: 1000,
            compressed_size: 750,
            was_compressed: true,
            algorithm: Some("jpeg_q85".to_string()),
            format: Some("jpeg".to_string()),
        };
        assert!((result.compression_ratio() - 0.25).abs() < 0.001);
        assert_eq!(result.bytes_saved(), 250);
    }
}
