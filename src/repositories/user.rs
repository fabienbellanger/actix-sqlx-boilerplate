use crate::models::user::{Login, User};
use chrono::{TimeZone, Utc};
use futures::stream::BoxStream;
use sha2::{Digest, Sha512};
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};

pub struct UserRepository;

impl UserRepository {
    /// Returns a User if credentials are right
    pub async fn login(pool: &MySqlPool, input: Login) -> Result<User, sqlx::Error> {
        let hashed_password = format!("{:x}", Sha512::digest(&input.password.as_bytes()));
        let result = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE email = ?
                    AND password = ?
                    AND deleted_at IS NULL
            "#,
            input.email,
            hashed_password
        )
        .fetch_one(pool)
        .await?;

        Ok(User::init(
            result.id,
            result.password,
            result.lastname,
            result.firstname,
            result.email,
            Utc.from_utc_datetime(&result.created_at),
            Utc.from_utc_datetime(&result.updated_at),
            match result.deleted_at {
                None => None,
                Some(d) => Some(Utc.from_utc_datetime(&d)),
            },
        ))
    }

    /// Returns all users not deleted
    pub fn get_all(pool: &MySqlPool) -> BoxStream<Result<Result<User, sqlx::Error>, sqlx::Error>> {
        sqlx::query(r#"SELECT * FROM users WHERE deleted_at IS NULL"#)
            .map(|row: MySqlRow| {
                Ok(User {
                    id: row.try_get(0)?,
                    lastname: row.try_get(1)?,
                    firstname: row.try_get(2)?,
                    email: row.try_get(3)?,
                    password: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                    deleted_at: row.try_get(7)?,
                })
            })
            .fetch(pool)
    }

    /// Returns a user by its ID
    /// TODO: faire comme get_all pour que cela fonctionne dans le middleware d'auth
    pub async fn get_by_id(pool: &MySqlPool, id: String) -> Result<User, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE id = ?
                    AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(User::init(
            result.id,
            result.password,
            result.lastname,
            result.firstname,
            result.email,
            Utc.from_utc_datetime(&result.created_at),
            Utc.from_utc_datetime(&result.updated_at),
            match result.deleted_at {
                None => None,
                Some(d) => Some(Utc.from_utc_datetime(&d)),
            },
        ))
    }
}
