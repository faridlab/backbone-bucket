// This file's extension (hand-written; ADR-0031). The generated workflow file
// pulls it in with #[path], so its lines resolve against that file.

//! The hand half of `ShareCreation` (ADR-0035). What this module cannot know
//! it does not pretend to: there is no User entity in bucket (users belong to
//! sapiens), so recipient resolution passes through what the event named, and
//! the permission check decides on in-module facts only (the file's owner).
//! The share row itself is written through the generated file-share service —
//! its guard, hooks and declared events all apply — with the token minted and
//! the password argon2-hashed exactly as the share-password verb stores it.

use std::collections::HashMap;

use async_trait::async_trait;
use backbone_core::service::{ServiceError, ServiceResult};
use backbone_core::write_hooks::WriteCtx;
use serde_json::{json, Value};

use super::super::{services, storage};
use super::*;

/// The minimum a share password may have, as the share-password verb enforces.
const MIN_PASSWORD_LEN: usize = 8;

#[derive(Default)]
pub struct ShareCreationActionsImpl;

impl ShareCreationActionsImpl {
    /// The argon2 PHC string of a share password (the share-password verb's
    /// own scheme, so its verify opens what this stores).
    fn hash_password(password: &str) -> ServiceResult<Option<String>> {
        if password.is_empty() {
            return Ok(None);
        }
        if password.chars().count() < MIN_PASSWORD_LEN {
            return Err(ServiceError::Validation(format!(
                "a share password has at least {MIN_PASSWORD_LEN} characters"
            )));
        }
        use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
        argon2::Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .map(|hash| Some(hash.to_string()))
            .map_err(|e| ServiceError::Internal(format!("hash the share password: {e}")))
    }
}

#[async_trait]
impl ShareCreationActions for ShareCreationActionsImpl {
    /// Whether `user_id` may manage `file_id`. The module holds no delegation
    /// data (permissions are sapiens'), so this decides on the one in-module
    /// fact: the file's owner. Anyone else is denied here and takes the
    /// workflow's permission-denied branch.
    async fn check_permission(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let user = params["user_id"].as_str().ok_or_else(|| ServiceError::Validation("user_id is required".into()))?;
        let file_id = params["file_id"].as_str().ok_or_else(|| ServiceError::Validation("file_id is required".into()))?;
        let file = services()?.stored_file.get_by_id(file_id).await?.ok_or(ServiceError::NotFound)?;
        Ok(json!({ "authorized": file.owner_id.to_string() == user }))
    }

    /// Whether the requester may make a public link. The module carries no
    /// per-user public-sharing preference; nothing in it ever disabled one,
    /// so the verb allows and the composing service's policy is the future
    /// home of a real toggle.
    async fn check_public_sharing(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        Ok(json!({ "allowed": true, "user_id": params["user_id"] }))
    }

    /// Resolve the recipient. A named user id passes through as the recipient
    /// (the module cannot confirm the account exists — that is sapiens' table);
    /// an e-mail resolves no in-module user, so the share is stored against
    /// the address and the invitation branch handles it.
    async fn find_share_recipient(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        if let Some(user) = params["user_id"].as_str().filter(|s| !s.is_empty()) {
            return Ok(json!({ "id": user }));
        }
        Ok(Value::Null)
    }

    /// The recipient's existing active share of this file, if one exists.
    async fn find_existing_share(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let file_id = params["file_id"].as_str().ok_or_else(|| ServiceError::Validation("file_id is required".into()))?;
        let user = params["user_id"].as_str().ok_or_else(|| ServiceError::Validation("user_id is required".into()))?;
        let mut filters = HashMap::new();
        filters.insert("file_id".to_string(), file_id.to_string());
        filters.insert("status".to_string(), "active".to_string());
        let (rows, _) = services()?.file_share.list(1, 50, filters).await?;
        let Ok(user) = uuid::Uuid::parse_str(user) else {
            return Err(ServiceError::Validation("user_id is not a uuid".into()));
        };
        rows.iter()
            .find(|share| share.shared_with.iter().any(|id| *id == user))
            .map(|share| serde_json::to_value(share).map_err(|e| ServiceError::Internal(e.to_string())))
            .unwrap_or(Err(ServiceError::NotFound))
    }

    /// Create the share: mint the token (a uuid pair, inside the schema's
    /// 32-64 rule), hash the password when one was given, and write through
    /// the generated file-share service so its guard and hooks apply.
    async fn create_share(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let file_id = params["file_id"].as_str().ok_or_else(|| ServiceError::Validation("file_id is required".into()))?;
        let shared_by = params["shared_by_id"].as_str().ok_or_else(|| ServiceError::Validation("shared_by_id is required".into()))?;
        let recipient = params["shared_with_user_id"].as_str().filter(|s| !s.is_empty());
        let email = params["shared_with_email"].as_str().filter(|s| !s.is_empty());
        if recipient.is_none() && email.is_none() && params["permission"].as_str() != Some("public") {
            return Err(ServiceError::Validation(
                "a share names a recipient user, an e-mail address, or is public".into(),
            ));
        }
        let token = format!("{}{}", uuid::Uuid::new_v4().simple(), uuid::Uuid::new_v4().simple());
        let password_hash = Self::hash_password(params["password"].as_str().unwrap_or_default())?;
        let mut body = json!({
            "file_id": file_id,
            "owner_id": shared_by,
            "token": token,
            "permission": params["permission"].as_str().unwrap_or("view"),
            "share_type": if recipient.is_some() { "user" } else { "link" },
            "max_downloads": params["max_downloads"],
            "expires_at": params["expires_at"],
            "message": params["message"],
        });
        if let Some(recipient) = recipient {
            body["shared_with"] = json!([recipient]);
        }
        if let Some(hash) = password_hash {
            body["password_hash"] = Value::String(hash);
        }
        let dto: crate::presentation::dto::CreateFileShareDto =
            serde_json::from_value(body)
                .map_err(|e| ServiceError::Validation(format!("the parameters do not make a share: {e}")))?;
        let saved = services()?.file_share.create(dto).await?;
        serde_json::to_value(&saved).map_err(|e| ServiceError::Internal(e.to_string()))
    }

    /// The share's URL: the backend's public URL when it serves one, else a
    /// one-hour presigned GET of the token's path.
    async fn generate_share_url(&self, _ctx: &WriteCtx, params: &Value, _wf: &Value) -> ServiceResult<Value> {
        let token = params["token"].as_str().ok_or_else(|| ServiceError::Validation("token is required".into()))?;
        let backend = storage()?;
        if let Some(url) = backend.public_url(&format!("shares/{token}")) {
            return Ok(json!({ "url": url.as_str() }));
        }
        let url = backend
            .presigned_get(&format!("shares/{token}"), std::time::Duration::from_secs(3600))
            .await
            .map_err(|e| ServiceError::Internal(format!("presign the share url: {e}")))?;
        Ok(json!({ "url": url.as_str() }))
    }
}
