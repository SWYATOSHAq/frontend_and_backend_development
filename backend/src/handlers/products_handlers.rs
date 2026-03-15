use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;
use crate::models::{CreateProduct, Product, UpdateProduct};
use crate::state::AppState;

/*pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("Главная страница")
}*/

pub async fn get_products(data: web::Data<AppState>) -> impl Responder {
    let products = data.products.lock().unwrap();
    HttpResponse::Ok().json(products.clone())
}

pub async fn get_product(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let products = data.products.lock().unwrap();

    if let Some(product) = products.iter().find(|p| p.id == id) {
        HttpResponse::Ok().json(product)
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

pub async fn create_product(
    product: web::Json<CreateProduct>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut products = data.products.lock().unwrap();

    let new_product = Product {
        id: Uuid::new_v4().to_string(),
        name: product.name.clone(),
        price: product.price,
        quantity: product.quantity,
        category: product.category.clone(),
        description: product.description.clone(),
        image_url: product.image_url.clone(),
    };

    products.push(new_product.clone());
    HttpResponse::Created().json(new_product)
}

pub async fn update_product(
    path: web::Path<String>,
    update: web::Json<UpdateProduct>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut products = data.products.lock().unwrap();

    if let Some(p) = products.iter_mut().find(|p| p.id == id) {
        p.name = update.name.clone();
        p.price = update.price;
        p.quantity = update.quantity;
        p.category = update.category.clone();
        p.description = update.description.clone();
        if update.image_url.is_some() {
            p.image_url = update.image_url.clone();
        }
        HttpResponse::Ok().json(p.clone())
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

pub async fn delete_product(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut products = data.products.lock().unwrap();

    products.retain(|p| p.id != id);
    HttpResponse::Ok().body("Deleted")
}
