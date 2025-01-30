use actix_backend::models::user::User;
use actix_backend::services::auth_service::{login_service, register_service};
use actix_web::{web, HttpResponse};

pub async fn register(user: web::Json<User>) -> HttpResponse {
    match register_service(user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn login(user: web::Json<User>) -> HttpResponse {
    match login_service(user.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}
