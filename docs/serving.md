# File Serving

Reference for the file-serving surface. Covers what ships in the
module, the public types a consumer imports, and the pluggable slots
for auth and storage.

> Consumers also need to coordinate **where** the URLs get served
> (Caddy / reverse proxy config, DNS, TLS). That half lives in the
> deployment workspace's `DEPLOYMENT-PLAN.md` and is deliberately out
> of scope here.

## Serving modes

The module supports three URL/serving patterns. Which ones apply to a
given deployment depends on the reverse-proxy layout; the module
provides the pieces for B and the signer used by C.

| Mode | Pattern | Flow | Served by |
|---|---|---|---|
| **A** Public fast-path | `bucket.example.com/public/*` | No Rust in the request path | Reverse proxy (Caddy) → public MinIO/S3 bucket |
| **B** Auth-aware (default) | `bucket.example.com/<key>` | 302 → presigned URL | Reverse proxy → `serving_router` → `ObjectStorage::presigned_get` |
| **C** Raw presigned | `s3.example.com/bucket/key?X-Amz-Signature=…` | Client goes straight to S3/MinIO | Reverse proxy → MinIO; URLs minted by `ObjectStorage::presigned_get` |

Mode B is the one the Axum handler in this crate answers directly. The
handler's response strategy is controlled by [`ServingMode`]:

- `Redirect` (default) — 302 to a short-lived presigned URL.
- `Stream` — proxy the bytes through the service. Useful for
  `LocalStorage` dev and CORS-restricted clients.
- `SignedUrl` — return `{"url": "..."}` JSON for clients that want the
  presigned URL without following the redirect.

## Public surface

Everything a consumer imports:

```rust
use backbone_bucket::{
    // Module + builder
    BucketModule, BucketConfig, StorageConfig, S3Config,
    ServingConfig, ServingMode,

    // Storage
    storage::{ObjectStorage, ObjectMeta, LocalStorage, S3Storage},

    // Auth slots
    auth::{AuthExtractor, AuthzPolicy, AuthzDecision, HasOwnerId,
           DefaultOwnerOnlyPolicy, ArcAuthzPolicy},

    // High-level file ops
    FileService, FileMeta,

    // Errors
    BucketError, BucketResult,
};
```

Any change to these names or signatures is a breaking change and bumps
the module version.

## Storage abstraction

`ObjectStorage` is the single boundary between the module and whichever
backend serves bytes. All methods except `public_url` are async —
`aws-sdk-s3`'s presigner needs a Tokio runtime context.

```rust
#[async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn put(&self, key: &str, body: Bytes, content_type: &str) -> BucketResult<()>;
    async fn get(&self, key: &str) -> BucketResult<Bytes>;
    async fn delete(&self, key: &str) -> BucketResult<()>;
    async fn head(&self, key: &str) -> BucketResult<ObjectMeta>;

    async fn presigned_get(&self, key: &str, ttl: Duration) -> BucketResult<Url>;
    async fn presigned_put(&self, key: &str, ttl: Duration, content_type: &str) -> BucketResult<Url>;

    fn public_url(&self, key: &str) -> Option<Url>;
}
```

### Backends shipped

- **`LocalStorage`** — filesystem. Presigned URLs are module-signed
  HMAC-SHA256 tokens that the serving handler validates in-process.
  Fine for dev and single-node deployments; **not S3-compatible**.
- **`S3Storage`** — AWS S3 / MinIO via `aws-sdk-s3`. Real SigV4
  presigned URLs. Gated behind the default `s3` feature.

Both backends reject path-traversal keys (`..` path segments) but
allow `..` inside a filename, e.g. `reports/report..v2.pdf`.

### Buffering

`get` / `put` are buffered — the full body sits in memory during the
call. Adequate for beta-scale traffic; set a body-size limit on the
router to cap exposure. Streaming `Body::from_stream` is planned
post-beta.

## Configuration

Loaded via the existing Backbone config overlay chain (YAML + env) —
no new config system.

```rust
pub struct BucketConfig {
    pub enabled: bool,
    pub storage: StorageConfig,
    pub serving: ServingConfig,
}

pub enum StorageConfig {
    Local {
        root: PathBuf,
        base_url: Url,
        signing_secret_env: String,   // env var name, not the secret
    },
    S3(S3Config),                     // also used for MinIO
}

pub struct S3Config {
    pub endpoint: Url,
    pub region: String,
    pub access_key_env: String,       // env var name, not the key
    pub secret_key_env: String,
    pub private_bucket: String,
    pub public_bucket: Option<String>,     // None ⇒ mode A disabled
    pub public_endpoint: Option<Url>,      // e.g. https://bucket.example.com
    pub force_path_style: bool,            // true for MinIO
}

pub struct ServingConfig {
    pub default_mode: ServingMode,
    pub public_prefix: String,             // "public/" by default
    pub presigned_ttl: Duration,           // serialized as whole seconds
}
```

**Secrets are never stored in config.** Every credential field on
`S3Config` and `StorageConfig::Local` is the *name* of the env var to
read at startup.

## Authentication & authorization

The module deliberately does **not** own identity or domain authz.
Consumers plug in two pieces:

### `AuthExtractor`

Any Axum `FromRequestParts` impl that yields a typed identity. There
is no separate extractor/identity split — the extractor type *is* the
identity, because Axum extractors yield themselves.

```rust
#[derive(Clone)]
struct MyUser { user_id: Uuid }

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for MyUser {
    type Rejection = Response;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // read JWT / session / header, construct MyUser
    }
}
```

### `AuthzPolicy<Identity>`

Decides whether a given identity may read a given `StoredFile`:

```rust
#[async_trait]
pub trait AuthzPolicy<Identity>: Send + Sync + 'static {
    async fn decide(&self, identity: &Identity, file: &StoredFile)
        -> Result<AuthzDecision, BucketError>;
}
```

The module ships `DefaultOwnerOnlyPolicy` as a starter. It requires the
consumer's identity to implement `HasOwnerId`. Richer rules (sharing,
workspace membership, public files) are a custom `AuthzPolicy` impl.

## Wiring

```rust
let storage: Arc<dyn ObjectStorage> = Arc::new(S3Storage::new(s3_cfg, serving_cfg)?);
let module = BucketModule::builder()
    .with_database(pool)
    .with_config(bucket_config)
    .with_storage(storage)
    .build()?;

let serving = module.serving_router::<MyUser>(Arc::new(MyPolicy))?;

let app = Router::new()
    .merge(module.crud_router())           // /api/v1/bucket/*
    .nest("/cdn", serving);                // /cdn/<key>
```

`crud_router()` is unchanged — it is the existing `/api/v1/bucket/*`
router, just renamed symmetrically. `serving_router::<A>(policy)`
returns `BucketResult<Router>`; it errors when either `.with_storage(...)`
or `.with_config(...)` was not called on the builder.

See [`examples/serving/main.rs`](../examples/serving/main.rs) for a
full copy-paste starting point, including a custom `AuthExtractor` and
`AuthzPolicy`.

## Key naming

`FileService` has two upload surfaces:

```rust
impl FileService {
    // Auto-UUID key — stable across renames, not human readable.
    pub async fn upload(&self, body: Bytes, meta: FileMeta) -> BucketResult<StoredFile>;

    // Caller-controlled key — for human-visible URLs.
    pub async fn upload_with_key(&self, key: &str, body: Bytes, meta: FileMeta)
        -> BucketResult<StoredFile>;
}
```

Keys beginning with `ServingConfig::public_prefix` (default `"public/"`)
route to the configured public bucket when one exists. When no public
bucket is configured, the key still writes to the private bucket and
a `debug` log event records the mismatch — dev environments without a
public bucket still work.

> ⚠️ **Upload permission is the real authz boundary for public files.**
> `AuthzPolicy` decides who can *read*; `FileService::upload_with_key`
> does not gate who may write under `public/*`. Ensure the caller of
> `upload_with_key` has already checked whether the current user is
> allowed to publish to the public namespace.

## Backward compatibility

`CdnService` (HMAC-signed URLs, not S3-compatible) is still exported
for one release. It is `#[deprecated]` and scheduled for removal in
`v0.3.0`. Replace call sites with:

```rust
let url = storage.presigned_get(&file.storage_key, ttl).await?;
```

Existing `/api/v1/bucket/*` CRUD routes are untouched.

## Non-goals

- Replicating every S3 feature (IAM policies, lifecycle rules, CORS
  config). Use the object store's own admin tools.
- Becoming a CDN. Put Cloudflare/Fastly in front of the public
  endpoint if CDN behavior is needed.
- Built-in image resizing or thumbnail generation. Separate concern,
  separate module.
- Hot backend swap at runtime. Backend is pinned at builder time.
- GCS backend. The `StorageBackend::Gcs` enum variant is preserved for
  schema stability but has no `ObjectStorage` impl.
