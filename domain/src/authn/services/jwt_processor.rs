use uuid::Uuid;
use async_trait::async_trait;

use crate::authn::models::auth::Claims;

#[async_trait]
pub trait JwtProcessor: Send + Sync {
    fn generate(
        &self,
        user_id: Uuid,
        user_lastname: String,
        user_firstname: String,
        user_email: String,
        secret_key: String,
        jwt_lifetime: i64,
    ) -> Result<(String, i64), Box<dyn std::error::Error>>;

    fn parse(&self, token: String, secret_key: String) -> Result<Claims, Box<dyn std::error::Error>>;
}
