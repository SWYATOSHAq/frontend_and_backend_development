use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub username: String, 
    pub iat: usize,  // время выдачи (as a timestamp)
    pub exp: usize,  // время истечения (as a timestamp)
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}
