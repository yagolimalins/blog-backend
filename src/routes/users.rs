use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::user::{get_user, get_users};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_users))
        .route("/{username}", get(get_user))
}
