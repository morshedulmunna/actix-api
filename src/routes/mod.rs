pub mod auth;
pub mod user;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    auth_route::auth_config(cfg);
    user_route::user_config(cfg);
}
