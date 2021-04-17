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
extern crate log;

use crate::config::Config;
use crate::ws::chat::server;
use actix::Actor;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use color_eyre::Result;
use sqlx::{Postgres, Pool};

#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret_key: String,
    pub jwt_lifetime: i64,
}

pub async fn run(settings: Config, db_pool: Pool<Postgres>) -> Result<()> {
    // Logger
    // ------
    logger::init(settings.rust_log);

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

    // Start server
    // ------------
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(data.clone())
            .data(chat_server.clone())
            .wrap(middlewares::request_id::RequestIdService)
            .wrap(middlewares::timer::Timer)
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a | %{x-request-id}o"))
            .wrap(prometheus.clone())
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::UNAUTHORIZED, handlers::errors::render_401)
                    .handler(http::StatusCode::FORBIDDEN, handlers::errors::render_403)
                    .handler(http::StatusCode::REQUEST_TIMEOUT, handlers::errors::render_408)
                    .handler(http::StatusCode::BAD_GATEWAY, handlers::errors::render_502)
                    .handler(http::StatusCode::SERVICE_UNAVAILABLE, handlers::errors::render_503)
                    .handler(http::StatusCode::GATEWAY_TIMEOUT, handlers::errors::render_504),
            )
            .wrap(
                Cors::new()
                    // .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .configure(routes::web)
            .configure(routes::api)
            .service(fs::Files::new("/assets", "./static"))
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await?;

    Ok(())
}