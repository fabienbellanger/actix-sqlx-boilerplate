//! Web handlers module

use crate::{errors::AppError, middlewares::request_id::RequestId};
use actix::Arbiter;
use actix_web::{HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};

#[derive(Template)]
#[template(path = "ws.html")]
struct WsTemplate {}

// Route: GET "/health-check"
pub async fn health_check(request_id: RequestId) -> Result<impl Responder, AppError> {
    debug!("Request ID: {}", request_id.get());
    Ok(HttpResponse::Ok().finish())
}

async fn long_task() {
    debug!("INSIDE LONG TASK...");
}

// Route: GET "/async-process"
pub async fn async_process() -> Result<HttpResponse, AppError> {
    debug!("BEFORE LONG TASK...");
    Arbiter::spawn(long_task());
    debug!("AFTER LONG TASK...");

    debug!("BEFORE LONG TASK...");
    Arbiter::spawn_fn(|| async {
        debug!("INSIDE CLOSURE LONG TASK...");
    });
    debug!("AFTER LONG TASK...");

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
