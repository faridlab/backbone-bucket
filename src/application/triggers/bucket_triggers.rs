//! Bucket trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::Bucket;

pub type BucketTriggerEvent      = TriggerEvent;
pub type BucketTriggerContext    = TriggerContext<Bucket>;
pub type BucketTriggerContextMut = TriggerContextMut<Bucket>;
pub type BucketActionExecutor    = ActionExecutor;
pub type BucketTriggerRegistry   = TriggerRegistry<Bucket>;
pub type BucketTriggerHandlerObj = dyn TriggerHandler<TriggerContext<Bucket>, TriggerEvent>;


// Lifecycle trigger handlers

/// BeforeCreate handler
pub struct BucketBeforeCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BucketEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BucketActionExecutor>>,
}

impl BucketBeforeCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BucketEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BucketActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketBeforeCreateHandler1 {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::BeforeCreate]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'if': slug == null || slug == '' { set: slug = slugify
        // Unknown action type 'if': root_path == null || root_path == '' { set: root_path = owner_id + '/' + slug }
        Ok(())
    }
}

/// AfterCreate handler
pub struct BucketAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BucketEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BucketActionExecutor>>,
}

impl BucketAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BucketEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BucketActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketAfterCreateHandler2 {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Call storageprovisioningservice.create_bucket_directory
        // <<< CUSTOM SERVICE CALL START >>>
        // self.storageprovisioningservice_service.create_bucket_directory(&ctx.entity).await?;
        // <<< CUSTOM SERVICE CALL END >>>
        // Emit bucketcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterCreate handler
pub struct BucketAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BucketEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BucketActionExecutor>>,
}

impl BucketAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BucketEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BucketActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketAfterCreateHandler3 {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// BeforeDelete handler
pub struct BucketBeforeDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BucketEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BucketActionExecutor>>,
}

impl BucketBeforeDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BucketEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BucketActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketBeforeDeleteHandler4 {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::BeforeDelete]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct BucketAfterDeleteHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BucketEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BucketActionExecutor>>,
}

impl BucketAfterDeleteHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BucketEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BucketActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketAfterDeleteHandler5 {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'queue': cleanupbucketstorage
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering active state
pub struct BucketOnEnterActiveHandler {}

impl BucketOnEnterActiveHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketOnEnterActiveHandler {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::OnEnterState("active".to_string())]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering readonly state
pub struct BucketOnEnterReadonlyHandler {}

impl BucketOnEnterReadonlyHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketOnEnterReadonlyHandler {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::OnEnterState("readonly".to_string())]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering archived state
pub struct BucketOnEnterArchivedHandler {}

impl BucketOnEnterArchivedHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketOnEnterArchivedHandler {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::OnEnterState("archived".to_string())]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Unknown action type 'queue': compressarchivedbucket
        Ok(())
    }
}

/// Handler for entering deleted state
pub struct BucketOnEnterDeletedHandler {
    pub event_publisher: Option<Arc<crate::domain::event::BucketEventPublisher>>,
}

impl BucketOnEnterDeletedHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<BucketTriggerContext, BucketTriggerEvent> for BucketOnEnterDeletedHandler {
    fn events(&self) -> Vec<BucketTriggerEvent> {
        vec![BucketTriggerEvent::OnEnterState("deleted".to_string())]
    }

    async fn handle(&self, ctx: &BucketTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set metadata.deleted_at to current timestamp
        // <<< CUSTOM SET: metadata.deleted_at = now >>>
        // ctx.entity.metadata.deleted_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // Unknown action type 'queue': schedulebucketpurge
        // Emit bucketdeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Action executor for Bucket triggers

pub fn bucket_trigger_registry() -> BucketTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(BucketBeforeCreateHandler1::new()));
        r.register(Arc::new(BucketAfterCreateHandler2::new()));
        r.register(Arc::new(BucketAfterCreateHandler3::new()));
        r.register(Arc::new(BucketBeforeDeleteHandler4::new()));
        r.register(Arc::new(BucketAfterDeleteHandler5::new()));
        r.register(Arc::new(BucketOnEnterActiveHandler::new()));
        r.register(Arc::new(BucketOnEnterReadonlyHandler::new()));
        r.register(Arc::new(BucketOnEnterArchivedHandler::new()));
        r.register(Arc::new(BucketOnEnterDeletedHandler::new()));
    })
}
