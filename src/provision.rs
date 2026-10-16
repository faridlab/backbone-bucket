//! Object-storage bootstrap: the buckets, their anonymous access, and a
//! least-privilege service user.
//!
//! A service runs this once per deploy, as a one-shot job ahead of the
//! service itself (`<service> storage-init`), the way it runs its migrations.
//! It replaces a shell script around the MinIO client (`mc`), whose images
//! can no longer be pulled.
//!
//! Every step is idempotent, so a second run changes nothing:
//!
//! 1. Create the private and the public bucket.
//! 2. Give the public bucket anonymous read (get objects, list the bucket);
//!    remove any bucket policy from the private bucket.
//! 3. With service credentials configured: create a policy that allows
//!    get / put / delete / list on those two buckets only, create the user
//!    (or rotate its secret key), and attach the policy to it.
//!
//! Buckets and bucket policies go through the S3 API. Users and policies go
//! through the RustFS admin API (`/rustfs/admin/v3`), which takes plain JSON
//! signed with SigV4; its MinIO-compatible twin under `/minio/admin` encrypts
//! request bodies, which nothing here needs.

use std::time::{Duration, Instant, SystemTime};

use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::SharedCredentialsProvider;
use aws_sdk_s3::Client;
use aws_sigv4::http_request::{
    sign, PayloadChecksumKind, SignableBody, SignableRequest, SigningSettings,
};
use aws_sigv4::sign::v4;
use serde_json::{json, Value};
use url::Url;

use crate::error::{BucketError, BucketResult};

/// The region RustFS signs for unless it is configured otherwise.
const REGION: &str = "us-east-1";
/// How long to wait for the storage server to report ready.
const READY_TIMEOUT: Duration = Duration::from_secs(60);
/// Attempts for an admin call the server answers with 503 while it starts.
const ADMIN_ATTEMPTS: u32 = 10;

/// Credentials for the least-privilege user the service runs as.
#[derive(Debug, Clone)]
pub struct ServiceUser {
    pub access_key: String,
    pub secret_key: String,
    /// Name of the policy created for, and attached to, this user.
    pub policy_name: String,
}

/// What [`provision`] sets up.
#[derive(Debug, Clone)]
pub struct ProvisionConfig {
    /// The storage server, e.g. `http://minio:9000`.
    pub endpoint: Url,
    /// Root credentials. Used only by this bootstrap, never by the service.
    pub root_access_key: String,
    pub root_secret_key: String,
    pub private_bucket: String,
    pub public_bucket: String,
    /// `None` skips the user and its policy (the service then uses root keys).
    pub service_user: Option<ServiceUser>,
}

impl ProvisionConfig {
    /// Read the configuration from the deployment's `MINIO_*` contract:
    /// `MINIO_ENDPOINT`, `MINIO_ROOT_USER`, `MINIO_ROOT_PASSWORD`,
    /// `MINIO_PRIVATE_BUCKET`, `MINIO_PUBLIC_BUCKET`, and optionally the pair
    /// `MINIO_ACCESS_KEY` / `MINIO_SECRET_KEY` with `MINIO_SERVICE_POLICY`.
    pub fn from_env() -> BucketResult<Self> {
        Self::from_lookup(|k| std::env::var(k).ok())
    }

    /// [`ProvisionConfig::from_env`] over any variable lookup.
    pub fn from_lookup(get: impl Fn(&str) -> Option<String>) -> BucketResult<Self> {
        let required = |k: &str| {
            get(k)
                .filter(|v| !v.trim().is_empty())
                .ok_or_else(|| BucketError::Config(format!("{k} must be set")))
        };
        let endpoint = required("MINIO_ENDPOINT")?;
        let endpoint = Url::parse(&endpoint)
            .map_err(|e| BucketError::Config(format!("MINIO_ENDPOINT {endpoint:?}: {e}")))?;
        let access = get("MINIO_ACCESS_KEY").filter(|v| !v.is_empty());
        let secret = get("MINIO_SECRET_KEY").filter(|v| !v.is_empty());
        let service_user = match (access, secret) {
            (Some(access_key), Some(secret_key)) => Some(ServiceUser {
                access_key,
                secret_key,
                policy_name: required("MINIO_SERVICE_POLICY")?,
            }),
            (None, None) => None,
            _ => {
                return Err(BucketError::Config(
                    "set both MINIO_ACCESS_KEY and MINIO_SECRET_KEY, or neither".into(),
                ))
            }
        };
        Ok(Self {
            endpoint,
            root_access_key: required("MINIO_ROOT_USER")?,
            root_secret_key: required("MINIO_ROOT_PASSWORD")?,
            private_bucket: required("MINIO_PRIVATE_BUCKET")?,
            public_bucket: required("MINIO_PUBLIC_BUCKET")?,
            service_user,
        })
    }
}

/// One line of what a run did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub what: String,
    /// `false` when the state was already in place.
    pub changed: bool,
}

/// Anonymous read on `bucket`: get its objects and list it.
pub fn public_read_policy(bucket: &str) -> Value {
    json!({
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Principal": { "AWS": ["*"] },
                "Action": ["s3:GetBucketLocation", "s3:ListBucket"],
                "Resource": [format!("arn:aws:s3:::{bucket}")]
            },
            {
                "Effect": "Allow",
                "Principal": { "AWS": ["*"] },
                "Action": ["s3:GetObject"],
                "Resource": [format!("arn:aws:s3:::{bucket}/*")]
            }
        ]
    })
}

/// Get / put / delete / list on `buckets` and their objects, nothing else.
pub fn service_user_policy(buckets: &[&str]) -> Value {
    let resources: Vec<String> = buckets
        .iter()
        .flat_map(|b| [format!("arn:aws:s3:::{b}"), format!("arn:aws:s3:::{b}/*")])
        .collect();
    json!({
        "Version": "2012-10-17",
        "Statement": [{
            "Effect": "Allow",
            "Action": [
                "s3:GetObject", "s3:PutObject", "s3:DeleteObject",
                "s3:ListBucket", "s3:ListBucketMultipartUploads",
                "s3:ListMultipartUploadParts", "s3:AbortMultipartUpload",
                "s3:GetBucketLocation"
            ],
            "Resource": resources
        }]
    })
}

/// Set up buckets, anonymous access and the service user. Safe to re-run.
pub async fn provision(cfg: &ProvisionConfig) -> BucketResult<Vec<Step>> {
    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| BucketError::Other(format!("http client: {e}")))?;
    wait_ready(&http, &cfg.endpoint).await?;

    let s3 = s3_client(cfg);
    let mut steps = Vec::new();
    for bucket in [&cfg.private_bucket, &cfg.public_bucket] {
        let changed = create_bucket(&s3, bucket).await?;
        steps.push(Step { what: format!("bucket {bucket}"), changed });
    }

    s3.put_bucket_policy()
        .bucket(&cfg.public_bucket)
        .policy(public_read_policy(&cfg.public_bucket).to_string())
        .send()
        .await
        .map_err(|e| BucketError::S3(format!("public-read policy on {}: {e}", cfg.public_bucket)))?;
    steps.push(Step { what: format!("anonymous read on {}", cfg.public_bucket), changed: true });

    s3.delete_bucket_policy()
        .bucket(&cfg.private_bucket)
        .send()
        .await
        .map_err(|e| BucketError::S3(format!("clear policy on {}: {e}", cfg.private_bucket)))?;
    steps.push(Step { what: format!("no anonymous access on {}", cfg.private_bucket), changed: true });

    if let Some(user) = &cfg.service_user {
        let admin = Admin { http: &http, cfg };
        let policy = service_user_policy(&[&cfg.private_bucket, &cfg.public_bucket]);
        admin
            .call("PUT", &format!("add-canned-policy?name={}", user.policy_name), Some(policy))
            .await?;
        steps.push(Step { what: format!("policy {}", user.policy_name), changed: true });

        admin
            .call(
                "PUT",
                &format!("add-user?accessKey={}", user.access_key),
                Some(json!({ "secretKey": user.secret_key, "status": "enabled" })),
            )
            .await?;
        steps.push(Step { what: format!("user {}", user.access_key), changed: true });

        let attached = admin
            .call(
                "POST",
                "idp/builtin/policy/attach",
                Some(json!({ "policies": [user.policy_name], "user": user.access_key })),
            )
            .await?;
        let newly = attached
            .get("policiesAttached")
            .and_then(Value::as_array)
            .is_some_and(|a| !a.is_empty());
        steps.push(Step {
            what: format!("policy {} attached to {}", user.policy_name, user.access_key),
            changed: newly,
        });
    }
    Ok(steps)
}

/// The `storage-init` entry point: read the environment, provision, print
/// one line per step. A service's `main` calls this for its subcommand.
pub async fn run_from_env() -> BucketResult<()> {
    let cfg = ProvisionConfig::from_env()?;
    if cfg.service_user.is_none() {
        println!("storage-init: MINIO_ACCESS_KEY / MINIO_SECRET_KEY not set, so no service user is provisioned");
    }
    for step in provision(&cfg).await? {
        let mark = if step.changed { "set" } else { "already in place" };
        println!("storage-init: {}: {mark}", step.what);
    }
    println!("storage-init: done");
    Ok(())
}

fn s3_client(cfg: &ProvisionConfig) -> Client {
    let creds = Credentials::new(
        &cfg.root_access_key,
        &cfg.root_secret_key,
        None,
        None,
        "backbone-bucket-provision",
    );
    let conf = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .endpoint_url(cfg.endpoint.as_str())
        .credentials_provider(SharedCredentialsProvider::new(creds))
        .force_path_style(true)
        .build();
    Client::from_conf(conf)
}

/// `true` when the bucket was created, `false` when it already existed.
///
/// Asks first: in `us-east-1`, S3 (and RustFS) answers a create for a bucket
/// you already own with success, so the create alone cannot tell.
async fn create_bucket(s3: &Client, bucket: &str) -> BucketResult<bool> {
    if s3.head_bucket().bucket(bucket).send().await.is_ok() {
        return Ok(false);
    }
    match s3.create_bucket().bucket(bucket).send().await {
        Ok(_) => Ok(true),
        Err(e) => {
            let service = e.into_service_error();
            if service.is_bucket_already_owned_by_you() || service.is_bucket_already_exists() {
                Ok(false)
            } else {
                Err(BucketError::S3(format!("create bucket {bucket}: {service}")))
            }
        }
    }
}

/// Wait until `/health/ready` answers 200. `/health` alone goes green before
/// the server accepts requests.
async fn wait_ready(http: &reqwest::Client, endpoint: &Url) -> BucketResult<()> {
    let url = endpoint
        .join("/health/ready")
        .map_err(|e| BucketError::Url(e.to_string()))?;
    let start = Instant::now();
    loop {
        match http.get(url.clone()).send().await {
            Ok(r) if r.status().is_success() => return Ok(()),
            _ if start.elapsed() >= READY_TIMEOUT => {
                return Err(BucketError::Other(format!(
                    "{url} did not report ready within {}s",
                    READY_TIMEOUT.as_secs()
                )))
            }
            _ => tokio::time::sleep(Duration::from_millis(500)).await,
        }
    }
}

struct Admin<'a> {
    http: &'a reqwest::Client,
    cfg: &'a ProvisionConfig,
}

impl Admin<'_> {
    /// A root-signed admin call; returns the JSON body (`null` when empty).
    async fn call(&self, method: &str, path_and_query: &str, body: Option<Value>) -> BucketResult<Value> {
        let url = self
            .cfg
            .endpoint
            .join(&format!("/rustfs/admin/v3/{path_and_query}"))
            .map_err(|e| BucketError::Url(e.to_string()))?;
        let body = body.map(|b| b.to_string()).unwrap_or_default();
        let mut wait = Duration::from_millis(250);
        for attempt in 1..=ADMIN_ATTEMPTS {
            let resp = self.signed(method, &url, &body)?.send().await.map_err(|e| {
                BucketError::Other(format!("{method} {path_and_query}: {e}"))
            })?;
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            if status.is_success() {
                return Ok(serde_json::from_str(&text).unwrap_or(Value::Null));
            }
            if status.as_u16() != 503 || attempt == ADMIN_ATTEMPTS {
                return Err(BucketError::Other(format!(
                    "{method} /rustfs/admin/v3/{path_and_query}: {status} {text}"
                )));
            }
            tokio::time::sleep(wait).await;
            wait = (wait * 2).min(Duration::from_secs(4));
        }
        unreachable!("the last attempt returns")
    }

    fn signed(&self, method: &str, url: &Url, body: &str) -> BucketResult<reqwest::RequestBuilder> {
        let host = match url.port() {
            Some(p) => format!("{}:{p}", url.host_str().unwrap_or_default()),
            None => url.host_str().unwrap_or_default().to_string(),
        };
        let content_type = "application/json";
        let headers = [("host", host.as_str()), ("content-type", content_type)];
        let identity = Credentials::new(
            &self.cfg.root_access_key,
            &self.cfg.root_secret_key,
            None,
            None,
            "backbone-bucket-provision",
        )
        .into();
        let mut settings = SigningSettings::default();
        settings.payload_checksum_kind = PayloadChecksumKind::XAmzSha256;
        let params = v4::SigningParams::builder()
            .identity(&identity)
            .region(REGION)
            .name("s3")
            .time(SystemTime::now())
            .settings(settings)
            .build()
            .map_err(|e| BucketError::Other(format!("signing params: {e}")))?
            .into();
        let signable = SignableRequest::new(
            method,
            url.as_str(),
            headers.iter().copied(),
            SignableBody::Bytes(body.as_bytes()),
        )
        .map_err(|e| BucketError::Other(format!("signable request: {e}")))?;
        let (instructions, _) = sign(signable, &params)
            .map_err(|e| BucketError::Other(format!("sign: {e}")))?
            .into_parts();

        let method = reqwest::Method::from_bytes(method.as_bytes())
            .map_err(|e| BucketError::Other(e.to_string()))?;
        let mut req = self
            .http
            .request(method, url.clone())
            .header("content-type", content_type)
            .body(body.to_string());
        for (name, value) in instructions.headers() {
            req = req.header(name, value);
        }
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> =
            pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        move |k| map.get(k).cloned()
    }

    const BASE: &[(&str, &str)] = &[
        ("MINIO_ENDPOINT", "http://minio:9000"),
        ("MINIO_ROOT_USER", "root"),
        ("MINIO_ROOT_PASSWORD", "root-secret"),
        ("MINIO_PRIVATE_BUCKET", "app-private"),
        ("MINIO_PUBLIC_BUCKET", "app-public"),
    ];

    #[test]
    fn reads_the_minio_contract_without_a_service_user() {
        let cfg = ProvisionConfig::from_lookup(env(BASE)).unwrap();
        assert_eq!(cfg.endpoint.as_str(), "http://minio:9000/");
        assert_eq!(cfg.private_bucket, "app-private");
        assert!(cfg.service_user.is_none());
    }

    #[test]
    fn a_service_user_needs_both_keys_and_a_policy_name() {
        let mut half = BASE.to_vec();
        half.push(("MINIO_ACCESS_KEY", "svc"));
        assert!(ProvisionConfig::from_lookup(env(&half)).is_err());

        let mut no_policy = half.clone();
        no_policy.push(("MINIO_SECRET_KEY", "svc-secret"));
        assert!(ProvisionConfig::from_lookup(env(&no_policy)).is_err());

        let mut full = no_policy.clone();
        full.push(("MINIO_SERVICE_POLICY", "app-svc"));
        let user = ProvisionConfig::from_lookup(env(&full)).unwrap().service_user.unwrap();
        assert_eq!((user.access_key.as_str(), user.policy_name.as_str()), ("svc", "app-svc"));
    }

    #[test]
    fn a_missing_or_blank_required_variable_is_named() {
        let mut blank = BASE.to_vec();
        blank.retain(|(k, _)| *k != "MINIO_PUBLIC_BUCKET");
        blank.push(("MINIO_PUBLIC_BUCKET", "  "));
        let err = ProvisionConfig::from_lookup(env(&blank)).unwrap_err().to_string();
        assert!(err.contains("MINIO_PUBLIC_BUCKET"), "{err}");
    }

    #[test]
    fn the_service_policy_covers_exactly_its_buckets() {
        let p = service_user_policy(&["a", "b"]);
        let res: Vec<&str> = p["Statement"][0]["Resource"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(res, ["arn:aws:s3:::a", "arn:aws:s3:::a/*", "arn:aws:s3:::b", "arn:aws:s3:::b/*"]);
        let actions = p["Statement"][0]["Action"].as_array().unwrap();
        assert!(!actions.iter().any(|a| a.as_str().unwrap().starts_with("admin:")));
    }

    #[test]
    fn the_public_policy_matches_mc_anonymous_download() {
        let p = public_read_policy("pub");
        assert_eq!(p["Statement"][0]["Resource"][0], "arn:aws:s3:::pub");
        assert_eq!(p["Statement"][1]["Action"][0], "s3:GetObject");
        assert_eq!(p["Statement"][1]["Resource"][0], "arn:aws:s3:::pub/*");
    }
}
