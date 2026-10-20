//! The generated row-rule guard, run directly: a transition rule (`progress_monotonic`,
//! `progress >= $old.progress` on update) refuses backwards progress with `invalid_progress`.

use backbone_bucket::application::service::ConversionJobRowRules;
use backbone_bucket::domain::entity::{ConversionJob, ConversionStatus};
use backbone_core::write_guard::{WriteCtx, WriteGuard, WriteKind};
use uuid::Uuid;

fn job(progress: i32) -> ConversionJob {
    ConversionJob::new(Uuid::nil(), "pdf".into(), ConversionStatus::Processing, progress)
}

fn refused(v: &[backbone_core::violation::Violation]) -> Vec<(String, String)> {
    v.iter().map(|x| (x.path.clone(), x.code.clone())).collect()
}

#[tokio::test]
async fn progress_moving_backwards_is_refused() {
    let (before, after) = (job(60), job(40));
    let outcome = ConversionJobRowRules
        .check(&WriteCtx::new(WriteKind::Update, Some(&before), Some(&after)))
        .await;
    assert_eq!(
        refused(&outcome.refuse),
        vec![("progress".to_string(), "invalid_progress".to_string())],
        "backwards progress is refused on the progress field"
    );
}

#[tokio::test]
async fn progress_moving_forwards_or_a_create_breaks_nothing() {
    let (before, after) = (job(40), job(60));
    let forward = ConversionJobRowRules
        .check(&WriteCtx::new(WriteKind::Update, Some(&before), Some(&after)))
        .await;
    assert!(forward.refuse.is_empty(), "{:?}", refused(&forward.refuse));
    // On create there is no row before the write: `$old.progress` is null and the rule holds.
    let created = ConversionJobRowRules.check(&WriteCtx::new(WriteKind::Create, None, Some(&after))).await;
    assert!(created.refuse.is_empty(), "{:?}", refused(&created.refuse));
}
