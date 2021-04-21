use chrono::Utc;
use color_eyre::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use northwind_domain::authn::{models::auth::Claims, services::jwt_processor::JwtProcessor};

pub struct Jwt {}

impl JwtProcessor for Jwt {
    fn generate(
        user_id: uuid::Uuid,
        user_lastname: String,
        user_firstname: String,
        user_email: String,
        secret_key: String,
        jwt_lifetime: i64,
    ) -> Result<(String, i64), Box<dyn std::error::Error>> {
        let header = Header::new(Algorithm::HS512);
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let expired_at = now + (jwt_lifetime * 3600);

        let payload = Claims {
            sub: user_id.clone().to_string(),
            exp: expired_at,
            iat: now,
            nbf: now,
            user_id,
            user_lastname,
            user_firstname,
            user_email,
        };

        let token = encode(&header, &payload, &EncodingKey::from_secret(secret_key.as_bytes()))?;

        Ok((token, expired_at))
    }

    fn parse(token: String, secret_key: String) -> Result<Claims, Box<dyn std::error::Error>> {
        let validation = Validation::new(Algorithm::HS512);
        let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret_key.as_bytes()), &validation)?;

        Ok(token.claims)
    }
}
