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
use std::{cell::RefCell, pin::Pin, rc::Rc};
use std::task::{Context, Poll};
use crate::repositories::user::UserRepository;

pub struct Authentication;

impl<S: 'static, B> Transform<S> for Authentication
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
        ok(AuthenticationMiddleware { 
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
        let mut service_cloned = self.service.clone();
        let mut is_token_valid = false;
        let mut user_id = String::new();

        if Method::OPTIONS == *req.method() {
            is_token_valid = true;
        } else if let Some(app_state) = req.app_data::<Data<AppState>>() {
            let secret_key = &app_state.jwt_secret_key;
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| {
                    let words = h.split("Bearer").collect::<Vec<&str>>();
                    words.get(1).map(|w| w.trim())
                });

            is_token_valid = match token {
                Some(token) => {
                    let claims = auth::JWT::parse(token.to_owned(), secret_key.to_owned());
                    match claims {
                        Ok(claims) => {
                            user_id = claims.user_id;
                            true
                        },
                        _ => false,
                    }
                },
                _ => false,
            };
        }

        if is_token_valid {
            Box::pin(async move {
                // Check if user is still valid
                let pool = req.app_data::<Data<MySqlPool>>();
                let ok = match pool {
                    Some(pool) => UserRepository::get_by_id(pool.get_ref(), user_id).await.is_ok(),
                    None => false,
                };

                if ok {
                    service_cloned.call(req).await
                } else {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(crate::errors::AppErrorMessage {
                                code: StatusCode::UNAUTHORIZED.as_u16(),
                                error: "Unauthorized".to_owned(),
                                message: "Unauthorized".to_owned(),
                            })
                            .into_body(),
                    ))
                }
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
