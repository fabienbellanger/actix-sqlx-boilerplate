//! JWT middleware module

use crate::models::auth;
use crate::AppState;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    http::StatusCode,
    web::Data,
    Error, HttpResponse,
};
use color_eyre::Result;
use futures::{
    future::{ok, Ready},
    Future,
};
use sqlx::MySqlPool;
use std::pin::Pin;
use std::task::{Context, Poll};

const AUTHORIZATION: &str = "Authorization";

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut auth_success: bool = false;

        if Method::OPTIONS == *req.method() {
            auth_success = true;
        } else if let Some(app_state) = req.app_data::<Data<AppState>>() {
            let secret_key = &app_state.jwt_secret_key;
            if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                        let token = auth_str[6..auth_str.len()].trim();
                        if let Ok(claims) = auth::JWT::parse(token.to_owned(), secret_key.to_owned()) {
                            if let Some(pool) = req.app_data::<Data<MySqlPool>>() {
                                let _pool = pool.get_ref();
                                // TODO: Regarder : https://github.com/biluohc/actixweb-sqlx-jwt/blob/master/src/middlewares/auth.rs
                                // let user = UserRepository::get_by_id(_pool, claims.user_id).await;
                                // if let Ok(conn) = db::mysql_pool_handler(pool.clone()) {
                                //     let user = User::get_by_id(&conn, claims.user_id);
                                //     if user.is_ok() {
                                //         auth_success = true;
                                //     }
                                // }
                            }
                        } else {
                            error!("Failed to parse token: {}", token);
                        }
                        todo!("Add User model and database");
                    }
                }
            }
        }

        if auth_success {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(crate::errors::AppErrorMessage {
                            code: StatusCode::UNAUTHORIZED.as_u16(),
                            error: "Unauthorized".to_owned(),
                            message: "Unauthorized".to_owned(),
                        })
                        .into_body(),
                ))
            })
        }
    }
}
