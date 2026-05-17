use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "version_type", rename_all = "snake_case")]
pub enum VersionType {
    Upload,
    Replace,
    Edit,
    Restore,
    AutoSave,
}

impl std::fmt::Display for VersionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Upload => write!(f, "upload"),
            Self::Replace => write!(f, "replace"),
            Self::Edit => write!(f, "edit"),
            Self::Restore => write!(f, "restore"),
            Self::AutoSave => write!(f, "auto_save"),
        }
    }
}

impl FromStr for VersionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "upload" => Ok(Self::Upload),
            "replace" => Ok(Self::Replace),
            "edit" => Ok(Self::Edit),
            "restore" => Ok(Self::Restore),
            "auto_save" => Ok(Self::AutoSave),
            _ => Err(format!("Unknown VersionType variant: {}", s)),
        }
    }
}

impl Default for VersionType {
    fn default() -> Self {
        Self::Upload
    }
}
