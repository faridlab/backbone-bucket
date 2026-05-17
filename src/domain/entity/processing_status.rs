use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "processing_status", rename_all = "snake_case")]
pub enum ProcessingStatus {
    Pending,
    Processing,
    ThumbnailsReady,
    ScanComplete,
    Complete,
    Failed,
}

impl std::fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::ThumbnailsReady => write!(f, "thumbnails_ready"),
            Self::ScanComplete => write!(f, "scan_complete"),
            Self::Complete => write!(f, "complete"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

impl FromStr for ProcessingStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "thumbnails_ready" => Ok(Self::ThumbnailsReady),
            "scan_complete" => Ok(Self::ScanComplete),
            "complete" => Ok(Self::Complete),
            "failed" => Ok(Self::Failed),
            _ => Err(format!("Unknown ProcessingStatus variant: {}", s)),
        }
    }
}

impl Default for ProcessingStatus {
    fn default() -> Self {
        Self::Pending
    }
}
