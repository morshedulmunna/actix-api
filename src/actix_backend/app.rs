use actix_cors::Cors;
// use actix_limitation::Limiter;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;

pub async fn run(pool: PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::permissive(); // Allow all origins (configure properly for production)

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            // .wrap(limiter)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
