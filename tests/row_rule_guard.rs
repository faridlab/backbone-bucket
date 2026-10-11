//! The generated row-rule guard, run directly: a transition rule (`progress_monotonic`,
//! `progress >= $old.progress` on update) reports a break in shadow and refuses nothing.

use backbone_bucket::application::service::ConversionJobRowRules;
use backbone_bucket::domain::entity::{ConversionJob, ConversionStatus};
use backbone_core::write_guard::{WriteCtx, WriteGuard, WriteKind};
use uuid::Uuid;

fn job(progress: i32) -> ConversionJob {
    ConversionJob::new(Uuid::nil(), "pdf".into(), ConversionStatus::Processing, progress)
}

#[tokio::test]
async fn progress_moving_backwards_is_reported_in_shadow() {
    let (before, after) = (job(60), job(40));
    let outcome = ConversionJobRowRules
        .check(&WriteCtx::new(WriteKind::Update, Some(&before), Some(&after)))
        .await;
    assert!(outcome.refuse.is_empty(), "a shadow rule refuses nothing");
    let codes: Vec<&str> = outcome.shadow.iter().map(|v| v.code.as_str()).collect();
    assert!(codes.contains(&"invalid_progress"), "{codes:?}");
}

#[tokio::test]
async fn progress_moving_forwards_or_a_create_breaks_nothing() {
    let (before, after) = (job(40), job(60));
    let forward = ConversionJobRowRules
        .check(&WriteCtx::new(WriteKind::Update, Some(&before), Some(&after)))
        .await;
    assert!(forward.shadow.iter().all(|v| v.code != "invalid_progress"), "{:?}", forward.shadow);
    // On create there is no row before the write: `$old.progress` is null and the rule holds.
    let created = ConversionJobRowRules.check(&WriteCtx::new(WriteKind::Create, None, Some(&after))).await;
    assert!(created.shadow.iter().all(|v| v.code != "invalid_progress"), "{:?}", created.shadow);
}
