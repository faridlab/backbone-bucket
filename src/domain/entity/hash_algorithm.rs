use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "hash_algorithm", rename_all = "snake_case")]
pub enum HashAlgorithm {
    Sha256,
    Md5,
    Xxhash,
}

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sha256 => write!(f, "sha256"),
            Self::Md5 => write!(f, "md5"),
            Self::Xxhash => write!(f, "xxhash"),
        }
    }
}

impl FromStr for HashAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sha256" => Ok(Self::Sha256),
            "md5" => Ok(Self::Md5),
            "xxhash" => Ok(Self::Xxhash),
            _ => Err(format!("Unknown HashAlgorithm variant: {}", s)),
        }
    }
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::Sha256
    }
}
