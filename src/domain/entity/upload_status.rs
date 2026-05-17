use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "upload_status", rename_all = "snake_case")]
pub enum UploadStatus {
    Initiated,
    Uploading,
    Completing,
    Completed,
    Expired,
    Failed,
}

impl std::fmt::Display for UploadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initiated => write!(f, "initiated"),
            Self::Uploading => write!(f, "uploading"),
            Self::Completing => write!(f, "completing"),
            Self::Completed => write!(f, "completed"),
            Self::Expired => write!(f, "expired"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

impl FromStr for UploadStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "initiated" => Ok(Self::Initiated),
            "uploading" => Ok(Self::Uploading),
            "completing" => Ok(Self::Completing),
            "completed" => Ok(Self::Completed),
            "expired" => Ok(Self::Expired),
            "failed" => Ok(Self::Failed),
            _ => Err(format!("Unknown UploadStatus variant: {}", s)),
        }
    }
}

impl Default for UploadStatus {
    fn default() -> Self {
        Self::Initiated
    }
}
