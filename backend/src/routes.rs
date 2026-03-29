use actix_files::Files;
use actix_web::web;
use crate::handlers::{products_handlers as products, users_handlers as users, upload, auth_handlers as auth};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Products
        .route("/api/products", web::get().to(products::get_products))
        .route("/api/products", web::post().to(products::create_product))
        .route("/api/products/{id}", web::get().to(products::get_product))
        .route("/api/products/{id}", web::put().to(products::update_product))
        .route("/api/products/{id}", web::delete().to(products::delete_product))
        // Users
        .route("/api/users", web::get().to(users::get_users))
        .route("/api/users/{id}", web::get().to(users::get_user_by_id))
        .route("/api/users/{id}", web::patch().to(users::update_user))
        .route("/api/users/{id}", web::delete().to(users::delete_user))
        // Auth
        .route("/api/auth/register", web::post().to(users::register_user))
        .route("/api/auth/login", web::post().to(users::login_user))
        .route("/api/auth/me", web::get().to(auth::get_me))
        .route("/api/auth/refresh", web::post().to(auth::refresh_token))
        // Upload
        .route("/upload", web::post().to(upload::upload_image))
        .service(Files::new("/uploads", "uploads"));
}
