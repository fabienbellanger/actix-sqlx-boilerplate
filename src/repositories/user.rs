use crate::models::user::{Login, User};
use futures::stream::BoxStream;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use sha2::{Digest, Sha512};
use chrono::{TimeZone, Utc};

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

    pub async fn login(pool: &MySqlPool, input: Login) -> Result<User, sqlx::Error> {
        let hashed_password = format!("{:x}", Sha512::digest(&input.password.as_bytes()));
        let result = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE email = ?
                    AND password = ?
                    AND deleted_at IS NULL
            "#, input.email, hashed_password
        )
        .fetch_one(pool).await?;

        Ok(User{
            id: result.id,
            password: result.password,
            lastname: result.lastname,
            firstname: result.firstname,
            email: result.email,
            created_at: Utc.from_utc_datetime(&result.created_at),
            updated_at: Utc.from_utc_datetime(&result.updated_at),
            deleted_at: None,
        })
    }
}
