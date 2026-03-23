use actix_web::{web, HttpResponse, Responder, HttpRequest};
use crate::utils::jwt_fn::verify_token;
use crate::models::{Claims, ErrorResponse};
use crate::state::AppState;

pub fn extract_claims(request: &HttpRequest) -> Result<Claims, HttpResponse> {
    let headers = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let parts: Vec<&str> = headers.split(' ').collect();

    if parts.len() != 2 || parts[0] != "Bearer" {
        return Err(HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Недопустимый токен".to_string(),
        }));
    }
    verify_token(parts[1]).map_err(|_| {
        HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Недопустимый токен".into(),
        })
    })
}

pub async fn get_me(request: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    match extract_claims(&request) {
        Ok(claims) => {
            let users = data.users.lock().unwrap();
            match users.iter().find(|u| u.id == claims.sub) {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().json(ErrorResponse {
                    error: "Пользователь не найден".into(),
                }),
            }
        }
        Err(resp) => resp,
    }
}