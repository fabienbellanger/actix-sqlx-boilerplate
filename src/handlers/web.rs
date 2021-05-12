//! Web handlers module

use crate::actors::cache::*;
use crate::{errors::AppError, middlewares::request_id::RequestId};
use actix::Addr;
use actix::Arbiter;
use actix_web::{web, HttpResponse, Responder};
use std::time::Duration;
use tokio::time::delay_for;

// Route: GET "/health-check"
pub async fn health_check(request_id: RequestId) -> Result<impl Responder, AppError> {
    debug!("Request ID: {}", request_id.get());

    Ok(HttpResponse::Ok().finish())
}

// Route: GET "/actor-cache/{item}"
pub async fn actor_cache(
    web::Path(item): web::Path<String>,
    list: web::Data<Addr<Cache>>,
) -> Result<impl Responder, AppError> {
    // Send a message to the actor to push String in the list
    list.send(AddMessage(item)).await?;

    // Send a message to the actor to get string list
    let item = list.send(CacheMessage {}).await?;

    Ok(HttpResponse::Ok().json(item))
}

// Long task that waits for 5s
async fn long_task() {
    delay_for(Duration::from_secs(10)).await;
    debug!("Inside long task...");
}

// Route: GET "/async-process"
pub async fn async_process() -> Result<HttpResponse, AppError> {
    debug!("Before long task...");
    Arbiter::spawn(long_task());
    debug!("After long task...");

    // Immediate response
    Ok(HttpResponse::Ok().finish())
}
