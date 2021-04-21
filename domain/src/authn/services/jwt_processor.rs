use uuid::Uuid;

use crate::authn::models::auth::Claims;

pub trait JwtProcessor {
    fn generate(
        user_id: Uuid,
        user_lastname: String,
        user_firstname: String,
        user_email: String,
        secret_key: String,
        jwt_lifetime: i64,
    ) -> Result<(String, i64), Box<dyn std::error::Error>>;

    fn parse(token: String, secret_key: String) -> Result<Claims, Box<dyn std::error::Error>>;
}
