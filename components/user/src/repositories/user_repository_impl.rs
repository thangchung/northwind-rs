use chrono::Utc;
use sha2::{Digest, Sha512};
use sqlx::{PgPool};
use uuid::Uuid;
use crate::domain::user::{Login, User, UpdateUserModel};
use northwind_core::errors::AppError;

pub struct UserRepositoryImpl;

impl UserRepositoryImpl {
    /// Returns a User if credentials are right
    pub async fn login(pool: &PgPool, input: Login) -> Result<Option<User>, AppError> {
        let hashed_password = format!("{:x}", Sha512::digest(&input.password.as_bytes()));
        let result = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE email = $1 AND
                    password = $2 AND
                    deleted_at IS NULL
            "#,
            input.email,
            hashed_password
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| -> AppError { e.into() })?;

        match result {
            Some(result) => Ok(Some(User::init(
                result.id,
                result.password,
                result.lastname,
                result.firstname,
                result.email,
                result.created_at,
                result.updated_at,
                result.deleted_at.into(),
            ))),
            None => Ok(None),
        }
    }

    /// Add a new user
    pub async fn create(pool: &PgPool, user: &mut User) -> Result<Option<u64>, AppError> {
        user.password = format!("{:x}", Sha512::digest(&user.password.as_bytes()));

        let affected_rows = sqlx::query!(
            r#"
                INSERT INTO users (id, lastname, firstname, email, password, created_at, updated_at, deleted_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.lastname,
            user.firstname,
            user.email,
            user.password,
            user.created_at,
            user.updated_at,
            user.deleted_at,
        )
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
        .map_err(|e| -> AppError { e.into() });

        match affected_rows {
            Ok(r) => Ok(Some(r)),
            Err(e) => Err(AppError::from(e))
        }
    }

    /// Returns all users not deleted
    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(User, r#"SELECT * FROM users WHERE deleted_at IS NULL"#)
            .fetch_all(pool)
            .await
            .map_err(|e| -> AppError { e.into() })
    }

    /// Returns a user by its ID
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
        let result = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE id = $1
                    AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        match result {
            Some(result) => Ok(Some(User::init(
                result.id,
                result.lastname,
                result.firstname,
                result.email,
                result.password,
                result.created_at,
                result.updated_at,
                result.deleted_at.into(),
            ))),
            None => Ok(None),
        }
    }

    /// Delete a user
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Option<u64>, AppError> {
        let affected_rows = sqlx::query!(
            r#"
                UPDATE users
                SET deleted_at = $1
                WHERE id = $2
            "#,
            Utc::now().naive_utc(),
            id
        )
        .execute(pool)
        .await
            .map(|r| r.rows_affected())
        .map_err(|e| -> AppError { e.into() });

        match affected_rows {
            Ok(r) => Ok(Some(r)),
            Err(e) => Err(AppError::from(e))
        }
    }

    /// Update a user
    pub async fn update(pool: &PgPool, id: Uuid, user: &UpdateUserModel) -> Result<Option<u64>, AppError> {
        let affected_rows = sqlx::query!(
            r#"
                UPDATE users
                SET lastname = $1, firstname = $2, updated_at = $3
                WHERE id = $4
            "#,
            user.lastname,
            user.firstname,
            Utc::now().naive_utc(),
            id
        )
        .execute(pool)
        .await
            .map(|r| r.rows_affected())
        .map_err(|e| -> AppError { e.into() });

        match affected_rows {
            Ok(r) => Ok(Some(r)),
            Err(e) => Err(AppError::from(e))
        }
    }
}
