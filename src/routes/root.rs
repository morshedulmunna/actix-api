use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(root));
    cfg.route("/health", web::get().to(health));
}

async fn root() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Welcome to the Actix Web API"
    }))
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "OK",
        "database": "Connected"
    }))
}
