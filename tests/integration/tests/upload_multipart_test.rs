//! Multipart HTTP Upload integration tests
//!
//! Hand-written — NOT generated. Covers the custom routes mounted by
//! `BucketModule::upload_router<A>(config)` in
//! `src/presentation/http/upload.rs`:
//!
//! - `POST   /uploads`                            single-shot
//! - `POST   /uploads/sessions`                   initiate
//! - `POST   /uploads/sessions/:id/parts/:n`      chunk
//! - `POST   /uploads/sessions/:id/complete`      finalize
//! - `DELETE /uploads/sessions/:id`               abort
//!
//! Like the rest of this suite, these tests run *against a live API
//! server* — set `API_BASE_URL` (and `BUCKET_TEST_BUCKET_ID`, see below)
//! before invoking `cargo test`. When the server is unreachable, every
//! test returns `skipped` rather than failing, so the suite is safe to
//! include in CI environments without a deployment.

use std::time::Duration;

use chrono::Utc;
use reqwest::{multipart, Client};
use serde_json::Value;
use uuid::Uuid;

use crate::integration::framework::{TestResult, TestError};

/// Shared fixture state for the upload suite.
pub struct UploadMultipartTest {
    api_base_url: String,
    auth_token: Option<String>,
    /// Bucket UUID to upload into. Sourced from `BUCKET_TEST_BUCKET_ID`.
    /// When absent we still attempt the requests (the server validates
    /// the id) so the suite reports the actual failure mode rather than
    /// short-circuiting locally.
    bucket_id: Uuid,
    client: Client,
}

impl Default for UploadMultipartTest {
    fn default() -> Self {
        Self::new()
    }
}

impl UploadMultipartTest {
    pub fn new() -> Self {
        let api_base_url = std::env::var("API_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
        let auth_token = std::env::var("API_AUTH_TOKEN").ok();
        let bucket_id = std::env::var("BUCKET_TEST_BUCKET_ID")
            .ok()
            .and_then(|s| Uuid::parse_str(&s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("build reqwest client");

        Self { api_base_url, auth_token, bucket_id, client }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base_url, path)
    }

    fn auth(&self, rb: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth_token {
            Some(t) => rb.bearer_auth(t),
            None => rb,
        }
    }

    /// Quick liveness probe; mirrors the convention used by `GenericCrudTest`.
    async fn server_reachable(&self) -> bool {
        match self.auth(self.client.get(self.url("/uploads"))).send().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn skipped(test_name: &str, why: &str) -> TestResult {
        // Convention from `crud_test_base.rs`: skipped results are
        // emitted as successes with a SKIPPED: prefix so the suite
        // doesn't fail in CI environments lacking a live server.
        TestResult::success(test_name, format!("SKIPPED: {why}"))
    }

    fn failure(test_name: &str, details: impl Into<String>) -> TestResult {
        TestResult::failure(test_name, details)
    }

    fn success(test_name: &str, details: impl Into<String>) -> TestResult {
        TestResult::success(test_name, details)
    }

    // ─── Test cases ────────────────────────────────────────────────────

    /// Single-shot happy path: post one form with `file` + metadata,
    /// expect 201 and a UUID `id` in the response.
    async fn test_single_shot_happy_path(&self) -> TestResult {
        let name = "Uploads - SingleShot Happy Path";
        let body = b"hello multipart world".to_vec();
        let form = multipart::Form::new()
            .text("bucket_id", self.bucket_id.to_string())
            .text("path", format!("test/{}/hello.txt", Utc::now().timestamp()))
            .part(
                "file",
                multipart::Part::bytes(body.clone())
                    .file_name("hello.txt")
                    .mime_str("text/plain")
                    .unwrap(),
            );

        let resp = match self
            .auth(self.client.post(self.url("/uploads")).multipart(form))
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Self::failure(name, format!("request error: {e}")),
        };
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if status.as_u16() == 401 {
            return Self::skipped(
                name,
                "401 Unauthorized — set API_AUTH_TOKEN to a valid bearer token to exercise this path",
            );
        }
        if !status.is_success() {
            return Self::failure(name, format!("status={} body={}", status, text));
        }
        match serde_json::from_str::<Value>(&text) {
            Ok(v) if v.get("id").is_some() && v.get("storage_key").is_some() => {
                Self::success(name, format!("id={}", v["id"]))
            }
            Ok(v) => Self::failure(name, format!("unexpected body: {v}")),
            Err(e) => Self::failure(name, format!("non-JSON body: {e} — raw={text}")),
        }
    }

    /// Posting without a `file` part should be rejected (4xx).
    async fn test_single_shot_missing_file_field(&self) -> TestResult {
        let name = "Uploads - SingleShot Missing File Field";
        let form = multipart::Form::new()
            .text("bucket_id", self.bucket_id.to_string())
            .text("path", "test/missing.txt");

        let resp = match self
            .auth(self.client.post(self.url("/uploads")).multipart(form))
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Self::failure(name, format!("request error: {e}")),
        };
        let status = resp.status().as_u16();
        if status == 401 {
            return Self::skipped(name, "401 — set API_AUTH_TOKEN");
        }
        if (400..500).contains(&status) {
            Self::success(name, format!("rejected with {status}"))
        } else {
            Self::failure(name, format!("expected 4xx, got {status}"))
        }
    }

    /// Resumable: initiate → upload one part → complete. Exercises the
    /// full session lifecycle in the smallest possible configuration
    /// (chunk_size = file_size = total_chunks = 1).
    async fn test_resumable_round_trip(&self) -> TestResult {
        let name = "Uploads - Resumable Round Trip";

        let payload = b"resumable-bytes".to_vec();
        let file_size = payload.len() as i64;

        // 1) Initiate.
        let init_body = serde_json::json!({
            "bucket_id": self.bucket_id,
            "path": format!("test/{}/resumable.bin", Utc::now().timestamp()),
            "filename": "resumable.bin",
            "mime_type": "application/octet-stream",
            "file_size": file_size,
            "chunk_size": file_size,
        });
        let init_resp = match self
            .auth(self.client.post(self.url("/uploads/sessions")).json(&init_body))
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Self::failure(name, format!("initiate request: {e}")),
        };
        if init_resp.status().as_u16() == 401 {
            return Self::skipped(name, "401 — set API_AUTH_TOKEN");
        }
        if !init_resp.status().is_success() {
            let status = init_resp.status();
            let body = init_resp.text().await.unwrap_or_default();
            return Self::failure(name, format!("initiate failed: status={status} body={body}"));
        }
        let init_value: Value = match init_resp.json().await {
            Ok(v) => v,
            Err(e) => return Self::failure(name, format!("initiate body parse: {e}")),
        };
        let session_id = match init_value.get("session_id").and_then(Value::as_str) {
            Some(s) => s.to_string(),
            None => {
                return Self::failure(
                    name,
                    format!("initiate response missing session_id: {init_value}"),
                )
            }
        };

        // 2) Upload part 1.
        let chunk_form = multipart::Form::new().part(
            "chunk",
            multipart::Part::bytes(payload)
                .file_name("part-1.bin")
                .mime_str("application/octet-stream")
                .unwrap(),
        );
        let part_resp = match self
            .auth(
                self.client
                    .post(self.url(&format!("/uploads/sessions/{session_id}/parts/1")))
                    .multipart(chunk_form),
            )
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Self::failure(name, format!("part request: {e}")),
        };
        if !part_resp.status().is_success() {
            let status = part_resp.status();
            let body = part_resp.text().await.unwrap_or_default();
            return Self::failure(name, format!("part upload failed: status={status} body={body}"));
        }

        // 3) Complete.
        let complete_resp = match self
            .auth(
                self.client
                    .post(self.url(&format!("/uploads/sessions/{session_id}/complete")))
                    .json(&serde_json::json!({})),
            )
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Self::failure(name, format!("complete request: {e}")),
        };
        if !complete_resp.status().is_success() {
            let status = complete_resp.status();
            let body = complete_resp.text().await.unwrap_or_default();
            return Self::failure(name, format!("complete failed: status={status} body={body}"));
        }
        let final_body: Value = complete_resp.json().await.unwrap_or(Value::Null);
        if final_body.get("id").is_some() {
            Self::success(name, format!("file id={}", final_body["id"]))
        } else {
            Self::failure(name, format!("complete response missing id: {final_body}"))
        }
    }

    /// Abort right after initiate; should return 204.
    async fn test_resumable_abort(&self) -> TestResult {
        let name = "Uploads - Resumable Abort";

        let init_body = serde_json::json!({
            "bucket_id": self.bucket_id,
            "path": format!("test/{}/abort.bin", Utc::now().timestamp()),
            "filename": "abort.bin",
            "file_size": 1024,
            "chunk_size": 512,
        });
        let init_resp = match self
            .auth(self.client.post(self.url("/uploads/sessions")).json(&init_body))
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Self::failure(name, format!("initiate request: {e}")),
        };
        if init_resp.status().as_u16() == 401 {
            return Self::skipped(name, "401 — set API_AUTH_TOKEN");
        }
        if !init_resp.status().is_success() {
            return Self::failure(
                name,
                format!("initiate failed: status={}", init_resp.status()),
            );
        }
        let v: Value = init_resp.json().await.unwrap_or(Value::Null);
        let session_id = match v.get("session_id").and_then(Value::as_str) {
            Some(s) => s.to_string(),
            None => return Self::failure(name, "missing session_id"),
        };

        let abort_resp = match self
            .auth(
                self.client
                    .delete(self.url(&format!("/uploads/sessions/{session_id}"))),
            )
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Self::failure(name, format!("abort request: {e}")),
        };
        match abort_resp.status().as_u16() {
            204 => Self::success(name, "204 No Content"),
            other => Self::failure(name, format!("expected 204, got {other}")),
        }
    }

    pub async fn run_all(&mut self) -> Vec<TestResult> {
        let mut results = Vec::new();

        if !self.server_reachable().await {
            let why = "API server not reachable. Set API_BASE_URL and ensure server is running.";
            results.push(Self::skipped("Uploads - SingleShot Happy Path", why));
            results.push(Self::skipped("Uploads - SingleShot Missing File Field", why));
            results.push(Self::skipped("Uploads - Resumable Round Trip", why));
            results.push(Self::skipped("Uploads - Resumable Abort", why));
            return results;
        }

        results.push(self.test_single_shot_happy_path().await);
        results.push(self.test_single_shot_missing_file_field().await);
        results.push(self.test_resumable_round_trip().await);
        results.push(self.test_resumable_abort().await);
        results
    }
}

// Silence dead-code warnings when only some methods are exercised.
#[allow(dead_code)]
fn _exhaustive_use(e: TestError) -> TestError {
    e
}
