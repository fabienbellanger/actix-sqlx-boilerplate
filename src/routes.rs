//! List all server routes

use crate::handlers;
use actix_web::web;

/// Defines Web's routes
pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/health-check", web::get().to(handlers::web::health_check))
        .route("/ws-client", web::get().to(handlers::web::ws_client))
        .route("/async-process", web::get().to(handlers::web::async_process))
        .route("/ws", web::get().to(handlers::ws::index));
}

/// Defines API's routes
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/login", web::post().to(handlers::users::login))
            .route("/register", web::post().to(handlers::users::register))
            .service(
                web::scope("/users")
                    .wrap(crate::middlewares::auth::Authentication)
                    .route("", web::get().to(handlers::users::get_all))
                    .route("/{id}", web::get().to(handlers::users::get_by_id))
                    .route("/{id}", web::delete().to(handlers::users::delete))
                    .route("/{id}", web::put().to(handlers::users::update)),
            ),
    );
}
