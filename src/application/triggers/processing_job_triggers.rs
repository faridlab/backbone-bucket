//! ProcessingJob trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::ProcessingJob;

pub type ProcessingJobTriggerEvent      = TriggerEvent;
pub type ProcessingJobTriggerContext    = TriggerContext<ProcessingJob>;
pub type ProcessingJobTriggerContextMut = TriggerContextMut<ProcessingJob>;
pub type ProcessingJobActionExecutor    = ActionExecutor;
pub type ProcessingJobTriggerRegistry   = TriggerRegistry<ProcessingJob>;
pub type ProcessingJobTriggerHandlerObj = dyn TriggerHandler<TriggerContext<ProcessingJob>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct ProcessingJobAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProcessingJobActionExecutor>>,
}

impl ProcessingJobAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProcessingJobEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProcessingJobActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobAfterCreateHandler1 {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit processingjobcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct ProcessingJobAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProcessingJobActionExecutor>>,
}

impl ProcessingJobAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProcessingJobEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProcessingJobActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobAfterCreateHandler2 {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterCreate handler
pub struct ProcessingJobAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProcessingJobActionExecutor>>,
}

impl ProcessingJobAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProcessingJobEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProcessingJobActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobAfterCreateHandler3 {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'dispatch': processingjob where status == 'pending' and
        Ok(())
    }
}

/// AfterCreate handler
pub struct ProcessingJobAfterCreateHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProcessingJobActionExecutor>>,
}

impl ProcessingJobAfterCreateHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProcessingJobEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProcessingJobActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobAfterCreateHandler4 {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'update': processingjob
        // Unknown action type 'update': processingjob
        Ok(())
    }
}

/// AfterCreate handler
pub struct ProcessingJobAfterCreateHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProcessingJobActionExecutor>>,
}

impl ProcessingJobAfterCreateHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProcessingJobEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProcessingJobActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobAfterCreateHandler5 {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'update': storedfile
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering pending state
pub struct ProcessingJobOnEnterPendingHandler {}

impl ProcessingJobOnEnterPendingHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobOnEnterPendingHandler {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::OnEnterState("pending".to_string())]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering running state
pub struct ProcessingJobOnEnterRunningHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
}

impl ProcessingJobOnEnterRunningHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobOnEnterRunningHandler {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::OnEnterState("running".to_string())]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set started_at to current timestamp
        // <<< CUSTOM SET: started_at = now >>>
        // ctx.entity.started_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // Emit jobstartedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: jobstartedevent
            // <<< CUSTOM EMIT: jobstartedevent >>>
        }
        Ok(())
    }
}

/// Handler for entering completed state
pub struct ProcessingJobOnEnterCompletedHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
}

impl ProcessingJobOnEnterCompletedHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobOnEnterCompletedHandler {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::OnEnterState("completed".to_string())]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set completed_at to current timestamp
        // <<< CUSTOM SET: completed_at = now >>>
        // ctx.entity.completed_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // Emit jobcompletedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: jobcompletedevent
            // <<< CUSTOM EMIT: jobcompletedevent >>>
        }
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering failed state
pub struct ProcessingJobOnEnterFailedHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
}

impl ProcessingJobOnEnterFailedHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobOnEnterFailedHandler {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::OnEnterState("failed".to_string())]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set completed_at to current timestamp
        // <<< CUSTOM SET: completed_at = now >>>
        // ctx.entity.completed_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // Emit jobfailedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: jobfailedevent
            // <<< CUSTOM EMIT: jobfailedevent >>>
        }
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering cancelled state
pub struct ProcessingJobOnEnterCancelledHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ProcessingJobEventPublisher>>,
}

impl ProcessingJobOnEnterCancelledHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ProcessingJobTriggerContext, ProcessingJobTriggerEvent> for ProcessingJobOnEnterCancelledHandler {
    fn events(&self) -> Vec<ProcessingJobTriggerEvent> {
        vec![ProcessingJobTriggerEvent::OnEnterState("cancelled".to_string())]
    }

    async fn handle(&self, ctx: &ProcessingJobTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set completed_at to current timestamp
        // <<< CUSTOM SET: completed_at = now >>>
        // ctx.entity.completed_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // Emit jobcancelledevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: jobcancelledevent
            // <<< CUSTOM EMIT: jobcancelledevent >>>
        }
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for ProcessingJob triggers

pub fn processing_job_trigger_registry() -> ProcessingJobTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(ProcessingJobAfterCreateHandler1::new()));
        r.register(Arc::new(ProcessingJobAfterCreateHandler2::new()));
        r.register(Arc::new(ProcessingJobAfterCreateHandler3::new()));
        r.register(Arc::new(ProcessingJobAfterCreateHandler4::new()));
        r.register(Arc::new(ProcessingJobAfterCreateHandler5::new()));
        r.register(Arc::new(ProcessingJobOnEnterPendingHandler::new()));
        r.register(Arc::new(ProcessingJobOnEnterRunningHandler::new()));
        r.register(Arc::new(ProcessingJobOnEnterCompletedHandler::new()));
        r.register(Arc::new(ProcessingJobOnEnterFailedHandler::new()));
        r.register(Arc::new(ProcessingJobOnEnterCancelledHandler::new()));
    })
}
