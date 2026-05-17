use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "share_status", rename_all = "snake_case")]
pub enum ShareStatus {
    Active,
    Expired,
    Exhausted,
    Revoked,
}

impl std::fmt::Display for ShareStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Expired => write!(f, "expired"),
            Self::Exhausted => write!(f, "exhausted"),
            Self::Revoked => write!(f, "revoked"),
        }
    }
}

impl FromStr for ShareStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "exhausted" => Ok(Self::Exhausted),
            "revoked" => Ok(Self::Revoked),
            _ => Err(format!("Unknown ShareStatus variant: {}", s)),
        }
    }
}

impl Default for ShareStatus {
    fn default() -> Self {
        Self::Active
    }
}
