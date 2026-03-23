use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use crate::models::user::User;
use crate::models::Claims;

pub const JWT_SECRET: &str = "your_secret_key";
pub const ACCESS_EXPIRES_MIN: i64 = 15; // Access token expires in 15 minutes

pub fn create_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(ACCESS_EXPIRES_MIN)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}