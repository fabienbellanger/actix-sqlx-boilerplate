use crate::errors::AppError;
use crate::models::auth::JWT;
use crate::models::user::{Login, LoginResponse, User, UserCreation};
use crate::repositories::user::UserRepository;
use crate::AppState;
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use futures::TryStreamExt;
use sqlx::{Done, MySqlPool};

// Route: POST "/v1/login"
pub async fn login(
    pool: web::Data<MySqlPool>,
    data: web::Data<AppState>,
    form: web::Json<Login>,
) -> Result<impl Responder, AppError> {
    let user = UserRepository::login(pool.get_ref(), form.into_inner()).await?;

    match user {
        None => Err(AppError::Unauthorized {}),
        Some(user) => {
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
                        expires_at: expires_at.to_rfc3339_opts(SecondsFormat::Secs, true),
                    }))
                }
                _ => Err(AppError::Unauthorized {}),
            }
        }
    }
}

// Route: POST "/v1/register"
// TODO: Make a validation of UserCreation fileds
pub async fn register(pool: web::Data<MySqlPool>, form: web::Json<UserCreation>) -> Result<impl Responder, AppError> {
    let mut user = User::new(form.0);
    let result = UserRepository::create(pool.get_ref(), &mut user).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::InternalError {
            message: String::from("Error during user creation"),
        }),
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
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::NotFound {
            message: String::from("No user found"),
        }),
    }
}

// Route: DELETE "/v1/users/{id}"
pub async fn delete(pool: web::Data<MySqlPool>, web::Path(id): web::Path<String>) -> Result<impl Responder, AppError> {
    let result = UserRepository::delete(pool.get_ref(), id).await;
    match result {
        Ok(result) => {
            if result.rows_affected() == 1 {
                Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
            } else {
                Err(AppError::InternalError {
                    message: String::from("No user or user already deleted"),
                })
            }
        }
        _ => Err(AppError::InternalError {
            message: String::from("Error during user deletion"),
        }),
    }
}

// Route: PUT "/v1/users/{id}"
pub async fn update(
    pool: web::Data<MySqlPool>,
    web::Path(id): web::Path<String>,
    form: web::Json<UserCreation>,
) -> Result<impl Responder, AppError> {
    UserRepository::update(pool.get_ref(), id.clone(), &form.0).await?;

    let user = UserRepository::get_by_id(pool.get_ref(), id).await?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::NotFound {
            message: String::from("No user found"),
        }),
    }
}
