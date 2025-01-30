use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::io::Result;

pub mod config;
use crate::routes;

pub async fn run(pool: PgPool) -> Result<()> {
    println!("✅ Database connection established successfully!");

    let addr = "127.0.0.1:8080";
    println!("🚀 Server is running on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind(addr)?
    .run()
    .await
}
