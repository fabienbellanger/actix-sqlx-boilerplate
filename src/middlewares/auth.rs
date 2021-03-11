//! JWT middleware module

use crate::models::auth;
use crate::repositories::user::UserRepository;
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
use std::task::{Context, Poll};
use std::{cell::RefCell, pin::Pin, rc::Rc};

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
        let mut is_authorized = false;
        let mut user_id = String::new();

        if Method::OPTIONS == *req.method() {
            is_authorized = true;
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

            is_authorized = match token {
                Some(token) => {
                    let claims = auth::JWT::parse(token.to_owned(), secret_key.to_owned());
                    match claims {
                        Ok(claims) => {
                            user_id = claims.user_id;
                            true
                        }
                        _ => false,
                    }
                }
                _ => false,
            };
        }

        Box::pin(async move {
            if is_authorized {
                // Check if user is still valid
                is_authorized = match req.app_data::<Data<MySqlPool>>() {
                    Some(pool) => match UserRepository::get_by_id(pool.get_ref(), user_id).await {
                        Ok(user) => user.is_some(),
                        _ => false,
                    },
                    None => false,
                };
            }

            if is_authorized {
                service_cloned.call(req).await
            } else {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(crate::errors::AppErrorMessage {
                            code: StatusCode::UNAUTHORIZED.as_u16(),
                            message: "Unauthorized".to_owned(),
                        })
                        .into_body(),
                ))
            }
        })
    }
}
