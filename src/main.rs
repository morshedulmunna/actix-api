mod actix_backend;

use actix_backend::app::run;
use actix_backend::config::establish_db_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish DB connection
    let pool = establish_db_connection().await;

    // Run the app
    run(pool).await
}
