use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "lock_status", rename_all = "snake_case")]
pub enum LockStatus {
    Active,
    Expired,
    Released,
}

impl std::fmt::Display for LockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Expired => write!(f, "expired"),
            Self::Released => write!(f, "released"),
        }
    }
}

impl FromStr for LockStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "released" => Ok(Self::Released),
            _ => Err(format!("Unknown LockStatus variant: {}", s)),
        }
    }
}

impl Default for LockStatus {
    fn default() -> Self {
        Self::Active
    }
}
