pub mod product;
pub use product::{CreateProduct, Product, UpdateProduct};
pub mod user;
pub use user::{LoginRequest, RegisterRequest, LoginResponse, ErrorResponse, UpdateUserRequest, User};
