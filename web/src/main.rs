use actix_web::{App, HttpServer};

use web::routes::{balances, ingest};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ingest).service(balances))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
