use std::env;
use std::net::SocketAddr;

use actix_web::HttpServer;

use api::create_app;
use api::infrastructure::db::postgres::{db_migrate, db_pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number")));
    println!("Playground: http://{}", addr);

    let pool = db_pool().await;
    db_migrate(&pool).await;

    HttpServer::new(move || {
        create_app::create_app(pool.clone())
    })
        .bind(addr)?
        .run()
        .await
}
