use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "access_action", rename_all = "snake_case")]
pub enum AccessAction {
    Download,
    View,
    Preview,
}

impl std::fmt::Display for AccessAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Download => write!(f, "download"),
            Self::View => write!(f, "view"),
            Self::Preview => write!(f, "preview"),
        }
    }
}

impl FromStr for AccessAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "download" => Ok(Self::Download),
            "view" => Ok(Self::View),
            "preview" => Ok(Self::Preview),
            _ => Err(format!("Unknown AccessAction variant: {}", s)),
        }
    }
}

impl Default for AccessAction {
    fn default() -> Self {
        Self::Download
    }
}
