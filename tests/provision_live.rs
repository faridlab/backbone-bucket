//! The storage bootstrap against a live RustFS.
//!
//! Skipped unless `PROVISION_TEST_ENDPOINT` names a running server whose root
//! keys are `PROVISION_TEST_ROOT_USER` / `PROVISION_TEST_ROOT_PASSWORD`:
//!
//! ```text
//! docker run -d --rm -p 127.0.0.1:19000:9000 \
//!   -e RUSTFS_ACCESS_KEY=root-test -e RUSTFS_SECRET_KEY=root-test-secret \
//!   rustfs/rustfs:1.0.2 server /data
//! PROVISION_TEST_ENDPOINT=http://127.0.0.1:19000 \
//! PROVISION_TEST_ROOT_USER=root-test PROVISION_TEST_ROOT_PASSWORD=root-test-secret \
//!   cargo test --features s3 --test provision_live
//! ```
#![cfg(feature = "s3")]

use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::SharedCredentialsProvider;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use backbone_bucket::provision::{provision, ProvisionConfig, ServiceUser};

fn live() -> Option<(String, String, String)> {
    Some((
        std::env::var("PROVISION_TEST_ENDPOINT").ok()?,
        std::env::var("PROVISION_TEST_ROOT_USER").ok()?,
        std::env::var("PROVISION_TEST_ROOT_PASSWORD").ok()?,
    ))
}

fn client(endpoint: &str, access: &str, secret: &str) -> Client {
    let conf = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .endpoint_url(endpoint)
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            access, secret, None, None, "test",
        )))
        .force_path_style(true)
        .build();
    Client::from_conf(conf)
}

#[tokio::test]
async fn provisions_buckets_and_a_least_privilege_user_and_reruns_cleanly() {
    let Some((endpoint, root, root_secret)) = live() else {
        eprintln!("skipped: PROVISION_TEST_ENDPOINT not set");
        return;
    };
    // Unique names, so the test can run against a server that ran it before.
    let tag = format!("{:x}", std::process::id() ^ std::time::SystemTime::now().elapsed().map_or(0, |d| d.as_nanos() as u32));
    let private = format!("t{tag}-private");
    let public = format!("t{tag}-public");
    let other = format!("t{tag}-other");
    let svc = format!("svc{tag}");
    let cfg = ProvisionConfig {
        endpoint: endpoint.parse().unwrap(),
        root_access_key: root.clone(),
        root_secret_key: root_secret.clone(),
        private_bucket: private.clone(),
        public_bucket: public.clone(),
        service_user: Some(ServiceUser {
            access_key: svc.clone(),
            secret_key: "svc-secret-12345678".into(),
            policy_name: format!("{svc}-policy"),
        }),
    };

    let first = provision(&cfg).await.expect("first run");
    assert!(first.iter().filter(|s| s.what.starts_with("bucket ")).all(|s| s.changed));
    let second = provision(&cfg).await.expect("second run");
    assert!(second.iter().filter(|s| s.what.starts_with("bucket ")).all(|s| !s.changed), "{second:?}");
    assert!(second.iter().any(|s| s.what.contains("attached") && !s.changed), "{second:?}");

    let root_c = client(&endpoint, &root, &root_secret);
    root_c.create_bucket().bucket(&other).send().await.unwrap();
    let svc_c = client(&endpoint, &svc, "svc-secret-12345678");
    let body = || ByteStream::from_static(b"hello");

    // The service user works inside its two buckets…
    for b in [&private, &public] {
        svc_c.put_object().bucket(b).key("a.txt").body(body()).send().await
            .unwrap_or_else(|e| panic!("put into {b}: {e:?}"));
        let got = svc_c.get_object().bucket(b).key("a.txt").send().await.unwrap();
        assert_eq!(got.body.collect().await.unwrap().into_bytes().as_ref(), b"hello");
    }
    svc_c.list_objects_v2().bucket(&private).send().await.unwrap();
    svc_c.delete_object().bucket(&private).key("a.txt").send().await.unwrap();
    // …and nowhere else.
    assert!(svc_c.put_object().bucket(&other).key("a.txt").body(body()).send().await.is_err());
    assert!(svc_c.create_bucket().bucket(format!("t{tag}-made")).send().await.is_err());

    // Anonymous read on the public bucket only.
    let http = reqwest::Client::new();
    let anon = |b: &str, k: &str| http.get(format!("{endpoint}/{b}/{k}")).send();
    assert_eq!(anon(&public, "a.txt").await.unwrap().status(), 200);
    root_c.put_object().bucket(&private).key("p.txt").body(body()).send().await.unwrap();
    assert_eq!(anon(&private, "p.txt").await.unwrap().status(), 403);

    // A re-run with a new secret rotates the key and keeps the policy.
    let mut rotated = cfg.clone();
    rotated.service_user.as_mut().unwrap().secret_key = "svc-rotated-87654321".into();
    provision(&rotated).await.expect("rotation run");
    let new_c = client(&endpoint, &svc, "svc-rotated-87654321");
    new_c.get_object().bucket(&public).key("a.txt").send().await.expect("rotated key reads");
    assert!(svc_c.get_object().bucket(&public).key("a.txt").send().await.is_err(), "old key still works");
}
