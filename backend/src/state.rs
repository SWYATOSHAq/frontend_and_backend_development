use std::sync::Mutex;
use crate::models::Product;

pub struct AppState {
    pub products: Mutex<Vec<Product>>,
}
