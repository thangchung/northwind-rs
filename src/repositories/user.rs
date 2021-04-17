use crate::models::user::{Login, User, UpdateUserModel};
use chrono::Utc;
use futures::stream::BoxStream;
use sha2::{Digest, Sha512};
use sqlx::postgres::{PgDone, PgRow};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    /// Returns a User if credentials are right
    pub async fn login(pool: &PgPool, input: Login) -> Result<Option<User>, sqlx::Error> {
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
        .await?;

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
    pub async fn create(pool: &PgPool, user: &mut User) -> Result<PgDone, sqlx::Error> {
        user.password = format!("{:x}", Sha512::digest(&user.password.as_bytes()));

        sqlx::query!(
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
    }

    /// Returns all users not deleted
    pub fn get_all(pool: &PgPool) -> BoxStream<Result<Result<User, sqlx::Error>, sqlx::Error>> {
        sqlx::query(r#"SELECT * FROM users WHERE deleted_at IS NULL"#)
            .map(|row: PgRow| {
                Ok(User {
                    id: row.try_get(0)?,
                    lastname: row.try_get(1)?,
                    firstname: row.try_get(2)?,
                    email: row.try_get(3)?,
                    password: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                    deleted_at: row.try_get(7)?,
                })
            })
            .fetch(pool)
    }

    /// Returns a user by its ID
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
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
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<PgDone, sqlx::Error> {
        sqlx::query!(
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
    }

    /// Update a user
    pub async fn update(pool: &PgPool, id: Uuid, user: &UpdateUserModel) -> Result<PgDone, sqlx::Error> {
        sqlx::query!(
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
    }
}
