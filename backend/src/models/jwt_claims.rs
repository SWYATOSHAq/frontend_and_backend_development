use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshClaims {
    pub sub: String, // Subject (user ID)
    pub iat: usize,  // время выдачи (as a timestamp)
    pub exp: usize,  // время истечения (as a timestamp)
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}
