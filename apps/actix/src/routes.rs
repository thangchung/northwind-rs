//! List all server routes

use crate::handlers;
use actix_web::web;

/// Defines Web's routes
pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthz", web::get().to(handlers::web::health_check));
}

/// Defines API's routes
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/login", web::post().to(crate::handlers::users::login))
            .route("/register", web::post().to(crate::handlers::users::register))
            .service(
                web::scope("/users")
                    .wrap(crate::middlewares::auth::Authentication)
                    .route("", web::get().to(crate::handlers::users::get_all))
                    .route("/{id}", web::get().to(crate::handlers::users::get_by_id))
                    .route("/{id}", web::delete().to(crate::handlers::users::delete))
                    .route("/{id}", web::put().to(crate::handlers::users::update)),
            ),
    );
}
