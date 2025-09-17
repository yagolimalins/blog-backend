use crate::{config::Config, db::init_db_pool, utils::banner};
use axum::{Router, http::Method, routing::get};
use tower_http::cors::{Any, CorsLayer};

pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let pool = init_db_pool().await;

    let cors = CorsLayer::new()
        .allow_origin(config.origins)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { axum::http::StatusCode::OK }))
        .nest("/users", routes::users::routes())
        .with_state(pool)
        .layer(cors);

    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    banner(&address);

    axum::serve(listener, app).await.unwrap();
}
