use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::state::AppState;
use crate::models::{CreateUserRequest, ErrorResponse, UpdateUserRequest, User};

//GET /api/users -список всех пользователей-
#[utoipa::path
    (get, path ="/api/users",
    responses((status = 200, description ="Список пользователей", body = Vec<User>)),
    tag ="Users")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(users.clone())
}

//POST /api/users -создание нового пользователя-
#[utoipa::path(
    post, path ="/api/users",
    request_body = CreateUserRequest,
    responses(
    (status = 201, description ="Пользователь успешно создан", body = User),
    (status = 400, description = "Ошибка в теле запроса", body = ErrorResponse)),
    tag ="Users"
)]
pub async fn create_user(
    user: web::Json<CreateUserRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        name: user.name.clone(),
        age: user.age,
    };

    users.push(new_user.clone());
    HttpResponse::Created().json(new_user)
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
    if body.name.is_none() && body.age.is_none() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Nothing to update".into(),
        });
    }
    let id = path.into_inner();
    let mut users = data.users.lock().unwrap();
    match users.iter_mut().find(|u| u.id == id) {
        Some(user) => {
            if let Some(name) = &body.name {
                user.name = name.trim().to_string();
            }
            if let Some(age) = body.age {
                user.age = age;
            }
            HttpResponse::Ok().json(user.clone())
        }
        None => HttpResponse::NotFound().json(ErrorResponse {
            error: "User not found".into(),
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

