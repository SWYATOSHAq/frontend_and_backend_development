use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod models;
mod routes;
mod state;

use state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::users_handlers::get_users,
        handlers::users_handlers::create_user,
        handlers::users_handlers::get_user_by_id,
        handlers::users_handlers::update_user,
        handlers::users_handlers::delete_user,
    ),
    components(schemas(
        models::user::User,
        models::user::CreateUserRequest,
        models::user::UpdateUserRequest,
        models::user::ErrorResponse,
    )),
    tags((name = "Users", description = "User management"))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tokio::fs::create_dir_all("uploads").await.ok();

    let state = web::Data::new(AppState {
        products: Mutex::new(Vec::new()),
        users: Mutex::new(Vec::new()),
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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi())
            )
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
