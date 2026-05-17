use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "processing_job_type", rename_all = "snake_case")]
pub enum ProcessingJobType {
    ThumbnailGeneration,
    VideoThumbnail,
    DocumentPreview,
    Compression,
    VirusScan,
    DeduplicationCheck,
}

impl std::fmt::Display for ProcessingJobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThumbnailGeneration => write!(f, "thumbnail_generation"),
            Self::VideoThumbnail => write!(f, "video_thumbnail"),
            Self::DocumentPreview => write!(f, "document_preview"),
            Self::Compression => write!(f, "compression"),
            Self::VirusScan => write!(f, "virus_scan"),
            Self::DeduplicationCheck => write!(f, "deduplication_check"),
        }
    }
}

impl FromStr for ProcessingJobType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "thumbnail_generation" => Ok(Self::ThumbnailGeneration),
            "video_thumbnail" => Ok(Self::VideoThumbnail),
            "document_preview" => Ok(Self::DocumentPreview),
            "compression" => Ok(Self::Compression),
            "virus_scan" => Ok(Self::VirusScan),
            "deduplication_check" => Ok(Self::DeduplicationCheck),
            _ => Err(format!("Unknown ProcessingJobType variant: {}", s)),
        }
    }
}

impl Default for ProcessingJobType {
    fn default() -> Self {
        Self::ThumbnailGeneration
    }
}
