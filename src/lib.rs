mod actors;
pub mod config;
mod errors;
pub mod handlers;
mod logger;
pub mod middlewares;
mod models;
mod repositories;
mod routes;
mod ws;

extern crate chrono;
extern crate serde;

#[macro_use]
extern crate tracing;

use crate::config::Config;
use crate::ws::chat::server;
use actix::{Actor, Arbiter};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use color_eyre::Result;
use sqlx::{MySql, Pool};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret_key: String,
    pub jwt_lifetime: i64,
}

pub async fn run(settings: Config, db_pool: Pool<MySql>) -> Result<()> {
    // Logger
    // ------
    //logger::init(settings.rust_log);
    // logger::init_tracing("trace".to_owned());
    let subscriber = logger::get_subscriber(settings.rust_log, std::io::stdout);
    logger::init_subscriber(subscriber);

    // Init application state
    // ----------------------
    let data = AppState {
        jwt_secret_key: settings.jwt_secret_key.clone(),
        jwt_lifetime: settings.jwt_lifetime,
    };

    // Prometheus
    // ----------
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    // Start chat server actor
    // -----------------------
    let chat_server = server::ChatServer::new().start();

    // Test of actor
    // -------------
    let cache_actor = actors::cache::Cache::default().start();
    Arbiter::spawn(actors::cache::cache_loop(cache_actor.clone(), Duration::from_secs(600)));

    // Start server
    // ------------
    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("*")
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
            
        App::new()
            .data(db_pool.clone())
            .data(data.clone())
            .data(chat_server.clone())
            .data(cache_actor.clone())
            .wrap(middlewares::request_id::RequestIdService)
            .wrap(middlewares::timer::Timer)
            .wrap(prometheus.clone()) // Put before logger (issue #39)
            .wrap(Logger::new("request_id=%{x-request-id}o, client_ip_address=%a, request_path=\"%r\", status_code=%s, elapsed_seconds=%T, user_agent=\"%{User-Agent}i\""))
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::UNAUTHORIZED, handlers::errors::render_401)
                    .handler(http::StatusCode::FORBIDDEN, handlers::errors::render_403)
                    .handler(http::StatusCode::REQUEST_TIMEOUT, handlers::errors::render_408)
                    .handler(http::StatusCode::BAD_GATEWAY, handlers::errors::render_502)
                    .handler(http::StatusCode::SERVICE_UNAVAILABLE, handlers::errors::render_503)
                    .handler(http::StatusCode::GATEWAY_TIMEOUT, handlers::errors::render_504),
            )
            .wrap(cors)
            .configure(routes::web)
            .configure(routes::api)
            .service(fs::Files::new("/assets", "./static"))
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await?;

    Ok(())
}
