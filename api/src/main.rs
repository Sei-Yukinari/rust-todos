use std::env;
use std::net::SocketAddr;

use actix_web::HttpServer;
use sqlx::postgres::PgPoolOptions;

use api::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number")));
    println!("Playground: http://{}", addr);

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failed.");

    HttpServer::new(move || {
        create_app::create_app(pool.clone())
    })
        .bind(addr)?
        .run()
        .await
}
