use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_web::cookie::{Cookie, SameSite};
use crate::utils::jwt_fn::{verify_access_token, verify_refresh_token, create_access_token, create_refresh_token};
use crate::models::{Claims, ErrorResponse, LoginResponse};
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
    verify_access_token(parts[1]).map_err(|_| {
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

pub async fn refresh_token(request: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let token_value = match request.cookie("refreshToken") {
        Some(cookie) => cookie.value().to_string(),
        None => return HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Refresh токен не найден".to_string(),
        }),
    };

    match verify_refresh_token(&token_value) {
        Ok(claims) => {
            let users = data.users.lock().unwrap();
            match users.iter().find(|u| u.id == claims.sub) {
                Some(user) => {
                    let access_token = match create_access_token(user) {
                        Ok(token) => token,
                        Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Ошибка при создании токена".to_string(),
                        }),
                    };
                    let new_refresh_token = match create_refresh_token(user) {
                        Ok(token) => token,
                        Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Ошибка при создании токена".to_string(),
                        }),
                    };
                    let cookie = Cookie::build("refreshToken", new_refresh_token)
                        .http_only(true)
                        .secure(false)
                        .same_site(SameSite::Lax)
                        .path("/")
                        .finish();
                    HttpResponse::Ok()
                        .cookie(cookie)
                        .json(LoginResponse { access_token })
                }
                None => HttpResponse::NotFound().json(ErrorResponse {
                    error: "Пользователь не найден".into(),
                }),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Недопустимый токен".into(),
        }),
    }
}



