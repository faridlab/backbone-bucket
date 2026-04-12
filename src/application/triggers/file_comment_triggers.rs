//! FileComment trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::FileComment;

pub type FileCommentTriggerEvent      = TriggerEvent;
pub type FileCommentTriggerContext    = TriggerContext<FileComment>;
pub type FileCommentTriggerContextMut = TriggerContextMut<FileComment>;
pub type FileCommentActionExecutor    = ActionExecutor;
pub type FileCommentTriggerRegistry   = TriggerRegistry<FileComment>;
pub type FileCommentTriggerHandlerObj = dyn TriggerHandler<TriggerContext<FileComment>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct FileCommentAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::FileCommentEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<FileCommentActionExecutor>>,
}

impl FileCommentAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::FileCommentEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<FileCommentActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<FileCommentTriggerContext, FileCommentTriggerEvent> for FileCommentAfterCreateHandler1 {
    fn events(&self) -> Vec<FileCommentTriggerEvent> {
        vec![FileCommentTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &FileCommentTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit commentcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        // Unknown action type 'if': mentions != null && mentions.length > 0 { notify
        // Unknown action type 'if': user_id != file.owner_id { notify
        // Unknown action type 'if': parent_id != null && get_comment
        Ok(())
    }
}

/// AfterCreate handler
pub struct FileCommentAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::FileCommentEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<FileCommentActionExecutor>>,
}

impl FileCommentAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::FileCommentEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<FileCommentActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<FileCommentTriggerContext, FileCommentTriggerEvent> for FileCommentAfterCreateHandler2 {
    fn events(&self) -> Vec<FileCommentTriggerEvent> {
        vec![FileCommentTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &FileCommentTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct FileCommentAfterDeleteHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::FileCommentEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<FileCommentActionExecutor>>,
}

impl FileCommentAfterDeleteHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::FileCommentEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<FileCommentActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<FileCommentTriggerContext, FileCommentTriggerEvent> for FileCommentAfterDeleteHandler3 {
    fn events(&self) -> Vec<FileCommentTriggerEvent> {
        vec![FileCommentTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &FileCommentTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Emit commentdeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

// State machine trigger handlers

/// Action executor for FileComment triggers

pub fn file_comment_trigger_registry() -> FileCommentTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(FileCommentAfterCreateHandler1::new()));
        r.register(Arc::new(FileCommentAfterCreateHandler2::new()));
        r.register(Arc::new(FileCommentAfterDeleteHandler3::new()));
    })
}
