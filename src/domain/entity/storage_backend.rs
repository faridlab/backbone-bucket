use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "storage_backend", rename_all = "snake_case")]
pub enum StorageBackend {
    Local,
    S3,
    Minio,
    Gcs,
}

impl std::fmt::Display for StorageBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::S3 => write!(f, "s3"),
            Self::Minio => write!(f, "minio"),
            Self::Gcs => write!(f, "gcs"),
        }
    }
}

impl FromStr for StorageBackend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "s3" => Ok(Self::S3),
            "minio" => Ok(Self::Minio),
            "gcs" => Ok(Self::Gcs),
            _ => Err(format!("Unknown StorageBackend variant: {}", s)),
        }
    }
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self::Local
    }
}
