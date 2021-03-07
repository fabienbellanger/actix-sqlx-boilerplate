//! List all server routes

use crate::handlers;
use actix_web::web;

/// Defines Web's routes
pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/health-check", web::get().to(handlers::web::health_check));
}

/// Defines API's routes
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1")
        .route("/login", web::post().to(handlers::users::login))
        .service(
            web::scope("/users")
                // .wrap(middlewares::auth::Authentication)
                .route("", web::get().to(handlers::users::get_all))
        ),
    );
}
