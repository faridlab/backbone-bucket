use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "share_permission", rename_all = "snake_case")]
pub enum SharePermission {
    Private,
    View,
    Edit,
    Full,
}

impl std::fmt::Display for SharePermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Private => write!(f, "private"),
            Self::View => write!(f, "view"),
            Self::Edit => write!(f, "edit"),
            Self::Full => write!(f, "full"),
        }
    }
}

impl FromStr for SharePermission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "private" => Ok(Self::Private),
            "view" => Ok(Self::View),
            "edit" => Ok(Self::Edit),
            "full" => Ok(Self::Full),
            _ => Err(format!("Unknown SharePermission variant: {}", s)),
        }
    }
}

impl Default for SharePermission {
    fn default() -> Self {
        Self::View
    }
}
