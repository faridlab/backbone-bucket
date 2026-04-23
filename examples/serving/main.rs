//! Example: wiring the bucket module for mode-B serving.
//!
//! Demonstrates the full integration surface a consumer must implement:
//!
//! 1. An [`AuthExtractor`] — an Axum [`FromRequestParts`] impl that reads
//!    the consumer's session/JWT/cookie and yields a typed identity.
//! 2. An [`AuthzPolicy`] — decides whether a given identity may read a
//!    given [`StoredFile`].
//! 3. Building the module with [`BucketConfig`] + an [`ObjectStorage`] +
//!    the database pool.
//! 4. Mounting `.crud_router()` and `.serving_router()` on the same Axum app.
//!
//! This file intentionally does not start a server or hit a real database
//! — its purpose is to type-check under CI and serve as a copy-paste
//! starting point for the consumer workspace (`bersihir-service`).

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Router;
use url::Url;
use uuid::Uuid;

use backbone_bucket::auth::{AuthzDecision, AuthzPolicy, HasOwnerId};
use backbone_bucket::config::{ServingConfig, ServingMode};
use backbone_bucket::storage::{LocalStorage, ObjectStorage};
use backbone_bucket::{
    BucketConfig, BucketError, BucketModule, StorageConfig, StoredFile,
};

// ─── 1. Identity ──────────────────────────────────────────────────────────

/// The consumer's own identity type. Typically resolved from a JWT or session.
#[derive(Debug, Clone)]
struct ExampleUser {
    user_id: Uuid,
}

impl HasOwnerId for ExampleUser {
    fn owner_id(&self) -> Uuid {
        self.user_id
    }
}

// ─── 2. AuthExtractor — reads `x-user-id` header ──────────────────────────

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ExampleUser {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("x-user-id")
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "missing x-user-id").into_response())?
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, "bad x-user-id").into_response())?;

        let user_id = Uuid::parse_str(header)
            .map_err(|_| (StatusCode::BAD_REQUEST, "x-user-id not a uuid").into_response())?;

        Ok(Self { user_id })
    }
}

// ─── 3. AuthzPolicy — owner OR files explicitly marked public ─────────────

struct ExamplePolicy;

#[async_trait]
impl AuthzPolicy<ExampleUser> for ExamplePolicy {
    async fn decide(
        &self,
        identity: &ExampleUser,
        file: &StoredFile,
    ) -> Result<AuthzDecision, BucketError> {
        // Public-prefix files are readable by any authenticated user.
        if file.storage_key.starts_with("public/") {
            return Ok(AuthzDecision::Allow);
        }
        // Otherwise, owner-only.
        if identity.user_id == file.owner_id {
            Ok(AuthzDecision::Allow)
        } else {
            Ok(AuthzDecision::Deny)
        }
    }
}

// ─── 4. Composition ───────────────────────────────────────────────────────

#[allow(dead_code)]
fn build_app(pool: sqlx::PgPool) -> anyhow::Result<Router> {
    let storage: Arc<dyn ObjectStorage> = Arc::new(LocalStorage::new(
        "/tmp/bucket-example",
        Url::parse("http://localhost:8080/cdn/")?,
        b"dev-signing-secret".to_vec(),
    ));

    let config = BucketConfig {
        enabled: true,
        storage: StorageConfig::Local {
            root: "/tmp/bucket-example".into(),
            base_url: Url::parse("http://localhost:8080/cdn/")?,
            signing_secret_env: "BUCKET_LOCAL_SIGNING_SECRET".into(),
        },
        serving: ServingConfig {
            default_mode: ServingMode::Redirect,
            public_prefix: "public/".into(),
            presigned_ttl: Duration::from_secs(300),
        },
    };

    let module = BucketModule::builder()
        .with_database(pool)
        .with_config(config)
        .with_storage(storage)
        .build()?;

    let serving = module.serving_router::<ExampleUser>(Arc::new(ExamplePolicy))?;

    Ok(Router::new()
        .merge(module.crud_router())
        .nest("/cdn", serving))
}

fn main() {
    // Example is build-only: don't require a running Postgres in CI.
    eprintln!("backbone-bucket serving example — see `build_app` for wiring.");
}
