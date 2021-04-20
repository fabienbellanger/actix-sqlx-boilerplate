//! Web handlers module

use crate::actors::list::*;
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

// Route: GET "/list-actor/{item}"
pub async fn list_actor(
    web::Path(item): web::Path<String>,
    list: web::Data<Addr<StringList>>,
) -> Result<impl Responder, AppError> {
    // Send a message to the actor to push String in the list
    list.send(AddMessage(item)).await?;

    // Send a message to the actor to get string list
    let l = list.send(StringListMessage {}).await?;

    Ok(HttpResponse::Ok().json(l))
}

// Long task that waits for 5s
async fn long_task() {
    delay_for(Duration::from_secs(5)).await;
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
