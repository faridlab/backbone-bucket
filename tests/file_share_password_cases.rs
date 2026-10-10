//! A file share's password is set through its own verb, stored as an argon2 hash, and never
//! written or read by a generic route. Runs on a fresh database (`sqlx::test`) with the module
//! migrations applied; DATABASE_URL names the server.

use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

use backbone_bucket::presentation::http::file_share_password::{file_share_password_routes, FileSharePassword};

/// One share row, seeded straight into the table: the share's stored file is not under test,
/// so this scratch database drops the foreign key to it.
async fn seed_share(pool: &PgPool) -> Uuid {
    sqlx::query("ALTER TABLE bucket.file_shares DROP CONSTRAINT IF EXISTS fk_file_shares_file_id")
        .execute(pool)
        .await
        .unwrap();
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO bucket.file_shares (id, file_id, owner_id, token, shared_with) VALUES ($1, $2, $3, $4, '{}')",
    )
    .bind(id)
    .bind(Uuid::new_v4())
    .bind(Uuid::new_v4())
    .bind(Uuid::new_v4().simple().to_string())
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn stored_hash(pool: &PgPool, id: Uuid) -> Option<String> {
    sqlx::query_scalar("SELECT password_hash FROM bucket.file_shares WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn call(pool: &PgPool, method: &str, path: String, body: serde_json::Value) -> u16 {
    let req = axum::http::Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(body.to_string()))
        .unwrap();
    file_share_password_routes(pool.clone()).oneshot(req).await.unwrap().status().as_u16()
}

#[sqlx::test(migrations = "./migrations")]
async fn a_share_password_is_stored_as_an_argon2_hash_and_verified_against_it(pool: PgPool) {
    let share = seed_share(&pool).await;
    assert_eq!(call(&pool, "POST", format!("/file_shares/{share}/password"), serde_json::json!({ "password": "correct horse" })).await, 204);
    let hash = stored_hash(&pool, share).await.expect("a hash is stored");
    assert!(hash.starts_with("$argon2"), "stored as argon2, never as sent: {hash}");
    assert!(!hash.contains("correct horse"));

    let svc = FileSharePassword::new(pool.clone());
    assert!(svc.verify(share, "correct horse").await.unwrap());
    assert!(!svc.verify(share, "wrong horse").await.unwrap());

    assert_eq!(call(&pool, "POST", format!("/file_shares/{share}/password"), serde_json::json!({ "password": "short" })).await, 422, "too short");
    assert_eq!(call(&pool, "POST", format!("/file_shares/{}/password", Uuid::new_v4()), serde_json::json!({ "password": "correct horse" })).await, 404);

    assert_eq!(call(&pool, "DELETE", format!("/file_shares/{share}/password"), serde_json::json!({})).await, 204);
    assert_eq!(stored_hash(&pool, share).await, None, "cleared");
    assert!(svc.verify(share, "anything").await.unwrap(), "a share without a password needs none");
}
