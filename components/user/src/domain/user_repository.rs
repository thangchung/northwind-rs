use async_trait::async_trait;
use crate::domain::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync  {
    async fn login(email: String, password: String) -> Result<Option<User>, Box<dyn std::error::Error>>;
}