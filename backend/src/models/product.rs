use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub category: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub category: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub category: String,
    pub description: String,
    pub image_url: Option<String>,
}


