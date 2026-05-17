use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "comment_status", rename_all = "snake_case")]
pub enum CommentStatus {
    Active,
    Resolved,
    Deleted,
    Archived,
}

impl std::fmt::Display for CommentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Resolved => write!(f, "resolved"),
            Self::Deleted => write!(f, "deleted"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

impl FromStr for CommentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "resolved" => Ok(Self::Resolved),
            "deleted" => Ok(Self::Deleted),
            "archived" => Ok(Self::Archived),
            _ => Err(format!("Unknown CommentStatus variant: {}", s)),
        }
    }
}

impl Default for CommentStatus {
    fn default() -> Self {
        Self::Active
    }
}
