use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod handlers;
mod models;
mod routes;
mod state;

use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tokio::fs::create_dir_all("uploads").await.ok();

    let state = web::Data::new(AppState {
        products: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec!["Content-Type"]),
            )
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
