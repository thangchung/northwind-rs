//! API users handlers module

use crate::AppState;
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use actix_web_validator::Json;
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use futures::TryStreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use northwind_user::repositories::user::UserRepository;

use northwind_user::services::jwt_service::Jwt;
use northwind_domain::authn::models::user::{Login, LoginResponse, UpdateUserModel, User, UserCreation};
use northwind_domain::authn::services::jwt_processor::JwtProcessor;
use crate::errors::AppError;

// Route: POST "/v1/login"
pub async fn login(
    pool: web::Data<PgPool>,
    data: web::Data<AppState>,
    form: Json<Login>,
) -> Result<impl Responder, AppError> {
    let user = UserRepository::login(pool.get_ref(), form.into_inner()).await?;

    match user {
        None => Err(AppError::Unauthorized {}),
        Some(user) => {
            // generate the token
            // -------------------
            let secret = &data.jwt_secret_key;
            let jwt_lifetime = data.jwt_lifetime;
            let token = Jwt::generate(
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
                _ => Err(AppError::Unauthorized {}),
            }
        }
    }
}

// Route: POST "/v1/register"
pub async fn register(pool: web::Data<PgPool>, form: Json<UserCreation>) -> Result<impl Responder, AppError> {
    let mut user = User::new(form.0);
    let result = UserRepository::create(pool.get_ref(), &mut user).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::InternalError {
            message: String::from("Error during user creation"),
        }),
    }
}

// Route: GET "/v1/users"
pub async fn get_all(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let mut stream = UserRepository::get_all(pool.get_ref());
    let mut users: Vec<User> = Vec::new();
    while let Some(row) = stream.try_next().await? {
        users.push(row?);
    }

    Ok(HttpResponse::Ok().json(users))
}

// Route: GET "/v1/users/{id}"
pub async fn get_by_id(pool: web::Data<PgPool>, web::Path(id): web::Path<Uuid>) -> Result<impl Responder, AppError> {
    let user = UserRepository::get_by_id(pool.get_ref(), id).await?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::NotFound {
            message: String::from("No user found"),
        }),
    }
}

// Route: DELETE "/v1/users/{id}"
pub async fn delete(pool: web::Data<PgPool>, web::Path(id): web::Path<Uuid>) -> Result<impl Responder, AppError> {
    let result = UserRepository::delete(pool.get_ref(), id).await;
    match result {
        Ok(result) => {
            if result.rows_affected() == 1 {
                Ok(HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish())
            } else {
                Err(AppError::InternalError {
                    message: String::from("No user or user already deleted"),
                })
            }
        }
        _ => Err(AppError::InternalError {
            message: String::from("Error during user deletion"),
        }),
    }
}

// Route: PUT "/v1/users/{id}"
pub async fn update(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<Uuid>,
    form: Json<UpdateUserModel>,
) -> Result<impl Responder, AppError> {
    UserRepository::update(pool.get_ref(), id.clone(), &form.0).await?;

    let user = UserRepository::get_by_id(pool.get_ref(), id).await?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        _ => Err(AppError::NotFound {
            message: String::from("No user found"),
        }),
    }
}
