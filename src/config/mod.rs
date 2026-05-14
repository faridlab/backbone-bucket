//! Configuration module for bucket
//!
//! This module provides configuration management functionality
//! for the bucket bounded context.

pub mod app_config;
pub mod bucket_config;

pub use app_config::*;
pub use bucket_config::{
    BucketConfig, ConfigEnvError, S3Config, ServingConfig, ServingMode, StorageConfig,
};
