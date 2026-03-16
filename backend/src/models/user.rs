use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: String,
    pub username: String,
    pub age: u32,
    #[serde(rename = "hashedPassword")]
    pub hashed_password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub age: u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub age: Option<u32>,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse { pub login: bool, }

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse { pub error: String,}