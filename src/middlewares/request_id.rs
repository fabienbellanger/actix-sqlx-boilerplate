//! Request ID middleware module

use std::pin::Pin;
use std::task::{Context, Poll};

use actix_http::http::header;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{Payload, ServiceRequest, ServiceResponse},
    Error, FromRequest, HttpMessage, HttpRequest,
};
use color_eyre::Result;
use futures::future::{ok, Ready};
use futures::Future;
use uuid::Uuid;

// Inspired from https://github.com/pastjean/actix-web-requestid/blob/master/src/lib.rs
trait RequestIdMessage {
    fn id(&self) -> String;
}

/// The extractor type to obtain your Request ID from a request.
///
/// ```rust
/// use actix_web::*;
/// use actix_sqlx_boilerplate::middlewares::request_id::RequestId;
///
/// async fn index(request_id: RequestId) -> String {
///         format!("Request ID: {}", request_id.get())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct RequestId(HttpRequest);

impl RequestId {
    pub fn get(&self) -> String {
        self.id()
    }
}

impl RequestIdMessage for RequestId {
    fn id(&self) -> String {
        self.0.id()
    }
}

impl<T> RequestIdMessage for T
where
    T: HttpMessage,
{
    fn id(&self) -> String {
        if let Some(id) = self.extensions().get::<String>() {
            return id.clone();
        }

        let id: String = Uuid::new_v4().to_string();

        self.extensions_mut().insert(id.clone());

        id
    }
}

/// Extractor implementation for RequestId type.
impl FromRequest for RequestId {
    type Error = Error;
    type Future = Ready<Result<RequestId, Error>>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ok(RequestId(req.clone()))
    }
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct RequestIdService;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for RequestIdService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestIdServiceMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestIdServiceMiddleware { service })
    }
}

pub struct RequestIdServiceMiddleware<S> {
    service: S,
}

impl<S, B> Service for RequestIdServiceMiddleware<S>
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
        let request_id = req.id(); // ie. RequestID(req).id();
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if let Ok(name) = header::HeaderName::from_lowercase(b"x-request-id") {
                if let Ok(value) = header::HeaderValue::from_str(&request_id) {
                    res.headers_mut().insert(name, value);
                }
            }

            Ok(res)
        })
    }
}
