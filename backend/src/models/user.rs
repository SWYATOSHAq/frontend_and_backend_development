use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub age: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}