use crate::errors::AppError;
use actix_web::{HttpResponse, Responder};

// Route: GET "/health_check"
pub async fn health_check() -> Result<impl Responder, AppError> {
    error!("In health check");
    Ok(HttpResponse::Ok().finish())
}
