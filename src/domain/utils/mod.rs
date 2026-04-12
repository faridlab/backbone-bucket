//! Utility functions for Bucket domain
//!
//! This module contains helper functions for common domain operations
//! that are used across entities but don't belong to a specific entity.

use chrono::{Datelike, Utc};
use rand::distributions::Alphanumeric;
use rand::Rng;

/// Generate a secure random token for file shares
///
/// Creates a URL-safe random string of specified length.
/// Uses cryptographically secure random generator.
///
/// # Arguments
/// * `length` - The length of the token to generate (default: 32)
///
/// # Returns
/// A URL-safe random string
///
/// # Example
/// ```rust,ignore
/// let token = backbone_bucket::domain::utils::generate_share_token(32);
/// assert_eq!(token.len(), 32);
/// ```
pub fn generate_share_token(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate a URL-friendly slug from a string
///
/// Converts the input string to lowercase, replaces spaces and
/// special characters with hyphens, and removes consecutive hyphens.
///
/// # Arguments
/// * `input` - The string to slugify
///
/// # Returns
/// A URL-friendly slug string
///
/// # Example
/// ```rust,ignore
/// let slug = backbone_bucket::domain::utils::slugify("My Bucket Name!");
/// assert_eq!(slug, "my-bucket-name");
/// ```
pub fn slugify(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| {
            match c {
                'a'..='z' | '0'..='9' => c,
                ' ' | '_' | '-' => '-',
                _ => '-',
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Generate a unique storage key for a file
///
/// Creates a hierarchical path using date prefixes and random identifier.
/// Format: `YYYY/MM/DD/<random_uuid>`
///
/// # Returns
/// A unique storage path string
pub fn generate_storage_key() -> String {
    let now = chrono::Utc::now();
    let uuid = uuid::Uuid::new_v4();
    format!(
        "{:04}/{:02}/{:02}/{}",
        now.year(),
        now.month(),
        now.day(),
        uuid.to_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_share_token() {
        let token = generate_share_token(32);
        assert_eq!(token.len(), 32);
        // Token should be URL-safe (no special chars)
        assert!(token.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_slugify_basic() {
        assert_eq!(slugify("My Bucket Name"), "my-bucket-name");
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_slugify_special_chars() {
        assert_eq!(slugify("My Bucket!!!"), "my-bucket");
        assert_eq!(slugify("Test@#$%Bucket"), "test-bucket");
    }

    #[test]
    fn test_slugify_multiple_spaces() {
        assert_eq!(slugify("My   Bucket   Name"), "my-bucket-name");
        assert_eq!(slugify("  leading  trailing  "), "leading-trailing");
    }

    #[test]
    fn test_generate_storage_key() {
        let key = generate_storage_key();
        let parts: Vec<&str> = key.split('/').collect();
        assert_eq!(parts.len(), 4); // YYYY/MM/DD/uuid
        assert!(parts[3].len() == 36); // UUID format
    }
}
