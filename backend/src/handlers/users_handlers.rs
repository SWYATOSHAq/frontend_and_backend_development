use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::state::AppState;
use crate::models::{LoginRequest, RegisterRequest, LoginResponse, ErrorResponse, UpdateUserRequest, User};
use crate::utils::hash_fn::{hash_password, verify_password};
use crate::utils::jwt_fn::{create_access_token, create_refresh_token};
use validator::Validate;

//GET /api/users -список всех пользователей-
#[utoipa::path
    (get, path ="/api/users",
    responses(
        (status = 200, description ="Список пользователей", body = Vec<User>)
    ),
    tag ="Users"
)]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(users.clone())
}

//POST /api/users/register -регистрация нового пользователя-
#[utoipa::path(
    post, path ="/api/users/register",
    request_body = RegisterRequest,
    responses(
    (status = 201, description ="Пользователь создан", body = User),
    (status = 400, description = "Некорректные данные", body = ErrorResponse)
),
    tag ="Users"
)]
pub async fn register_user(
    user: web::Json<RegisterRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(e) = user.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        });
    }
    if user.username.is_empty() || user.password.is_empty() || user.age == 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Некорректные данные".to_string(),
        });
    }
    {
        let users = data.users.lock().unwrap();
        if users.iter().any(|u| u.username == user.username) {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Пользователь с таким именем уже существует".to_string(),
            });
        }
    }
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: user.username.trim().to_string(),
        age: user.age,
        email: user.email.trim().to_string(),
        hashed_password: match hash_password(&user.password) {
            Ok(hash) => hash,
            Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Ошибка при хешировании пароля".to_string(),
            }),
        }
    };
    data.users.lock().unwrap().push(new_user.clone());
    HttpResponse::Created().json(new_user)
}

//POST /api/users/login -аутентификация пользователя-
#[utoipa::path(
    post, path ="/api/users/login",
    request_body = LoginRequest,
    responses(
    (status = 200, description ="Успешный вход", body = LoginResponse),
    (status = 400, description = "Некорректные данные", body = ErrorResponse),
    (status = 401, description = "Неверные имя пользователя или пароль", body = ErrorResponse),
    (status = 404, description = "Пользователь не найден", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn login_user(
    credentials: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    if let Err(e) = credentials.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        });
    }
    if credentials.username.is_empty() || credentials.password.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Некорректные данные".to_string(),
        });
    }
    let users = data.users.lock().unwrap();
    match users.iter().find(|u| u.username == credentials.username) {
        Some(user) => match verify_password(&credentials.password, &user.hashed_password) {
            Ok(true) => {
                let access_token = match create_access_token(user) {
                    Ok(token) => token,
                    Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Ошибка при создании токена".to_string(),
                    }),
                };
                let refresh_token = match create_refresh_token(user) {
                    Ok(token) => token,
                    Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Ошибка при создании токена".to_string(),
                    }),
                };
                HttpResponse::Ok().json(LoginResponse {
                    access_token,
                    refresh_token,
                })
            }
            Ok(false) => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Неверные имя пользователя или пароль".to_string(),
            }),
            Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Ошибка при проверке пароля".to_string(),
            }),
        },
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Пользователь не найден".to_string(),
        }),
    }

}

//GET /api/users/{id} -получение данных пользователя по ID-
#[utoipa::path
    (get, path = "/api/users/{id}", 
    params(("id" = String, Path, description = "ID пользователя")), 
    responses(
    (status = 200, description = "Данные пользователя", body = User),
    (status = 404, description = "Пользователь не найден", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn get_user_by_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let users = data.users.lock().unwrap();
    match users.iter().find(|u| u.id == id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Пользователь не найден".to_string(),
        }),
    }
}

//PATCH /api/users/{id} -обновление данных пользователя по ID-
#[utoipa::path(
    patch, path ="/api/users/{id}",
    params(("id" = String, Path, description = "ID пользователя")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Обновлённый пользователь", body = User),
        (status = 400, description = "Нет данных для обновления", body = ErrorResponse),
        (status = 404, description = "Пользователь не найден", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> impl Responder {
    if body.username.is_none() && body.age.is_none() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Нет данных для обновления".into(),
        });
    }
    let id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    match users.iter_mut().find(|u| u.id == id) {
        Some(user) => {
            if let Some(username) = &body.username {
                user.username = username.trim().to_string();
            }
            if let Some(age) = body.age {
                user.age = age;
            }
            HttpResponse::Ok().json(user.clone())
        }
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "Пользователь не найден".into(),
        }),
    }
}

//DELETE /api/users/{id} -удаление пользователя по ID-
#[utoipa::path(
    delete,path = "/api/users/{id}",
    params(("id" = String, Path, description = "ID пользователя")),
    responses(
        (status = 204, description = "Пользователь удалён"),
        (status = 404, description = "Пользователь не найден", body = ErrorResponse)
    ),
    tag = "Users"
)]
pub async fn delete_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    let len_before = users.len();
    users.retain(|u| u.id != id);
    if users.len() < len_before {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(ErrorResponse {
            error: "Пользователь не найден".into(),
        })
    }
}

