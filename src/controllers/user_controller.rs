use actix_backend::models::user::User;
use actix_backend::services::user_service::{get_user_service, update_user_service};
use actix_web::{web, HttpResponse};

pub async fn get_user_details(user_id: web::Path<i32>) -> HttpResponse {
    match get_user_service(user_id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

pub async fn update_user_details(user_id: web::Path<i32>, user: web::Json<User>) -> HttpResponse {
    match update_user_service(user_id.into_inner(), user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
