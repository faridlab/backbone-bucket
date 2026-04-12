use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "bucket_type", rename_all = "snake_case")]
pub enum BucketType {
    User,
    Shared,
    System,
    Temp,
}

impl std::fmt::Display for BucketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Shared => write!(f, "shared"),
            Self::System => write!(f, "system"),
            Self::Temp => write!(f, "temp"),
        }
    }
}

impl FromStr for BucketType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Self::User),
            "shared" => Ok(Self::Shared),
            "system" => Ok(Self::System),
            "temp" => Ok(Self::Temp),
            _ => Err(format!("Unknown BucketType variant: {}", s)),
        }
    }
}

impl Default for BucketType {
    fn default() -> Self {
        Self::User
    }
}
