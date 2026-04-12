use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AccessAction;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FileAccessLog {
    pub id: Uuid,
    pub file_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<Uuid>,
    pub action: AccessAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    pub accessed_at: DateTime<Utc>,
}

impl FileAccessLog {
    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }


    // <<< CUSTOM METHODS START >>>
    // Add custom entity methods here
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for FileAccessLog {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "FileAccessLog"
    }
}
