use actix_files::Files;
use actix_web::web;
use crate::handlers::{products_handlers as products, users_handlers as users, upload};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Products
        .route("/products", web::get().to(products::get_products))
        .route("/products", web::post().to(products::create_product))
        .route("/products/{id}", web::get().to(products::get_product))
        .route("/products/{id}", web::patch().to(products::update_product))
        .route("/products/{id}", web::delete().to(products::delete_product))
        // Users
        .route("/api/users", web::get().to(users::get_users))
        .route("/api/users", web::post().to(users::create_user))
        .route("/api/users/{id}", web::get().to(users::get_user_by_id))
        .route("/api/users/{id}", web::patch().to(users::update_user))
        .route("/api/users/{id}", web::delete().to(users::delete_user))
        // Upload
        .route("/upload", web::post().to(upload::upload_image))
        .service(Files::new("/uploads", "uploads"));
}
