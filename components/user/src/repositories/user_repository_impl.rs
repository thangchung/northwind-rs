use chrono::Utc;
use sha2::{Digest, Sha512};
use sqlx::{PgPool};
use uuid::Uuid;
use async_trait::async_trait;
use crate::domain::user::{User};
use northwind_core::errors::AppError;
use crate::domain::user_repository::UserRepository;
use std::sync::Arc;

pub struct UserRepositoryImpl {
    pub pool: Arc<PgPool>,
}

impl UserRepositoryImpl {
    pub fn new(&self, pool: Arc<PgPool>) -> Self {
        UserRepositoryImpl{
            pool
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    /// Returns a User if credentials are right
    async fn login(&self, email: String, password: String) -> Result<Option<User>, AppError> {
        let hashed_password = format!("{:x}", Sha512::digest(&password.as_bytes()));
        let result = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE email = $1 AND
                    password = $2 AND
                    deleted_at IS NULL
            "#,
            email,
            hashed_password
        )
        .fetch_optional(self.pool.as_ref())
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
    async fn create(&self, user: &mut User) -> Result<Option<u64>, AppError> {
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
        .execute(self.pool.as_ref())
        .await
        .map(|r| r.rows_affected())
        .map_err(|e| -> AppError { e.into() });

        match affected_rows {
            Ok(r) => Ok(Some(r)),
            Err(e) => Err(AppError::from(e))
        }
    }

    /// Returns all users not deleted
    async fn get_all(&self) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(User, r#"SELECT * FROM users WHERE deleted_at IS NULL"#)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| -> AppError { e.into() })
    }

    /// Returns a user by its ID
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let result = sqlx::query!(
            r#"
                SELECT * 
                FROM users 
                WHERE id = $1
                    AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(self.pool.as_ref())
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
    async fn delete(&self, id: Uuid) -> Result<Option<u64>, AppError> {
        let affected_rows = sqlx::query!(
            r#"
                UPDATE users
                SET deleted_at = $1
                WHERE id = $2
            "#,
            Utc::now().naive_utc(),
            id
        )
        .execute(self.pool.as_ref())
        .await
        .map(|r| r.rows_affected())
        .map_err(|e| -> AppError { e.into() });

        match affected_rows {
            Ok(r) => Ok(Some(r)),
            Err(e) => Err(AppError::from(e))
        }
    }

    /// Update a user
    async fn update(&self, id: Uuid, firstname: String, lastname: String) -> Result<Option<u64>, AppError> {
        let affected_rows = sqlx::query!(
            r#"
                UPDATE users
                SET lastname = $1, firstname = $2, updated_at = $3
                WHERE id = $4
            "#,
            lastname,
            firstname,
            Utc::now().naive_utc(),
            id
        )
        .execute(self.pool.as_ref())
        .await
        .map(|r| r.rows_affected())
        .map_err(|e| -> AppError { e.into() });

        match affected_rows {
            Ok(r) => Ok(Some(r)),
            Err(e) => Err(AppError::from(e))
        }
    }
}
