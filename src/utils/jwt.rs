use jsonwebtoken::{
    encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm
};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn create_jwt(user: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET missing");

    let claims = Claims {
        sub: user,
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT SECRET IS MISSING");

    let validation = Validation::new(Algorithm::HS256);
    
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(data.claims)
}