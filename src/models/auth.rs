//! JWT module

use chrono::Utc;
use color_eyre::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

static ONE_MONTH: i64 = 60 * 60 * 24 * 30; // In seconds

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

pub struct JWT {}

impl JWT {
    // Generate JWT
    pub fn generate(
        user_id: String,
        user_lastname: String,
        user_firstname: String,
        user_email: String,
        secret_key: String,
    ) -> Result<(String, i64), Box<dyn std::error::Error>> {
        let header = Header::new(Algorithm::HS512);
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = Claims {
            sub: user_id.clone(),
            exp: now + ONE_MONTH,
            iat: now,
            nbf: now,
            user_id,
            user_lastname,
            user_firstname,
            user_email,
        };

        let token = encode(&header, &payload, &EncodingKey::from_secret(secret_key.as_bytes()))?;

        Ok((token, now))
    }

    // Parse JWT
    pub fn parse(token: String, secret_key: String) -> Result<Claims, Box<dyn std::error::Error>> {
        let validation = Validation::new(Algorithm::HS512);
        let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret_key.as_bytes()), &validation)?;

        Ok(token.claims)
    }
}
