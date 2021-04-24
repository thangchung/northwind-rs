use async_trait::async_trait;

use crate::authn::models::user::{User};

#[async_trait]
pub trait UserRepository {
    async fn login(email: String, password: String) -> Result<Option<User>, Box<dyn std::error::Error>>;
}