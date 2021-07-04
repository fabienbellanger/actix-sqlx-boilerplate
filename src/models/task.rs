//! Task model module

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Task {
    fn _init(
        id: String,
        name: String,
        description: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn new(task: TaskCreation) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: task.name,
            description: task.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct TaskCreation {
    pub name: String,
    pub description: Option<String>,
}
