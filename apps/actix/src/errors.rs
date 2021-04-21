//! Custom error module

use actix_http::ResponseBuilder;
use actix_web::{error::ResponseError, http::header, http::StatusCode, HttpResponse};
use northwind_user::errors::{AppError, AppErrorMessage};

#[derive(Debug)]
pub struct ApiError(AppError);

impl From<AppError> for ApiError {
    fn from(error: AppError) -> Self {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
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

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => AppError::InternalError {
                message: "Database Error".to_owned(),
            }.into(),
        }
    }
}
