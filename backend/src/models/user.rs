use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct User {
    pub id: String,
    #[validate(length(min = 4, max = 30, message = "Username must be between 4 and 30 characters"))]
    pub username: String,
    #[validate(range(min = 0, message = "Age must be a positive number"))]
    pub age: u32,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[serde(rename = "hashedPassword")]
    pub hashed_password: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub age: u32,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub age: Option<u32>,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub login: bool,
    pub access_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse { pub error: String,}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub age: u32,
    pub email: String,
}