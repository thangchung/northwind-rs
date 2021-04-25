use async_trait::async_trait;
use crate::domain::user::User;
use northwind_core::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync  {
    async fn login(&self, email: String, password: String) -> Result<Option<User>, AppError>;
    async fn create(&self, user: &mut User) -> Result<Option<u64>, AppError>;
    async fn get_all(&self) -> Result<Vec<User>, AppError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<Option<u64>, AppError>;
    async fn update(&self, id: Uuid, firstname: String, lastname: String) -> Result<Option<u64>, AppError>;
}