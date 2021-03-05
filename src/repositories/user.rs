use crate::models::user::User;
use futures::stream::BoxStream;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};

pub struct UserRepository;

impl UserRepository {
    pub fn get_all(pool: &MySqlPool) -> BoxStream<Result<User, sqlx::Error>> {
        sqlx::query(
            r#"
            SELECT * FROM users WHERE deleted_at IS NULL
        "#,
        )
        .map(|row: MySqlRow| User {
            // TODO: Try to use try_get to avoid panic!
            id: row.get(0),
            lastname: row.get(1),
            firstname: row.get(2),
            email: row.get(3),
            password: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
            deleted_at: row.get(7),
        })
        .fetch(pool)
    }
}
