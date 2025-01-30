use actix_backend::controllers::auth_controller::{login, register};
use actix_web::{web, HttpResponse, Responder, ServiceConfig};

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/auth/register").route(web::post().to(register)));
    cfg.service(web::resource("/auth/login").route(web::post().to(login)));
}
