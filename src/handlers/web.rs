//! Web handlers module

use crate::{errors::AppError, middlewares::request_id::RequestId};
use actix_web::{HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};

#[derive(Template)]
#[template(path = "ws.html")]
struct WsTemplate {}

// Route: GET "/health_check"
pub async fn health_check(request_id: RequestId) -> Result<impl Responder, AppError> {
    debug!("Request ID: {}", request_id.get());
    Ok(HttpResponse::Ok().finish())
}

// Route: GET "/ws-client"
pub async fn ws_client() -> Result<HttpResponse, AppError> {
    WsTemplate {}.into_response().map_err(|e| {
        error!("{}", e);
        AppError::InternalError {
            message: "Failed to load HelloTemplate.".to_owned(),
        }
    })
}
