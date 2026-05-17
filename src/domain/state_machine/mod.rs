mod bucket_state_machine;
mod conversion_job_state_machine;
mod file_comment_state_machine;
mod file_lock_state_machine;
mod file_share_state_machine;
mod processing_job_state_machine;
mod stored_file_state_machine;
mod upload_session_state_machine;
mod user_quota_state_machine;

/// Shared error type for all state machines in this module
#[derive(Debug, Clone, thiserror::Error)]
pub enum StateMachineError {
    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Invalid transition: {0}")]
    InvalidTransition(String),

    #[error("Transition '{transition}' not allowed from state '{from}'")]
    TransitionNotAllowed {
        transition: String,
        from: String,
    },

    #[error("Role '{role}' not authorized for transition '{transition}'")]
    RoleNotAuthorized {
        role: String,
        transition: String,
    },

    #[error("Guard condition failed for transition '{0}'")]
    GuardFailed(String),

    #[error("Cannot transition from final state '{0}'")]
    FinalStateReached(String),
}

pub use bucket_state_machine::{BucketState, BucketTransition, BucketStateMachine};
pub use conversion_job_state_machine::{ConversionJobState, ConversionJobTransition, ConversionJobStateMachine};
pub use file_comment_state_machine::{FileCommentState, FileCommentTransition, FileCommentStateMachine};
pub use file_lock_state_machine::{FileLockState, FileLockTransition, FileLockStateMachine};
pub use file_share_state_machine::{FileShareState, FileShareTransition, FileShareStateMachine};
pub use processing_job_state_machine::{ProcessingJobState, ProcessingJobTransition, ProcessingJobStateMachine};
pub use stored_file_state_machine::{StoredFileState, StoredFileTransition, StoredFileStateMachine};
pub use upload_session_state_machine::{UploadSessionState, UploadSessionTransition, UploadSessionStateMachine};
pub use user_quota_state_machine::{UserQuotaState, UserQuotaTransition, UserQuotaStateMachine};
