use crate::errors::AppError;
use actix_web::{HttpResponse, Responder};
use tracing::instrument;

// Route: GET "/health_check"
#[instrument]
pub async fn health_check() -> Result<impl Responder, AppError> {
    error!("In health check");
    Ok(HttpResponse::Ok().finish())
}
