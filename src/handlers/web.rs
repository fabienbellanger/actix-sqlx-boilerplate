//! Web handlers module

use crate::errors::AppError;
use actix_web::{HttpResponse, Responder};

// Route: GET "/health_check"
pub async fn health_check() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().finish())
}
