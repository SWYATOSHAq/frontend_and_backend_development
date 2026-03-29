use actix_web::{HttpResponse, HttpRequest, Responder, web};
use uuid::Uuid;
use crate::models::{CreateProduct, Product, UpdateProduct};
use crate::state::AppState;
use crate::handlers::auth_handlers::{extract_claims, require_role};


pub async fn get_products(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };
    if let Err(resp) = require_role(&claims, &["user", "seller", "admin"]) {
        return resp;
    }
    let products = data.products.lock().unwrap();
    HttpResponse::Ok().json(products.clone())
}

pub async fn get_product(req: HttpRequest, path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };
    if let Err(resp) = require_role(&claims, &["user", "seller", "admin"]) {
        return resp;
    }
    let id = path.into_inner();
    let products = data.products.lock().unwrap();

    if let Some(product) = products.iter().find(|p| p.id == id) {
        HttpResponse::Ok().json(product)
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

pub async fn create_product(
    req: HttpRequest,
    product: web::Json<CreateProduct>,
    data: web::Data<AppState>,
) -> impl Responder {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };
    if let Err(resp) = require_role(&claims, &["seller", "admin"]) {
        return resp;
    }
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
    req: HttpRequest,
    path: web::Path<String>,
    update: web::Json<UpdateProduct>,
    data: web::Data<AppState>,
) -> impl Responder {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };
    if let Err(resp) = require_role(&claims, &["seller", "admin"]) {
        return resp;
    }
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
    req: HttpRequest,
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };
    if let Err(resp) = require_role(&claims, &["admin"]) {
        return resp;
    }
    let id = path.into_inner();
    let mut products = data.products.lock().unwrap();

    products.retain(|p| p.id != id);
    HttpResponse::Ok().body("Deleted")
}
