//! Web handlers module

use crate::{errors::AppError, middlewares::request_id::RequestId};
use actix_web::{HttpResponse, Responder};

// Route: GET "/health-check"
pub async fn health_check(request_id: RequestId) -> Result<impl Responder, AppError> {
    debug!("Request ID: {}", request_id.get());
    Ok(HttpResponse::Ok().finish())
}
