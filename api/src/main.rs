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


    let pool = db_pool().await;
    db_migrate(&pool).await;

    let server = HttpServer::new(move || {
        create_app::create_app(pool.clone())
    })
        .bind(addr);

    println!("Playground: http://{}", addr);
    // サーバーを起動

    server.expect("server cannot run ").run().await?;
    Ok(())

}
