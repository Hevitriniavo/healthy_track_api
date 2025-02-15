use jsonwebtoken::{encode, Header, EncodingKey, errors::Error, DecodingKey, decode, Validation, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::constants;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
}

pub fn new(user_email: String, user_id: i32, expiration_duration_hours: i64) -> Claims {
    let current_time = Utc::now().timestamp() as usize;
    Claims {
        exp: (Utc::now() + Duration::hours(expiration_duration_hours)).timestamp() as usize,
        iat: current_time,
        email: user_email,
        id: user_id,
    }
}

    pub fn generate_token(claims: Claims) -> Result<String, Error> {
        let secret_key = constants::CONFIG.jwt_secret.clone();
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
    }

    pub fn extract_claims_from_jwt(token: String) -> Result<TokenData<Claims>, Error> {
        let secret_key = constants::CONFIG.jwt_secret.clone();
        let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
        let claim_data = decode(&token, &decoding_key, &Validation::default())?;
        Ok(claim_data)
    }

