use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "quota_status", rename_all = "snake_case")]
pub enum QuotaStatus {
    Normal,
    Exceeded,
}

impl std::fmt::Display for QuotaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Exceeded => write!(f, "exceeded"),
        }
    }
}

impl FromStr for QuotaStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Self::Normal),
            "exceeded" => Ok(Self::Exceeded),
            _ => Err(format!("Unknown QuotaStatus variant: {}", s)),
        }
    }
}

impl Default for QuotaStatus {
    fn default() -> Self {
        Self::Normal
    }
}
