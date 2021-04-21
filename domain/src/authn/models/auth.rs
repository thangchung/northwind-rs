//! Json Web Token module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub user_id: Uuid,
    pub user_lastname: String,
    pub user_firstname: String,
    pub user_email: String,
}
