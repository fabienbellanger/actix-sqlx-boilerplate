mod config;
mod middlewares;

extern crate chrono;
extern crate serde;

use crate::config::Config;
use actix_cors::Cors;
// use actix_web::middleware::errhandlers::ErrorHandlers;
// use actix_web::middleware::Logger;
use actix_web::{http, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use color_eyre::Result;

#[derive(Debug, Clone)]
pub struct AppState {}

pub async fn run() -> Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::from_env().expect("Cannot find or invalid .env file");
    // let db_url = settings.database_url;
    // let jwt_secret_key = settings.jwt_secret_key;
    // let github_api_username = settings.github_api_username;
    // let github_api_token = settings.github_api_token;

    // Installation de Color Eyre
    // --------------------------
    color_eyre::install()?;

    // Logger
    // ------
    // logger::init(settings.server_log_level);

    // Initialisation du state de l'application
    // ----------------------------------------
    let data = AppState {};

    // Initialisation du pool MySQL via r2d2
    // -------------------------------------
    // let pool = db::init(&db_url).expect("Failed to create MySQL pool.");

    // Prometheus
    // ----------
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    // Start server
    // ------------
    HttpServer::new(move || {
        App::new()
            // .data(pool.clone())
            .data(data.clone())
            .wrap(prometheus.clone())
            .wrap(middlewares::timer::Timer)
            .wrap(middlewares::request_id::RequestId)
            // .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .wrap(
                Cors::new()
                    // .allowed_origin("*")
                    .allowed_methods(vec![
                        "GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "OPTIONS",
                    ])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await?;

    Ok(())
}
