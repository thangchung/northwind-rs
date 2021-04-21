//! Web handlers module

use crate::{middlewares::request_id::RequestId};
use crate::errors::ApiError;
use actix_web::{HttpResponse, Responder, web};

// Route: GET "/health-check"
pub async fn health_check(request_id: RequestId) -> Result<impl Responder, ApiError> {
    debug!("Request ID: {}", request_id.get());
    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthz", web::get().to(health_check));
}
