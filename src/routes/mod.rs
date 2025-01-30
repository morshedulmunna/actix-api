use actix_web::web;

mod root;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("").configure(root::init));
}
