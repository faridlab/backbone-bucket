use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "thumbnail_size", rename_all = "snake_case")]
pub enum ThumbnailSize {
    Tiny,
    Small,
    Medium,
    Large,
    Xlarge,
}

impl std::fmt::Display for ThumbnailSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tiny => write!(f, "tiny"),
            Self::Small => write!(f, "small"),
            Self::Medium => write!(f, "medium"),
            Self::Large => write!(f, "large"),
            Self::Xlarge => write!(f, "xlarge"),
        }
    }
}

impl FromStr for ThumbnailSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tiny" => Ok(Self::Tiny),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "xlarge" => Ok(Self::Xlarge),
            _ => Err(format!("Unknown ThumbnailSize variant: {}", s)),
        }
    }
}

impl Default for ThumbnailSize {
    fn default() -> Self {
        Self::Small
    }
}
