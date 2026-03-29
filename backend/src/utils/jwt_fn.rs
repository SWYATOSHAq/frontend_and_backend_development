use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use crate::models::user::User;
use crate::models::{Claims, RefreshClaims};

pub const JWT_SECRET: &str = "your_secret_key";
pub const ACCESS_EXPIRES_MIN: i64 = 15;
pub const REFRESH_EXPIRES_DAYS: i64 = 7;

pub fn create_access_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        role: user.role.clone(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(ACCESS_EXPIRES_MIN)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

pub fn create_refresh_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = RefreshClaims {
        sub: user.id.clone(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::days(REFRESH_EXPIRES_DAYS)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

pub fn verify_access_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn verify_refresh_token(token: &str) -> Result<RefreshClaims, jsonwebtoken::errors::Error> {
    let token_data = decode::<RefreshClaims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}