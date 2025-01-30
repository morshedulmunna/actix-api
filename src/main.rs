mod app;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = app::config::establish_db_connection().await;
    app::run(pool).await
}
