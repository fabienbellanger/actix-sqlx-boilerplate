//! Json Web Token module

use chrono::Utc;
use color_eyre::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub user_id: String,
    pub user_lastname: String,
    pub user_firstname: String,
    pub user_email: String,
}

pub struct Jwt {}

impl Jwt {
    // Generate JWT
    pub fn generate(
        user_id: String,
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
            sub: user_id.clone(),
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

    // Parse JWT
    pub fn parse(token: String, secret_key: String) -> Result<Claims, Box<dyn std::error::Error>> {
        let validation = Validation::new(Algorithm::HS512);
        let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret_key.as_bytes()), &validation)?;

        Ok(token.claims)
    }
}
