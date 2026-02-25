use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
struct Product {
    id: String,
    name: String,
    price: f64,
}

struct AppState {
    products: Mutex<Vec<Product>>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("Главная страница")
}

async fn get_products(data: web::Data<AppState>) -> impl Responder {
    let products = data.products.lock().unwrap();
    HttpResponse::Ok().json(products.clone())
}

async fn get_product(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let products = data.products.lock().unwrap();

    if let Some(product) = products.iter().find(|p| p.id == id) {
        HttpResponse::Ok().json(product)
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

async fn create_product(product: web::Json<Product>, data: web::Data<AppState>) -> impl Responder {
    let mut products = data.products.lock().unwrap();

    let new_product = Product {
        id: Uuid::new_v4().to_string(),
        name: product.name.clone(),
        price: product.price,
    };
    products.push(new_product.clone());
    HttpResponse::Created().json(new_product)
}

async fn update_product(
    path: web::Path<String>,
    product: web::Json<Product>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut products = data.products.lock().unwrap();

    if let Some(p) = products.iter_mut().find(|p| p.id == id) {
        p.name = product.name.clone();
        p.price = product.price;
        HttpResponse::Ok().json(p)
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

async fn delete_product(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let mut products = data.products.lock().unwrap();

    products.retain(|p| p.id != id);
    HttpResponse::Ok().body("Deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        products: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .route("/products", web::get().to(get_products))
            .route("/products", web::post().to(create_product))
            .route("/products/{id}", web::get().to(get_product))
            .route("/products/{id}", web::patch().to(update_product))
            .route("/products/{id}", web::delete().to(delete_product))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
