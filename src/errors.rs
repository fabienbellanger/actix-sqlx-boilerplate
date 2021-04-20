//! Custom error module

use actix::MailboxError;
use actix_http::ResponseBuilder;
use actix_web::{error::ResponseError, http::header, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

/// Represents the custom error message
#[derive(Serialize)]
pub struct AppErrorMessage {
    pub code: u16,
    pub message: String,
}

/// Defines available errors
#[derive(Display, Debug, Error)]
pub enum AppError {
    #[display(fmt = "{}", message)]
    InternalError { message: String },

    #[display(fmt = "{}", message)]
    BadRequest { message: String },

    #[display(fmt = "{}", message)]
    NotFound { message: String },

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl AppError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound { message: m } => m.to_owned(),
            Self::BadRequest { message: m } => m.to_owned(),
            Self::InternalError { message: m } => m.to_owned(),
            Self::Unauthorized => "Unauthorized".to_owned(),
        }
    }
}

// Actix-web errors
// ----------------
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(AppErrorMessage {
                code: self.status_code().as_u16(),
                message: self.to_string(),
            })
    }
}

// SQLx errors
// -----------
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => Self::InternalError {
                message: "Database Error".to_owned(),
            },
        }
    }
}

// Actix errors
// ------------
impl From<MailboxError> for AppError {
    fn from(error: MailboxError) -> Self {
        match error {
            _ => Self::InternalError {
                message: "Actor Error".to_owned(),
            },
        }
    }
}
