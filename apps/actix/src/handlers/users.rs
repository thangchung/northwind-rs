//! API users handlers module

use crate::AppState;
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use actix_web_validator::Json;
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;
use northwind_user::domain::user::{Login, LoginResponse, UpdateUserModel, User, UserCreation};
use northwind_user::domain::jwt_processor::JwtProcessor;
use northwind_core::errors::AppError;
use northwind_user::repositories::user_repository_impl::UserRepositoryImpl;

// Route: POST "/v1/login"
pub async fn login(
    pool: web::Data<PgPool>,
    data: web::Data<AppState>,
    jwt_processor: web::Data<dyn JwtProcessor>,
    form: Json<Login>,
) -> Result<impl Responder, ApiError> {
    let user = UserRepositoryImpl::login(pool.get_ref(), form.into_inner()).await?;

    match user {
        None => Err(AppError::Unauthorized {}.into()),
        Some(user) => {
            // generate the token
            // -------------------
            let secret = &data.jwt_secret_key;
            let jwt_lifetime = data.jwt_lifetime;
            let token = jwt_processor.generate(
                user.id.to_owned(),
                user.lastname.to_owned(),
                user.firstname.to_owned(),
                user.email.to_owned(),
                String::from(secret.to_owned()),
                jwt_lifetime,
            );

            match token {
                Ok(token) => {
                    let expires_at = NaiveDateTime::from_timestamp(token.1, 0);
                    let expires_at: DateTime<Utc> = DateTime::from_utc(expires_at, Utc);

                    Ok(HttpResponse::Ok().json(LoginResponse {
                        id: user.id.to_owned().to_string(),
                        lastname: user.lastname.to_owned(),
                        firstname: user.firstname.to_owned(),
                        email: user.email,
                        token: token.0,
                        expires_at: expires_at.to_rfc3339_opts(SecondsFormat::Secs, true),
                    }))
                }
                _ => Err(AppError::Unauthorized {}.into()),
            }
        }
    }
}

// Route: POST "/v1/register"
pub async fn register(pool: web::Data<PgPool>, form: Json<UserCreation>) -> Result<impl Responder, ApiError> {
    let mut user = User::new(form.0);
    let result = UserRepositoryImpl::create(pool.get_ref(), &mut user).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::InternalError {
            message: String::from("Error during user creation"),
        }
        .into()),
    }
}

// Route: GET "/v1/users"
pub async fn get_all(pool: web::Data<PgPool>) -> Result<impl Responder, ApiError> {
    let users = UserRepositoryImpl::get_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(users))
}

// Route: GET "/v1/users/{id}"
pub async fn get_by_id(pool: web::Data<PgPool>, web::Path(id): web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    let user = UserRepositoryImpl::get_by_id(pool.get_ref(), id).await?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::NotFound {
            message: String::from("No user found"),
        }
        .into()),
    }
}

// Route: DELETE "/v1/users/{id}"
pub async fn delete(pool: web::Data<PgPool>, web::Path(id): web::Path<Uuid>) -> Result<impl Responder, ApiError> {
    let result = UserRepositoryImpl::delete(pool.get_ref(), id).await;
    match result {
        Ok(result) => {
            if result.unwrap() == 1 {
                Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
            } else {
            Err(AppError::InternalError {
            message: String::from("No user or user already deleted"),
            }
            .into())
            }
        }
        _ => Err(AppError::InternalError {
            message: String::from("Error during user deletion"),
        }
        .into()),
    }
}

// Route: PUT "/v1/users/{id}"
pub async fn update(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<Uuid>,
    form: Json<UpdateUserModel>,
) -> Result<impl Responder, ApiError> {
    UserRepositoryImpl::update(pool.get_ref(), id.clone(), &form.0).await?;

    let user = UserRepositoryImpl::get_by_id(pool.get_ref(), id).await?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::NotFound {
            message: String::from("No user found"),
        }
        .into()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(crate::handlers::users::login))
        .route("/register", web::post().to(crate::handlers::users::register));
}

pub fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(crate::handlers::users::get_all))
        .route("/{id}", web::get().to(crate::handlers::users::get_by_id))
        .route("/{id}", web::delete().to(crate::handlers::users::delete))
        .route("/{id}", web::put().to(crate::handlers::users::update));
}
