//! Extract a bearer token from an axum `Request` for use with
//! `backbone_auth::AuthMiddleware::authenticate(token: &str)`.
//!
//! The schema generator emits middleware that passes `&Request` directly to
//! `authenticate`, but the framework signature wants a token string. This
//! helper bridges the two.

use axum::http::Request;

/// Pull the bearer token out of the `Authorization` header, if present.
/// Returns the empty string when no header is present so the call site can
/// still hand a `&str` to `authenticate(...)` without a separate Option dance.
pub fn extract_bearer_token<B>(req: &Request<B>) -> &str {
    req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|raw| raw.strip_prefix("Bearer ").or_else(|| raw.strip_prefix("bearer ")))
        .unwrap_or("")
}
