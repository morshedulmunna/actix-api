use actix_backend::controllers::user_controller::{get_user_details, update_user_details};
use actix_web::{web, HttpResponse, Responder};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{id}")
            .route(web::get().to(get_user_details))
            .route(web::put().to(update_user_details)),
    );
}
