use std::env;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;


pub async fn db_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");
    pool
}

pub async fn db_migrate(pool: &PgPool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Migration failed.");
}