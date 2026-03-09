use actix_files::Files;
use actix_web::web;
use crate::handlers::{products, upload};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(products::index))
        .route("/products", web::get().to(products::get_products))
        .route("/products", web::post().to(products::create_product))
        .route("/products/{id}", web::get().to(products::get_product))
        .route("/products/{id}", web::patch().to(products::update_product))
        .route("/products/{id}", web::delete().to(products::delete_product))
        .route("/upload", web::post().to(upload::upload_image))
        .service(Files::new("/uploads", "uploads"));
}
