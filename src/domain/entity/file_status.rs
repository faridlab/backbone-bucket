use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "file_status", rename_all = "snake_case")]
pub enum FileStatus {
    Uploading,
    Processing,
    Active,
    Quarantined,
    Deleted,
    Purged,
}

impl std::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uploading => write!(f, "uploading"),
            Self::Processing => write!(f, "processing"),
            Self::Active => write!(f, "active"),
            Self::Quarantined => write!(f, "quarantined"),
            Self::Deleted => write!(f, "deleted"),
            Self::Purged => write!(f, "purged"),
        }
    }
}

impl FromStr for FileStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "uploading" => Ok(Self::Uploading),
            "processing" => Ok(Self::Processing),
            "active" => Ok(Self::Active),
            "quarantined" => Ok(Self::Quarantined),
            "deleted" => Ok(Self::Deleted),
            "purged" => Ok(Self::Purged),
            _ => Err(format!("Unknown FileStatus variant: {}", s)),
        }
    }
}

impl Default for FileStatus {
    fn default() -> Self {
        Self::Active
    }
}
