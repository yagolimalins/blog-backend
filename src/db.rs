use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn init_db_pool() -> PgPool {
    let database = env::var("DATABASE").expect("DATABASE_URL must be set in .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await
        .expect("Failed to create Postgres connection pool")
}
