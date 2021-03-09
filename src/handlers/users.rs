use crate::errors::AppError;
use crate::models::auth::JWT;
use crate::models::user::{Login, LoginResponse, User};
use crate::repositories::user::UserRepository;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use futures::TryStreamExt;
use sqlx::MySqlPool;

// Route: POST "/v1/login"
pub async fn login(
    pool: web::Data<MySqlPool>,
    data: web::Data<AppState>,
    form: web::Json<Login>,
) -> Result<impl Responder, AppError> {
    let user = UserRepository::login(pool.get_ref(), form.into_inner()).await?;

    // Génération du token
    // -------------------
    let secret = &data.jwt_secret_key;
    let jwt_lifetime = data.jwt_lifetime;
    let token = JWT::generate(
        user.id.to_owned(),
        user.lastname.to_owned(),
        user.firstname.to_owned(),
        user.email.to_owned(),
        secret.to_owned(),
        jwt_lifetime,
    );

    match token {
        Ok(token) => {
            let expires_at = NaiveDateTime::from_timestamp(token.1, 0);
            let expires_at: DateTime<Utc> = DateTime::from_utc(expires_at, Utc);

            Ok(HttpResponse::Ok().json(LoginResponse {
                id: user.id.to_owned(),
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

// Route: GET "/v1/users"
pub async fn get_all(pool: web::Data<MySqlPool>) -> Result<impl Responder, AppError> {
    let mut stream = UserRepository::get_all(pool.get_ref());
    let mut users: Vec<User> = Vec::new();
    while let Some(row) = stream.try_next().await? {
        users.push(row?);
    }

    Ok(HttpResponse::Ok().json(users))
}

// Route: GET "/v1/users/{id}"
pub async fn get_by_id(
    pool: web::Data<MySqlPool>,
    web::Path(id): web::Path<String>,
) -> Result<impl Responder, AppError> {
    let user = UserRepository::get_by_id(pool.get_ref(), id).await?;

    Ok(HttpResponse::Ok().json(user))
}
