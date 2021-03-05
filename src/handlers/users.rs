use crate::errors::AppError;
use crate::models::user::User;
use crate::repositories::user::UserRepository;
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use futures::TryStreamExt;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use tracing::instrument;

// Route: GET "/v1/users"
#[instrument]
pub async fn get_all(pool: web::Data<MySqlPool>) -> Result<impl Responder, AppError> {
    let mut stream = UserRepository::get_all(pool.get_ref());
    let mut users: Vec<User> = Vec::new();
    while let Some(row) = stream.try_next().await? {
        users.push(row);
    }

    Ok(HttpResponse::Ok().json(users))
}
