use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "share_type", rename_all = "snake_case")]
pub enum ShareType {
    User,
    Link,
    Password,
}

impl std::fmt::Display for ShareType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Link => write!(f, "link"),
            Self::Password => write!(f, "password"),
        }
    }
}

impl FromStr for ShareType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Self::User),
            "link" => Ok(Self::Link),
            "password" => Ok(Self::Password),
            _ => Err(format!("Unknown ShareType variant: {}", s)),
        }
    }
}

impl Default for ShareType {
    fn default() -> Self {
        Self::Link
    }
}
