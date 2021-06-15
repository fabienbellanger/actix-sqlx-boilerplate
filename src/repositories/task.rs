use crate::models::task::Task;
// use chrono::{TimeZone, Utc};
use futures::stream::BoxStream;
use sqlx::mysql::{MySqlDone, MySqlRow};
use sqlx::{MySqlPool, Row};

pub struct TaskRepository;

impl TaskRepository {
    /// Add a new task
    pub async fn create(pool: &MySqlPool, task: &mut Task) -> Result<MySqlDone, sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO task (id, name, description, created_at, updated_at, deleted_at)
                VALUES (?, ?, ?, ?, ?, ?)
            "#,
            task.id,
            task.name,
            task.description,
            task.created_at,
            task.updated_at,
            task.deleted_at,
        )
        .execute(pool)
        .await
    }

    /// Returns all tasks not deleted
    pub fn get_all(pool: &MySqlPool) -> BoxStream<Result<Result<Task, sqlx::Error>, sqlx::Error>> {
        sqlx::query(r#"SELECT * FROM task WHERE deleted_at IS NULL"#)
            .map(|row: MySqlRow| {
                Ok(Task {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    created_at: row.try_get(3)?,
                    updated_at: row.try_get(4)?,
                    deleted_at: row.try_get(5)?,
                })
            })
            .fetch(pool)
    }
}
