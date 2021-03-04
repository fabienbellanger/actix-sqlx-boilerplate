//! Errors handlers module

use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{body::Body, body::ResponseBody, dev, http};
use actix_web::{error, http::StatusCode};
use color_eyre::Result;
use serde_json::json;

fn render_error<B>(
    mut res: dev::ServiceResponse<B>,
    code: u16,
    error: String,
    message: String,
) -> ErrorHandlerResponse<B> {
    let err = json!(crate::errors::AppErrorMessage { code, error, message });

    res.request();
    res.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    res = res.map_body(|_, _| ResponseBody::Body(Body::from(err)).into_body());

    ErrorHandlerResponse::Response(res)
}

/// Render 401 error
pub fn render_401<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::UNAUTHORIZED.as_u16(),
        String::from("Unauthorized"),
        "Unauthorized".to_owned(),
    ))
}

/// Render 403 error
pub fn render_403<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::FORBIDDEN.as_u16(),
        String::from("Forbidden"),
        "Forbidden".to_owned(),
    ))
}

/// Render 408 error
pub fn render_408<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::REQUEST_TIMEOUT.as_u16(),
        String::from("Request Time-out"),
        "Request Time-out".to_owned(),
    ))
}

/// Render 502 error
pub fn render_502<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::BAD_GATEWAY.as_u16(),
        String::from("Bad Gateway"),
        "Bad Gateway".to_owned(),
    ))
}

/// Render 503 error
pub fn render_503<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        String::from("Service Unavailable"),
        "Service Unavailable".to_owned(),
    ))
}

/// Render 504 error
pub fn render_504<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::GATEWAY_TIMEOUT.as_u16(),
        String::from("Gateway Time-out"),
        "Gateway Time-out".to_owned(),
    ))
}
