//! A file share's password: set, cleared and checked through this verb only.
//!
//! Hand-written (user-owned; see `metaphor.codegen.yaml`). `password_hash` is `@sensitive` and
//! `@immutable`, so no generic route serves it or lets a client write it: a client used to set it
//! to whatever it sent, hashed or not. Here the plaintext comes in, an argon2 hash goes to the
//! row, and [`FileSharePassword::verify`] is what a share route checks a visitor's password with.
//!
//!   POST   /file_shares/:id/password   body { password }   → 204, 404, 422 (too short)
//!   DELETE /file_shares/:id/password                       → 204, 404
//!
//! Statements ride the request-dedicated connection when the composing service bound one.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use backbone_orm::{company_scope::fetch_optional_scalar_scoped, org_scope};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

/// The shortest share password accepted.
pub const MIN_PASSWORD_LEN: usize = 8;

/// Why a share password could not be set or checked.
#[derive(Debug)]
pub enum FileSharePasswordError {
    /// No live share has this id.
    NotFound(Uuid),
    /// The password is shorter than [`MIN_PASSWORD_LEN`].
    TooShort,
    /// The database or the hasher failed.
    Internal(String),
}

impl FileSharePasswordError {
    fn response(&self) -> Response {
        let (status, code, message) = match self {
            Self::NotFound(id) => (StatusCode::NOT_FOUND, "file_share_not_found", format!("no file share {id}")),
            Self::TooShort => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "file_share_password_too_short",
                format!("a share password has at least {MIN_PASSWORD_LEN} characters"),
            ),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "the password could not be stored".to_string()),
        };
        (status, Json(json!({ "success": false, "error": code, "message": message }))).into_response()
    }
}

fn internal(e: impl std::fmt::Display) -> FileSharePasswordError {
    FileSharePasswordError::Internal(e.to_string())
}

/// Sets, clears and checks share passwords.
#[derive(Clone)]
pub struct FileSharePassword {
    pool: PgPool,
}

impl FileSharePassword {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Store the argon2 hash of `password` on the share.
    pub async fn set(&self, share: Uuid, password: &str) -> Result<(), FileSharePasswordError> {
        if password.chars().count() < MIN_PASSWORD_LEN {
            return Err(FileSharePasswordError::TooShort);
        }
        let hash = hash(password)?;
        self.write(share, Some(hash)).await
    }

    /// Remove the share's password.
    pub async fn clear(&self, share: Uuid) -> Result<(), FileSharePasswordError> {
        self.write(share, None).await
    }

    /// Whether `password` opens the share: true when it matches the stored hash, or when the
    /// share has no password.
    pub async fn verify(&self, share: Uuid, password: &str) -> Result<bool, FileSharePasswordError> {
        let stored: Option<Option<String>> = fetch_optional_scalar_scoped(
            &self.pool,
            sqlx::query_scalar(
                "SELECT password_hash FROM bucket.file_shares WHERE id = $1 AND (metadata->>'deleted_at') IS NULL",
            )
            .bind(share),
        )
        .await
        .map_err(internal)?;
        match stored {
            None => Err(FileSharePasswordError::NotFound(share)),
            Some(None) => Ok(true),
            Some(Some(hash)) => {
                use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
                let parsed = PasswordHash::new(&hash).map_err(internal)?;
                Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
            }
        }
    }

    async fn write(&self, share: Uuid, hash: Option<String>) -> Result<(), FileSharePasswordError> {
        let res = org_scope::execute_scoped(
            &self.pool,
            sqlx::query(
                "UPDATE bucket.file_shares SET password_hash = $2 WHERE id = $1 AND (metadata->>'deleted_at') IS NULL",
            )
            .bind(share)
            .bind(hash),
        )
        .await
        .map_err(internal)?;
        if res.rows_affected() == 0 {
            return Err(FileSharePasswordError::NotFound(share));
        }
        Ok(())
    }
}

fn hash(password: &str) -> Result<String, FileSharePasswordError> {
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    use argon2::{Argon2, PasswordHasher};
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default().hash_password(password.as_bytes(), &salt).map_err(internal)?.to_string())
}

#[derive(Deserialize)]
struct SetPasswordBody {
    password: String,
}

async fn set_password(
    State(svc): State<FileSharePassword>,
    Path(id): Path<Uuid>,
    Json(body): Json<SetPasswordBody>,
) -> Response {
    match svc.set(id, &body.password).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.response(),
    }
}

async fn clear_password(State(svc): State<FileSharePassword>, Path(id): Path<Uuid>) -> Response {
    match svc.clear(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.response(),
    }
}

/// The share-password verb routes; the host mounts them beside the bucket collections, behind
/// the same guard.
pub fn file_share_password_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/file_shares/:id/password", post(set_password).delete(clear_password))
        .with_state(FileSharePassword::new(pool))
}
