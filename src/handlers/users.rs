use crate::AppState;
use crate::errors::AppError;
use crate::models::user::{User, Login, LoginResponse};
use crate::models::auth::JWT;
use crate::repositories::user::UserRepository;
use actix_web::{web, HttpResponse, Responder};
use futures::TryStreamExt;
use sqlx::MySqlPool;
use tracing::instrument;
use chrono::{Utc, NaiveDateTime, DateTime, SecondsFormat};

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

// #[instrument]
pub async fn login(pool: web::Data<MySqlPool>, data: web::Data<AppState>, form: web::Json<Login>,) -> Result<impl Responder, AppError> {
    let user = UserRepository::login(pool.get_ref(), form.into_inner()).await?;

    // Génération du token
    // -------------------
    let secret = &data.jwt_secret_key;
    let token = JWT::generate(
        user.id.to_owned(),
        user.lastname.to_owned(),
        user.firstname.to_owned(),
        user.email.to_owned(),
        secret.to_owned(),
    );

    match token {
        Ok(token) => {
            let expires_at = NaiveDateTime::from_timestamp(token.1, 0);
            let expires_at: DateTime<Utc> = DateTime::from_utc(expires_at, Utc);

            Ok(HttpResponse::Ok().json(LoginResponse {
                lastname: user.lastname.to_owned(),
                firstname: user.firstname.to_owned(),
                email: user.email,
                token: token.0,
                expires_at: expires_at.to_rfc3339_opts(SecondsFormat::Secs, true), // format("%Y-%m-%d %H:%M:%S").to_string(),
            }))
        }
        _ => Err(AppError::Unauthorized {}),
    }
}
