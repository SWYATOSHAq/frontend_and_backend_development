use std::sync::Mutex;
use crate::models::{Product, User};

pub struct AppState {
    pub products: Mutex<Vec<Product>>,
    pub users: Mutex<Vec<User>>,
}
